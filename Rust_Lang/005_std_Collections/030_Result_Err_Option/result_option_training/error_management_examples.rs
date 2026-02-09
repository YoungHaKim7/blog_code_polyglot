// 10 Examples of Error Management WITHOUT unwrap/expect
// All examples use idiomatic Result-based error handling

#![allow(dead_code)]

use std::fmt;
use std::fs;
use std::io::{self, Read, Write};
use std::num::ParseIntError;

// ============================================================================
// Example 1: Basic ok_or() Pattern - Option → Result
// ============================================================================
#[derive(Debug, PartialEq)]
enum ConfigError {
    MissingKey(String),
}

fn get_config_value(map: &std::collections::HashMap<String, String>, key: &str) -> Result<String, ConfigError> {
    map.get(key)
        .cloned()
        .ok_or_else(|| ConfigError::MissingKey(key.to_string()))
}

#[test]
fn ex1_basic_ok_or() {
    let mut map = std::collections::HashMap::new();
    map.insert("host".to_string(), "localhost".to_string());

    assert_eq!(get_config_value(&map, "host"), Ok("localhost".to_string()));
    assert_eq!(get_config_value(&map, "port"), Err(ConfigError::MissingKey("port".to_string())));
}

// ============================================================================
// Example 2: String Parsing with ? Operator
// ============================================================================
fn parse_port(s: &str) -> Result<u16, ParseIntError> {
    let port: u16 = s.parse()?;  // ? propagates the error
    Ok(port)
}

#[test]
fn ex2_parse_with_question_mark() {
    assert_eq!(parse_port("8080"), Ok(8080));
    assert!(parse_port("abc").is_err());
    assert!(parse_port("99999").is_err());  // Too large for u16
}

// ============================================================================
// Example 3: Full Error Enum with Display + std::error::Error
// ============================================================================
#[derive(Debug)]
enum AppError {
    MissingConfig(&'static str),
    InvalidPort(u16),
    Io(io::Error),
    Parse(ParseIntError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::MissingConfig(key) => write!(f, "Missing config key: {}", key),
            AppError::InvalidPort(port) => write!(f, "Invalid port: {} (must be 1-65535)", port),
            AppError::Io(err) => write!(f, "IO error: {}", err),
            AppError::Parse(err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Io(err) => Some(err),
            AppError::Parse(err) => Some(err),
            _ => None,
        }
    }
}

// ============================================================================
// Example 4: Chaining Multiple ? Operations
// ============================================================================
fn load_and_validate(config_str: &str) -> Result<u16, AppError> {
    // Simulated config loading
    let port_str = config_str
        .lines()
        .find(|line| line.starts_with("port="))
        .map(|line| line.trim_start_matches("port="))
        .ok_or(AppError::MissingConfig("port"))?;

    let port: u16 = port_str.parse().map_err(AppError::Parse)?;

    if port == 0 {
        return Err(AppError::InvalidPort(port));
    }

    Ok(port)
}

#[test]
fn ex4_chaining_question_mark() {
    assert!(matches!(load_and_validate("port=8080"), Ok(8080)));
    assert!(matches!(load_and_validate("port=0"), Err(AppError::InvalidPort(0))));
    assert!(matches!(load_and_validate("port=abc"), Err(AppError::Parse(_))));
    assert!(matches!(load_and_validate("host=localhost"), Err(AppError::MissingConfig(_))));
}

// ============================================================================
// Example 5: File I/O without unwrap
// ============================================================================
fn read_file_contents(path: &str) -> Result<String, AppError> {
    let mut file = fs::File::open(path).map_err(AppError::Io)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(AppError::Io)?;
    Ok(contents)
}

fn write_file_contents(path: &str, data: &str) -> Result<(), AppError> {
    let mut file = fs::File::create(path).map_err(AppError::Io)?;
    file.write_all(data.as_bytes()).map_err(AppError::Io)?;
    Ok(())
}

// ============================================================================
// Example 6: Result combinator - map()
// ============================================================================
fn get_port_or_default(result: Result<u16, AppError>) -> u16 {
    result.map(|port| port).unwrap_or(8080)  // unwrap_or is OK! It's not unwrap()
}

fn double_if_ok(value: Result<i32, AppError>) -> Result<i32, AppError> {
    value.map(|v| v * 2)
}

#[test]
fn ex6_result_map() {
    assert!(matches!(double_if_ok(Ok(5)), Ok(10)));
    assert!(double_if_ok(Err(AppError::MissingConfig("x"))).is_err());
}

// ============================================================================
// Example 7: Result combinator - and_then()
// ============================================================================
fn parse_then_validate(s: &str) -> Result<u16, AppError> {
    s.parse::<u16>()
        .map_err(AppError::Parse)
        .and_then(|port| {
            if port == 0 {
                Err(AppError::InvalidPort(port))
            } else {
                Ok(port)
            }
        })
}

#[test]
fn ex7_and_then() {
    assert!(matches!(parse_then_validate("8080"), Ok(8080)));
    assert!(matches!(parse_then_validate("0"), Err(AppError::InvalidPort(0))));
    assert!(matches!(parse_then_validate("abc"), Err(AppError::Parse(_))));
}

// ============================================================================
// Example 8: Combining Multiple Results - collect()
// ============================================================================
fn parse_all_ports(inputs: &[&str]) -> Result<Vec<u16>, AppError> {
    inputs
        .iter()
        .map(|s| s.parse::<u16>().map_err(AppError::Parse))
        .collect()
}

#[test]
fn ex8_collect_results() {
    assert!(matches!(parse_all_ports(&["80", "443", "8080"]), Ok(ports) if ports == &[80, 443, 8080]));
    assert!(parse_all_ports(&["80", "abc", "8080"]).is_err());
}

// ============================================================================
// Example 9: Enriching Error Context
// ============================================================================
#[derive(Debug)]
enum EnrichedError {
    ConfigLoadFailed { key: String, source: io::Error },
    InvalidValue { key: String, value: String },
}

impl fmt::Display for EnrichedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnrichedError::ConfigLoadFailed { key, source } => {
                write!(f, "Failed to load config key '{}': {}", key, source)
            }
            EnrichedError::InvalidValue { key, value } => {
                write!(f, "Invalid value '{}' for key '{}'", value, key)
            }
        }
    }
}

impl std::error::Error for EnrichedError {}

fn load_config_with_context(path: &str, key: &str) -> Result<String, EnrichedError> {
    let contents = fs::read_to_string(path).map_err(|e| EnrichedError::ConfigLoadFailed {
        key: key.to_string(),
        source: e,
    })?;

    contents
        .lines()
        .find_line(|line| line.starts_with(&format!("{}=", key)))
        .map(|line| line.trim_start_matches(&format!("{}=", key)))
        .ok_or_else(|| EnrichedError::InvalidValue {
            key: key.to_string(),
            value: "<not found>".to_string(),
        })
        .map(|s| s.to_string())
}

// Helper trait for the example above
trait FindLine: Iterator {
    fn find_line<F>(&mut self, pred: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool;
}

impl<I: Iterator> FindLine for I {
    fn find_line<F>(&mut self, mut pred: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool,
    {
        self.find(|x| pred(x))
    }
}

// ============================================================================
// Example 10: From Trait for Automatic Error Conversion
// ============================================================================
#[derive(Debug)]
enum UnifiedError {
    Io(io::Error),
    Parse(ParseIntError),
    Custom(String),
}

impl From<io::Error> for UnifiedError {
    fn from(err: io::Error) -> Self {
        UnifiedError::Io(err)
    }
}

impl From<ParseIntError> for UnifiedError {
    fn from(err: ParseIntError) -> Self {
        UnifiedError::Parse(err)
    }
}

impl fmt::Display for UnifiedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnifiedError::Io(e) => write!(f, "IO error: {}", e),
            UnifiedError::Parse(e) => write!(f, "Parse error: {}", e),
            UnifiedError::Custom(msg) => write!(f, "Custom error: {}", msg),
        }
    }
}

impl std::error::Error for UnifiedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            UnifiedError::Io(e) => Some(e),
            UnifiedError::Parse(e) => Some(e),
            UnifiedError::Custom(_) => None,
        }
    }
}

// Now ? works automatically for io::Error and ParseIntError!
fn read_port_from_file(path: &str) -> Result<u16, UnifiedError> {
    let contents = fs::read_to_string(path)?;  // io::Error → UnifiedError automatically
    let port: u16 = contents.trim().parse()?;  // ParseIntError → UnifiedError automatically
    Ok(port)
}

// ============================================================================
// Bonus: Type-safe error handling with thiserror crate pattern
// ============================================================================
// In real projects, use the thiserror crate for cleaner code:
/*
use thiserror::Error;

#[derive(Error, Debug)]
enum ModernError {
    #[error("Missing config: {0}")]
    MissingConfig(String),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] ParseIntError),
}
*/

fn main() {
    println!("Run with: cargo test -- --nocapture");
}
