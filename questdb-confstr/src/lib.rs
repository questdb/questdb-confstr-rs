/*******************************************************************************
 *     ___                  _   ____  ____
 *    / _ \ _   _  ___  ___| |_|  _ \| __ )
 *   | | | | | | |/ _ \/ __| __| | | |  _ \
 *   | |_| | |_| |  __/\__ \ |_| |_| | |_) |
 *    \__\_\\__,_|\___||___/\__|____/|____/
 *
 *  Copyright (c) 2014-2019 Appsicle
 *  Copyright (c)  2019-2025 QuestDB
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *
 ******************************************************************************/

#![doc = include_str!("../README.md")]

use crate::peekable2::{Peekable2, Peekable2Ext};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::str::CharIndices;

/// Error returned by [`ConfStr::get`] when a key appears more than once.
///
/// Use [`ConfStr::get_all`] instead if the key is expected to have multiple values.
#[derive(Debug, Clone)]
pub struct DuplicateKeyError {
    key: String,
}

impl DuplicateKeyError {
    /// The key that appeared more than once.
    pub fn key(&self) -> &str {
        &self.key
    }
}

impl Display for DuplicateKeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "key {:?} appears more than once; use get_all() for duplicate keys",
            self.key
        )
    }
}

impl std::error::Error for DuplicateKeyError {}

mod peekable2;

/// Parameter keys are ascii lowercase strings.
pub type Key = String;

/// Parameter values are strings.
pub type Value = String;

/// Parameters are stored in a `Vec` of `(Key, Value)` pairs.
/// Duplicate keys are allowed (e.g. multiple `addr` entries for multi-url support).
pub type Params = Vec<(Key, Value)>;

/// Parsed configuration string.
///
/// The parameters are stored in a `Vec` of `(Key, Value)` pairs.
pub struct ConfStr {
    service: String,
    params: Params,
}

impl Debug for ConfStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Params are hidden from debug output in case they contain sensitive information.
        write!(f, "ConfStr {{ service: {:?}, .. }}", self.service)
    }
}

impl ConfStr {
    /// Create a new configuration string object.
    pub fn new(service: String, params: Params) -> Self {
        ConfStr { service, params }
    }

    /// Access the service name.
    pub fn service(&self) -> &str {
        &self.service
    }

    /// Access the parameters.
    pub fn params(&self) -> &Params {
        &self.params
    }

    /// Get the value for a parameter key.
    ///
    /// Returns `Err(DuplicateKeyError)` if the key appears more than once.
    /// Use [`get_all`](Self::get_all) for keys that are expected to have multiple values.
    pub fn get(&self, key: &str) -> Result<Option<&str>, DuplicateKeyError> {
        let mut found: Option<&str> = None;
        for (k, v) in &self.params {
            if k == key {
                if found.is_some() {
                    return Err(DuplicateKeyError {
                        key: key.to_string(),
                    });
                }
                found = Some(v.as_str());
            }
        }
        Ok(found)
    }

    /// Get all values for a parameter key, in insertion order.
    /// Useful for keys that may appear multiple times (e.g. `addr`).
    pub fn get_all(&self, key: &str) -> Vec<&str> {
        self.params
            .iter()
            .filter(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
            .collect()
    }
}

/// Byte position in the input string where the parsing error occurred.
pub type Position = usize;

/// The type of parsing error.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    ExpectedIdentifierNot(char),
    MustBeAlphanumeric(char),
    ExpectedIdentifierNotEmpty,
    BadSeparator((char, char)),
    IncompleteKeyValue,
    InvalidCharInValue(char),
}

impl<'a> PartialEq<&'a ErrorKind> for ErrorKind {
    fn eq(&self, other: &&'a ErrorKind) -> bool {
        self == *other
    }
}

impl PartialEq<ErrorKind> for &ErrorKind {
    fn eq(&self, other: &ErrorKind) -> bool {
        *self == other
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Ensure no values are leaked in error messages.
        match self {
            ErrorKind::ExpectedIdentifierNot(c) => {
                write!(
                    f,
                    "expected identifier to start with ascii letter, not {:?}",
                    c
                )
            }
            ErrorKind::MustBeAlphanumeric(c) => write!(f, "must be alphanumeric, not {:?}", c),
            ErrorKind::ExpectedIdentifierNotEmpty => {
                write!(f, "expected identifier, not an empty string")
            }
            ErrorKind::BadSeparator((e, c)) => {
                write!(f, "bad separator, expected {:?} got {:?}", e, c)
            }
            ErrorKind::IncompleteKeyValue => {
                write!(f, "incomplete key-value pair before end of input")
            }
            ErrorKind::InvalidCharInValue(c) => write!(f, "invalid char {:?} in value", c),
        }
    }
}

/// The parsing error.
#[derive(Debug)]
pub struct ParsingError {
    kind: ErrorKind,
    position: usize,
}

impl ParsingError {
    /// Access the byte position in the input string where the parsing error occurred.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Access the type of parsing error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

fn parse_err(kind: ErrorKind, position: Position) -> ParsingError {
    ParsingError { kind, position }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} at position {}", self.kind, self.position)
    }
}

impl std::error::Error for ParsingError {}

fn parse_ident(
    iter: &mut Peekable2<CharIndices>,
    next_pos: &mut Position,
) -> Result<Key, ParsingError> {
    let mut token = String::new();
    while let Some((pos, c)) = iter.peek0() {
        *next_pos = *pos;
        if c.is_ascii_alphanumeric() || *c == '_' {
            token.push(*c);
            iter.next();
        } else {
            if token.is_empty() {
                return Err(parse_err(ErrorKind::ExpectedIdentifierNot(*c), *next_pos));
            } else if !c.is_ascii() || matches!(c, '\0'..=' ') {
                return Err(parse_err(ErrorKind::MustBeAlphanumeric(*c), *next_pos));
            }
            break;
        }
    }

    if token.is_empty() {
        return Err(parse_err(ErrorKind::ExpectedIdentifierNotEmpty, *next_pos));
    }

    Ok(token)
}

fn parse_value(
    iter: &mut Peekable2<CharIndices>,
    next_pos: &mut Position,
) -> Result<Value, ParsingError> {
    let mut value = String::new();
    loop {
        let c1 = iter.peek0().cloned();
        let c2 = iter.peek1().cloned();
        if let Some((p, _)) = c1 {
            *next_pos = p;
        }
        match (c1, c2) {
            (Some((_, ';')), Some((_, ';'))) => {
                let _ = iter.next();
                let _ = iter.next();
                value.push(';');
            }
            (Some((_, ';')), _) => break,
            (Some((p, c)), _) => {
                if matches!(c, '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}') {
                    return Err(parse_err(ErrorKind::InvalidCharInValue(c), p));
                }
                value.push(c);
                let _ = iter.next();
            }
            (None, _) => break,
        }
    }
    Ok(value)
}

fn parse_double_colon(
    iter: &mut Peekable2<CharIndices>,
    next_pos: &mut Position,
) -> Result<bool, ParsingError> {
    let c1 = iter.next();
    let c2 = iter.next();
    match (c1, c2) {
        (Some((_, ':')), Some((_, ':'))) => {
            *next_pos += 2;
            Ok(true)
        }
        (None, None) => Ok(false),
        (Some((_, ':')), Some((p, c))) => Err(parse_err(ErrorKind::BadSeparator((':', c)), p)),
        (Some((p, c)), _) => Err(parse_err(ErrorKind::BadSeparator((':', c)), p)),
        (None, _) => unreachable!("peekable2 guarantees that the second item is always None"),
    }
}

fn parse_params(
    iter: &mut Peekable2<CharIndices>,
    next_pos: &mut Position,
    input_len: usize,
) -> Result<Params, ParsingError> {
    let mut params = Params::new();
    while let Some((p, _)) = iter.peek0() {
        *next_pos = *p;
        let key = parse_ident(iter, next_pos)?;
        match iter.next() {
            Some((p, '=')) => *next_pos = p + 1,
            Some((p, c)) => return Err(parse_err(ErrorKind::BadSeparator(('=', c)), p)),
            None => return Err(parse_err(ErrorKind::IncompleteKeyValue, input_len)),
        }
        let value = parse_value(iter, next_pos)?;
        iter.next(); // skip ';', if present.
        params.push((key, value));
    }
    Ok(params)
}

/// Parse a config string.
///
/// ```
/// use questdb_confstr::parse_conf_str;
/// # use questdb_confstr::ParsingError;
/// let config = parse_conf_str("service::key1=value1;key2=value2;")?;
/// assert_eq!(config.service(), "service");
/// assert_eq!(config.get("key1").unwrap(), Some("value1"));
/// assert_eq!(config.get("key2").unwrap(), Some("value2"));
/// # Ok::<(), ParsingError>(())
/// ```
pub fn parse_conf_str(input: &str) -> Result<ConfStr, ParsingError> {
    let mut iter = input.char_indices().peekable2();
    let mut next_pos = 0;
    let service = parse_ident(&mut iter, &mut next_pos)?;
    let has_separator = parse_double_colon(&mut iter, &mut next_pos)?;
    if !has_separator {
        return Ok(ConfStr::new(service, Params::new()));
    }
    let params = parse_params(&mut iter, &mut next_pos, input.len())?;
    Ok(ConfStr::new(service, params))
}
