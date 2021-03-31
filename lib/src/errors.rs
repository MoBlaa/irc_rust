use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub enum ParserError {
    NoTagKeyEnd,
    NoTagValueEnd,
    NoCommand,
    PrefixWithoutName,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::NoTagKeyEnd => write!(f, "Tag Key has no ending '='"),
            ParserError::NoTagValueEnd => write!(f, "Tag Value has no ending ';' or ' '"),
            ParserError::NoCommand => write!(f, "Missing command in message"),
            ParserError::PrefixWithoutName => write!(f, "Prefix has to have name included"),
        }
    }
}

impl Error for ParserError {}
