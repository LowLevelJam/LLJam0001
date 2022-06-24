mod ais;
mod dynasm;

use crate::ais::{AisError, Instruction, Opcode, Register, SubOpXalu, DpCntl, Size};
use crate::dynasm::{DynAsm, DynAsmError};

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

    // Gen some code
    let mut asm = DynAsm::new();
    
    asm.gen_header();
    asm.gen_load("EAX".into(), 0x1)?;
    asm.gen_load("EBX".into(), 0x8000_0000)?;
    asm.gen(Instruction::xalur(
        SubOpXalu::ADD,
        DpCntl::Word,
        "EAX".into(),
        "EAX".into(),
        "EBX".into(),
    ))?;



    let label = asm.new_sym();
    //asm.gen_jmp(label)?;
    asm.gen_load("R4".into(), 0x480041)?;
    asm.gen(Instruction::xj("R4".into()))?;

    asm.gen_load("EBX".into(), 0x444400)?;
    asm.gen(Instruction::xalur(
        SubOpXalu::XOR,
        DpCntl::Word,
        "EAX".into(),
        "EAX".into(),
        "EBX".into(),
    ))?;

    asm.set_sym_here(label)?;
    asm.gen_load("EBX".into(), 0x18000)?;


    asm.gen(Instruction::xalur(
        SubOpXalu::ADD,
        DpCntl::Word,
        "EAX".into(),
        "EAX".into(),
        "EBX".into(),
    ))?;


    // asm.gen_load("R4".into(), 0)?;
    // asm.gen_load("R4".into(), 0x5C5C)?;
    // asm.gen_load("R4".into(), 0x86860000)?;
    // asm.gen_load("R4".into(), 0x12345678)?;

    // asm.gen_load( "EDX".into(), 0x3F8)?;
    // asm.gen_load( "EAX".into(), 0x40)?;
    // asm.gen(Instruction::xiow(Size::Bits8, "EDX".into(), "EAX".into()))?;


    asm.gen_footer();

    // Show dynamic assembled instructions
    asm.dump();

    // Write payload, add header and footer
    let mut output = File::create("out.bin")?;
    output.by_ref().write_all(asm.data())?;
    output.flush()?;

    // Show generated disassembly in regular x86 instructions.
    let output = Command::new("objdump")
        .args(["-D", "-bbinary", "-mi386", "-Mintel", "out.bin"])
        .output()?;

    println!("{}", std::str::from_utf8(&output.stdout).unwrap());

    Ok(())
}
