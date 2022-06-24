use std::{ops::Deref, cell::Cell};

use crate::ais::{AisError, Instruction, Register, Opcode, Offset, Function, SubOpXls};

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

const HEADER: &[u8] = &[
    0xE8, 0x00, 0x00, 0x00, 0x00,   //     call 1f
    0x58,                           // 1:  pop eax
    0x83, 0xC0, 0x06,               //     add eax, 6
    0x0F, 0x3F                      //     jmpai eax
    // <- jmpai should jump to here, this is where the AI wrapper instruction start.
];

const FOOTER: &[u8] = &[
    0xC3,           // ret
];

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

        println!("addr = {:X}", addr);

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

    pub fn gen_load(&mut self, dst: Register, imm: u32) -> Result<(), DynAsmError> {
        let low_zero = imm & 0xFFFF == 0;
        let high_zero = imm & 0xFFFF0000 == 0;

        match (high_zero, low_zero) {
            (false, false) => {
                self.gen(Instruction::i_type(Opcode::ORI, dst.clone(), 0.into(), imm as u16))?;
                self.gen(Instruction::i_type(Opcode::ORIU, dst.clone(), dst, (imm >> 16)as u16))?;
            },
            (false, true) => self.gen(Instruction::i_type(Opcode::ORIU, dst, 0.into(), (imm >> 16) as u16))?,
            (true, _) => self.gen(Instruction::i_type(Opcode::ORI, dst, 0.into(), imm as u16))?,
        }

        Ok(())
    }

    pub fn gen_jmp_near(&mut self, sym: Sym) -> Result<(), DynAsmError> {

        // load sym into R4
        //self.gen(Instruction::j("R4".into()))?;
        Ok(())
    }

    pub fn gen_header(&mut self) {
        self.data.extend_from_slice(HEADER);
    }

    pub fn gen_footer(&mut self){
        self.data.extend_from_slice(FOOTER);
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn dump(&self) {
        let mut bytes = &self.data[HEADER.len()..self.data.len() - FOOTER.len()];
        loop {

            if bytes.is_empty() {
                break;
            }

            match Instruction::decode(bytes) {
                Ok((i, size)) => {
                    println!("{:?}", i);
                    bytes = &bytes[size..];
                }
                Err(e) => {
                    println!("{:?}", e);
                    break;
                }
            }
        }
    }

}
