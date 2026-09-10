//! Restricted, bounded SavedVariables table parsing without Lua execution.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Number, Value};

#[derive(Debug, Clone, Copy)]
pub(crate) struct ParseLimits {
    pub max_depth: usize,
    pub max_tokens: usize,
    pub max_entries: usize,
    pub max_string_bytes: usize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum EmptyTable {
    Array,
    Object,
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub(crate) struct ParseError(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Key {
    String(String),
    Integer(u64),
}

#[derive(Debug, Clone)]
enum ParsedValue {
    Nil,
    Bool(bool),
    Integer(i64),
    String(String),
    Table(Vec<(Key, ParsedValue)>),
}

pub(crate) fn parse_assignment(
    source: &str,
    expected_root: &str,
    limits: ParseLimits,
    empty_table: EmptyTable,
) -> Result<Value, ParseError> {
    let mut parser = Parser {
        source,
        limits,
        offset: 0,
        tokens: 0,
        entries: 0,
    };
    parser.skip_whitespace();
    let root = parser.identifier()?;
    if root != expected_root {
        return rejected("unexpected SavedVariables root");
    }
    parser.skip_whitespace();
    parser.expect_byte(b'=')?;
    let value = parser.value(0)?;
    parser.skip_whitespace();
    if !parser.at_end() {
        return rejected("trailing syntax after SavedVariables envelope");
    }
    to_json(value, empty_table)
}

struct Parser<'a> {
    source: &'a str,
    limits: ParseLimits,
    offset: usize,
    tokens: usize,
    entries: usize,
}

impl Parser<'_> {
    fn value(&mut self, depth: usize) -> Result<ParsedValue, ParseError> {
        self.token()?;
        if depth > self.limits.max_depth {
            return rejected("SavedVariables table exceeds the nesting limit");
        }
        self.skip_whitespace();
        match self.peek_byte() {
            Some(b'{') => self.table(depth + 1),
            Some(b'"') => self.string().map(ParsedValue::String),
            Some(b'-' | b'0'..=b'9') => self.integer().map(ParsedValue::Integer),
            Some(b't') if self.consume_keyword("true") => Ok(ParsedValue::Bool(true)),
            Some(b'f') if self.consume_keyword("false") => Ok(ParsedValue::Bool(false)),
            Some(b'n') if self.consume_keyword("nil") => Ok(ParsedValue::Nil),
            _ => rejected("SavedVariables contains unsupported or executable syntax"),
        }
    }

    fn table(&mut self, depth: usize) -> Result<ParsedValue, ParseError> {
        self.expect_byte(b'{')?;
        let mut values = Vec::new();
        let mut keys = BTreeSet::new();
        loop {
            self.skip_whitespace();
            if self.consume_byte(b'}') {
                break;
            }
            self.expect_byte(b'[')?;
            self.skip_whitespace();
            let key = match self.peek_byte() {
                Some(b'"') => Key::String(self.string()?),
                Some(b'0'..=b'9') => Key::Integer(self.unsigned_integer()?),
                _ => {
                    return rejected(
                        "SavedVariables table keys must be strings or positive integers",
                    )
                }
            };
            self.skip_whitespace();
            self.expect_byte(b']')?;
            self.skip_whitespace();
            self.expect_byte(b'=')?;
            if !keys.insert(key.clone()) {
                return rejected("SavedVariables table contains a duplicate key");
            }
            self.entries += 1;
            if self.entries > self.limits.max_entries {
                return rejected("SavedVariables exceeds the table-entry limit");
            }
            values.push((key, self.value(depth)?));
            self.skip_whitespace();
            if self.consume_byte(b',') {
                continue;
            }
            if self.peek_byte() != Some(b'}') {
                return rejected("SavedVariables table entries require commas");
            }
        }
        Ok(ParsedValue::Table(values))
    }

    fn string(&mut self) -> Result<String, ParseError> {
        self.expect_byte(b'"')?;
        let mut result = String::new();
        loop {
            let Some(byte) = self.peek_byte() else {
                return rejected("unterminated SavedVariables string");
            };
            if byte == b'"' {
                self.offset += 1;
                break;
            }
            if byte == b'\\' {
                self.offset += 1;
                let Some(escaped) = self.peek_byte() else {
                    return rejected("unterminated SavedVariables escape");
                };
                self.offset += 1;
                let value = match escaped {
                    b'"' => '"',
                    b'\\' => '\\',
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    b'b' => '\u{0008}',
                    b'f' => '\u{000c}',
                    _ => return rejected("SavedVariables contains an unsupported string escape"),
                };
                result.push(value);
            } else {
                if byte < 0x20 {
                    return rejected("SavedVariables string contains an unescaped control byte");
                }
                let rest = &self.source[self.offset..];
                let character = rest
                    .chars()
                    .next()
                    .ok_or_else(|| ParseError("invalid UTF-8 boundary".into()))?;
                result.push(character);
                self.offset += character.len_utf8();
            }
            if result.len() > self.limits.max_string_bytes {
                return rejected("SavedVariables string exceeds the parser limit");
            }
        }
        Ok(result)
    }

    fn integer(&mut self) -> Result<i64, ParseError> {
        let negative = self.consume_byte(b'-');
        let unsigned = self.unsigned_integer()?;
        if negative {
            if unsigned == i64::MAX as u64 + 1 {
                return Ok(i64::MIN);
            }
            let magnitude = i64::try_from(unsigned)
                .map_err(|_| ParseError("integer is out of range".into()))?;
            Ok(-magnitude)
        } else {
            i64::try_from(unsigned).map_err(|_| ParseError("integer is out of range".into()))
        }
    }

    fn unsigned_integer(&mut self) -> Result<u64, ParseError> {
        let start = self.offset;
        while matches!(self.peek_byte(), Some(b'0'..=b'9')) {
            self.offset += 1;
        }
        if start == self.offset {
            return rejected("expected integer");
        }
        let token = &self.source[start..self.offset];
        if token.len() > 1 && token.starts_with('0') {
            return rejected("SavedVariables integers must use canonical decimal syntax");
        }
        token
            .parse()
            .map_err(|_| ParseError("integer is out of range".into()))
    }

    fn identifier(&mut self) -> Result<&str, ParseError> {
        let start = self.offset;
        while matches!(
            self.peek_byte(),
            Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_')
        ) {
            self.offset += 1;
        }
        if start == self.offset {
            return rejected("expected SavedVariables root");
        }
        Ok(&self.source[start..self.offset])
    }

    fn consume_keyword(&mut self, keyword: &str) -> bool {
        if self.source[self.offset..].starts_with(keyword) {
            let end = self.offset + keyword.len();
            if self
                .source
                .as_bytes()
                .get(end)
                .is_none_or(|byte| !matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'))
            {
                self.offset = end;
                return true;
            }
        }
        false
    }

    fn token(&mut self) -> Result<(), ParseError> {
        self.tokens += 1;
        if self.tokens > self.limits.max_tokens {
            rejected("SavedVariables exceeds the parser token limit")
        } else {
            Ok(())
        }
    }

    fn expect_byte(&mut self, expected: u8) -> Result<(), ParseError> {
        self.skip_whitespace();
        if self.consume_byte(expected) {
            Ok(())
        } else {
            rejected(format!("expected '{}'", char::from(expected)))
        }
    }

    fn consume_byte(&mut self, expected: u8) -> bool {
        if self.peek_byte() == Some(expected) {
            self.offset += 1;
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek_byte(), Some(b' ' | b'\t' | b'\r' | b'\n')) {
            self.offset += 1;
        }
    }

    fn peek_byte(&self) -> Option<u8> {
        self.source.as_bytes().get(self.offset).copied()
    }

    fn at_end(&self) -> bool {
        self.offset == self.source.len()
    }
}

fn to_json(value: ParsedValue, empty_table: EmptyTable) -> Result<Value, ParseError> {
    match value {
        ParsedValue::Nil => Ok(Value::Null),
        ParsedValue::Bool(value) => Ok(Value::Bool(value)),
        ParsedValue::Integer(value) => Ok(Value::Number(Number::from(value))),
        ParsedValue::String(value) => Ok(Value::String(value)),
        ParsedValue::Table(values) if values.is_empty() => Ok(match empty_table {
            EmptyTable::Array => Value::Array(Vec::new()),
            EmptyTable::Object => Value::Object(Map::new()),
        }),
        ParsedValue::Table(values) => table_to_json(values, empty_table),
    }
}

fn table_to_json(
    values: Vec<(Key, ParsedValue)>,
    empty_table: EmptyTable,
) -> Result<Value, ParseError> {
    if values.iter().all(|(key, _)| matches!(key, Key::String(_))) {
        let mut object = Map::new();
        for (key, value) in values {
            let Key::String(key) = key else {
                unreachable!();
            };
            object.insert(key, to_json(value, empty_table)?);
        }
        return Ok(Value::Object(object));
    }
    if values.iter().all(|(key, _)| matches!(key, Key::Integer(_))) {
        let by_index: BTreeMap<_, _> = values
            .into_iter()
            .map(|(key, value)| {
                let Key::Integer(key) = key else {
                    unreachable!();
                };
                (key, value)
            })
            .collect();
        if by_index.keys().copied().ne(1..=by_index.len() as u64) {
            return rejected("SavedVariables arrays must use contiguous one-based indexes");
        }
        return by_index
            .into_values()
            .map(|value| to_json(value, empty_table))
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Array);
    }
    rejected("SavedVariables tables cannot mix string and integer keys")
}

fn rejected<T>(message: impl Into<String>) -> Result<T, ParseError> {
    Err(ParseError(message.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIMITS: ParseLimits = ParseLimits {
        max_depth: 4,
        max_tokens: 20,
        max_entries: 20,
        max_string_bytes: 32,
    };

    #[test]
    fn parses_only_the_expected_assignment_and_data_grammar() {
        let parsed = parse_assignment(
            "Root = {[\"flag\"] = true, [\"number\"] = -2, [\"none\"] = nil}",
            "Root",
            LIMITS,
            EmptyTable::Object,
        )
        .unwrap();
        assert_eq!(parsed["flag"], true);
        assert_eq!(parsed["number"], -2);
        assert!(parsed["none"].is_null());

        for invalid in [
            "Other = {}",
            "Root = function() end",
            "Root = setmetatable({}, {})",
            "Root = {}; os.execute(\"x\")",
            "Root = {[\"a\"] = 1, [\"a\"] = 2}",
            "Root = {[1] = true, [3] = false}",
            "Root = {[1] = true, [\"two\"] = false}",
        ] {
            assert!(parse_assignment(invalid, "Root", LIMITS, EmptyTable::Object).is_err());
        }
    }

    #[test]
    fn empty_table_policy_is_explicit() {
        let array = parse_assignment("Root = {}", "Root", LIMITS, EmptyTable::Array).unwrap();
        let object = parse_assignment("Root = {}", "Root", LIMITS, EmptyTable::Object).unwrap();
        assert!(array.is_array());
        assert!(object.is_object());
    }
}
