use std::convert::From;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct ParseError(String);

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> ParseError {
        ParseError(format!("there was a problem: {}", e))
    }
}

impl From<String> for ParseError {
    fn from(e: String) -> ParseError {
        ParseError(format!("there was a problem: {}", e))
    }
}