use std::sync::PoisonError;

use serde_json::Error as SerdeJsonError;
use specta::{datatype::PrimitiveType, Generics, Type, TypeMap};

// A custom error type that represents all possible in our command
#[derive(Debug, thiserror::Error)]
pub enum TauriError {
  #[error("Failed to read file: {0}")]
  Io(#[from] std::io::Error),

  #[error("File is not valid utf8: {0}")]
  Utf8(#[from] std::string::FromUtf8Error),

  #[error("String error: {0}")]
  StringError(String),

  #[error("Serde Json error: {0}")]
  SerdeJson(#[from] SerdeJsonError),

  #[error("Mutex PoisonError")]
  PoisonError(String),

  #[error(transparent)]
  Tauri(#[from] tauri::Error),
}

impl<T> From<PoisonError<T>> for TauriError {
  fn from(err: PoisonError<T>) -> Self {
    TauriError::PoisonError(err.to_string())
  }
}

// we must also implement serde::Serialize
impl serde::Serialize for TauriError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

impl Type for TauriError {
  fn inline(_type_map: &mut TypeMap, _generics: Generics) -> specta::DataType {
    specta::DataType::Primitive(PrimitiveType::String)
  }
}
