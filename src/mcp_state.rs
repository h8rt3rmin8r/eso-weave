//! Stateless MCP resource adapter for canonical local player state.

use rmcp::model::{
    Implementation, ListResourcesResult, PaginatedRequestParams, ReadResourceRequestParams,
    ReadResourceResponse, ReadResourceResult, Resource, ResourceContents, ServerCapabilities,
    ServerConfig,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData as McpError, RoleServer, ServerHandler};

use crate::player_state::SnapshotPublisher;

pub(crate) const CAPABILITIES_RESOURCE_URI: &str = "esoweave://capabilities";
pub(crate) const PLAYER_STATE_RESOURCE_URI: &str = "esoweave://player-state";
const JSON_MEDIA_TYPE: &str = "application/json";

#[derive(Clone)]
pub(crate) struct PlayerStateMcp {
    publisher: SnapshotPublisher,
    generation: u64,
}

impl PlayerStateMcp {
    pub(crate) fn new(publisher: SnapshotPublisher, generation: u64) -> Self {
        Self {
            publisher,
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
        ]
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
        ServerConfig::new(ServerCapabilities::builder().enable_resources().build())
            .with_server_info(
                Implementation::new("eso-weave", crate::version())
                    .with_title("ESO Weave Local Service")
                    .with_description("Read-only local ESO Weave player-state resources."),
            )
            .with_instructions(
                "Read the capabilities resource before consuming the canonical player-state resource.",
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
        _context: RequestContext<RoleServer>,
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
            _ => Err(McpError::resource_not_found(
                "Resource URI is not available.",
                None,
            )),
        }
    }
}
