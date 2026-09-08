//! Immutable bundled documentation and its loopback-only HTTP service.

#[cfg(windows)]
use std::ffi::OsStr;
use std::fmt;
use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub const MOUNT_PATH: &str = "/eso-weave/";
const MAX_REQUEST_BYTES: usize = 8 * 1024;
const IO_TIMEOUT: Duration = Duration::from_millis(250);
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

        let mut child = Command::new("xdg-open")
            .arg(url)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        thread::Builder::new()
            .name("eso-weave-browser-open".to_owned())
            .spawn(move || {
                let _ = child.wait();
            })?;
        Ok(())
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
    let _ = stream.set_read_timeout(Some(IO_TIMEOUT));
    let _ = stream.set_write_timeout(Some(IO_TIMEOUT));
    let response = read_request(&mut stream)
        .map(|request| route_request(&request))
        .unwrap_or_else(|status| Response::error(status, false));
    let _ = response.write_to(&mut stream);
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

fn read_request(stream: &mut TcpStream) -> Result<Request, Status> {
    let mut bytes = Vec::with_capacity(1024);
    let mut chunk = [0_u8; 1024];
    loop {
        match stream.read(&mut chunk) {
            Ok(0) => return Err(Status::BadRequest),
            Ok(count) => {
                bytes.extend_from_slice(&chunk[..count]);
                if bytes.len() > MAX_REQUEST_BYTES {
                    return Err(Status::HeadersTooLarge);
                }
                if bytes.windows(4).any(|window| window == b"\r\n\r\n") {
                    break;
                }
            }
            Err(_) => return Err(Status::BadRequest),
        }
    }
    let text = std::str::from_utf8(&bytes).map_err(|_| Status::BadRequest)?;
    let line = text.split("\r\n").next().ok_or(Status::BadRequest)?;
    let mut fields = line.split(' ');
    let method = match fields.next().ok_or(Status::BadRequest)? {
        "GET" => Method::Get,
        "HEAD" => Method::Head,
        _ => Method::Other,
    };
    let target = fields.next().ok_or(Status::BadRequest)?;
    let version = fields.next().ok_or(Status::BadRequest)?;
    if fields.next().is_some() || !matches!(version, "HTTP/1.0" | "HTTP/1.1") {
        return Err(Status::BadRequest);
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

    fn write_to(&self, stream: &mut TcpStream) -> io::Result<()> {
        write!(
            stream,
            "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\nReferrer-Policy: no-referrer\r\nContent-Security-Policy: {}\r\n{}\r\n",
            self.status,
            self.content_type,
            self.body.len(),
            CSP,
            self.extra.unwrap_or("")
        )?;
        if !self.head {
            stream.write_all(self.body)?;
        }
        Ok(())
    }
}
