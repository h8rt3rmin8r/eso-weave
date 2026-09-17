use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::time::{Duration, Instant};

use eso_weave::local_service::{
    valid_credential, DiscoveryRecord, LocalServiceConfig, LocalServiceController,
    LocalServicePrefs, ServiceFailureCode, ServicePhase, ServiceStatus, DISCOVERY_FILE_NAME,
};
use eso_weave::player_state::{bootstrap_content, observed, SnapshotPublisher};

const TEST_CREDENTIAL: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

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
    assert_eq!(capabilities["schema_version"], "1.0.0");
    assert_eq!(
        capabilities["http_operations"],
        serde_json::json!(["capabilities", "player_state"])
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
    wait_for(&controller, ServicePhase::Running);
    controller.stop();
    wait_for(&controller, ServicePhase::Stopped);
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
        shutdown_timeout: Duration::from_millis(200),
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
