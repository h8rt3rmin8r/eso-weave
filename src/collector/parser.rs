use serde_json::Value;

use super::{
    CollectorError, MAX_CAPTURE_STRING_BYTES, MAX_CHUNK_BYTES, MAX_PARSE_DEPTH, MAX_PARSE_TOKENS,
    MAX_TABLE_ENTRIES,
};
use crate::saved_variables::{self, EmptyTable, ParseLimits};

const ROOT: &str = "EsoWeaveDataSaved";

pub fn parse_saved_variables(source: &str) -> Result<Value, CollectorError> {
    parse_optional_saved_variables(source)?.ok_or_else(|| {
        CollectorError::Validation("shared SavedVariables root has no catalog module".into())
    })
}

pub fn parse_optional_saved_variables(source: &str) -> Result<Option<Value>, CollectorError> {
    let mut root = saved_variables::parse_assignment(
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
    .map_err(|error| CollectorError::Validation(error.to_string()))?;
    crate::data_addon::take_optional_module(&mut root, "catalog")
        .map_err(CollectorError::Validation)
}
