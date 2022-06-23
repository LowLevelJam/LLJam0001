use std::{ops::Deref, cell::Cell};

use crate::ais::{AisError, Instruction, Register, Opcode};

#[derive(Debug)]
pub enum DynAsmError {
    AisError(AisError),
    InvalidSym,
    SymbolRedefined,
    ResolveUnstable,
}

impl From<AisError> for DynAsmError {
    fn from(x: AisError) -> Self {
        Self::AisError(x)
    }
}

#[derive(Copy, Clone)]
pub struct Sym (usize);

struct Symbol {
    addr: Cell<Option<u32>>,
    refs: Vec<SymRef>,
}

enum SymRefKind {
    RelativeImm,
}

struct SymRef {
    kind: SymRefKind,
    addr: u32,
    base: u32,
}

pub struct DynAsm {
    data: Vec<u8>,
    symbols: Vec<Symbol>,
}

impl DynAsm {
    pub fn new() -> Self {
        Self { data: Vec::new(), symbols: Vec::new() }
    }

    fn addr(&self) -> u32 {
        self.data.len().try_into().unwrap()
    }

    fn resolve(data: &mut Vec<u8>, sym_ref: &SymRef, addr: u32) -> Result<(), DynAsmError> {

        let start = sym_ref.addr.try_into().unwrap();
        let end = data.len();

        let bytes = data.get_mut(start..end).unwrap();
        let (mut instr, size) = Instruction::decode(bytes)?;

        match sym_ref.kind {
            SymRefKind::RelativeImm => {
                instr.imm = Some(0);
            },
        }

        let new_bytes = instr.encode()?;

        if new_bytes.len() != size {
            return Err(DynAsmError::ResolveUnstable);
        }

        bytes.copy_from_slice(&new_bytes);
        Ok(())
    }


    pub fn new_sym(&mut self) -> Sym {
        let entry = Symbol { addr: Cell::new(None), refs: Vec::new() };
        self.symbols.push(entry);
        Sym(self.symbols.len() - 1)
    }

    pub fn new_sym_here(&mut self) -> Sym {
        let entry = Symbol { addr: Cell::new(Some(self.addr())), refs: Vec::new() };
        self.symbols.push(entry);
        Sym(self.symbols.len() - 1)
    }

    pub fn set_sym_here(&mut self, sym: Sym) -> Result<(), DynAsmError> {

        let symbol = self.symbols.get(sym.0).ok_or_else(|| DynAsmError::InvalidSym)?;

        if symbol.addr.get().is_some() {
            return Err(DynAsmError::SymbolRedefined);
        }

        let addr = self.addr();
        symbol.addr.set(Some(addr));

        for sym_ref in symbol.refs.iter() {
            Self::resolve(&mut self.data, sym_ref, addr)?
        }

        Ok(())
    }

    pub fn gen(&mut self, instruction: Instruction) -> Result<(), DynAsmError> {
        let instr = instruction.encode()?;
        self.data.extend_from_slice(instr.as_slice());
        Ok(())
    }

    pub fn gen_addi(&mut self, dst: Register, src: Register, imm: u16) -> Result<(), DynAsmError> {
        self.gen(Instruction::i_type(Opcode::ADDI, dst, src, imm))
    }



    pub fn gen_jmp_near(&mut self, sym: Sym) -> Result<(), DynAsmError> {

        // load sym into R4
        self.gen(Instruction::j("R4".into()))?;
        Ok(())
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
