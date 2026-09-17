use std::fs;
use std::time::{Duration, Instant};

use eso_weave::database_query::{
    DatabaseQueryService, QueryParameter, QueryParameterValue, QueryRequest, ResultValue,
    MAX_RESULT_BYTES,
};
use rusqlite::Connection;
use tokio_util::sync::CancellationToken;

fn fixture() -> (tempfile::TempDir, DatabaseQueryService) {
    let root = tempfile::tempdir().unwrap();
    let catalog = root.path().join("catalog.sqlite");
    let encounters = root.path().join("encounters.sqlite");
    let connection = Connection::open(&catalog).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE entity (
                id INTEGER PRIMARY KEY,
                kind TEXT NOT NULL,
                score REAL,
                payload BLOB,
                note TEXT
             );
             INSERT INTO entity (id, kind, score, payload, note)
             VALUES (1, 'skill', 1.25, X'00FF', NULL),
                    (2, 'item', 2.5, X'', 'ready'),
                    (3, 'skill', 3.75, X'ABCD', 'later');",
        )
        .unwrap();
    drop(connection);
    let connection = Connection::open(&encounters).unwrap();
    connection
        .execute_batch(
            "CREATE TABLE raw_encounters (
                session_id TEXT NOT NULL,
                encounter_id TEXT NOT NULL,
                payload BLOB NOT NULL,
                PRIMARY KEY (session_id, encounter_id)
             );",
        )
        .unwrap();
    drop(connection);
    let service = DatabaseQueryService::new(catalog, Some(encounters));
    (root, service)
}

fn run<T>(future: impl std::future::Future<Output = T>) -> T {
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap()
        .block_on(future)
}

#[test]
fn inventory_is_fixed_ordered_safe_and_truthful() {
    let (root, service) = fixture();
    let inventory = run(service.inventory(CancellationToken::new())).unwrap();
    assert_eq!(inventory.schema_version, "1.1.0");
    assert_eq!(inventory.databases.len(), 2);
    assert_eq!(inventory.databases[0].database_id, "catalog");
    assert_eq!(inventory.databases[1].database_id, "encounters");
    assert!(inventory.databases.iter().all(|value| value.available));
    let entity = inventory.databases[0]
        .objects
        .iter()
        .find(|value| value.name == "entity")
        .unwrap();
    assert_eq!(entity.kind, "table");
    assert_eq!(entity.columns[0].name, "id");
    assert_eq!(entity.columns[0].declared_type.as_deref(), Some("INTEGER"));
    assert_eq!(entity.columns[0].primary_key_position, 1);
    let json = serde_json::to_string(&inventory).unwrap();
    assert!(!json.contains(root.path().to_string_lossy().as_ref()));
    assert!(!json.contains("CREATE TABLE"));

    fs::remove_file(root.path().join("encounters.sqlite")).unwrap();
    let inventory = run(service.inventory(CancellationToken::new())).unwrap();
    assert!(!inventory.databases[1].available);
    assert_eq!(
        inventory.databases[1].availability_reason,
        Some("not_present")
    );
    assert!(inventory.databases[1].objects.is_empty());
}

#[test]
fn positional_query_preserves_types_order_and_duplicate_columns() {
    let (_root, service) = fixture();
    let result = run(service.execute(
        "catalog",
        QueryRequest {
            sql: "SELECT id AS repeated, id AS repeated, score, payload, note FROM entity WHERE kind = ?1 ORDER BY id"
                .into(),
            parameters: vec![QueryParameter::positional(QueryParameterValue::Text(
                "skill".into(),
            ))],
            row_limit: None,
        },
        CancellationToken::new(),
    ))
    .unwrap();
    assert_eq!(
        result
            .columns
            .iter()
            .map(|value| value.name.as_str())
            .collect::<Vec<_>>(),
        ["repeated", "repeated", "score", "payload", "note"]
    );
    assert_eq!(result.row_count, 2);
    assert_eq!(result.rows[0][0], ResultValue::Integer("1".into()));
    assert_eq!(result.rows[0][2], ResultValue::Real("1.25".into()));
    assert_eq!(result.rows[0][3], ResultValue::Blob("AP8=".into()));
    assert_eq!(result.rows[0][4], ResultValue::Null);
    assert!(!result.truncated);
    assert!(serde_json::to_vec(&result).unwrap().len() <= MAX_RESULT_BYTES);
}

#[test]
fn named_parameters_boolean_binding_and_empty_results_are_exact() {
    let (_root, service) = fixture();
    let result = run(service.execute(
        "catalog",
        QueryRequest {
            sql: "SELECT :enabled AS enabled, kind FROM entity WHERE kind = :kind AND id > :minimum ORDER BY id"
                .into(),
            parameters: vec![
                QueryParameter::named(":enabled", QueryParameterValue::Boolean(true)),
                QueryParameter::named(":kind", QueryParameterValue::Text("missing".into())),
                QueryParameter::named(":minimum", QueryParameterValue::Integer("0".into())),
            ],
            row_limit: Some(10),
        },
        CancellationToken::new(),
    ))
    .unwrap();
    assert_eq!(result.row_count, 0);
    assert!(result.rows.is_empty());
    assert_eq!(result.columns.len(), 2);
}

#[test]
fn mutation_multi_statement_and_invalid_bindings_are_rejected_without_changes() {
    let (root, service) = fixture();
    let catalog = root.path().join("catalog.sqlite");
    let before = fs::read(&catalog).unwrap();
    for sql in [
        "UPDATE entity SET kind = 'changed'",
        "DROP TABLE entity",
        "PRAGMA user_version = 4",
        "ATTACH DATABASE ':memory:' AS extra",
        "SELECT 1; SELECT 2",
    ] {
        let error =
            run(service.execute("catalog", QueryRequest::new(sql), CancellationToken::new()))
                .unwrap_err();
        assert!(
            matches!(error.code(), "query_denied" | "query_invalid"),
            "unexpected code for {sql}: {}",
            error.code()
        );
    }
    let mixed = QueryRequest {
        sql: "SELECT ?1, :named".into(),
        parameters: vec![
            QueryParameter::positional(QueryParameterValue::Integer("1".into())),
            QueryParameter::named(":named", QueryParameterValue::Integer("2".into())),
        ],
        row_limit: None,
    };
    assert_eq!(
        run(service.execute("catalog", mixed, CancellationToken::new()))
            .unwrap_err()
            .code(),
        "invalid_request"
    );
    assert_eq!(fs::read(catalog).unwrap(), before);
}

#[test]
fn row_and_byte_limits_return_only_complete_rows() {
    let (_root, service) = fixture();
    let rows = run(service.execute(
        "catalog",
        QueryRequest {
            sql: "SELECT id FROM entity ORDER BY id".into(),
            parameters: vec![],
            row_limit: Some(2),
        },
        CancellationToken::new(),
    ))
    .unwrap();
    assert_eq!(rows.row_count, 2);
    assert!(rows.truncated);
    assert_eq!(rows.truncation_reason.as_deref(), Some("rows"));

    let oversized = run(service.execute(
        "catalog",
        QueryRequest::new("SELECT zeroblob(1048576)"),
        CancellationToken::new(),
    ))
    .unwrap_err();
    assert_eq!(oversized.code(), "result_too_large");

    let truncated = run(service.execute(
        "catalog",
        QueryRequest::new("SELECT zeroblob(600000) AS payload UNION ALL SELECT zeroblob(600000)"),
        CancellationToken::new(),
    ))
    .unwrap();
    assert_eq!(truncated.row_count, 1);
    assert!(truncated.truncated);
    assert_eq!(truncated.truncation_reason.as_deref(), Some("bytes"));
    assert!(serde_json::to_vec(&truncated).unwrap().len() <= MAX_RESULT_BYTES);
}

#[test]
fn malformed_values_and_every_request_bound_fail_before_execution() {
    let (_root, service) = fixture();
    let invalid = [
        QueryRequest::new(" "),
        QueryRequest::new("x".repeat(16 * 1024 + 1)),
        QueryRequest {
            sql: "SELECT 1".into(),
            parameters: vec![],
            row_limit: Some(0),
        },
        QueryRequest {
            sql: "SELECT 1".into(),
            parameters: vec![],
            row_limit: Some(1001),
        },
        QueryRequest {
            sql: "SELECT ?2".into(),
            parameters: vec![
                QueryParameter::positional(QueryParameterValue::Integer("1".into())),
                QueryParameter::positional(QueryParameterValue::Integer("2".into())),
            ],
            row_limit: None,
        },
        QueryRequest {
            sql: "SELECT ?1".into(),
            parameters: vec![QueryParameter::positional(QueryParameterValue::Integer(
                "not-an-integer".into(),
            ))],
            row_limit: None,
        },
        QueryRequest {
            sql: "SELECT ?1".into(),
            parameters: vec![QueryParameter::positional(QueryParameterValue::Integer(
                "01".into(),
            ))],
            row_limit: None,
        },
        QueryRequest {
            sql: "SELECT ?1".into(),
            parameters: vec![QueryParameter::positional(QueryParameterValue::Real(
                "NaN".into(),
            ))],
            row_limit: None,
        },
        QueryRequest {
            sql: "SELECT ?1".into(),
            parameters: vec![QueryParameter::positional(QueryParameterValue::Blob(
                "not base64".into(),
            ))],
            row_limit: None,
        },
    ];
    for request in invalid {
        assert_eq!(
            run(service.execute("catalog", request, CancellationToken::new()))
                .unwrap_err()
                .code(),
            "invalid_request"
        );
    }
    let parameters = (0..65)
        .map(|_| QueryParameter::positional(QueryParameterValue::Null))
        .collect();
    let error = run(service.execute(
        "catalog",
        QueryRequest {
            sql: "SELECT 1".into(),
            parameters,
            row_limit: None,
        },
        CancellationToken::new(),
    ))
    .unwrap_err();
    assert_eq!(error.code(), "invalid_request");
}

#[test]
fn concurrency_and_database_lock_fail_immediately_and_retryably() {
    let (root, service) = fixture();
    let long_sql = "WITH RECURSIVE counter(value) AS (VALUES(0) UNION ALL SELECT value + 1 FROM counter WHERE value < 1000000000) SELECT sum(value) FROM counter";
    let first_cancel = CancellationToken::new();
    let second_cancel = CancellationToken::new();
    run(async {
        let first_service = service.clone();
        let first_token = first_cancel.clone();
        let first = tokio::spawn(async move {
            first_service
                .execute("catalog", QueryRequest::new(long_sql), first_token)
                .await
        });
        let second_service = service.clone();
        let second_token = second_cancel.clone();
        let second = tokio::spawn(async move {
            second_service
                .execute("catalog", QueryRequest::new(long_sql), second_token)
                .await
        });
        tokio::time::sleep(Duration::from_millis(25)).await;
        let busy = service
            .execute(
                "catalog",
                QueryRequest::new("SELECT 1"),
                CancellationToken::new(),
            )
            .await
            .unwrap_err();
        assert_eq!(busy.code(), "query_busy");
        assert!(busy.retryable());
        first_cancel.cancel();
        second_cancel.cancel();
        let _ = first.await.unwrap();
        let _ = second.await.unwrap();
    });

    let blocker = Connection::open(root.path().join("catalog.sqlite")).unwrap();
    blocker.execute_batch("BEGIN EXCLUSIVE").unwrap();
    let locked = run(service.execute(
        "catalog",
        QueryRequest::new("SELECT id FROM entity"),
        CancellationToken::new(),
    ))
    .unwrap_err();
    assert_eq!(locked.code(), "database_busy");
    assert!(locked.retryable());
}

#[test]
fn catalog_replacement_affects_later_requests_only() {
    let (root, service) = fixture();
    let replacement = root.path().join("replacement.sqlite");
    let connection = Connection::open(&replacement).unwrap();
    connection
        .execute_batch("CREATE TABLE marker (value TEXT); INSERT INTO marker VALUES ('new');")
        .unwrap();
    drop(connection);
    let prior = run(service.execute(
        "catalog",
        QueryRequest::new("SELECT kind FROM entity WHERE id = 1"),
        CancellationToken::new(),
    ))
    .unwrap();
    service.set_catalog_path(replacement);
    let current = run(service.execute(
        "catalog",
        QueryRequest::new("SELECT value FROM marker"),
        CancellationToken::new(),
    ))
    .unwrap();
    assert_eq!(prior.rows[0][0], ResultValue::Text("skill".into()));
    assert_eq!(current.rows[0][0], ResultValue::Text("new".into()));
}

#[test]
fn deadlines_and_generation_cancellation_interrupt_sqlite() {
    let (_root, service) = fixture();
    let long_sql = "WITH RECURSIVE counter(value) AS (VALUES(0) UNION ALL SELECT value + 1 FROM counter WHERE value < 1000000000) SELECT sum(value) FROM counter";
    let started = Instant::now();
    let timeout = run(service.execute(
        "catalog",
        QueryRequest::new(long_sql),
        CancellationToken::new(),
    ))
    .unwrap_err();
    assert_eq!(timeout.code(), "query_timeout");
    assert!(started.elapsed() < Duration::from_millis(2500));

    let cancellation = CancellationToken::new();
    let child = cancellation.clone();
    let service_clone = service.clone();
    let error = run(async move {
        let query = service_clone.execute("catalog", QueryRequest::new(long_sql), child);
        tokio::pin!(query);
        tokio::select! {
            result = &mut query => result.unwrap_err(),
            _ = tokio::time::sleep(Duration::from_millis(25)) => {
                cancellation.cancel();
                query.await.unwrap_err()
            }
        }
    });
    assert_eq!(error.code(), "service_stopping");
}
