use std::{error::Error, fmt};

#[derive(Debug)]
pub enum SerializeError {
    BufferOverflow,
}

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerializeError::BufferOverflow => write!(f, "Buffer overflow"),
        }
    }
}

impl Error for SerializeError {}

#[derive(Debug)]
pub enum DeserializeError {
    Invalid,
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeserializeError::Invalid => write!(f, "Invalid payload"),
        }
    }
}

impl Error for DeserializeError {}
