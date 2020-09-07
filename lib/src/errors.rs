use core::fmt;
use std::error::Error;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MessageBuildError {
    UserWithoutHost,
    MissingCommand
}

impl fmt::Display for MessageBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            MessageBuildError::UserWithoutHost => "irc prefix can only contain a user if host is also present",
            MessageBuildError::MissingCommand => "irc message requires an command"
        };

        write!(f, "{}", message)
    }
}

impl Error for MessageBuildError {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum InvalidIrcFormatError {
    Tag(String),
    NoTagEnd(String)
}

impl fmt::Display for InvalidIrcFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidIrcFormatError::Tag(raw) => write!(f, "Invalid tags format: {}", raw),
            InvalidIrcFormatError::NoTagEnd(raw_message) => write!(f, "No space to end the Tag found in message: {}", raw_message)
        }
    }
}

impl Error for InvalidIrcFormatError {}
