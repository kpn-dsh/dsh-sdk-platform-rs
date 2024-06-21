use crate::error::DshError;

pub fn extract_header_and_payload(raw_token: &str) -> Result<&str, DshError> {
    let parts: Vec<&str> = raw_token.split('.').collect();
    parts
        .get(1)
        .copied()
        .ok_or_else(|| DshError::TokenError("Header and payload are missing".to_string()))
}

pub fn decode_payload(payload: &str) -> Result<Vec<u8>, DshError> {
    use base64::{alphabet, engine, read};
    use std::io::Read;

    let engine = engine::GeneralPurpose::new(&alphabet::STANDARD, engine::general_purpose::NO_PAD);
    let mut decoder = read::DecoderReader::new(payload.as_bytes(), &engine);

    let mut decoded_token = Vec::new();
    decoder
        .read_to_end(&mut decoded_token)
        .map_err(DshError::IoError)?;

    Ok(decoded_token)
}
