// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use napi_derive::napi;

use super::*;
use napi_text_index::*;

#[napi(namespace = "parse_error")]
#[derive(PartialEq, Clone)]
pub struct ParseError(RustParseError);

impl From<RustParseError> for ParseError {
    fn from(value: RustParseError) -> Self {
        Self(value)
    }
}

#[napi(namespace = "parse_error")]
impl ParseError {
    #[napi(getter)]
    pub fn text_range(&self) -> TextRange {
        self.0.text_range().into()
    }

    pub fn tokens_that_would_have_allowed_more_progress(&self) -> Vec<String> {
        self.0
            .tokens_that_would_have_allowed_more_progress()
            .iter()
            .map(|x| x.to_string())
            .collect()
    }

    #[napi(namespace = "parse_error")]
    pub fn to_error_report(&self, source_id: String, source: String, with_colour: bool) -> String {
        self.0.to_error_report(&source_id, &source, with_colour)
    }
}
