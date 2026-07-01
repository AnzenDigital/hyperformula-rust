use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum Error {
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Type error: expected {expected}, got {actual}")]
    TypeError { expected: String, actual: String },

    #[error("Division by zero")]
    DivisionByZero,

    #[error("Value error: {0}")]
    ValueError(String),

    #[error("Reference error: {0}")]
    ReferenceError(String),

    #[error("Name error: unknown function {0}")]
    NameError(String),

    #[error("#N/A")]
    NA,

    #[error("#VALUE!")]
    Value,

    #[error("#DIV/0!")]
    Div0,

    #[error("#NAME?")]
    Name,

    #[error("#NUM!")]
    Num,

    #[error("#REF!")]
    Ref,

    #[error("#NULL!")]
    Null,
}

pub type Result<T> = std::result::Result<T, Error>;
