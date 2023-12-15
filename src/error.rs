use std::io;

#[derive(Debug)]
pub enum Error {
	ReqwestError(reqwest::Error),
	JsonDeserializationError(serde_json::Error),
	TomlDeserializationError(toml::de::Error),
	UnsuccessfulApiRequestError,
	IoError(io::Error),
	Errno(i32),
	InvalidArgument(String)
}

impl From<reqwest::Error> for Error {
	fn from(value: reqwest::Error) -> Self {
		Error::ReqwestError(value)
	}
}

impl From<serde_json::Error> for Error {
	fn from(value: serde_json::Error) -> Self {
		Error::JsonDeserializationError(value)
	}
}

impl From<toml::de::Error> for Error {
	fn from(value: toml::de::Error) -> Self {
		Error::TomlDeserializationError(value)
	}
}

impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
		Error::IoError(value)
	}
}