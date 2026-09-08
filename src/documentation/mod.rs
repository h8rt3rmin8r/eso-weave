//! Immutable bundled documentation and its loopback-only HTTP service.

#[cfg(windows)]
use std::ffi::OsStr;
use std::fmt;
use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

pub const MOUNT_PATH: &str = "/eso-weave/";
const MAX_REQUEST_BYTES: usize = 8 * 1024;
const IO_TIMEOUT: Duration = Duration::from_millis(250);
const CONNECTION_LIFETIME: Duration = Duration::from_millis(500);
const ACCEPT_PAUSE: Duration = Duration::from_millis(10);
const CSP: &str = "default-src 'none'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src 'none'; object-src 'none'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'";

#[derive(Debug, Clone, Copy)]
pub struct EmbeddedAsset {
    pub path: &'static str,
    pub content_type: &'static str,
    pub bytes: &'static [u8],
}

include!(concat!(env!("OUT_DIR"), "/embedded_docs.rs"));

pub fn asset(path: &str) -> Option<&'static EmbeddedAsset> {
    EMBEDDED_ASSETS
        .binary_search_by_key(&path, |entry| entry.path)
        .ok()
        .map(|index| &EMBEDDED_ASSETS[index])
}

pub fn content_type(path: &str) -> &'static str {
    asset(path).map_or_else(
        || content_type_for_extension(path),
        |entry| entry.content_type,
    )
}

fn content_type_for_extension(path: &str) -> &'static str {
    match path.rsplit_once('.').map_or("", |(_, extension)| extension) {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "ico" => "image/x-icon",
        "ttf" => "font/ttf",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "wasm" => "application/wasm",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

#[derive(Debug, Default)]
pub struct DocumentationService {
    running: Option<RunningService>,
}

#[derive(Debug)]
struct RunningService {
    address: SocketAddr,
    url: String,
    shutdown: Sender<()>,
    worker: Option<JoinHandle<()>>,
}

impl DocumentationService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) -> io::Result<&str> {
        if self.running.is_none() {
            let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
            listener.set_nonblocking(true)?;
            let address = listener.local_addr()?;
            let (shutdown, shutdown_rx) = mpsc::channel();
            let worker = thread::Builder::new()
                .name("eso-weave-docs".to_owned())
                .spawn(move || run_server(listener, shutdown_rx))?;
            self.running = Some(RunningService {
                address,
                url: format!("http://{address}{MOUNT_PATH}"),
                shutdown,
                worker: Some(worker),
            });
        }
        Ok(&self.running.as_ref().expect("service initialized").url)
    }

    pub fn address(&self) -> Option<SocketAddr> {
        self.running.as_ref().map(|running| running.address)
    }

    pub fn open_with(&mut self, opener: &dyn BrowserOpener) -> io::Result<String> {
        let url = self.start()?.to_owned();
        opener.open(&url)?;
        Ok(url)
    }

    pub fn open(&mut self) -> io::Result<String> {
        self.open_with(&NativeBrowser)
    }
}

impl Drop for DocumentationService {
    fn drop(&mut self) {
        if let Some(mut running) = self.running.take() {
            let _ = running.shutdown.send(());
            if let Some(worker) = running.worker.take() {
                let _ = worker.join();
            }
        }
    }
}

pub trait BrowserOpener {
    fn open(&self, url: &str) -> io::Result<()>;
}

pub struct NativeBrowser;

#[cfg(windows)]
impl BrowserOpener for NativeBrowser {
    fn open(&self, url: &str) -> io::Result<()> {
        use std::os::windows::ffi::OsStrExt;
        use windows_sys::Win32::UI::Shell::ShellExecuteW;
        use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

        let operation: Vec<u16> = OsStr::new("open").encode_wide().chain(Some(0)).collect();
        let target: Vec<u16> = OsStr::new(url).encode_wide().chain(Some(0)).collect();
        let result = unsafe {
            ShellExecuteW(
                std::ptr::null_mut(),
                operation.as_ptr(),
                target.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                SW_SHOWNORMAL,
            )
        };
        if result as isize > 32 {
            Ok(())
        } else {
            Err(io::Error::other(format!(
                "the operating system rejected the documentation URL (code {})",
                result as isize
            )))
        }
    }
}

#[cfg(target_os = "linux")]
impl BrowserOpener for NativeBrowser {
    fn open(&self, url: &str) -> io::Result<()> {
        use std::process::{Command, Stdio};

        let status = Command::new("xdg-open")
            .arg(url)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        xdg_open_result(status)
    }
}

#[cfg(target_os = "linux")]
fn xdg_open_result(status: std::process::ExitStatus) -> io::Result<()> {
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "xdg-open could not open the documentation ({status})"
        )))
    }
}

#[cfg(not(any(windows, target_os = "linux")))]
impl BrowserOpener for NativeBrowser {
    fn open(&self, _url: &str) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "opening documentation is unsupported on this platform",
        ))
    }
}

fn run_server(listener: TcpListener, shutdown: Receiver<()>) {
    loop {
        if shutdown.try_recv().is_ok() {
            return;
        }
        match listener.accept() {
            Ok((stream, _)) => handle_connection(stream),
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => thread::sleep(ACCEPT_PAUSE),
            Err(_) => return,
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let deadline = Instant::now() + CONNECTION_LIFETIME;
    let response = read_request(&mut stream, deadline)
        .map(|request| route_request(&request))
        .unwrap_or_else(|failure| Response::error(failure.status, failure.head));
    let _ = response.write_to(&mut stream, deadline);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Method {
    Get,
    Head,
    Other,
}

#[derive(Debug)]
struct Request {
    method: Method,
    target: String,
}

#[derive(Debug, Clone, Copy)]
struct RequestFailure {
    status: Status,
    head: bool,
}

impl RequestFailure {
    fn from_bytes(status: Status, bytes: &[u8]) -> Self {
        Self {
            status,
            head: bytes.starts_with(b"HEAD "),
        }
    }
}

fn read_request(stream: &mut TcpStream, deadline: Instant) -> Result<Request, RequestFailure> {
    let mut bytes = Vec::with_capacity(1024);
    let mut chunk = [0_u8; 1024];
    loop {
        let timeout = operation_timeout(deadline)
            .map_err(|_| RequestFailure::from_bytes(Status::BadRequest, &bytes))?;
        stream
            .set_read_timeout(Some(timeout))
            .map_err(|_| RequestFailure::from_bytes(Status::BadRequest, &bytes))?;
        match stream.read(&mut chunk) {
            Ok(0) => return Err(RequestFailure::from_bytes(Status::BadRequest, &bytes)),
            Ok(count) => {
                bytes.extend_from_slice(&chunk[..count]);
                if bytes.len() > MAX_REQUEST_BYTES {
                    return Err(RequestFailure::from_bytes(Status::HeadersTooLarge, &bytes));
                }
                if bytes.windows(4).any(|window| window == b"\r\n\r\n") {
                    break;
                }
            }
            Err(_) => return Err(RequestFailure::from_bytes(Status::BadRequest, &bytes)),
        }
    }
    let failure = || RequestFailure::from_bytes(Status::BadRequest, &bytes);
    let text = std::str::from_utf8(&bytes).map_err(|_| failure())?;
    let line = text.split("\r\n").next().ok_or_else(failure)?;
    let mut fields = line.split(' ');
    let method = match fields.next().ok_or_else(failure)? {
        "GET" => Method::Get,
        "HEAD" => Method::Head,
        _ => Method::Other,
    };
    let target = fields.next().ok_or_else(failure)?;
    let version = fields.next().ok_or_else(failure)?;
    if fields.next().is_some() || !matches!(version, "HTTP/1.0" | "HTTP/1.1") {
        return Err(failure());
    }
    Ok(Request {
        method,
        target: target.to_owned(),
    })
}

fn route_request(request: &Request) -> Response {
    if request.method == Method::Other {
        return Response::error(Status::MethodNotAllowed, false);
    }
    let head = request.method == Method::Head;
    let target = request
        .target
        .split_once('?')
        .map_or(request.target.as_str(), |(path, _)| path);
    if target == "/" {
        return Response::redirect(head);
    }
    let Some(mut path) = target.strip_prefix(MOUNT_PATH) else {
        return Response::error(Status::NotFound, head);
    };
    if path.contains(['%', '\\', '\0'])
        || path.starts_with('/')
        || (!path.is_empty()
            && path
                .trim_end_matches('/')
                .split('/')
                .any(|part| part.is_empty() || part == "." || part == ".."))
    {
        return Response::error(Status::BadRequest, head);
    }
    let nested_index;
    if path.is_empty() {
        path = "index.html";
    } else if target.ends_with('/') {
        nested_index = format!("{path}index.html");
        path = &nested_index;
    }
    asset(path)
        .map(|entry| Response::asset(entry, head))
        .unwrap_or_else(|| Response::error(Status::NotFound, head))
}

#[derive(Debug, Clone, Copy)]
enum Status {
    BadRequest,
    NotFound,
    MethodNotAllowed,
    HeadersTooLarge,
}

impl fmt::Display for Status {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::BadRequest => "400 Bad Request",
            Self::NotFound => "404 Not Found",
            Self::MethodNotAllowed => "405 Method Not Allowed",
            Self::HeadersTooLarge => "431 Request Header Fields Too Large",
        })
    }
}

struct Response {
    status: String,
    content_type: &'static str,
    body: &'static [u8],
    head: bool,
    extra: Option<&'static str>,
}

impl Response {
    fn asset(entry: &'static EmbeddedAsset, head: bool) -> Self {
        Self {
            status: "200 OK".to_owned(),
            content_type: entry.content_type,
            body: entry.bytes,
            head,
            extra: None,
        }
    }

    fn redirect(head: bool) -> Self {
        Self {
            status: "302 Found".to_owned(),
            content_type: "text/plain; charset=utf-8",
            body: b"Documentation moved.",
            head,
            extra: Some("Location: /eso-weave/\r\n"),
        }
    }

    fn error(status: Status, head: bool) -> Self {
        Self {
            status: status.to_string(),
            content_type: "text/plain; charset=utf-8",
            body: b"Request unavailable.",
            head,
            extra: matches!(status, Status::MethodNotAllowed).then_some("Allow: GET, HEAD\r\n"),
        }
    }

    fn write_to(&self, stream: &mut TcpStream, deadline: Instant) -> io::Result<()> {
        let headers = format!(
            "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\nReferrer-Policy: no-referrer\r\nContent-Security-Policy: {}\r\n{}\r\n",
            self.status,
            self.content_type,
            self.body.len(),
            CSP,
            self.extra.unwrap_or("")
        );
        write_all_before(stream, headers.as_bytes(), deadline)?;
        if !self.head {
            write_all_before(stream, self.body, deadline)?;
        }
        Ok(())
    }
}

fn operation_timeout(deadline: Instant) -> io::Result<Duration> {
    let remaining = deadline.saturating_duration_since(Instant::now());
    if remaining.is_zero() {
        Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "documentation connection deadline elapsed",
        ))
    } else {
        Ok(remaining.min(IO_TIMEOUT))
    }
}

fn write_all_before(stream: &mut TcpStream, mut bytes: &[u8], deadline: Instant) -> io::Result<()> {
    while !bytes.is_empty() {
        stream.set_write_timeout(Some(operation_timeout(deadline)?))?;
        match stream.write(bytes) {
            Ok(0) => return Err(io::Error::from(io::ErrorKind::WriteZero)),
            Ok(count) => bytes = &bytes[count..],
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

#[cfg(all(test, target_os = "linux"))]
mod linux_tests {
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;

    use super::xdg_open_result;

    #[test]
    fn xdg_open_status_controls_browser_result() {
        assert!(xdg_open_result(ExitStatus::from_raw(0)).is_ok());
        let error = xdg_open_result(ExitStatus::from_raw(4 << 8)).unwrap_err();
        assert!(error.to_string().contains("exit status: 4"));
    }
}
