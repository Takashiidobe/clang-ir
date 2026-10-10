//! CIR enum attributes generated from TableGen enum definitions.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseEnumError {
    type_name: &'static str,
    input: String,
}
impl ParseEnumError {
    pub fn new(type_name: &'static str, input: impl Into<String>) -> Self {
        Self {
            type_name,
            input: input.into(),
        }
    }
}
impl std::fmt::Display for ParseEnumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}` is not a valid {}", self.input, self.type_name)
    }
}
impl std::error::Error for ParseEnumError {}