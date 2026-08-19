use crate::lexer::LexError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to run cir-opt: {0}")]
    Toolchain(String),
    #[error(transparent)]
    Lex(#[from] LexError),
    #[error("parse error at {line}:{col}: {msg}")]
    Parse {
        line: usize,
        col: usize,
        msg: String,
    },
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
