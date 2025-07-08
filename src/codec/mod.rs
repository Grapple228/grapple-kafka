#![allow(unused)]

mod error;

// region:    --- Modules

use bincode::{Decode, Encode};

pub use error::{Error, Result};

// endregion: --- Modules

pub fn encode<D: Encode>(data: &D) -> Result<Vec<u8>> {
    bincode::encode_to_vec(data, bincode::config::standard()).map_err(|_| Error::Encode)
}

pub fn decode<D: Decode<()>>(data: &[u8]) -> Result<D> {
    let (decoded, _) =
        bincode::decode_from_slice(data, bincode::config::standard()).map_err(|_| Error::Decode)?;

    Ok(decoded)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

    use super::*;

    const FX_RESULT: [u8; 31] = [
        1, 4, 110, 97, 109, 101, 3, 0, 0, 0, 0, 0, 0, 240, 63, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0,
        0, 0, 8, 64,
    ];

    #[derive(Debug, Encode, Decode, PartialEq)]
    struct Data {
        id: u32,
        name: String,
        values: Vec<f64>,
    }

    fn get_data(id: u32) -> Data {
        Data {
            id,
            name: "name".to_string(),
            values: vec![1.0, 2.0, 3.0],
        }
    }

    #[test]
    fn test_encode() -> Result<()> {
        let data = get_data(1);
        let encoded = encode(&data)?;

        assert_eq!(encoded, FX_RESULT);

        Ok(())
    }

    #[test]
    fn test_decode() -> Result<()> {
        let fx_data = get_data(1);
        let decoded = decode::<Data>(&FX_RESULT)?;

        assert_eq!(fx_data, decoded);

        Ok(())
    }
}

// endregion: --- Tests
