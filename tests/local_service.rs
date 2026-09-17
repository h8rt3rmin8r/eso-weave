use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::time::{Duration, Instant};

use eso_weave::database_query::DatabaseQueryService;
use eso_weave::local_service::{
    valid_credential, DiscoveryRecord, LocalServiceConfig, LocalServiceController,
    LocalServicePrefs, ServiceFailureCode, ServicePhase, ServiceStatus, DISCOVERY_FILE_NAME,
};
use eso_weave::player_state::{bootstrap_content, observed, SnapshotPublisher};
use rmcp::model::{CallToolRequestParams, ErrorCode, ReadResourceRequestParams, ResourceContents};
use rmcp::transport::streamable_http_client::StreamableHttpClientTransportConfig;
use rmcp::transport::StreamableHttpClientTransport;
use rmcp::{ServiceError, ServiceExt};
use rusqlite::Connection;

const TEST_CREDENTIAL: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

const CAPABILITIES_RESOURCE_URI: &str = "esoweave://capabilities";
const PLAYER_STATE_RESOURCE_URI: &str = "esoweave://player-state";
const DATABASES_RESOURCE_URI: &str = "esoweave://databases";

#[test]
fn preferences_default_off_and_create_one_stable_credential() {
    let mut prefs = LocalServicePrefs::default();
    assert!(!prefs.enabled);
    assert_eq!(prefs.credential, None);

    let first = prefs.ensure_credential().unwrap().to_owned();
    let second = prefs.ensure_credential().unwrap().to_owned();
    assert_eq!(first, second);
    assert!(valid_credential(&first));
    assert_eq!(first.len(), 64);

    let stored = prefs.store();
    assert_eq!(LocalServicePrefs::load(&stored), prefs);
}

#[test]
fn malformed_preferences_fail_closed() {
    for value in [
        serde_json::json!("enabled"),
        serde_json::json!({"enabled": "yes"}),
        serde_json::json!({"credential": 42}),
    ] {
        assert_eq!(
            LocalServicePrefs::load(&value),
            LocalServicePrefs::default()
        );
    }
}

#[test]
fn one_authenticated_listener_serves_mcp_and_protects_every_route() {
    let dir = tempfile::tempdir().unwrap();
    let mut controller = controller(dir.path(), 0);
    assert!(controller.start(TEST_CREDENTIAL));
    let running = wait_for(&controller, ServicePhase::Running);
    let connection = running.connection.unwrap();
    let authority = connection
        .mcp_url
        .strip_prefix("http://")
        .unwrap()
        .strip_suffix("/mcp")
        .unwrap();

    let unauthenticated = request(authority, "GET", "/api/v1", &[], "");
    assert!(unauthenticated.starts_with("HTTP/1.1 401"));
    assert!(!unauthenticated
        .to_ascii_lowercase()
        .contains("access-control-allow-origin"));

    let wrong_host = request(
        authority,
        "GET",
        "/api/v1",
        &[
            ("Host", "localhost:1"),
            ("Authorization", &format!("Bearer {TEST_CREDENTIAL}")),
        ],
        "",
    );
    assert!(wrong_host.starts_with("HTTP/1.1 400"));

    let wrong_origin = request(
        authority,
        "GET",
        "/api/v1",
        &[
            ("Authorization", &format!("Bearer {TEST_CREDENTIAL}")),
            ("Origin", "https://example.com"),
        ],
        "",
    );
    assert!(wrong_origin.starts_with("HTTP/1.1 403"));

    let duplicate_auth = request(
        authority,
        "GET",
        "/api/v1",
        &[
            ("Authorization", &format!("Bearer {TEST_CREDENTIAL}")),
            ("Authorization", &format!("Bearer {TEST_CREDENTIAL}")),
        ],
        "",
    );
    assert!(duplicate_auth.starts_with("HTTP/1.1 401"));

    let oversized_body = "x".repeat(64 * 1024 + 1);
    let oversized = request(
        authority,
        "POST",
        "/mcp",
        &[
            ("Authorization", &format!("Bearer {TEST_CREDENTIAL}")),
            ("Content-Type", "application/json"),
        ],
        &oversized_body,
    );
    assert!(oversized.starts_with("HTTP/1.1 413"));

    let oversized_chunked = chunked_request(
        authority,
        "/api/v1",
        &[("Authorization", &format!("Bearer {TEST_CREDENTIAL}"))],
        &oversized_body,
    );
    assert!(oversized_chunked.starts_with("HTTP/1.1 413"));

    let api = request(
        authority,
        "GET",
        "/api/v1",
        &[("Authorization", &format!("Bearer {TEST_CREDENTIAL}"))],
        "",
    );
    assert!(api.starts_with("HTTP/1.1 200"));
    let capabilities: serde_json::Value = serde_json::from_str(response_body(&api)).unwrap();
    assert_eq!(capabilities["schema_version"], "1.1.0");
    assert_eq!(
        capabilities["http_operations"],
        serde_json::json!([
            "capabilities",
            "player_state",
            "databases",
            "query_database"
        ])
    );

    let state = request(
        authority,
        "GET",
        "/api/v1/player-state",
        &[("Authorization", &format!("Bearer {TEST_CREDENTIAL}"))],
        "",
    );
    assert!(state.starts_with("HTTP/1.1 200"), "{state}");
    let state: serde_json::Value = serde_json::from_str(response_body(&state)).unwrap();
    assert_eq!(state["service_generation"], connection.generation);
    assert_eq!(state["snapshot_revision"], 1);
    assert!(state["application"].is_object());

    let method = request(
        authority,
        "POST",
        "/api/v1/player-state",
        &[("Authorization", &format!("Bearer {TEST_CREDENTIAL}"))],
        "",
    );
    assert!(method.starts_with("HTTP/1.1 405"), "{method}");
    assert!(method.contains("method_not_allowed"));

    let missing = request(
        authority,
        "GET",
        "/api/v1/missing",
        &[("Authorization", &format!("Bearer {TEST_CREDENTIAL}"))],
        "",
    );
    assert!(missing.starts_with("HTTP/1.1 404"), "{missing}");

    let initialize = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2025-03-26",
            "capabilities": {},
            "clientInfo": {"name": "eso-weave-test", "version": "1"}
        }
    })
    .to_string();
    let mcp = request(
        authority,
        "POST",
        "/mcp",
        &[
            ("Authorization", &format!("Bearer {TEST_CREDENTIAL}")),
            ("Content-Type", "application/json"),
            ("Accept", "application/json, text/event-stream"),
        ],
        &initialize,
    );
    assert!(mcp.starts_with("HTTP/1.1 200"), "{mcp}");
    assert!(mcp.contains("protocolVersion"), "{mcp}");
    assert!(!mcp.contains(TEST_CREDENTIAL));

    controller.stop();
    wait_for(&controller, ServicePhase::Stopped);
    controller.shutdown().unwrap();
}

#[test]
fn mcp_client_discovers_reads_and_matches_http_state() {
    let dir = tempfile::tempdir().unwrap();
    let publisher = SnapshotPublisher::default();
    let mut content = bootstrap_content();
    content.game["runtime"] = observed(serde_json::json!("active"), "game_process");
    publisher.publish(content, time::OffsetDateTime::UNIX_EPOCH);
    let mut controller = LocalServiceController::new_with_publisher(
        LocalServiceConfig {
            config_dir: Some(dir.path().to_owned()),
            port: 0,
            shutdown_timeout: Duration::from_secs(3),
        },
        publisher.clone(),
    );
    assert!(controller.start(TEST_CREDENTIAL));
    let running = wait_for(&controller, ServicePhase::Running);
    let connection = running.connection.unwrap();
    let authority = connection
        .http_base_url
        .strip_prefix("http://")
        .unwrap()
        .strip_suffix("/api/v1")
        .unwrap();
    let http_capabilities: serde_json::Value = serde_json::from_str(response_body(&request(
        authority,
        "GET",
        "/api/v1/capabilities",
        &[("Authorization", &format!("Bearer {TEST_CREDENTIAL}"))],
        "",
    )))
    .unwrap();
    let http_state: serde_json::Value = serde_json::from_str(response_body(&request(
        authority,
        "GET",
        "/api/v1/player-state",
        &[("Authorization", &format!("Bearer {TEST_CREDENTIAL}"))],
        "",
    )))
    .unwrap();
    let http_databases: serde_json::Value = serde_json::from_str(response_body(&request(
        authority,
        "GET",
        "/api/v1/databases",
        &[("Authorization", &format!("Bearer {TEST_CREDENTIAL}"))],
        "",
    )))
    .unwrap();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let transport = StreamableHttpClientTransport::from_config(
            StreamableHttpClientTransportConfig::with_uri(connection.mcp_url.clone())
                .auth_header(TEST_CREDENTIAL),
        );
        let client = ().serve(transport).await.unwrap();
        let server = client.peer_info().expect("initialize server info");
        let resources_capability = server
            .capabilities
            .resources
            .as_ref()
            .expect("resources advertised");
        assert_eq!(resources_capability.subscribe, None);
        assert_eq!(resources_capability.list_changed, None);
        assert!(server.capabilities.tools.is_some());
        assert_eq!(server.capabilities.prompts, None);

        let resources = client.list_all_resources().await.unwrap();
        assert_eq!(resources.len(), 3);
        assert_resource(
            &resources[0],
            CAPABILITIES_RESOURCE_URI,
            "capabilities",
            "ESO Weave Capabilities",
        );
        assert_resource(
            &resources[1],
            PLAYER_STATE_RESOURCE_URI,
            "player-state",
            "ESO Weave Player State",
        );
        assert_resource(
            &resources[2],
            DATABASES_RESOURCE_URI,
            "databases",
            "ESO Weave Databases",
        );

        let mcp_capabilities = read_json_resource(&client, CAPABILITIES_RESOURCE_URI).await;
        let mcp_state = read_json_resource(&client, PLAYER_STATE_RESOURCE_URI).await;
        let mcp_databases = read_json_resource(&client, DATABASES_RESOURCE_URI).await;
        assert_eq!(mcp_capabilities, http_capabilities);
        assert_eq!(mcp_state, http_state);
        assert_eq!(mcp_databases, http_databases);
        assert_eq!(mcp_capabilities["mcp_player_state"], true);
        assert_eq!(mcp_capabilities["query_execution"], true);
        assert_eq!(mcp_state["snapshot_revision"], 2);
        assert_eq!(mcp_state["service_generation"], connection.generation);
        assert_eq!(mcp_state["game"]["runtime"]["value"], "active");

        let (reader_one, reader_two, reader_three) = tokio::join!(
            read_json_resource(&client, PLAYER_STATE_RESOURCE_URI),
            read_json_resource(&client, PLAYER_STATE_RESOURCE_URI),
            read_json_resource(&client, PLAYER_STATE_RESOURCE_URI),
        );
        assert_eq!(reader_one, mcp_state);
        assert_eq!(reader_two, mcp_state);
        assert_eq!(reader_three, mcp_state);

        let mut recovered = bootstrap_content();
        recovered.game["runtime"] = observed(serde_json::json!("recovered"), "game_process");
        publisher.publish(
            recovered,
            time::OffsetDateTime::UNIX_EPOCH + time::Duration::seconds(1),
        );
        let recovered_http: serde_json::Value = serde_json::from_str(response_body(&request(
            authority,
            "GET",
            "/api/v1/player-state",
            &[("Authorization", &format!("Bearer {TEST_CREDENTIAL}"))],
            "",
        )))
        .unwrap();
        let recovered_mcp = read_json_resource(&client, PLAYER_STATE_RESOURCE_URI).await;
        assert_eq!(recovered_mcp, recovered_http);
        assert_eq!(recovered_mcp["snapshot_revision"], 3);
        assert_eq!(recovered_mcp["game"]["runtime"]["value"], "recovered");

        let missing = client
            .read_resource(ReadResourceRequestParams::new(
                "esoweave://player-state?untrusted=secret",
            ))
            .await
            .expect_err("unknown resource should fail");
        match missing {
            ServiceError::McpError(error) => {
                assert_eq!(error.code, ErrorCode::RESOURCE_NOT_FOUND);
                assert!(!error.message.contains("untrusted"));
                assert!(!error.message.contains("secret"));
            }
            other => panic!("expected MCP error, got {other:?}"),
        }
        client.cancel().await.unwrap();
    });

    controller.shutdown().unwrap();
}

#[test]
fn repeated_mcp_client_completion_cannot_race_bounded_shutdown() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    for iteration in 0..12 {
        let dir = tempfile::tempdir().unwrap();
        let mut controller = LocalServiceController::new(LocalServiceConfig {
            config_dir: Some(dir.path().to_owned()),
            port: 0,
            shutdown_timeout: Duration::from_millis(500),
        });
        assert!(controller.start(TEST_CREDENTIAL));
        let connection = wait_for(&controller, ServicePhase::Running)
            .connection
            .unwrap();
        runtime.block_on(async {
            let transport = StreamableHttpClientTransport::from_config(
                StreamableHttpClientTransportConfig::with_uri(connection.mcp_url)
                    .auth_header(TEST_CREDENTIAL),
            );
            let client = ().serve(transport).await.unwrap();
            client.cancel().await.unwrap();
        });

        controller
            .shutdown()
            .unwrap_or_else(|failure| panic!("shutdown iteration {iteration} failed: {failure:?}"));
        assert!(!dir.path().join(DISCOVERY_FILE_NAME).exists());
    }
}

#[test]
fn mcp_client_without_bearer_cannot_initialize() {
    let dir = tempfile::tempdir().unwrap();
    let mut controller = controller(dir.path(), 0);
    assert!(controller.start(TEST_CREDENTIAL));
    let connection = wait_for(&controller, ServicePhase::Running)
        .connection
        .unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let transport = StreamableHttpClientTransport::from_config(
            StreamableHttpClientTransportConfig::with_uri(connection.mcp_url),
        );
        assert!(().serve(transport).await.is_err());
    });
    controller.shutdown().unwrap();
}

#[test]
fn http_and_mcp_database_inventory_queries_and_errors_match() {
    let dir = tempfile::tempdir().unwrap();
    let catalog = dir.path().join("catalog.sqlite");
    let connection = Connection::open(&catalog).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE entity (id INTEGER PRIMARY KEY, kind TEXT NOT NULL);
             INSERT INTO entity VALUES (1, 'skill'), (2, 'item'), (3, 'skill');",
        )
        .unwrap();
    drop(connection);
    let encounters = dir.path().join("encounters.sqlite");
    let connection = Connection::open(&encounters).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE encounter (id INTEGER PRIMARY KEY, outcome TEXT NOT NULL);
             INSERT INTO encounter VALUES (7, 'complete'), (8, 'partial');",
        )
        .unwrap();
    drop(connection);
    let queries = DatabaseQueryService::new(catalog, Some(encounters));
    let mut controller = LocalServiceController::new_with_services(
        LocalServiceConfig {
            config_dir: Some(dir.path().to_owned()),
            port: 0,
            shutdown_timeout: Duration::from_millis(500),
        },
        SnapshotPublisher::default(),
        queries,
    );
    assert!(controller.start(TEST_CREDENTIAL));
    let connection = wait_for(&controller, ServicePhase::Running)
        .connection
        .unwrap();
    let authority = connection
        .http_base_url
        .strip_prefix("http://")
        .unwrap()
        .strip_suffix("/api/v1")
        .unwrap();
    let auth = format!("Bearer {TEST_CREDENTIAL}");
    let http_inventory = request(
        authority,
        "GET",
        "/api/v1/databases",
        &[("Authorization", &auth)],
        "",
    );
    assert!(
        http_inventory.starts_with("HTTP/1.1 200"),
        "{http_inventory}"
    );
    let http_inventory: serde_json::Value =
        serde_json::from_str(response_body(&http_inventory)).unwrap();
    assert_eq!(http_inventory["databases"][0]["available"], true);
    assert_eq!(http_inventory["databases"][1]["available"], true);

    let query = serde_json::json!({
        "sql": "SELECT id, kind FROM entity WHERE kind = ?1 ORDER BY id",
        "parameters": [{"type": "text", "value": "skill"}],
        "row_limit": 10
    });
    let http_query = request(
        authority,
        "POST",
        "/api/v1/databases/catalog/query",
        &[
            ("Authorization", &auth),
            ("Content-Type", "application/json"),
        ],
        &query.to_string(),
    );
    assert!(http_query.starts_with("HTTP/1.1 200"), "{http_query}");
    let mut http_query: serde_json::Value =
        serde_json::from_str(response_body(&http_query)).unwrap();
    let encounter_query = serde_json::json!({
        "sql": "SELECT id, outcome FROM encounter WHERE id = :id",
        "parameters": [{"name": ":id", "type": "integer", "value": "7"}],
        "row_limit": 1
    });
    let http_encounter_query = request(
        authority,
        "POST",
        "/api/v1/databases/encounters/query",
        &[
            ("Authorization", &auth),
            ("Content-Type", "application/json"),
        ],
        &encounter_query.to_string(),
    );
    assert!(
        http_encounter_query.starts_with("HTTP/1.1 200"),
        "{http_encounter_query}"
    );
    let mut http_encounter_query: serde_json::Value =
        serde_json::from_str(response_body(&http_encounter_query)).unwrap();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let transport = StreamableHttpClientTransport::from_config(
            StreamableHttpClientTransportConfig::with_uri(connection.mcp_url.clone())
                .auth_header(TEST_CREDENTIAL),
        );
        let client = ().serve(transport).await.unwrap();
        let inventory = read_json_resource(&client, DATABASES_RESOURCE_URI).await;
        assert_eq!(inventory, http_inventory);
        let tools = client.list_all_tools().await.unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "query_database");
        let annotations = tools[0].annotations.as_ref().unwrap();
        assert_eq!(annotations.read_only_hint, Some(true));
        assert_eq!(annotations.destructive_hint, Some(false));
        let parameter_variants = tools[0].input_schema["properties"]["parameters"]["items"]
            ["oneOf"]
            .as_array()
            .unwrap();
        assert_eq!(parameter_variants.len(), 3);
        assert_eq!(parameter_variants[0]["properties"]["type"]["const"], "null");
        assert_eq!(
            parameter_variants[1]["properties"]["value"]["type"],
            "string"
        );
        assert_eq!(
            parameter_variants[2]["properties"]["value"]["type"],
            "boolean"
        );

        let mut arguments = query.as_object().unwrap().clone();
        arguments.insert("database_id".into(), serde_json::json!("catalog"));
        let mcp_query = client
            .call_tool(CallToolRequestParams::new("query_database").with_arguments(arguments))
            .await
            .unwrap();
        assert_eq!(mcp_query.is_error, Some(false));
        let mut mcp_query = mcp_query.structured_content.unwrap();
        http_query["elapsed_ms"] = serde_json::json!(0);
        mcp_query["elapsed_ms"] = serde_json::json!(0);
        assert_eq!(mcp_query, http_query);

        let mut encounter_arguments = encounter_query.as_object().unwrap().clone();
        encounter_arguments.insert("database_id".into(), serde_json::json!("encounters"));
        let mcp_encounter_query = client
            .call_tool(
                CallToolRequestParams::new("query_database").with_arguments(encounter_arguments),
            )
            .await
            .unwrap();
        assert_eq!(mcp_encounter_query.is_error, Some(false));
        let mut mcp_encounter_query = mcp_encounter_query.structured_content.unwrap();
        http_encounter_query["elapsed_ms"] = serde_json::json!(0);
        mcp_encounter_query["elapsed_ms"] = serde_json::json!(0);
        assert_eq!(mcp_encounter_query, http_encounter_query);

        let denied_arguments = serde_json::json!({
            "database_id": "catalog",
            "sql": "DELETE FROM entity",
            "parameters": []
        })
        .as_object()
        .unwrap()
        .clone();
        let mcp_denied = client
            .call_tool(
                CallToolRequestParams::new("query_database").with_arguments(denied_arguments),
            )
            .await
            .unwrap();
        assert_eq!(mcp_denied.is_error, Some(true));
        let mcp_denied = mcp_denied.structured_content.unwrap();
        let http_denied = request(
            authority,
            "POST",
            "/api/v1/databases/catalog/query",
            &[
                ("Authorization", &auth),
                ("Content-Type", "application/json"),
            ],
            r#"{"sql":"DELETE FROM entity","parameters":[]}"#,
        );
        assert!(http_denied.starts_with("HTTP/1.1 403"), "{http_denied}");
        let http_denied: serde_json::Value =
            serde_json::from_str(response_body(&http_denied)).unwrap();
        assert_eq!(mcp_denied, http_denied);
        client.cancel().await.unwrap();
    });
    controller.shutdown().unwrap();
}

#[test]
fn mcp_player_state_tracks_each_service_generation_after_restart() {
    let dir = tempfile::tempdir().unwrap();
    let mut controller = controller(dir.path(), 0);
    assert!(controller.start(TEST_CREDENTIAL));
    let first = wait_for(&controller, ServicePhase::Running)
        .connection
        .unwrap();
    let first_address = first
        .mcp_url
        .strip_prefix("http://")
        .unwrap()
        .strip_suffix("/mcp")
        .unwrap()
        .to_owned();
    let first_state = read_mcp_json_once(&first.mcp_url, PLAYER_STATE_RESOURCE_URI);
    assert_eq!(first_state["service_generation"], first.generation);
    let first_revision = first_state["snapshot_revision"].clone();

    controller.stop();
    wait_for(&controller, ServicePhase::Stopped);
    assert!(TcpStream::connect(&first_address).is_err());
    assert!(!dir.path().join(DISCOVERY_FILE_NAME).exists());
    assert!(controller.start(TEST_CREDENTIAL));
    let second = wait_for(&controller, ServicePhase::Running)
        .connection
        .unwrap();
    assert!(second.generation > first.generation);
    let second_state = read_mcp_json_once(&second.mcp_url, PLAYER_STATE_RESOURCE_URI);
    assert_eq!(second_state["service_generation"], second.generation);
    assert_eq!(second_state["snapshot_revision"], first_revision);

    controller.shutdown().unwrap();
}

#[test]
fn player_state_route_clones_the_latest_immutable_revision() {
    let dir = tempfile::tempdir().unwrap();
    let publisher = SnapshotPublisher::default();
    let mut content = bootstrap_content();
    content.game["runtime"] = observed(serde_json::json!("active"), "game_process");
    publisher.publish(content, time::OffsetDateTime::UNIX_EPOCH);
    let mut controller = LocalServiceController::new_with_publisher(
        LocalServiceConfig {
            config_dir: Some(dir.path().to_owned()),
            port: 0,
            shutdown_timeout: Duration::from_millis(200),
        },
        publisher,
    );
    controller.start(TEST_CREDENTIAL);
    let running = wait_for(&controller, ServicePhase::Running);
    let connection = running.connection.unwrap();
    let authority = connection
        .http_base_url
        .strip_prefix("http://")
        .unwrap()
        .strip_suffix("/api/v1")
        .unwrap();

    let response = request(
        authority,
        "GET",
        "/api/v1/player-state",
        &[("Authorization", &format!("Bearer {TEST_CREDENTIAL}"))],
        "",
    );
    let value: serde_json::Value = serde_json::from_str(response_body(&response)).unwrap();
    assert_eq!(value["snapshot_revision"], 2);
    assert_eq!(value["service_generation"], connection.generation);
    assert_eq!(value["game"]["runtime"]["value"], "active");

    controller.shutdown().unwrap();
}

#[test]
fn discovery_matches_running_status_and_owned_cleanup_preserves_foreign_record() {
    let dir = tempfile::tempdir().unwrap();
    let mut controller = controller(dir.path(), 0);
    controller.start(TEST_CREDENTIAL);
    let status = wait_for(&controller, ServicePhase::Running);
    let connection = status.connection.unwrap();
    let path = dir.path().join(DISCOVERY_FILE_NAME);
    let discovery: DiscoveryRecord =
        serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    assert_eq!(discovery.process_id, connection.process_id);
    assert_eq!(discovery.service_generation, connection.generation);
    assert_eq!(discovery.http_base_url, connection.http_base_url);
    assert_eq!(discovery.mcp_url, connection.mcp_url);
    assert!(!String::from_utf8(std::fs::read(&path).unwrap())
        .unwrap()
        .contains(TEST_CREDENTIAL));

    let foreign = DiscoveryRecord {
        process_id: connection.process_id.wrapping_add(1),
        service_generation: connection.generation + 10,
        ..discovery
    };
    std::fs::write(&path, serde_json::to_vec(&foreign).unwrap()).unwrap();
    controller.stop();
    wait_for(&controller, ServicePhase::Stopped);
    let retained: DiscoveryRecord = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    assert_eq!(retained, foreign);
    controller.shutdown().unwrap();
}

#[test]
fn collision_fails_atomically_and_reenable_recovers() {
    let occupied = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = occupied.local_addr().unwrap().port();
    let dir = tempfile::tempdir().unwrap();
    let mut controller = controller(dir.path(), port);
    controller.start(TEST_CREDENTIAL);
    let failed = wait_for(&controller, ServicePhase::Failed);
    assert_eq!(
        failed.failure.unwrap().code,
        ServiceFailureCode::AddressInUse
    );
    assert!(!dir.path().join(DISCOVERY_FILE_NAME).exists());

    controller.stop();
    wait_for(&controller, ServicePhase::Stopped);
    drop(occupied);
    controller.start(TEST_CREDENTIAL);
    let recovered = wait_for(&controller, ServicePhase::Running);
    assert!(recovered.connection.is_some());
    assert!(dir.path().join(DISCOVERY_FILE_NAME).exists());
    controller.stop();
    wait_for(&controller, ServicePhase::Stopped);
    assert!(!dir.path().join(DISCOVERY_FILE_NAME).exists());
    assert!(TcpListener::bind(("127.0.0.1", port)).is_ok());
    controller.shutdown().unwrap();
}

#[test]
fn invalid_credential_and_unavailable_settings_fail_before_running() {
    let mut invalid = LocalServiceController::new(LocalServiceConfig {
        config_dir: Some(tempfile::tempdir().unwrap().path().to_owned()),
        port: 0,
        shutdown_timeout: Duration::from_millis(200),
    });
    assert!(!invalid.start("not-a-credential"));
    assert_eq!(
        invalid.status().failure.unwrap().code,
        ServiceFailureCode::CredentialUnavailable
    );
    invalid.shutdown().unwrap();

    let mut unavailable = LocalServiceController::new(LocalServiceConfig {
        config_dir: None,
        port: 0,
        shutdown_timeout: Duration::from_millis(200),
    });
    assert!(unavailable.start(TEST_CREDENTIAL));
    let failed = wait_for(&unavailable, ServicePhase::Failed);
    assert_eq!(
        failed.failure.unwrap().code,
        ServiceFailureCode::SettingsUnavailable
    );
    unavailable.shutdown().unwrap();
}

#[test]
fn discovery_failure_releases_the_partially_started_listener() {
    let dir = tempfile::tempdir().unwrap();
    let blocked_path = dir.path().join("not-a-directory");
    std::fs::write(&blocked_path, b"occupied").unwrap();
    let mut controller = LocalServiceController::new(LocalServiceConfig {
        config_dir: Some(blocked_path),
        port: 0,
        shutdown_timeout: Duration::from_millis(200),
    });
    controller.start(TEST_CREDENTIAL);
    let failed = wait_for(&controller, ServicePhase::Failed);
    assert_eq!(
        failed.failure.unwrap().code,
        ServiceFailureCode::DiscoveryFailed
    );
    controller.shutdown().unwrap();
}

#[test]
fn rapid_toggle_converges_and_drop_releases_owned_resources() {
    let dir = tempfile::tempdir().unwrap();
    let mut controller = controller(dir.path(), 0);
    controller.start(TEST_CREDENTIAL);
    controller.stop();
    controller.start(TEST_CREDENTIAL);
    let running = wait_for(&controller, ServicePhase::Running);
    let address = running
        .connection
        .unwrap()
        .mcp_url
        .trim_start_matches("http://")
        .trim_end_matches("/mcp")
        .to_owned();
    assert!(dir.path().join(DISCOVERY_FILE_NAME).exists());

    controller.shutdown().unwrap();
    assert!(!dir.path().join(DISCOVERY_FILE_NAME).exists());
    assert!(TcpListener::bind(address).is_ok());
}

#[test]
fn shutdown_reserves_time_to_join_after_a_stalled_connection() {
    let dir = tempfile::tempdir().unwrap();
    let mut controller = controller(dir.path(), 0);
    controller.start(TEST_CREDENTIAL);
    let running = wait_for(&controller, ServicePhase::Running);
    let address = running
        .connection
        .unwrap()
        .mcp_url
        .trim_start_matches("http://")
        .trim_end_matches("/mcp")
        .to_owned();
    let mut stalled = TcpStream::connect(&address).unwrap();
    stalled.write_all(b"POST /mcp HTTP/1.1\r\n").unwrap();
    std::thread::sleep(Duration::from_millis(25));

    let started = Instant::now();
    controller.stop();
    controller.shutdown().unwrap();
    assert!(started.elapsed() < Duration::from_millis(400));
    assert!(!dir.path().join(DISCOVERY_FILE_NAME).exists());
    drop(stalled);
    assert!(TcpListener::bind(address).is_ok());
}

#[test]
fn stopped_generation_interrupts_an_in_flight_database_query() {
    let dir = tempfile::tempdir().unwrap();
    let catalog = dir.path().join("catalog.sqlite");
    drop(Connection::open(&catalog).unwrap());
    let queries = DatabaseQueryService::new(catalog, None);
    let mut controller = LocalServiceController::new_with_services(
        LocalServiceConfig {
            config_dir: Some(dir.path().to_owned()),
            port: 0,
            shutdown_timeout: Duration::from_millis(500),
        },
        SnapshotPublisher::default(),
        queries,
    );
    controller.start(TEST_CREDENTIAL);
    let running = wait_for(&controller, ServicePhase::Running);
    let address = running
        .connection
        .unwrap()
        .mcp_url
        .trim_start_matches("http://")
        .trim_end_matches("/mcp")
        .to_owned();
    let body = serde_json::json!({
        "sql": "WITH RECURSIVE counter(value) AS (VALUES(0) UNION ALL SELECT value + 1 FROM counter WHERE value < 1000000000) SELECT sum(value) FROM counter"
    })
    .to_string();
    let mut stream = TcpStream::connect(&address).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    stream
        .write_all(
            format!(
                "POST /api/v1/databases/catalog/query HTTP/1.1\r\nHost: {address}\r\nAuthorization: Bearer {TEST_CREDENTIAL}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            )
            .as_bytes(),
        )
        .unwrap();
    std::thread::sleep(Duration::from_millis(50));

    let started = Instant::now();
    controller.stop();
    wait_for(&controller, ServicePhase::Stopped);
    assert!(started.elapsed() < Duration::from_millis(500));
    let mut response = String::new();
    let read = stream.read_to_string(&mut response);
    assert!(
        read.is_ok()
            || read.as_ref().is_err_and(|error| matches!(
                error.kind(),
                std::io::ErrorKind::ConnectionAborted
                    | std::io::ErrorKind::ConnectionReset
                    | std::io::ErrorKind::UnexpectedEof
            )),
        "{read:?}"
    );
    if !response.is_empty() {
        assert!(response.starts_with("HTTP/1.1 503"), "{response}");
        assert!(response.contains("service_stopping"), "{response}");
    }
    controller.shutdown().unwrap();
}

#[test]
fn stopped_generation_cancels_a_stalled_authenticated_body_before_reenable() {
    let dir = tempfile::tempdir().unwrap();
    let mut controller = controller(dir.path(), 0);
    controller.start(TEST_CREDENTIAL);
    let running = wait_for(&controller, ServicePhase::Running);
    let address = running
        .connection
        .unwrap()
        .mcp_url
        .trim_start_matches("http://")
        .trim_end_matches("/mcp")
        .to_owned();
    let mut old_connection = TcpStream::connect(&address).unwrap();
    old_connection
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    old_connection
        .write_all(
            format!(
                "POST /api/v1 HTTP/1.1\r\nHost: {address}\r\nAuthorization: Bearer {TEST_CREDENTIAL}\r\nContent-Length: 1\r\nConnection: keep-alive\r\n\r\n"
            )
            .as_bytes(),
        )
        .unwrap();
    std::thread::sleep(Duration::from_millis(25));

    controller.stop();
    wait_for(&controller, ServicePhase::Stopped);
    controller.start(TEST_CREDENTIAL);
    wait_for(&controller, ServicePhase::Running);

    let _ = old_connection.write_all(b"x");
    let mut old_response = String::new();
    let read = old_connection.read_to_string(&mut old_response);
    assert!(
        read.is_ok()
            || read.as_ref().is_err_and(|error| matches!(
                error.kind(),
                std::io::ErrorKind::ConnectionAborted
                    | std::io::ErrorKind::ConnectionReset
                    | std::io::ErrorKind::UnexpectedEof
            )),
        "{read:?}"
    );
    if !old_response.is_empty() {
        assert!(old_response.starts_with("HTTP/1.1 503"), "{old_response}");
    }
    assert!(!old_response.contains("service_unavailable"));

    controller.stop();
    wait_for(&controller, ServicePhase::Stopped);
    controller.shutdown().unwrap();
}

fn controller(config_dir: &Path, port: u16) -> LocalServiceController {
    LocalServiceController::new(LocalServiceConfig {
        config_dir: Some(config_dir.to_owned()),
        port,
        // Ordinary lifecycle tests use the production bound. Tests that verify
        // a tighter shutdown contract provide their own explicit timeout.
        shutdown_timeout: Duration::from_secs(3),
    })
}

fn assert_resource(resource: &rmcp::model::Resource, uri: &str, name: &str, title: &str) {
    assert_eq!(resource.uri, uri);
    assert_eq!(resource.name, name);
    assert_eq!(resource.title.as_deref(), Some(title));
    assert_eq!(resource.mime_type.as_deref(), Some("application/json"));
    assert!(resource
        .description
        .as_deref()
        .is_some_and(|text| !text.is_empty()));
}

async fn read_json_resource(
    client: &rmcp::service::RunningService<rmcp::RoleClient, ()>,
    uri: &str,
) -> serde_json::Value {
    let result = client
        .read_resource(ReadResourceRequestParams::new(uri))
        .await
        .unwrap();
    assert_eq!(result.contents.len(), 1);
    match &result.contents[0] {
        ResourceContents::TextResourceContents {
            uri: actual_uri,
            mime_type,
            text,
            ..
        } => {
            assert_eq!(actual_uri, uri);
            assert_eq!(mime_type.as_deref(), Some("application/json"));
            serde_json::from_str(text).unwrap()
        }
        other => panic!("expected JSON text resource, got {other:?}"),
    }
}

fn read_mcp_json_once(mcp_url: &str, uri: &str) -> serde_json::Value {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let transport = StreamableHttpClientTransport::from_config(
            StreamableHttpClientTransportConfig::with_uri(mcp_url.to_owned())
                .auth_header(TEST_CREDENTIAL),
        );
        let client = ().serve(transport).await.unwrap();
        let value = read_json_resource(&client, uri).await;
        client.cancel().await.unwrap();
        value
    })
}

fn wait_for(controller: &LocalServiceController, phase: ServicePhase) -> ServiceStatus {
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let status = controller.status();
        if status.phase == phase {
            return status;
        }
        assert!(
            Instant::now() < deadline,
            "timed out waiting for {phase:?}: {status:?}"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn request(
    authority: &str,
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
    body: &str,
) -> String {
    let mut stream = TcpStream::connect(authority).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    let has_host = headers
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("host"));
    let mut text = format!("{method} {path} HTTP/1.1\r\n");
    if !has_host {
        text.push_str(&format!("Host: {authority}\r\n"));
    }
    for (name, value) in headers {
        text.push_str(&format!("{name}: {value}\r\n"));
    }
    text.push_str(&format!(
        "Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    ));
    stream.write_all(text.as_bytes()).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    response
}

fn response_body(response: &str) -> &str {
    response.split_once("\r\n\r\n").unwrap().1
}

fn chunked_request(authority: &str, path: &str, headers: &[(&str, &str)], body: &str) -> String {
    let mut stream = TcpStream::connect(authority).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    let mut text = format!("POST {path} HTTP/1.1\r\nHost: {authority}\r\n");
    for (name, value) in headers {
        text.push_str(&format!("{name}: {value}\r\n"));
    }
    text.push_str("Transfer-Encoding: chunked\r\nConnection: close\r\n\r\n");
    text.push_str(&format!("{:x}\r\n{body}\r\n0\r\n\r\n", body.len()));
    stream.write_all(text.as_bytes()).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    response
}
