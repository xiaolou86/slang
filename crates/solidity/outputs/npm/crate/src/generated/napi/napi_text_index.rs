// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use napi_derive::napi;

use super::*;

#[napi(object, namespace = "text_index")]
#[derive(Copy, Clone)]
pub struct TextIndex {
    pub utf8: u32,
    pub utf16: u32,
    pub char: u32,
}

impl From<&RustTextIndex> for TextIndex {
    fn from(value: &RustTextIndex) -> Self {
        Self {
            utf8: value.utf8 as u32,
            utf16: value.utf16 as u32,
            char: value.char as u32,
        }
    }
}

#[napi(object, namespace = "text_index")]
#[derive(Copy, Clone)]
pub struct TextRange {
    pub start: TextIndex,
    pub end: TextIndex,
}

impl From<&RustTextRange> for TextRange {
    fn from(value: &RustTextRange) -> Self {
        Self {
            start: (&value.start).into(),
            end: (&value.end).into(),
        }
    }
}
