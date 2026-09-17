//! Stateless MCP resource adapter for canonical local player state.

use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, Implementation, JsonObject,
    ListResourcesResult, ListToolsResult, PaginatedRequestParams, ReadResourceRequestParams,
    ReadResourceResponse, ReadResourceResult, Resource, ResourceContents, ServerCapabilities,
    ServerConfig, Tool, ToolAnnotations,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData as McpError, RoleServer, ServerHandler};
use serde::Deserialize;
use tokio_util::sync::CancellationToken;

use crate::database_query::{DatabaseQueryService, QueryError, QueryRequest};
use crate::player_state::SnapshotPublisher;

pub(crate) const CAPABILITIES_RESOURCE_URI: &str = "esoweave://capabilities";
pub(crate) const PLAYER_STATE_RESOURCE_URI: &str = "esoweave://player-state";
pub(crate) const DATABASES_RESOURCE_URI: &str = "esoweave://databases";
const QUERY_DATABASE_TOOL: &str = "query_database";
const JSON_MEDIA_TYPE: &str = "application/json";

#[derive(Clone)]
pub(crate) struct PlayerStateMcp {
    publisher: SnapshotPublisher,
    queries: DatabaseQueryService,
    cancellation: CancellationToken,
    generation: u64,
}

impl PlayerStateMcp {
    pub(crate) fn new(
        publisher: SnapshotPublisher,
        queries: DatabaseQueryService,
        cancellation: CancellationToken,
        generation: u64,
    ) -> Self {
        Self {
            publisher,
            queries,
            cancellation,
            generation,
        }
    }

    fn resources() -> Vec<Resource> {
        vec![
            Resource::new(CAPABILITIES_RESOURCE_URI, "capabilities")
                .with_title("ESO Weave Capabilities")
                .with_description(
                    "Supported schema, state domains, source protocols, and read operations.",
                )
                .with_mime_type(JSON_MEDIA_TYPE),
            Resource::new(PLAYER_STATE_RESOURCE_URI, "player-state")
                .with_title("ESO Weave Player State")
                .with_description(
                    "Latest complete canonical read-only player-state snapshot with knowledge and freshness metadata.",
                )
                .with_mime_type(JSON_MEDIA_TYPE),
            Resource::new(DATABASES_RESOURCE_URI, "databases")
                .with_title("ESO Weave Databases")
                .with_description(
                    "Current availability and safe public schema for the catalog and encounter databases.",
                )
                .with_mime_type(JSON_MEDIA_TYPE),
        ]
    }

    fn query_tool() -> Tool {
        let schema = serde_json::json!({
            "type": "object",
            "additionalProperties": false,
            "required": ["database_id", "sql"],
            "properties": {
                "database_id": {"type": "string", "enum": ["catalog", "encounters"]},
                "sql": {"type": "string", "minLength": 1, "maxLength": 16384},
                "parameters": {
                    "type": "array",
                    "maxItems": 64,
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string"},
                            "type": {
                                "type": "string",
                                "enum": ["null", "integer", "real", "text", "blob", "boolean"]
                            },
                            "value": {}
                        },
                        "required": ["type"],
                        "additionalProperties": false
                    }
                },
                "row_limit": {"type": "integer", "minimum": 1, "maximum": 1000}
            }
        });
        let input_schema: JsonObject = schema
            .as_object()
            .expect("query tool schema is an object")
            .clone();
        Tool::new(
            QUERY_DATABASE_TOOL,
            "Execute one bounded parameterized read-only statement against an ESO Weave database.",
            input_schema,
        )
        .with_title("Query ESO Weave Database")
        .with_annotations(
            ToolAnnotations::new()
                .read_only(true)
                .destructive(false)
                .idempotent(true)
                .open_world(false),
        )
    }

    fn json_content<T: serde::Serialize>(
        value: &T,
        uri: &'static str,
    ) -> Result<ReadResourceResponse, McpError> {
        let text = serde_json::to_string(value)
            .map_err(|_| McpError::internal_error("Resource serialization failed.", None))?;
        Ok(ReadResourceResult::new(vec![
            ResourceContents::text(text, uri).with_mime_type(JSON_MEDIA_TYPE)
        ])
        .into())
    }
}

impl ServerHandler for PlayerStateMcp {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(
            ServerCapabilities::builder()
                .enable_resources()
                .enable_tools()
                .build(),
        )
        .with_server_info(
            Implementation::new("eso-weave", crate::version())
                .with_title("ESO Weave Local Service")
                .with_description("Read-only local ESO Weave state and database access."),
        )
        .with_instructions(
            "Read capabilities and database inventory before consuming state or querying data.",
        )
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult::with_all_items(Self::resources()))
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResponse, McpError> {
        let snapshot = self.publisher.current();
        match request.uri.as_str() {
            CAPABILITIES_RESOURCE_URI => {
                Self::json_content(&snapshot.capabilities(), CAPABILITIES_RESOURCE_URI)
            }
            PLAYER_STATE_RESOURCE_URI => Self::json_content(
                &snapshot.document(self.generation),
                PLAYER_STATE_RESOURCE_URI,
            ),
            DATABASES_RESOURCE_URI => {
                let cancellation = self.cancellation.child_token();
                let _drop_guard = cancellation.clone().drop_guard();
                let inventory = self.queries.inventory(cancellation.clone());
                tokio::pin!(inventory);
                let inventory = tokio::select! {
                    result = &mut inventory => result,
                    _ = context.ct.cancelled() => {
                        cancellation.cancel();
                        inventory.await
                    }
                }
                .map_err(|_| {
                    McpError::internal_error("Database inventory is temporarily unavailable.", None)
                })?;
                Self::json_content(&inventory, DATABASES_RESOURCE_URI)
            }
            _ => Err(McpError::resource_not_found(
                "Resource URI is not available.",
                None,
            )),
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        Ok(ListToolsResult::with_all_items(vec![Self::query_tool()]))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, McpError> {
        if request.name != QUERY_DATABASE_TOOL {
            return Err(McpError::invalid_params("Tool is not available.", None));
        }
        let arguments = serde_json::Value::Object(request.arguments.unwrap_or_default());
        let request = match serde_json::from_value::<McpQueryRequest>(arguments) {
            Ok(request) => request,
            Err(_) => {
                return Ok(CallToolResult::structured_error(
                    QueryError::invalid_request().envelope(),
                )
                .into())
            }
        };
        let cancellation = self.cancellation.child_token();
        let _drop_guard = cancellation.clone().drop_guard();
        let query = self
            .queries
            .execute(&request.database_id, request.query, cancellation.clone());
        tokio::pin!(query);
        let result = tokio::select! {
            result = &mut query => result,
            _ = context.ct.cancelled() => {
                cancellation.cancel();
                query.await
            }
        };
        Ok(match result {
            Ok(result) => CallToolResult::structured(
                serde_json::to_value(result)
                    .map_err(|_| McpError::internal_error("Query serialization failed.", None))?,
            )
            .into(),
            Err(error) => CallToolResult::structured_error(error.envelope()).into(),
        })
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct McpQueryRequest {
    database_id: String,
    #[serde(flatten)]
    query: QueryRequest,
}
