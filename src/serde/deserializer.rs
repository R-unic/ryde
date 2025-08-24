use bincode::error::DecodeError;

pub fn deserialize(buf: Vec<u8>) -> Result<super::Program, DecodeError> {
    bincode::decode_from_slice(buf.as_slice(), super::CONFIG).map(|v| v.0)
}
