use crate::errors::{FunctionError, ImportError};
use leo_ast::{ParserError, SyntaxError};
use leo_inputs::InputParserError;
use leo_types::IntegerError;

use std::{io, path::PathBuf};

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("creating: {}", _0)]
    Creating(io::Error),

    #[error("Attempt to access current directory failed - {:?}", _0)]
    DirectoryError(io::Error),

    #[error("{}", _0)]
    ImportError(#[from] ImportError),

    #[error("{}", _0)]
    InputParserError(#[from] InputParserError),

    #[error("{}", _0)]
    IntegerError(#[from] IntegerError),

    #[error("{}", _0)]
    FunctionError(#[from] FunctionError),

    #[error("Cannot read from the provided file path - {:?}", _0)]
    FileReadError(PathBuf),

    #[error("Syntax error. Cannot parse the file")]
    FileParsingError,

    #[error("Main function not found")]
    NoMain,

    #[error("Main must be a function")]
    NoMainFunction,

    #[error("{}", _0)]
    ParserError(#[from] ParserError),

    #[error("{}", _0)]
    SyntaxError(#[from] SyntaxError),

    #[error("Unable to construct abstract syntax tree")]
    SyntaxTreeError,

    #[error("writing: {}", _0)]
    Writing(io::Error),
}