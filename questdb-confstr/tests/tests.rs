/*******************************************************************************
 *     ___                  _   ____  ____
 *    / _ \ _   _  ___  ___| |_|  _ \| __ )
 *   | | | | | | |/ _ \/ __| __| | | |  _ \
 *   | |_| | |_| |  __/\__ \ |_| |_| | |_) |
 *    \__\_\\__,_|\___||___/\__|____/|____/
 *
 *  Copyright (c) 2014-2019 Appsicle
 *  Copyright (c) 2019-2024 QuestDB
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

use questdb_confstr::{parse_conf_str, ErrorKind, ParsingError};
use std::collections::HashMap;

#[test]
fn empty() -> Result<(), ParsingError> {
    let input = "";
    let config = parse_conf_str(input);
    assert!(config.is_err());
    let err = config.unwrap_err();
    assert_eq!(err.kind().clone(), ErrorKind::ExpectedIdentifierNotEmpty);
    assert_eq!(err.position(), 0);
    assert_eq!(
        err.to_string(),
        "expected identifier, not an empty string at position 0"
    );
    Ok(())
}

#[test]
fn basic() -> Result<(), ParsingError> {
    let input = "http::host=127.0.0.1;port=9000;";
    let config = parse_conf_str(input)?;
    assert_eq!(config.service(), "http");
    assert_eq!(config.get("host"), Some("127.0.0.1"));
    assert_eq!(config.get("port"), Some("9000"));
    assert_eq!(format!("{:?}", config), "ConfStr { service: \"http\", .. }");
    Ok(())
}

#[test]
fn case_sensitivity() -> Result<(), ParsingError> {
    let input = "TcP::Host=LoCaLhOsT;Port=9000;";
    let config = parse_conf_str(input)?;
    assert_eq!(config.service(), "TcP");
    assert_eq!(config.get("Host"), Some("LoCaLhOsT"));
    assert_eq!(config.get("host"), None);
    assert_eq!(config.get("Port"), Some("9000"));
    assert_eq!(config.get("port"), None);
    Ok(())
}

#[test]
fn duplicate_key() {
    let input = "http::host=127.0.0.1;host=localhost;port=9000;";
    let config = parse_conf_str(input);
    assert!(config.is_err());
    let err = config.unwrap_err();
    assert_eq!(
        err.kind().clone(),
        ErrorKind::DuplicateKey("host".to_string())
    );
    assert_eq!(err.position(), 21);
    assert_eq!(err.to_string(), "duplicate key \"host\" at position 21");
}

#[test]
fn key_can_start_with_number() -> Result<(), ParsingError> {
    let input = "https::123=456;";
    let config = parse_conf_str(input)?;
    assert_eq!(config.service(), "https");
    let mut expected = HashMap::new();
    expected.insert("123".to_string(), "456".to_string());
    assert_eq!(config.params(), &expected);
    Ok(())
}

#[test]
fn identifiers_can_contain_underscores() -> Result<(), ParsingError> {
    let input = "_A_::__x_Y__=42;";
    let config = parse_conf_str(input)?;
    assert_eq!(config.service(), "_A_");
    let mut expected = HashMap::new();
    expected.insert("__x_Y__".to_string(), "42".to_string());
    Ok(())
}

#[test]
fn key_must_be_alphanumeric() {
    let input = "https::ho st=localhost;";
    let config = parse_conf_str(input);
    assert!(config.is_err());
    let err = config.unwrap_err();
    assert_eq!(err.kind().clone(), ErrorKind::MustBeAlphanumeric(' '));
    assert_eq!(err.position(), 9);
    assert_eq!(
        err.to_string(),
        "must be alphanumeric, not ' ' at position 9"
    );
}

#[test]
fn key_cannot_be_empty() {
    let input = "https::=localhost;";
    let config = parse_conf_str(input);
    assert!(config.is_err());
    let err = config.unwrap_err();
    assert_eq!(err.kind().clone(), ErrorKind::ExpectedIdentifierNot('='));
    assert_eq!(err.position(), 7);
    assert_eq!(
        err.to_string(),
        "expected identifier to start with ascii letter, not '=' at position 7"
    );
}

#[test]
fn no_params_with_colons() -> Result<(), ParsingError> {
    let input = "https::";
    let config = parse_conf_str(input)?;
    assert_eq!(config.service(), "https");
    assert!(config.params().is_empty());
    Ok(())
}

#[test]
fn no_params_no_colons() -> Result<(), ParsingError> {
    let input = "https";
    let config = parse_conf_str(input)?;
    assert_eq!(config.service(), "https");
    assert!(config.params().is_empty());
    Ok(())
}

#[test]
fn bad_service_name_separator1() {
    let input = "x;/host=localhost;";
    let config = parse_conf_str(input);
    assert!(config.is_err());
    let err = config.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::BadSeparator((':', ';')));
    assert_eq!(err.position(), 1);
    assert_eq!(
        err.to_string(),
        "bad separator, expected ':' got ';' at position 1"
    );
}

#[test]
fn bad_service_name_separator2() {
    let input = "x:;host=localhost;";
    let config = parse_conf_str(input);
    assert!(config.is_err());
    let err = config.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::BadSeparator((':', ';')));
    assert_eq!(err.position(), 2);
    assert_eq!(
        err.to_string(),
        "bad separator, expected ':' got ';' at position 2"
    );
}

#[test]
fn url_as_service_separator() {
    let input = "http://localhost:9000;host=localhost;";
    let config = parse_conf_str(input);
    assert!(config.is_err());
    let err = config.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::BadSeparator((':', '/')));
    assert_eq!(err.position(), 5);
    assert_eq!(
        err.to_string(),
        "bad separator, expected ':' got '/' at position 5"
    );
}

#[test]
fn no_service_name_with_colons() {
    let input = "::host=localhost;";
    let config = parse_conf_str(input);
    assert!(config.is_err());
    let err = config.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::ExpectedIdentifierNot(':'));
    assert_eq!(err.position(), 0);
    assert_eq!(
        err.to_string(),
        "expected identifier to start with ascii letter, not ':' at position 0"
    );
}

#[test]
fn no_service_name_no_colons() {
    let input = "host=localhost;";
    let config = parse_conf_str(input);
    assert!(config.is_err());
    let err = config.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::BadSeparator((':', '=')));
    assert_eq!(err.position(), 4);
    assert_eq!(
        err.to_string(),
        "bad separator, expected ':' got '=' at position 4"
    );
}

#[test]
fn bad_key_value_separator_semicolon() {
    let input = "http::host=localhost;port9000;";
    let config = parse_conf_str(input);
    assert!(config.is_err());
    let err = config.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::BadSeparator(('=', ';')));
    assert_eq!(err.position(), 29);
    assert_eq!(
        err.to_string(),
        "bad separator, expected '=' got ';' at position 29"
    );
}

#[test]
fn bad_key_value_separator_colon() {
    let input = "s3::host:localhost;port=9000;";
    let config = parse_conf_str(input);
    assert!(config.is_err());
    let err = config.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::BadSeparator(('=', ':')));
    assert_eq!(err.position(), 8);
    assert_eq!(
        err.to_string(),
        "bad separator, expected '=' got ':' at position 8"
    );
}

#[test]
fn test_incomplete_key_no_value() {
    let input = "http::host";
    let config = parse_conf_str(input);
    assert!(config.is_err());
    let err = config.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::IncompleteKeyValue);
    assert_eq!(err.position(), 10);
    assert_eq!(
        err.to_string(),
        "incomplete key-value pair before end of input at position 10"
    );
}

#[test]
fn missing_trailing_semicolon() {
    let input = "http::host=localhost;port=9000";
    let config = parse_conf_str(input).unwrap();
    assert_eq!(config.service(), "http");
    assert_eq!(config.get("host"), Some("localhost"));
    assert_eq!(config.get("port"), Some("9000"));
}

#[test]
fn escaped_semicolon_missing_trailing() {
    let input = "http::host=localhost;port=9000;;";
    let config = parse_conf_str(input).unwrap();
    assert_eq!(config.service(), "http");
    assert_eq!(config.get("host"), Some("localhost"));
    assert_eq!(config.get("port"), Some("9000;"));
}

#[test]
fn escaped_semicolon() -> Result<(), ParsingError> {
    let input = "FTP::HOSTS=abc.com;;def.com;;ghi.net;PORTS=9000;;8000;;7000;;;";
    let config = parse_conf_str(input)?;
    assert_eq!(config.service(), "FTP");
    assert_eq!(config.get("HOSTS"), Some("abc.com;def.com;ghi.net"));
    assert_eq!(config.get("PORTS"), Some("9000;8000;7000;"));
    Ok(())
}

#[test]
fn byte_not_char_position() {
    let input = "p::n=静;:=42;";
    let config = parse_conf_str(input);
    assert!(config.is_err());
    let err = config.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::ExpectedIdentifierNot(':'));
    assert_eq!(err.position(), 9);
    assert_eq!(
        err.to_string(),
        "expected identifier to start with ascii letter, not ':' at position 9"
    );
}

#[test]
fn unicode_service_name() {
    let input = "協定";
    let config = parse_conf_str(input);
    assert!(config.is_err());
    let err = config.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::ExpectedIdentifierNot('協'));
    assert_eq!(err.position(), 0);
    assert_eq!(
        err.to_string(),
        "expected identifier to start with ascii letter, not '協' at position 0"
    );
}

#[test]
fn unicode_second_key_letter() {
    let input = "http::x協定=42;";
    let config = parse_conf_str(input);
    assert!(config.is_err());
    let err = config.unwrap_err();
    assert_eq!(err.kind(), ErrorKind::MustBeAlphanumeric('協'));
    assert_eq!(err.position(), 7);
    assert_eq!(
        err.to_string(),
        "must be alphanumeric, not '協' at position 7"
    );
}

#[test]
fn unicode_value() -> Result<(), ParsingError> {
    let input = "http::x=協定;";
    let config = parse_conf_str(input)?;
    assert_eq!(config.service(), "http");
    assert_eq!(config.get("x"), Some("協定"));
    Ok(())
}

#[test]
fn invalid_ctrl_chars_in_value() {
    let bad_chars = [
        '\x00', '\x01', '\x02', '\x03', '\x04', '\x1f', '\x7f', '\u{80}', '\u{8a}', '\u{9f}',
    ];
    for bad in bad_chars {
        let input = format!("http::x={};", bad);
        let config = parse_conf_str(&input);
        assert!(config.is_err());
        let err = config.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidCharInValue(bad));
        assert_eq!(err.position(), 8);
        assert_eq!(
            err.to_string(),
            format!("invalid char {:?} in value at position 8", bad)
        );
    }
}

#[test]
fn empty_value() -> Result<(), ParsingError> {
    let input = "http::x=;";
    let config = parse_conf_str(input)?;
    assert_eq!(config.service(), "http");
    assert_eq!(config.get("x"), Some(""));
    Ok(())
}
