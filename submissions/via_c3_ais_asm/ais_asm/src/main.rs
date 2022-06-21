mod ais;
mod dynasm;

use crate::ais::{decode, AisError, Instruction, Opcode, Register, SubOp};
use crate::dynasm::{DynAsm, DynAsmError};

use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::process::Command;

#[derive(Debug)]
enum TopError {
    DynAsmError(DynAsmError),
    IoError(std::io::Error),
}

impl From<DynAsmError> for TopError {
    fn from(x: DynAsmError) -> Self {
        Self::DynAsmError(x)
    }
}

impl From<std::io::Error> for TopError {
    fn from(x: std::io::Error) -> Self {
        Self::IoError(x)
    }
}

fn decode_bytes(b: &[u8]) {
    let mut bytes: VecDeque<u8> = b.iter().copied().collect();
    loop {

        if bytes.is_empty() {
            break;
        }

        match decode(bytes) {
            Ok((b, i)) => {
                println!("{:?}", i);
                bytes = b;
            }
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }
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

fn main() -> Result<(), TopError> {

    // Gen some code
    let mut asm = DynAsm::new();
    asm.gen(Instruction::i_type(Opcode::ORI, "EAX".into(), 0.into(), 0x1300))?;
    asm.gen(Instruction::i_type(Opcode::ORI, "EBX".into(), 0.into(), 0x37))?;
    asm.gen(Instruction::xalu_type(
        SubOp::ADD,
        "EAX".into(),
        "EAX".into(),
        "EBX".into(),
    ))?;

    // Show dynamic assembled instructions
    decode_bytes(asm.data());

    // Write payload, add header and footer
    let mut output = File::create("out.bin")?;
    output.by_ref().write_all( HEADER )?;
    output.by_ref().write_all(asm.data())?;
    output.by_ref().write_all( FOOTER )?;
    output.flush()?;

    // Show generated disassembly in regular x86 instructions.
    let output = Command::new("objdump")
        .args(["-D", "-bbinary", "-mi386", "-Mintel", "out.bin"])
        .output()?;

    println!("{}", std::str::from_utf8(&output.stdout).unwrap());

    Ok(())
}
