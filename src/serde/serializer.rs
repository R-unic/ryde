use bincode::error::EncodeError;

pub fn serialize(program: &super::Program) -> Result<Vec<u8>, EncodeError> {
    bincode::encode_to_vec(program, super::CONFIG)
}
