//! Opt-in local HTTP and MCP service lifecycle.
//!
//! One named background thread owns the current-thread Tokio runtime, loopback
//! listener, shared Axum router, stateless RMCP transport, cancellation tree,
//! and discovery record. UI-facing calls enqueue commands and read immutable
//! status snapshots without waiting for network work.

use std::future::Future;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use axum::body::{to_bytes, Body, Bytes};
use axum::extract::{Path as AxumPath, Request, State};
use axum::http::header::{AUTHORIZATION, CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, HOST, ORIGIN};
use axum::http::{HeaderMap, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use axum::{Json, Router};
use rmcp::transport::streamable_http_server::session::never::NeverSessionManager;
use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::database_query::{DatabaseQueryService, QueryError, QueryRequest};
use crate::mcp_state::PlayerStateMcp;
use crate::player_state::SnapshotPublisher;

/// Fixed production port selected by ADR 0002.
pub const DEFAULT_PORT: u16 = 18_765;
/// Non-secret discovery record name in the application configuration directory.
pub const DISCOVERY_FILE_NAME: &str = "local-extension.json";
/// External schema version selected by the S104 contract.
pub const EXTERNAL_SCHEMA_VERSION: &str = "1.0.0";
/// Shared request-body bound for both adapters.
pub const MAX_REQUEST_BODY_BYTES: usize = 64 * 1024;
const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(3);

/// Durable local-service user preferences stored in `config.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LocalServicePrefs {
    /// Whether the user requested the complete local service.
    #[serde(default)]
    pub enabled: bool,
    /// Persistent 256-bit bearer credential, encoded as lowercase hexadecimal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
}

impl LocalServicePrefs {
    /// Parses the opaque settings section and falls back closed on invalid data.
    pub fn load(value: &serde_json::Value) -> Self {
        if value.is_null() {
            return Self::default();
        }
        serde_json::from_value(value.clone()).unwrap_or_default()
    }

    /// Serializes this preference into the opaque settings section.
    pub fn store(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("LocalServicePrefs is serializable")
    }

    /// Returns the existing credential or creates a new OS-random credential.
    pub fn ensure_credential(&mut self) -> Result<&str, getrandom::Error> {
        if !self.credential.as_deref().is_some_and(valid_credential) {
            let mut bytes = [0u8; 32];
            getrandom::fill(&mut bytes)?;
            let mut encoded = String::with_capacity(64);
            for byte in bytes {
                use std::fmt::Write as _;
                write!(&mut encoded, "{byte:02x}").expect("writing to String cannot fail");
            }
            self.credential = Some(encoded);
        }
        Ok(self.credential.as_deref().expect("credential was set"))
    }
}

/// Validates the persisted credential representation without exposing it.
pub fn valid_credential(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// Externally visible lifecycle phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServicePhase {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
}

impl ServicePhase {
    /// Stable lowercase presentation text.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stopped => "stopped",
            Self::Starting => "starting",
            Self::Running => "running",
            Self::Stopping => "stopping",
            Self::Failed => "failed",
        }
    }
}

/// Safe connection facts shown in the UI and discovery record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionInfo {
    pub http_base_url: String,
    pub mcp_url: String,
    pub process_id: u32,
    pub generation: u64,
    pub schema_version: &'static str,
}

/// Stable safe lifecycle failure categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceFailureCode {
    SettingsUnavailable,
    CredentialUnavailable,
    AddressInUse,
    BindFailed,
    DiscoveryFailed,
    RuntimeFailed,
    ShutdownTimeout,
}

impl ServiceFailureCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SettingsUnavailable => "settings_unavailable",
            Self::CredentialUnavailable => "credential_unavailable",
            Self::AddressInUse => "address_in_use",
            Self::BindFailed => "bind_failed",
            Self::DiscoveryFailed => "discovery_failed",
            Self::RuntimeFailed => "runtime_failed",
            Self::ShutdownTimeout => "shutdown_timeout",
        }
    }
}

/// Safe failure status suitable for UI and structured diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceFailure {
    pub code: ServiceFailureCode,
    pub message: String,
    pub retryable: bool,
}

/// Immutable UI-facing lifecycle snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceStatus {
    pub phase: ServicePhase,
    pub generation: u64,
    pub connection: Option<ConnectionInfo>,
    pub failure: Option<ServiceFailure>,
}

impl Default for ServiceStatus {
    fn default() -> Self {
        Self {
            phase: ServicePhase::Stopped,
            generation: 0,
            connection: None,
            failure: None,
        }
    }
}

/// Injectable construction settings. Production uses [`Default`].
#[derive(Debug, Clone)]
pub struct LocalServiceConfig {
    pub config_dir: Option<PathBuf>,
    pub port: u16,
    pub shutdown_timeout: Duration,
}

impl LocalServiceConfig {
    pub fn production(config_dir: Option<PathBuf>) -> Self {
        Self {
            config_dir,
            port: DEFAULT_PORT,
            shutdown_timeout: SHUTDOWN_TIMEOUT,
        }
    }
}

#[derive(Debug)]
enum OwnerCommand {
    Start(String),
    Stop,
    Shutdown(Instant),
}

/// Non-blocking UI-facing controller for the dedicated service owner.
pub struct LocalServiceController {
    commands: mpsc::UnboundedSender<OwnerCommand>,
    status: Arc<Mutex<ServiceStatus>>,
    completion: std::sync::mpsc::Receiver<()>,
    owner: Option<JoinHandle<()>>,
    shutdown_timeout: Duration,
}

impl LocalServiceController {
    /// Starts the idle owner thread. No listener is created until `start`.
    pub fn new(config: LocalServiceConfig) -> Self {
        Self::new_with_services(
            config,
            SnapshotPublisher::default(),
            DatabaseQueryService::default(),
        )
    }

    /// Starts an idle owner over the application-owned canonical state source.
    pub fn new_with_publisher(config: LocalServiceConfig, publisher: SnapshotPublisher) -> Self {
        Self::new_with_services(config, publisher, DatabaseQueryService::default())
    }

    /// Starts an idle owner over the canonical state and database sources.
    pub fn new_with_services(
        config: LocalServiceConfig,
        publisher: SnapshotPublisher,
        queries: DatabaseQueryService,
    ) -> Self {
        let shutdown_timeout = config.shutdown_timeout;
        let (commands, receiver) = mpsc::unbounded_channel();
        let status = Arc::new(Mutex::new(ServiceStatus::default()));
        let owner_status = status.clone();
        let (completion_tx, completion) = std::sync::mpsc::channel();
        let owner = thread::Builder::new()
            .name("eso-weave-local-service".into())
            .spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_io()
                    .enable_time()
                    .build();
                match runtime {
                    Ok(runtime) => runtime.block_on(owner_loop(
                        config,
                        receiver,
                        owner_status,
                        publisher,
                        queries,
                    )),
                    Err(_) => set_status(
                        &owner_status,
                        failed_status(
                            0,
                            ServiceFailureCode::RuntimeFailed,
                            "The local service runtime could not start.",
                            false,
                        ),
                    ),
                }
                let _ = completion_tx.send(());
            })
            .ok();

        if owner.is_none() {
            set_status(
                &status,
                failed_status(
                    0,
                    ServiceFailureCode::RuntimeFailed,
                    "The local service worker could not start.",
                    false,
                ),
            );
        }

        Self {
            commands,
            status,
            completion,
            owner,
            shutdown_timeout,
        }
    }

    /// Requests a complete start. Returns false for invalid credentials or a
    /// disconnected owner.
    pub fn start(&self, credential: &str) -> bool {
        if !valid_credential(credential) {
            let generation = self.status().generation;
            set_status(
                &self.status,
                failed_status(
                    generation,
                    ServiceFailureCode::CredentialUnavailable,
                    "The local service credential is unavailable. Disable and enable the service to create it again.",
                    true,
                ),
            );
            return false;
        }
        self.commands
            .send(OwnerCommand::Start(credential.to_owned()))
            .is_ok()
    }

    /// Requests stop without waiting on the UI thread.
    pub fn stop(&self) -> bool {
        self.commands.send(OwnerCommand::Stop).is_ok()
    }

    /// Returns the latest immutable status snapshot.
    pub fn status(&self) -> ServiceStatus {
        self.status.lock().unwrap().clone()
    }

    /// Publishes a safe integration failure without starting network work.
    pub fn report_failure(&self, failure: ServiceFailure) {
        let generation = self.status().generation;
        set_status(
            &self.status,
            ServiceStatus {
                phase: ServicePhase::Failed,
                generation,
                connection: None,
                failure: Some(failure),
            },
        );
    }

    /// Stops and joins the owner within the lifecycle contract's bound.
    pub fn shutdown(&mut self) -> Result<(), ServiceFailure> {
        let Some(owner) = self.owner.take() else {
            return Ok(());
        };
        let deadline = Instant::now() + self.shutdown_timeout;
        let _ = self.commands.send(OwnerCommand::Shutdown(deadline));
        let remaining = deadline.saturating_duration_since(Instant::now());
        if self.completion.recv_timeout(remaining).is_err() {
            let generation = self.status().generation;
            let failure = ServiceFailure {
                code: ServiceFailureCode::ShutdownTimeout,
                message: "The local service did not stop within 3 seconds.".into(),
                retryable: false,
            };
            set_status(
                &self.status,
                ServiceStatus {
                    phase: ServicePhase::Failed,
                    generation,
                    connection: None,
                    failure: Some(failure.clone()),
                },
            );
            drop(owner);
            return Err(failure);
        }
        if owner.join().is_err() {
            let failure = ServiceFailure {
                code: ServiceFailureCode::RuntimeFailed,
                message: "The local service worker stopped unexpectedly.".into(),
                retryable: false,
            };
            return Err(failure);
        }
        Ok(())
    }
}

impl Drop for LocalServiceController {
    fn drop(&mut self) {
        if let Err(failure) = self.shutdown() {
            tracing::error!(
                target: "eso_weave::local_service",
                code = failure.code.as_str(),
                "local service shutdown failed"
            );
        }
    }
}

async fn owner_loop(
    config: LocalServiceConfig,
    mut commands: mpsc::UnboundedReceiver<OwnerCommand>,
    status: Arc<Mutex<ServiceStatus>>,
    publisher: SnapshotPublisher,
    queries: DatabaseQueryService,
) {
    let mut generation = 0u64;
    while let Some(command) = commands.recv().await {
        match command {
            OwnerCommand::Start(mut credential) => {
                let Some(config_dir) = config.config_dir.as_deref() else {
                    set_status(
                        &status,
                        failed_status(
                            generation,
                            ServiceFailureCode::SettingsUnavailable,
                            "The settings directory is unavailable, so the local service cannot start safely.",
                            true,
                        ),
                    );
                    continue;
                };
                generation = generation.saturating_add(1);
                set_status(
                    &status,
                    ServiceStatus {
                        phase: ServicePhase::Starting,
                        generation,
                        connection: None,
                        failure: None,
                    },
                );

                let bind = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), config.port);
                let listener = match tokio::net::TcpListener::bind(bind).await {
                    Ok(listener) => listener,
                    Err(error) => {
                        let (code, message) = if error.kind() == std::io::ErrorKind::AddrInUse {
                            (
                                ServiceFailureCode::AddressInUse,
                                format!(
                                    "The local service port {} is already in use.",
                                    config.port
                                ),
                            )
                        } else {
                            (
                                ServiceFailureCode::BindFailed,
                                "The local service could not bind its loopback listener.".into(),
                            )
                        };
                        set_status(&status, failed_status(generation, code, &message, true));
                        continue;
                    }
                };
                let effective = match listener.local_addr() {
                    Ok(address) => address,
                    Err(_) => {
                        set_status(
                            &status,
                            failed_status(
                                generation,
                                ServiceFailureCode::BindFailed,
                                "The local service could not inspect its loopback listener.",
                                true,
                            ),
                        );
                        continue;
                    }
                };

                let mut desired = true;
                let mut shutdown = false;
                while let Ok(queued) = commands.try_recv() {
                    match queued {
                        OwnerCommand::Start(next) => {
                            credential = next;
                            desired = true;
                        }
                        OwnerCommand::Stop => desired = false,
                        OwnerCommand::Shutdown(_) => {
                            desired = false;
                            shutdown = true;
                        }
                    }
                }
                if !desired {
                    set_status(&status, stopping_status(generation));
                    drop(listener);
                    if cleanup_discovery(config_dir, std::process::id(), generation).is_ok() {
                        set_status(&status, stopped_status(generation));
                    } else {
                        set_status(
                            &status,
                            failed_status(
                                generation,
                                ServiceFailureCode::DiscoveryFailed,
                                "The local service stopped, but its discovery record could not be removed.",
                                true,
                            ),
                        );
                    }
                    if shutdown {
                        break;
                    }
                    continue;
                }

                let cancellation = CancellationToken::new();
                let router = build_router(
                    effective,
                    credential,
                    cancellation.clone(),
                    publisher.clone(),
                    queries.clone(),
                    generation,
                );
                let connection = connection_info(effective, generation);
                let record = DiscoveryRecord::from(&connection);
                if publish_discovery(config_dir, &record).is_err() {
                    cancellation.cancel();
                    drop(listener);
                    let _ = cleanup_discovery(config_dir, std::process::id(), generation);
                    set_status(
                        &status,
                        failed_status(
                            generation,
                            ServiceFailureCode::DiscoveryFailed,
                            "The local service could not publish connection discovery.",
                            true,
                        ),
                    );
                    continue;
                }

                // Discovery publication is the final fallible start step. Drain
                // once more so a disable received during that filesystem write
                // never publishes a running state for an already-cancelled
                // generation.
                while let Ok(queued) = commands.try_recv() {
                    match queued {
                        OwnerCommand::Start(_) => desired = true,
                        OwnerCommand::Stop => desired = false,
                        OwnerCommand::Shutdown(_) => {
                            desired = false;
                            shutdown = true;
                        }
                    }
                }
                if !desired {
                    cancellation.cancel();
                    drop(listener);
                    set_status(&status, stopping_status(generation));
                    if cleanup_discovery(config_dir, std::process::id(), generation).is_ok() {
                        set_status(&status, stopped_status(generation));
                    } else {
                        set_status(
                            &status,
                            failed_status(
                                generation,
                                ServiceFailureCode::DiscoveryFailed,
                                "The local service stopped, but its discovery record could not be removed.",
                                true,
                            ),
                        );
                    }
                    if shutdown {
                        break;
                    }
                    continue;
                }

                set_status(
                    &status,
                    ServiceStatus {
                        phase: ServicePhase::Running,
                        generation,
                        connection: Some(connection),
                        failure: None,
                    },
                );

                let serve_cancellation = cancellation.clone();
                let listener = CancellationListener::new(listener, cancellation.clone());
                let server = std::future::IntoFuture::into_future(
                    axum::serve(listener, router).with_graceful_shutdown(async move {
                        serve_cancellation.cancelled().await;
                    }),
                );
                tokio::pin!(server);
                let exit = 'running: loop {
                    let next = tokio::select! {
                        result = &mut server => break 'running RunningExit::Server(result),
                        command = commands.recv() => command,
                    };
                    match next {
                        Some(OwnerCommand::Start(_)) => {
                            // Already running. The persisted credential is stable,
                            // so a repeated start is idempotent.
                        }
                        Some(OwnerCommand::Stop) => {
                            let mut stop_requested = true;
                            let mut shutdown = false;
                            let mut deadline = Instant::now() + config.shutdown_timeout;
                            while let Ok(queued) = commands.try_recv() {
                                match queued {
                                    OwnerCommand::Start(_) if !shutdown => stop_requested = false,
                                    OwnerCommand::Stop if !shutdown => stop_requested = true,
                                    OwnerCommand::Shutdown(next_deadline) => {
                                        stop_requested = true;
                                        shutdown = true;
                                        deadline = deadline.min(next_deadline);
                                    }
                                    _ => {}
                                }
                            }
                            if stop_requested {
                                break 'running RunningExit::Stop { deadline, shutdown };
                            }
                        }
                        Some(OwnerCommand::Shutdown(deadline)) => {
                            break 'running RunningExit::Stop {
                                deadline,
                                shutdown: true,
                            };
                        }
                        None => {
                            break 'running RunningExit::Stop {
                                deadline: Instant::now(),
                                shutdown: true,
                            };
                        }
                    }
                };

                match exit {
                    RunningExit::Stop { deadline, shutdown } => {
                        set_status(&status, stopping_status(generation));
                        cancellation.cancel();
                        let graceful_timeout = deadline
                            .saturating_duration_since(Instant::now())
                            .saturating_sub(shutdown_join_margin(config.shutdown_timeout));
                        let stopped_in_time = tokio::time::timeout(graceful_timeout, &mut server)
                            .await
                            .is_ok();
                        let discovery_removed =
                            cleanup_discovery(config_dir, std::process::id(), generation).is_ok();
                        if !stopped_in_time {
                            set_status(
                                &status,
                                failed_status(
                                    generation,
                                    ServiceFailureCode::ShutdownTimeout,
                                    "The local service did not stop within 3 seconds.",
                                    false,
                                ),
                            );
                        } else if !discovery_removed {
                            set_status(
                                &status,
                                failed_status(
                                    generation,
                                    ServiceFailureCode::DiscoveryFailed,
                                    "The local service stopped, but its discovery record could not be removed.",
                                    true,
                                ),
                            );
                        } else {
                            set_status(&status, stopped_status(generation));
                        }
                        if shutdown {
                            break;
                        }
                    }
                    RunningExit::Server(result) => {
                        cancellation.cancel();
                        let _ = cleanup_discovery(config_dir, std::process::id(), generation);
                        let message = if result.is_ok() {
                            "The local service stopped unexpectedly."
                        } else {
                            "The local service listener failed."
                        };
                        set_status(
                            &status,
                            failed_status(
                                generation,
                                ServiceFailureCode::RuntimeFailed,
                                message,
                                true,
                            ),
                        );
                    }
                }
            }
            OwnerCommand::Stop => set_status(&status, stopped_status(generation)),
            OwnerCommand::Shutdown(_) => break,
        }
    }
}

fn shutdown_join_margin(timeout: Duration) -> Duration {
    (timeout / 10).min(Duration::from_millis(100))
}

enum RunningExit {
    Server(std::io::Result<()>),
    Stop { deadline: Instant, shutdown: bool },
}

struct CancellationListener {
    listener: tokio::net::TcpListener,
    cancellation: CancellationToken,
}

impl CancellationListener {
    fn new(listener: tokio::net::TcpListener, cancellation: CancellationToken) -> Self {
        Self {
            listener,
            cancellation,
        }
    }
}

impl axum::serve::Listener for CancellationListener {
    type Io = CancellationIo;
    type Addr = SocketAddr;

    async fn accept(&mut self) -> (Self::Io, Self::Addr) {
        loop {
            match self.listener.accept().await {
                Ok((stream, address)) => {
                    return (
                        CancellationIo::new(stream, self.cancellation.clone()),
                        address,
                    );
                }
                Err(error) => {
                    tracing::warn!(
                        target: "eso_weave::local_service",
                        error = %error,
                        "local service accept failed; retrying"
                    );
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    fn local_addr(&self) -> io::Result<Self::Addr> {
        self.listener.local_addr()
    }
}

struct CancellationIo {
    stream: tokio::net::TcpStream,
    cancelled: Pin<Box<dyn Future<Output = ()> + Send>>,
}

impl CancellationIo {
    fn new(stream: tokio::net::TcpStream, cancellation: CancellationToken) -> Self {
        Self {
            stream,
            cancelled: Box::pin(cancellation.cancelled_owned()),
        }
    }

    fn cancellation_ready(&mut self, context: &mut Context<'_>) -> bool {
        self.cancelled.as_mut().poll(context).is_ready()
    }
}

impl AsyncRead for CancellationIo {
    fn poll_read(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffer: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        if self.cancellation_ready(context) {
            return Poll::Ready(Err(cancelled_connection()));
        }
        Pin::new(&mut self.stream).poll_read(context, buffer)
    }
}

impl AsyncWrite for CancellationIo {
    fn poll_write(
        mut self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffer: &[u8],
    ) -> Poll<io::Result<usize>> {
        if self.cancellation_ready(context) {
            return Poll::Ready(Err(cancelled_connection()));
        }
        Pin::new(&mut self.stream).poll_write(context, buffer)
    }

    fn poll_flush(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<io::Result<()>> {
        if self.cancellation_ready(context) {
            return Poll::Ready(Err(cancelled_connection()));
        }
        Pin::new(&mut self.stream).poll_flush(context)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<io::Result<()>> {
        if self.cancellation_ready(context) {
            return Poll::Ready(Err(cancelled_connection()));
        }
        Pin::new(&mut self.stream).poll_shutdown(context)
    }
}

fn cancelled_connection() -> io::Error {
    io::Error::new(
        io::ErrorKind::ConnectionAborted,
        "local service generation stopped",
    )
}

#[derive(Clone)]
struct SecurityState {
    authority: String,
    credential: String,
    cancellation: CancellationToken,
}

#[derive(Clone)]
struct ApiState {
    publisher: SnapshotPublisher,
    queries: DatabaseQueryService,
    cancellation: CancellationToken,
    generation: u64,
}

fn build_router(
    effective: SocketAddr,
    credential: String,
    cancellation: CancellationToken,
    publisher: SnapshotPublisher,
    queries: DatabaseQueryService,
    generation: u64,
) -> Router {
    let authority = effective.to_string();
    let mut mcp_config = StreamableHttpServerConfig::default()
        .with_allowed_hosts([authority.clone()])
        .with_allowed_origins([
            format!("http://{authority}"),
            format!("https://{authority}"),
        ]);
    mcp_config.legacy_session_mode = false;
    mcp_config.json_response = true;
    mcp_config.cancellation_token = cancellation.clone();
    mcp_config.max_request_body_bytes = MAX_REQUEST_BODY_BYTES;
    let mcp_handler = PlayerStateMcp::new(
        publisher.clone(),
        queries.clone(),
        cancellation.clone(),
        generation,
    );
    let mcp = StreamableHttpService::new(
        move || Ok(mcp_handler.clone()),
        Arc::new(NeverSessionManager::default()),
        mcp_config,
    );
    let security = SecurityState {
        authority,
        credential,
        cancellation: cancellation.clone(),
    };
    let api = ApiState {
        publisher,
        queries,
        cancellation,
        generation,
    };
    Router::new()
        .route_service("/mcp", mcp)
        .route("/api/v1", any(api_capabilities))
        .route("/api/v1/capabilities", any(api_capabilities))
        .route("/api/v1/player-state", any(api_player_state))
        .route("/api/v1/databases", any(api_databases))
        .route(
            "/api/v1/databases/{database_id}/query",
            any(api_database_query),
        )
        .route("/api/v1/{*path}", any(not_found))
        .fallback(not_found)
        .with_state(api)
        .layer(middleware::from_fn_with_state(security, request_guard))
}

async fn request_guard(
    State(state): State<SecurityState>,
    request: Request,
    next: Next,
) -> Response {
    let headers = request.headers();
    if !single_header_equals(headers, HOST, &state.authority) {
        return safe_error(
            StatusCode::BAD_REQUEST,
            "invalid_host",
            "Request Host is not allowed.",
        );
    }
    let origin = match optional_single_header(headers, ORIGIN) {
        Ok(origin) => origin,
        Err(()) => {
            return safe_error(
                StatusCode::FORBIDDEN,
                "invalid_origin",
                "Request Origin is not allowed.",
            );
        }
    };
    if let Some(origin) = origin {
        let allowed_http = format!("http://{}", state.authority);
        let allowed_https = format!("https://{}", state.authority);
        if origin != allowed_http && origin != allowed_https {
            return safe_error(
                StatusCode::FORBIDDEN,
                "invalid_origin",
                "Request Origin is not allowed.",
            );
        }
    }
    if headers
        .get(CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<usize>().ok())
        .is_some_and(|length| length > MAX_REQUEST_BODY_BYTES)
    {
        return safe_error(
            StatusCode::PAYLOAD_TOO_LARGE,
            "request_too_large",
            "Request body exceeds the 64 KiB limit.",
        );
    }
    let mut authorization = headers.get_all(AUTHORIZATION).iter();
    let Some(value) = authorization.next() else {
        return safe_error(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "Bearer authentication is required.",
        );
    };
    if authorization.next().is_some() {
        return safe_error(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "Bearer authentication is required.",
        );
    }
    let presented = value
        .to_str()
        .ok()
        .and_then(|value| value.strip_prefix("Bearer "));
    if !presented
        .is_some_and(|value| constant_work_eq(value.as_bytes(), state.credential.as_bytes()))
    {
        return safe_error(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "Bearer authentication is required.",
        );
    }
    if state.cancellation.is_cancelled() {
        return stopping_error();
    }
    let cancellation = state.cancellation.clone();
    let (parts, body) = request.into_parts();
    let body = match tokio::select! {
        biased;
        _ = cancellation.cancelled() => return stopping_error(),
        result = to_bytes(body, MAX_REQUEST_BODY_BYTES) => result,
    } {
        Ok(body) => body,
        Err(_) => {
            return safe_error(
                StatusCode::PAYLOAD_TOO_LARGE,
                "request_too_large",
                "Request body exceeds the 64 KiB limit.",
            );
        }
    };
    let request = Request::from_parts(parts, Body::from(body));
    tokio::select! {
        biased;
        _ = cancellation.cancelled() => stopping_error(),
        response = next.run(request) => response,
    }
}

fn single_header_equals(
    headers: &HeaderMap,
    name: axum::http::header::HeaderName,
    expected: &str,
) -> bool {
    let mut values = headers.get_all(name).iter();
    let matches = values
        .next()
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value == expected);
    matches && values.next().is_none()
}

fn optional_single_header(
    headers: &HeaderMap,
    name: axum::http::header::HeaderName,
) -> Result<Option<&str>, ()> {
    let mut values = headers.get_all(name).iter();
    let Some(first) = values.next() else {
        return Ok(None);
    };
    let first = first.to_str().map_err(|_| ())?;
    if values.next().is_some() {
        return Err(());
    }
    Ok(Some(first))
}

fn constant_work_eq(presented: &[u8], expected: &[u8]) -> bool {
    let mut difference = presented.len() ^ expected.len();
    for index in 0..expected.len() {
        let left = presented.get(index).copied().unwrap_or(0);
        let right = expected.get(index).copied().unwrap_or(0);
        difference |= usize::from(left ^ right);
    }
    difference == 0
}

async fn api_capabilities(State(state): State<ApiState>, method: Method) -> Response {
    if method != Method::GET {
        return method_not_allowed();
    }
    Json(state.publisher.current().capabilities()).into_response()
}

async fn api_player_state(State(state): State<ApiState>, method: Method) -> Response {
    if method != Method::GET {
        return method_not_allowed();
    }
    let snapshot = state.publisher.current();
    Json(snapshot.document(state.generation)).into_response()
}

async fn api_databases(State(state): State<ApiState>, method: Method) -> Response {
    if method != Method::GET {
        return method_not_allowed_for("This route supports GET only.");
    }
    let cancellation = state.cancellation.child_token();
    let _drop_guard = cancellation.clone().drop_guard();
    match state.queries.inventory(cancellation).await {
        Ok(inventory) => Json(inventory).into_response(),
        Err(error) => query_error_response(&error),
    }
}

async fn api_database_query(
    State(state): State<ApiState>,
    AxumPath(database_id): AxumPath<String>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if method != Method::POST {
        return method_not_allowed_for("This route supports POST only.");
    }
    if !headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| {
            value
                .split(';')
                .next()
                .is_some_and(|media_type| media_type.trim() == "application/json")
        })
    {
        return query_error_response(&QueryError::invalid_request());
    }
    let request = match serde_json::from_slice::<QueryRequest>(&body) {
        Ok(request) => request,
        Err(_) => return query_error_response(&QueryError::invalid_request()),
    };
    let cancellation = state.cancellation.child_token();
    let _drop_guard = cancellation.clone().drop_guard();
    match state
        .queries
        .execute(&database_id, request, cancellation)
        .await
    {
        Ok(result) => Json(result).into_response(),
        Err(error) => query_error_response(&error),
    }
}

fn method_not_allowed() -> Response {
    method_not_allowed_for("This route supports GET only.")
}

fn method_not_allowed_for(message: &'static str) -> Response {
    safe_error(
        StatusCode::METHOD_NOT_ALLOWED,
        "method_not_allowed",
        message,
    )
}

fn query_error_response(error: &QueryError) -> Response {
    let status = match error.code() {
        "invalid_request" | "query_invalid" => StatusCode::BAD_REQUEST,
        "database_not_found" => StatusCode::NOT_FOUND,
        "query_denied" => StatusCode::FORBIDDEN,
        "query_busy" => StatusCode::TOO_MANY_REQUESTS,
        "query_timeout" => StatusCode::REQUEST_TIMEOUT,
        "result_too_large" => StatusCode::PAYLOAD_TOO_LARGE,
        "database_unavailable" | "database_busy" | "service_stopping" => {
            StatusCode::SERVICE_UNAVAILABLE
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (status, Json(error.envelope())).into_response()
}

async fn not_found() -> Response {
    safe_error(StatusCode::NOT_FOUND, "not_found", "Route not found.")
}

fn safe_error(status: StatusCode, code: &'static str, message: &'static str) -> Response {
    (
        status,
        Json(serde_json::json!({
            "error": {
                "code": code,
                "message": message,
                "retryable": false
            }
        })),
    )
        .into_response()
}

fn stopping_error() -> Response {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        [(CONNECTION, "close")],
        Json(serde_json::json!({
            "error": {
                "code": "service_stopping",
                "message": "The local service generation is stopping.",
                "retryable": true
            }
        })),
    )
        .into_response()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiscoveryRecord {
    pub discovery_version: u32,
    pub service_generation: u64,
    pub process_id: u32,
    pub http_base_url: String,
    pub mcp_url: String,
    pub schema_version: String,
}

impl From<&ConnectionInfo> for DiscoveryRecord {
    fn from(connection: &ConnectionInfo) -> Self {
        Self {
            discovery_version: 1,
            service_generation: connection.generation,
            process_id: connection.process_id,
            http_base_url: connection.http_base_url.clone(),
            mcp_url: connection.mcp_url.clone(),
            schema_version: connection.schema_version.into(),
        }
    }
}

fn connection_info(address: SocketAddr, generation: u64) -> ConnectionInfo {
    ConnectionInfo {
        http_base_url: format!("http://{address}/api/v1"),
        mcp_url: format!("http://{address}/mcp"),
        process_id: std::process::id(),
        generation,
        schema_version: EXTERNAL_SCHEMA_VERSION,
    }
}

fn publish_discovery(config_dir: &Path, record: &DiscoveryRecord) -> std::io::Result<()> {
    std::fs::create_dir_all(config_dir)?;
    let mut candidate = tempfile::NamedTempFile::new_in(config_dir)?;
    serde_json::to_writer_pretty(&mut candidate, record).map_err(std::io::Error::other)?;
    candidate.write_all(b"\n")?;
    candidate.as_file().sync_all()?;
    crate::atomic_file::persist(
        candidate.into_temp_path(),
        &config_dir.join(DISCOVERY_FILE_NAME),
    )
}

fn cleanup_discovery(config_dir: &Path, process_id: u32, generation: u64) -> std::io::Result<()> {
    let path = config_dir.join(DISCOVERY_FILE_NAME);
    let owned = std::fs::read(&path)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<DiscoveryRecord>(&bytes).ok())
        .is_some_and(|record| {
            record.process_id == process_id && record.service_generation == generation
        });
    if owned {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

fn stopped_status(generation: u64) -> ServiceStatus {
    ServiceStatus {
        phase: ServicePhase::Stopped,
        generation,
        connection: None,
        failure: None,
    }
}

fn stopping_status(generation: u64) -> ServiceStatus {
    ServiceStatus {
        phase: ServicePhase::Stopping,
        generation,
        connection: None,
        failure: None,
    }
}

fn failed_status(
    generation: u64,
    code: ServiceFailureCode,
    message: &str,
    retryable: bool,
) -> ServiceStatus {
    ServiceStatus {
        phase: ServicePhase::Failed,
        generation,
        connection: None,
        failure: Some(ServiceFailure {
            code,
            message: message.into(),
            retryable,
        }),
    }
}

fn set_status(shared: &Arc<Mutex<ServiceStatus>>, next: ServiceStatus) {
    let mut current = shared.lock().unwrap();
    if *current == next {
        return;
    }
    if let Some(failure) = &next.failure {
        tracing::warn!(
            target: "eso_weave::local_service",
            phase = next.phase.as_str(),
            generation = next.generation,
            code = failure.code.as_str(),
            "local service lifecycle changed"
        );
    } else {
        tracing::info!(
            target: "eso_weave::local_service",
            phase = next.phase.as_str(),
            generation = next.generation,
            "local service lifecycle changed"
        );
    }
    *current = next;
}
