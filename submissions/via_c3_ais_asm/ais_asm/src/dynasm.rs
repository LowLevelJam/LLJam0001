use crate::ais::{AisError, Instruction};

#[derive(Debug)]
pub enum DynAsmError {
    AisError(AisError),
}

impl From<AisError> for DynAsmError {
    fn from(x: AisError) -> Self {
        Self::AisError(x)
    }
}

pub struct DynAsm {
    data: Vec<u8>,
}

impl DynAsm {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn gen(&mut self, instruction: Instruction) -> Result<(), DynAsmError> {
        let instr = instruction.encode()?;
        self.data.extend_from_slice(instr.as_slice());
        Ok(())
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
