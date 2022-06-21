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

fn main() -> Result<(), TopError> {
    let mut asm = DynAsm::new();

    asm.gen(Instruction::i_type(Opcode::ORI, "EAX".into(), 0.into(), 10))?;
    asm.gen(Instruction::i_type(Opcode::ORI, "EBX".into(), 0.into(), 10))?;
    asm.gen(Instruction::xalu_type(
        SubOp::ADD,
        "EAX".into(),
        "EAX".into(),
        "EBX".into(),
    ))?;

    let mut output = File::create("out.bin")?;
    output.by_ref().write_all(&asm.data())?;
    output.flush()?;

    let output = Command::new("objdump")
        .args(["-D", "-bbinary", "-mi386", "-Mintel", "out.bin"])
        .output()?;

    println!("{}", std::str::from_utf8(&output.stdout).unwrap());

    println!(
        "{:?}",
        decode([0x8D, 0x84, 0x00, 0x19, 0x08, 0xE0, 0x83].into())
    );

    println!(
        "{:?}",
        decode([0x8D, 0x84, 0x00, 0x47, 0x10, 0x00, 0x18].into())
    );

    println!(
        "{:?}",
        decode([0x8D, 0x84, 0x00, 0x18, 0x00, 0x10, 0x47].into())
    );

    let mut bytes: VecDeque<u8> = asm.data().into();
    loop {
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

    Ok(())
}
