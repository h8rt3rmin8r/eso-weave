//! Bounded read-only access to ESO Weave-owned SQLite databases.

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine as _;
use rusqlite::config::DbConfig;
use rusqlite::hooks::{AuthAction, AuthContext, Authorization};
use rusqlite::limits::Limit;
use rusqlite::types::{Value, ValueRef};
use rusqlite::{Connection, ErrorCode, OpenFlags, PrepFlags};
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;
use tokio_util::sync::CancellationToken;

pub const MAX_CONCURRENT_QUERIES: usize = 2;
pub const MAX_SQL_BYTES: usize = 16 * 1024;
pub const MAX_PARAMETERS: usize = 64;
pub const MAX_ROWS: usize = 1_000;
pub const MAX_RESULT_BYTES: usize = 1024 * 1024;
pub const MAX_DURATION: Duration = Duration::from_secs(2);

const EXTERNAL_SCHEMA_VERSION: &str = "1.0.0";
const CATALOG_ID: &str = "catalog";
const ENCOUNTERS_ID: &str = "encounters";

#[derive(Debug, Clone)]
struct DatabasePaths {
    catalog: Option<PathBuf>,
    encounters: Option<PathBuf>,
}

#[derive(Clone)]
pub struct DatabaseQueryService {
    paths: Arc<RwLock<DatabasePaths>>,
    permits: Arc<Semaphore>,
}

impl Default for DatabaseQueryService {
    fn default() -> Self {
        Self {
            paths: Arc::new(RwLock::new(DatabasePaths {
                catalog: None,
                encounters: None,
            })),
            permits: Arc::new(Semaphore::new(MAX_CONCURRENT_QUERIES)),
        }
    }
}

impl DatabaseQueryService {
    pub fn new(catalog: PathBuf, encounters: Option<PathBuf>) -> Self {
        Self {
            paths: Arc::new(RwLock::new(DatabasePaths {
                catalog: Some(catalog),
                encounters,
            })),
            permits: Arc::new(Semaphore::new(MAX_CONCURRENT_QUERIES)),
        }
    }

    pub fn set_catalog_path(&self, path: PathBuf) {
        self.paths
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .catalog = Some(path);
    }

    pub async fn inventory(
        &self,
        cancellation: CancellationToken,
    ) -> Result<DatabaseInventory, QueryError> {
        if cancellation.is_cancelled() {
            return Err(QueryError::service_stopping());
        }
        let permit = self
            .permits
            .clone()
            .try_acquire_owned()
            .map_err(|_| QueryError::query_busy())?;
        let paths = self.path_snapshot();
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            if cancellation.is_cancelled() {
                return Err(QueryError::service_stopping());
            }
            Ok(DatabaseInventory {
                schema_version: EXTERNAL_SCHEMA_VERSION,
                databases: vec![
                    inspect_database(CATALOG_ID, "active ESO catalog", paths.catalog),
                    inspect_database(ENCOUNTERS_ID, "user encounter history", paths.encounters),
                ],
            })
        })
        .await
        .map_err(|_| QueryError::internal())?
    }

    pub async fn execute(
        &self,
        database_id: &str,
        request: QueryRequest,
        cancellation: CancellationToken,
    ) -> Result<QueryResult, QueryError> {
        validate_request(&request)?;
        if cancellation.is_cancelled() {
            return Err(QueryError::service_stopping());
        }
        let path = self
            .path_for(database_id)
            .ok_or_else(|| match database_id {
                CATALOG_ID | ENCOUNTERS_ID => QueryError::database_unavailable(),
                _ => QueryError::database_not_found(),
            })?;
        let permit = self
            .permits
            .clone()
            .try_acquire_owned()
            .map_err(|_| QueryError::query_busy())?;
        let database_id = database_id.to_owned();
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            execute_blocking(&database_id, path, request, cancellation)
        })
        .await
        .map_err(|_| QueryError::internal())?
    }

    fn path_snapshot(&self) -> DatabasePaths {
        self.paths
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }

    fn path_for(&self, database_id: &str) -> Option<PathBuf> {
        let paths = self
            .paths
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        match database_id {
            CATALOG_ID => paths.catalog.clone(),
            ENCOUNTERS_ID => paths.encounters.clone(),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DatabaseInventory {
    pub schema_version: &'static str,
    pub databases: Vec<DatabaseDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DatabaseDescriptor {
    pub database_id: &'static str,
    pub role: &'static str,
    pub available: bool,
    pub availability_reason: Option<&'static str>,
    pub objects: Vec<SchemaObject>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchemaObject {
    pub name: String,
    pub kind: String,
    pub columns: Vec<SchemaColumn>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchemaColumn {
    pub ordinal: usize,
    pub name: String,
    pub declared_type: Option<String>,
    pub nullable: bool,
    pub primary_key_position: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueryRequest {
    pub sql: String,
    #[serde(default)]
    pub parameters: Vec<QueryParameter>,
    #[serde(default)]
    pub row_limit: Option<usize>,
}

impl QueryRequest {
    pub fn new(sql: impl Into<String>) -> Self {
        Self {
            sql: sql.into(),
            parameters: Vec::new(),
            row_limit: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueryParameter {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten)]
    pub value: QueryParameterValue,
}

impl QueryParameter {
    pub fn positional(value: QueryParameterValue) -> Self {
        Self { name: None, value }
    }

    pub fn named(name: impl Into<String>, value: QueryParameterValue) -> Self {
        Self {
            name: Some(name.into()),
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum QueryParameterValue {
    Null,
    Integer(String),
    Real(String),
    Text(String),
    Blob(String),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct QueryResult {
    pub database_id: String,
    pub columns: Vec<QueryColumn>,
    pub rows: Vec<Vec<ResultValue>>,
    pub row_count: usize,
    pub truncated: bool,
    pub truncation_reason: Option<String>,
    pub elapsed_ms: u64,
    pub limits: QueryLimits,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QueryColumn {
    pub ordinal: usize,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum ResultValue {
    Null,
    Integer(String),
    Real(String),
    Text(String),
    Blob(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct QueryLimits {
    pub rows: usize,
    pub bytes: usize,
    pub duration_ms: u64,
}

impl Default for QueryLimits {
    fn default() -> Self {
        Self {
            rows: MAX_ROWS,
            bytes: MAX_RESULT_BYTES,
            duration_ms: MAX_DURATION.as_millis() as u64,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QueryError {
    code: &'static str,
    message: &'static str,
    retryable: bool,
}

impl QueryError {
    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn retryable(&self) -> bool {
        self.retryable
    }

    pub fn envelope(&self) -> serde_json::Value {
        serde_json::json!({ "error": self })
    }

    pub(crate) fn invalid_request() -> Self {
        Self::new(
            "invalid_request",
            "The query request does not match the bounded query contract.",
            false,
        )
    }

    fn database_not_found() -> Self {
        Self::new(
            "database_not_found",
            "The requested database identifier is not available.",
            false,
        )
    }

    fn database_unavailable() -> Self {
        Self::new(
            "database_unavailable",
            "The requested database is currently unavailable.",
            true,
        )
    }

    fn query_denied() -> Self {
        Self::new(
            "query_denied",
            "The statement is not permitted by the read-only query contract.",
            false,
        )
    }

    fn query_invalid() -> Self {
        Self::new(
            "query_invalid",
            "The statement could not be prepared or executed.",
            false,
        )
    }

    fn query_busy() -> Self {
        Self::new(
            "query_busy",
            "The external query concurrency limit is active.",
            true,
        )
    }

    fn query_timeout() -> Self {
        Self::new(
            "query_timeout",
            "The query exceeded the two-second execution limit.",
            true,
        )
    }

    fn database_busy() -> Self {
        Self::new(
            "database_busy",
            "The requested database is temporarily busy.",
            true,
        )
    }

    fn result_too_large() -> Self {
        Self::new(
            "result_too_large",
            "No complete result row fits within the one-megabyte result limit.",
            false,
        )
    }

    fn service_stopping() -> Self {
        Self::new(
            "service_stopping",
            "The local service generation is stopping.",
            true,
        )
    }

    fn internal() -> Self {
        Self::new(
            "internal_error",
            "The query could not be completed safely.",
            false,
        )
    }

    const fn new(code: &'static str, message: &'static str, retryable: bool) -> Self {
        Self {
            code,
            message,
            retryable,
        }
    }
}

impl std::fmt::Display for QueryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code)
    }
}

impl std::error::Error for QueryError {}

fn validate_request(request: &QueryRequest) -> Result<(), QueryError> {
    if request.sql.trim().is_empty()
        || request.sql.len() > MAX_SQL_BYTES
        || request.parameters.len() > MAX_PARAMETERS
        || request
            .row_limit
            .is_some_and(|value| value == 0 || value > MAX_ROWS)
    {
        return Err(QueryError::invalid_request());
    }
    let named = request
        .parameters
        .iter()
        .filter(|parameter| parameter.name.is_some())
        .count();
    if named != 0 && named != request.parameters.len() {
        return Err(QueryError::invalid_request());
    }
    if named > 0 {
        let mut names = HashSet::with_capacity(named);
        for parameter in &request.parameters {
            let name = parameter.name.as_deref().unwrap_or_default();
            if !matches!(name.as_bytes().first(), Some(b':' | b'@' | b'$'))
                || name.len() < 2
                || !names.insert(name)
            {
                return Err(QueryError::invalid_request());
            }
        }
    }
    for parameter in &request.parameters {
        let _ = parameter_value(&parameter.value)?;
    }
    Ok(())
}

fn inspect_database(
    database_id: &'static str,
    role: &'static str,
    path: Option<PathBuf>,
) -> DatabaseDescriptor {
    let Some(path) = path else {
        return unavailable_descriptor(database_id, role, "not_present");
    };
    if !path.is_file() {
        return unavailable_descriptor(database_id, role, "not_present");
    }
    let result = (|| -> rusqlite::Result<Vec<SchemaObject>> {
        let connection = open_defended(&path)?;
        let mut objects = connection.prepare(
            "SELECT name, type FROM sqlite_schema
             WHERE type IN ('table', 'view') AND name NOT LIKE 'sqlite_%'
             ORDER BY type, name",
        )?;
        let objects = objects
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        let mut result = Vec::with_capacity(objects.len());
        for (name, kind) in objects {
            let mut columns = connection.prepare(
                "SELECT cid, name, type, \"notnull\", pk
                 FROM pragma_table_xinfo(?1) WHERE hidden = 0 ORDER BY cid",
            )?;
            let columns = columns
                .query_map([&name], |row| {
                    let declared_type = row.get::<_, String>(2)?;
                    Ok(SchemaColumn {
                        ordinal: row.get::<_, u32>(0)? as usize,
                        name: row.get(1)?,
                        declared_type: (!declared_type.is_empty()).then_some(declared_type),
                        nullable: row.get::<_, u32>(3)? == 0,
                        primary_key_position: row.get(4)?,
                    })
                })?
                .collect::<rusqlite::Result<Vec<_>>>()?;
            result.push(SchemaObject {
                name,
                kind,
                columns,
            });
        }
        Ok(result)
    })();
    match result {
        Ok(objects) => DatabaseDescriptor {
            database_id,
            role,
            available: true,
            availability_reason: None,
            objects,
        },
        Err(_) => unavailable_descriptor(database_id, role, "unavailable"),
    }
}

fn unavailable_descriptor(
    database_id: &'static str,
    role: &'static str,
    reason: &'static str,
) -> DatabaseDescriptor {
    DatabaseDescriptor {
        database_id,
        role,
        available: false,
        availability_reason: Some(reason),
        objects: Vec::new(),
    }
}

fn execute_blocking(
    database_id: &str,
    path: PathBuf,
    request: QueryRequest,
    cancellation: CancellationToken,
) -> Result<QueryResult, QueryError> {
    if !path.is_file() {
        return Err(QueryError::database_unavailable());
    }
    let started = Instant::now();
    let connection = open_defended(&path).map_err(|_| QueryError::database_unavailable())?;
    let denied = Arc::new(AtomicBool::new(false));
    let denied_hook = denied.clone();
    connection
        .authorizer(Some(move |context: AuthContext<'_>| match context.action {
            AuthAction::Select
            | AuthAction::Read { .. }
            | AuthAction::Function { .. }
            | AuthAction::Recursive => Authorization::Allow,
            _ => {
                denied_hook.store(true, Ordering::Relaxed);
                Authorization::Deny
            }
        }))
        .map_err(|_| QueryError::internal())?;
    let progress_cancellation = cancellation.clone();
    connection
        .progress_handler(
            1_000,
            Some(move || progress_cancellation.is_cancelled() || started.elapsed() >= MAX_DURATION),
        )
        .map_err(|_| QueryError::internal())?;

    let mut statement = connection
        .prepare_with_flags(&request.sql, PrepFlags::SQLITE_PREPARE_NO_VTAB)
        .map_err(|error| {
            map_query_error(
                error,
                &cancellation,
                started,
                denied.load(Ordering::Relaxed),
            )
        })?;
    if !statement.readonly() || statement.column_count() == 0 {
        return Err(QueryError::query_denied());
    }
    bind_parameters(&mut statement, &request.parameters)?;
    let columns = statement
        .column_names()
        .into_iter()
        .enumerate()
        .map(|(ordinal, name)| QueryColumn {
            ordinal,
            name: name.to_owned(),
        })
        .collect::<Vec<_>>();
    let row_limit = request.row_limit.unwrap_or(MAX_ROWS);
    let mut result = QueryResult {
        database_id: database_id.to_owned(),
        columns,
        rows: Vec::new(),
        row_count: 0,
        truncated: false,
        truncation_reason: None,
        elapsed_ms: 0,
        limits: QueryLimits::default(),
    };
    let mut reserved = result.clone();
    reserved.row_count = MAX_ROWS;
    reserved.truncated = true;
    reserved.truncation_reason = Some("bytes".into());
    reserved.elapsed_ms = MAX_DURATION.as_millis() as u64;
    let base_size = serde_json::to_vec(&reserved)
        .map_err(|_| QueryError::internal())?
        .len();
    let mut row_bytes = 0usize;
    let mut rows = statement.raw_query();
    while let Some(row) = rows
        .next()
        .map_err(|error| map_query_error(error, &cancellation, started, false))?
    {
        if result.rows.len() == row_limit {
            result.truncated = true;
            result.truncation_reason = Some("rows".into());
            break;
        }
        let mut materialized = Vec::with_capacity(result.columns.len());
        for index in 0..result.columns.len() {
            let value = row.get_ref(index).map_err(|_| QueryError::internal())?;
            materialized.push(result_value(value)?);
        }
        let encoded_size = serde_json::to_vec(&materialized)
            .map_err(|_| QueryError::internal())?
            .len();
        let separators = usize::from(!result.rows.is_empty());
        if base_size
            .saturating_add(row_bytes)
            .saturating_add(separators)
            .saturating_add(encoded_size)
            > MAX_RESULT_BYTES
        {
            if result.rows.is_empty() {
                return Err(QueryError::result_too_large());
            }
            result.truncated = true;
            result.truncation_reason = Some("bytes".into());
            break;
        }
        row_bytes += separators + encoded_size;
        result.rows.push(materialized);
    }
    result.row_count = result.rows.len();
    result.elapsed_ms = started.elapsed().as_millis().min(MAX_DURATION.as_millis()) as u64;
    drop(rows);
    drop(statement);
    drop(connection);
    if serde_json::to_vec(&result)
        .map_err(|_| QueryError::internal())?
        .len()
        > MAX_RESULT_BYTES
    {
        return Err(QueryError::internal());
    }
    Ok(result)
}

fn open_defended(path: &PathBuf) -> rusqlite::Result<Connection> {
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY
        | OpenFlags::SQLITE_OPEN_URI
        | OpenFlags::SQLITE_OPEN_NO_MUTEX
        | OpenFlags::SQLITE_OPEN_NOFOLLOW;
    let connection = Connection::open_with_flags(path, flags)?;
    connection.busy_timeout(Duration::ZERO)?;
    connection.set_db_config(DbConfig::SQLITE_DBCONFIG_DEFENSIVE, true)?;
    connection.set_db_config(DbConfig::SQLITE_DBCONFIG_TRUSTED_SCHEMA, false)?;
    connection.set_db_config(DbConfig::SQLITE_DBCONFIG_DQS_DML, false)?;
    connection.set_db_config(DbConfig::SQLITE_DBCONFIG_DQS_DDL, false)?;
    connection.pragma_update(None, "query_only", true)?;
    connection.set_limit(Limit::SQLITE_LIMIT_LENGTH, MAX_RESULT_BYTES as i32)?;
    connection.set_limit(Limit::SQLITE_LIMIT_SQL_LENGTH, MAX_SQL_BYTES as i32)?;
    connection.set_limit(Limit::SQLITE_LIMIT_COLUMN, 256)?;
    connection.set_limit(Limit::SQLITE_LIMIT_EXPR_DEPTH, 100)?;
    connection.set_limit(Limit::SQLITE_LIMIT_COMPOUND_SELECT, 20)?;
    connection.set_limit(Limit::SQLITE_LIMIT_VDBE_OP, 1_000_000)?;
    connection.set_limit(Limit::SQLITE_LIMIT_FUNCTION_ARG, MAX_PARAMETERS as i32)?;
    connection.set_limit(Limit::SQLITE_LIMIT_ATTACHED, 0)?;
    connection.set_limit(
        Limit::SQLITE_LIMIT_LIKE_PATTERN_LENGTH,
        MAX_SQL_BYTES as i32,
    )?;
    connection.set_limit(Limit::SQLITE_LIMIT_VARIABLE_NUMBER, MAX_PARAMETERS as i32)?;
    connection.set_limit(Limit::SQLITE_LIMIT_TRIGGER_DEPTH, 0)?;
    connection.set_limit(Limit::SQLITE_LIMIT_WORKER_THREADS, 0)?;
    Ok(connection)
}

fn bind_parameters(
    statement: &mut rusqlite::Statement<'_>,
    parameters: &[QueryParameter],
) -> Result<(), QueryError> {
    if statement.parameter_count() != parameters.len() {
        return Err(QueryError::invalid_request());
    }
    let named = parameters.first().is_some_and(|value| value.name.is_some());
    if named {
        for parameter in parameters {
            let name = parameter
                .name
                .as_deref()
                .ok_or_else(QueryError::invalid_request)?;
            if statement
                .parameter_index(name)
                .map_err(|_| QueryError::invalid_request())?
                .is_none()
            {
                return Err(QueryError::invalid_request());
            }
            statement
                .raw_bind_parameter(name, parameter_value(&parameter.value)?)
                .map_err(|_| QueryError::invalid_request())?;
        }
        for index in 1..=statement.parameter_count() {
            let Some(name) = statement.parameter_name(index) else {
                return Err(QueryError::invalid_request());
            };
            if !matches!(name.as_bytes().first(), Some(b':' | b'@' | b'$'))
                || !parameters
                    .iter()
                    .any(|parameter| parameter.name.as_deref() == Some(name))
            {
                return Err(QueryError::invalid_request());
            }
        }
    } else {
        let numbered = (1..=statement.parameter_count()).any(|index| {
            statement
                .parameter_name(index)
                .is_some_and(|name| name.starts_with('?'))
        });
        for (offset, parameter) in parameters.iter().enumerate() {
            let index = offset + 1;
            let name = statement.parameter_name(index);
            let valid = if numbered {
                name.is_some_and(|name| name == format!("?{index}"))
            } else {
                name.is_none()
            };
            if !valid {
                return Err(QueryError::invalid_request());
            }
            statement
                .raw_bind_parameter(index, parameter_value(&parameter.value)?)
                .map_err(|_| QueryError::invalid_request())?;
        }
    }
    Ok(())
}

fn parameter_value(value: &QueryParameterValue) -> Result<Value, QueryError> {
    match value {
        QueryParameterValue::Null => Ok(Value::Null),
        QueryParameterValue::Integer(value) => value
            .parse::<i64>()
            .ok()
            .filter(|parsed| parsed.to_string() == *value)
            .map(Value::Integer)
            .ok_or_else(QueryError::invalid_request),
        QueryParameterValue::Real(value) => value
            .parse::<f64>()
            .ok()
            .filter(|value| value.is_finite())
            .map(Value::Real)
            .ok_or_else(QueryError::invalid_request),
        QueryParameterValue::Text(value) => Ok(Value::Text(value.clone())),
        QueryParameterValue::Blob(value) => BASE64_STANDARD
            .decode(value)
            .map(Value::Blob)
            .map_err(|_| QueryError::invalid_request()),
        QueryParameterValue::Boolean(value) => Ok(Value::Integer(i64::from(*value))),
    }
}

fn result_value(value: ValueRef<'_>) -> Result<ResultValue, QueryError> {
    match value {
        ValueRef::Null => Ok(ResultValue::Null),
        ValueRef::Integer(value) => Ok(ResultValue::Integer(value.to_string())),
        ValueRef::Real(value) => Ok(ResultValue::Real(if value.is_nan() {
            "nan".into()
        } else if value == f64::INFINITY {
            "positive_infinity".into()
        } else if value == f64::NEG_INFINITY {
            "negative_infinity".into()
        } else {
            value.to_string()
        })),
        ValueRef::Text(value) => std::str::from_utf8(value)
            .map(|value| ResultValue::Text(value.to_owned()))
            .map_err(|_| QueryError::internal()),
        ValueRef::Blob(value) => Ok(ResultValue::Blob(BASE64_STANDARD.encode(value))),
    }
}

fn map_query_error(
    error: rusqlite::Error,
    cancellation: &CancellationToken,
    started: Instant,
    denied: bool,
) -> QueryError {
    if denied {
        return QueryError::query_denied();
    }
    if cancellation.is_cancelled() {
        return QueryError::service_stopping();
    }
    if started.elapsed() >= MAX_DURATION {
        return QueryError::query_timeout();
    }
    match error.sqlite_error_code() {
        Some(ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked) => QueryError::database_busy(),
        Some(ErrorCode::TooBig) => QueryError::result_too_large(),
        _ => QueryError::query_invalid(),
    }
}
