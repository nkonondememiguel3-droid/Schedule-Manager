use core::fmt;
use std::io;

#[derive(Debug)]
pub enum CreationError {
    // error occuring the echeance-date argument less than the current date in the creation of a taks.
    WrongEcheanceDate,

    // error occuring whilte trying to serialize the data struture to json object.
    SerializationError(serde_json::Error),

    // error occuring while writing the object to a json file.
    WriteError(io::Error),
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreationError::WrongEcheanceDate => write!(f, "The echeance date is invalide."),
            CreationError::SerializationError(err) => {
                write!(f, "Error while serializing JSON : {}", err)
            }
            CreationError::WriteError(err) => {
                write!(f, "Error while writing into the file : {}", err)
            }
        }
    }
}
