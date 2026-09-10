use serde_json::Value;

use super::{
    CollectorError, MAX_CAPTURE_STRING_BYTES, MAX_CHUNK_BYTES, MAX_PARSE_DEPTH, MAX_PARSE_TOKENS,
    MAX_TABLE_ENTRIES,
};
use crate::saved_variables::{self, EmptyTable, ParseLimits};

const ROOT: &str = "EsoWeaveCollectorSaved";

pub fn parse_saved_variables(source: &str) -> Result<Value, CollectorError> {
    saved_variables::parse_assignment(
        source,
        ROOT,
        ParseLimits {
            max_depth: MAX_PARSE_DEPTH,
            max_tokens: MAX_PARSE_TOKENS,
            max_entries: MAX_TABLE_ENTRIES,
            max_string_bytes: MAX_CAPTURE_STRING_BYTES.max(MAX_CHUNK_BYTES),
        },
        EmptyTable::Array,
    )
    .map_err(|error| CollectorError::Validation(error.to_string()))
}
