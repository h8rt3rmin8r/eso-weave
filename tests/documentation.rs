use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpStream};
use std::time::{Duration, Instant};

use eso_weave::documentation::{
    asset, content_type, BrowserOpener, DocumentationService, EMBEDDED_ASSETS, MOUNT_PATH,
};

fn request(address: SocketAddr, request: &[u8]) -> Vec<u8> {
    let mut stream = TcpStream::connect(address).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    stream.write_all(request).unwrap();
    let mut response = Vec::new();
    stream.read_to_end(&mut response).unwrap();
    response
}

fn response_text(address: SocketAddr, request_bytes: &[u8]) -> String {
    String::from_utf8(request(address, request_bytes)).unwrap()
}

#[test]
fn embedded_manifest_has_required_sorted_assets_and_types() {
    assert!(asset("index.html").is_some());
    assert!(EMBEDDED_ASSETS
        .windows(2)
        .all(|pair| pair[0].path < pair[1].path));
    if cfg!(debug_assertions) {
        assert!(asset("guide/index.html").is_some());
    } else {
        assert!(EMBEDDED_ASSETS
            .iter()
            .any(|entry| entry.path.starts_with("searchindex-") && entry.path.ends_with(".js")));
        assert!(asset("concepts/index.html").is_some());
    }
    assert_eq!(content_type("index.html"), "text/html; charset=utf-8");
    assert_eq!(content_type("theme.css"), "text/css; charset=utf-8");
    assert_eq!(content_type("app.js"), "text/javascript; charset=utf-8");
    assert_eq!(content_type("mark.svg"), "image/svg+xml");
    assert_eq!(content_type("font.woff2"), "font/woff2");
}

#[test]
fn service_binds_loopback_reuses_address_and_serves_get_and_head() {
    let mut service = DocumentationService::new();
    let first = service.start().unwrap().to_owned();
    let second = service.start().unwrap().to_owned();
    assert_eq!(first, second);
    assert_eq!(service.address().unwrap().ip(), Ipv4Addr::LOCALHOST);
    assert!(first.ends_with(MOUNT_PATH));

    let address = service.address().unwrap();
    let get = response_text(
        address,
        b"GET /eso-weave/ HTTP/1.1\r\nHost: localhost\r\n\r\n",
    );
    assert!(get.starts_with("HTTP/1.1 200 OK\r\n"));
    assert!(get.contains("Content-Type: text/html; charset=utf-8\r\n"));
    assert!(get.contains("Content-Security-Policy: "));
    assert!(get.contains("X-Content-Type-Options: nosniff\r\n"));
    assert!(get.contains("Referrer-Policy: no-referrer\r\n"));
    let expected_title = if cfg!(debug_assertions) {
        "ESO Weave development documentation fixture"
    } else {
        "ESO Weave Documentation"
    };
    assert!(get.contains(expected_title));

    let head = request(
        address,
        b"HEAD /eso-weave/ HTTP/1.1\r\nHost: localhost\r\n\r\n",
    );
    let split = head
        .windows(4)
        .position(|bytes| bytes == b"\r\n\r\n")
        .unwrap();
    assert_eq!(head.len(), split + 4);

    let missing_head = request(
        address,
        b"HEAD /eso-weave/missing.html HTTP/1.1\r\nHost: localhost\r\n\r\n",
    );
    let split = missing_head
        .windows(4)
        .position(|bytes| bytes == b"\r\n\r\n")
        .unwrap();
    assert_eq!(missing_head.len(), split + 4);
}

#[test]
fn root_redirect_and_nested_index_are_stable() {
    let mut service = DocumentationService::new();
    service.start().unwrap();
    let address = service.address().unwrap();
    let root = response_text(address, b"GET / HTTP/1.1\r\n\r\n");
    assert!(root.starts_with("HTTP/1.1 302 Found\r\n"));
    assert!(root.contains("Location: /eso-weave/\r\n"));
    let nested_request = if cfg!(debug_assertions) {
        b"GET /eso-weave/guide/ HTTP/1.1\r\n\r\n".as_slice()
    } else {
        b"GET /eso-weave/concepts/ HTTP/1.1\r\n\r\n".as_slice()
    };
    let nested = response_text(address, nested_request);
    assert!(nested.starts_with("HTTP/1.1 200 OK\r\n"));
}

#[test]
fn service_rejects_methods_unknown_paths_and_ambiguous_targets() {
    let mut service = DocumentationService::new();
    service.start().unwrap();
    let address = service.address().unwrap();

    let post = response_text(address, b"POST /eso-weave/ HTTP/1.1\r\n\r\n");
    assert!(post.starts_with("HTTP/1.1 405 Method Not Allowed\r\n"));
    assert!(post.contains("Allow: GET, HEAD\r\n"));

    for target in [
        "/missing",
        "/eso-weave/missing.html",
        "/eso-weave/../Cargo.toml",
        "/eso-weave/%2e%2e/Cargo.toml",
        "/eso-weave/guide\\index.html",
        "/eso-weave//guide/index.html",
    ] {
        let raw = format!("GET {target} HTTP/1.1\r\n\r\n");
        let response = response_text(address, raw.as_bytes());
        assert!(
            response.starts_with("HTTP/1.1 400 Bad Request\r\n")
                || response.starts_with("HTTP/1.1 404 Not Found\r\n"),
            "unexpected response for {target}: {response}"
        );
        assert!(!response.contains("[package]"));
    }
}

#[test]
fn service_bounds_requests_and_shuts_down_promptly() {
    let address;
    let start = Instant::now();
    {
        let mut service = DocumentationService::new();
        service.start().unwrap();
        address = service.address().unwrap();
        let oversized = format!("GET /eso-weave/ HTTP/1.1\r\nX-Fill: {}", "x".repeat(9000));
        let response = response_text(address, oversized.as_bytes());
        assert!(response.starts_with("HTTP/1.1 431 Request Header Fields Too Large\r\n"));

        let oversized_head = format!("HEAD /eso-weave/ HTTP/1.1\r\nX-Fill: {}", "x".repeat(9000));
        let response = request(address, oversized_head.as_bytes());
        let header_end = response
            .windows(4)
            .position(|bytes| bytes == b"\r\n\r\n")
            .unwrap();
        assert!(response.starts_with(b"HTTP/1.1 431 Request Header Fields Too Large\r\n"));
        assert_eq!(response.len(), header_end + 4);
    }
    assert!(start.elapsed() < Duration::from_secs(1));
    assert!(TcpStream::connect_timeout(&address, Duration::from_millis(100)).is_err());
}

#[test]
fn service_enforces_one_deadline_against_trickled_requests() {
    let mut service = DocumentationService::new();
    service.start().unwrap();
    let address = service.address().unwrap();
    let mut stream = TcpStream::connect(address).unwrap();
    stream.write_all(b"G").unwrap();
    let sender = std::thread::spawn(move || {
        for byte in b"ET /eso-weave/ HTTP/1.1\r\n" {
            if stream.write_all(std::slice::from_ref(byte)).is_err() {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    });

    std::thread::sleep(Duration::from_millis(50));
    let start = Instant::now();
    drop(service);
    assert!(start.elapsed() < Duration::from_secs(1));
    sender.join().unwrap();
}

#[test]
fn service_handles_concurrent_reads() {
    let mut service = DocumentationService::new();
    service.start().unwrap();
    let address = service.address().unwrap();
    let css = EMBEDDED_ASSETS
        .iter()
        .find(|entry| entry.path.ends_with(".css"))
        .expect("embedded stylesheet")
        .path;
    let workers: Vec<_> = (0..4)
        .map(|_| {
            std::thread::spawn(move || {
                let request = format!("GET /eso-weave/{css} HTTP/1.1\r\n\r\n");
                response_text(address, request.as_bytes())
            })
        })
        .collect();
    for worker in workers {
        assert!(worker.join().unwrap().starts_with("HTTP/1.1 200 OK\r\n"));
    }
}

#[derive(Default)]
struct RecordingOpener(std::sync::Mutex<Vec<String>>);

impl BrowserOpener for RecordingOpener {
    fn open(&self, url: &str) -> std::io::Result<()> {
        self.0.lock().unwrap().push(url.to_owned());
        Ok(())
    }
}

#[test]
fn browser_action_reuses_the_same_loopback_url() {
    let opener = RecordingOpener::default();
    let mut service = DocumentationService::new();
    let first = service.open_with(&opener).unwrap();
    let second = service.open_with(&opener).unwrap();
    assert_eq!(first, second);
    assert_eq!(opener.0.lock().unwrap().as_slice(), [first, second]);
}

struct FailingOpener;

impl BrowserOpener for FailingOpener {
    fn open(&self, _url: &str) -> std::io::Result<()> {
        Err(std::io::Error::other("no browser"))
    }
}

#[test]
fn browser_failure_leaves_the_service_reusable() {
    let mut service = DocumentationService::new();
    assert!(service.open_with(&FailingOpener).is_err());
    let address = service.address().expect("server remains available");
    let response = response_text(address, b"GET /eso-weave/ HTTP/1.1\r\n\r\n");
    assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
}
