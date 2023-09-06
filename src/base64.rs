use base64::{engine::general_purpose::STANDARD as standard_engine, DecodeError, Engine};
use std::fmt;

// MARK: - Error type

#[derive(Debug)]
pub enum Base64DecodeError {
    Base64Error(String),
}

impl From<DecodeError> for Base64DecodeError {
    fn from(err: DecodeError) -> Self {
        Base64DecodeError::Base64Error(err.to_string())
    }
}

impl fmt::Display for Base64DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Base64DecodeError::Base64Error(err_str) => {
                write!(f, "Base64 decoding error: {}", err_str)
            }
        }
    }
}

impl std::error::Error for Base64DecodeError {}

// MARK: - Actual encode/decode

pub fn base64_encode<T: AsRef<[u8]>>(input: T) -> String {
    standard_engine.encode(input)
}

pub fn base64_decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, Base64DecodeError> {
    standard_engine.decode(input).map_err(|e| e.into())
}

// MARK: - Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_works() {
        assert_eq!(base64_encode("test"), "dGVzdA==");
    }

    #[test]
    fn decode_works() {
        let decoded_bytes = base64_decode("dGVzdA==").unwrap();
        let decoded_string = String::from_utf8(decoded_bytes).unwrap();
        assert_eq!(decoded_string, "test");
    }
}
