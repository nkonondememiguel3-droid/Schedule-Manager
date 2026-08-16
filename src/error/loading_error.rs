use core::fmt;
use std::io;

#[derive(Debug)]
pub enum LoadingError {
    IoError(io::Error),
    DeserializationError(serde_json::Error),
}

impl fmt::Display for LoadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadingError::IoError(err) => write!(f, "Error while reading the JSON file : {}", err),
            LoadingError::DeserializationError(err) => {
                write!(f, "Error while deserializing the json object : {}", err)
            }
        }
    }
}
