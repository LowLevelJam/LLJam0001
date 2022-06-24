mod ais;
mod dynasm;

use crate::ais::{DpCntl, Instruction, SubOpXalu};
use crate::dynasm::{DynAsm, DynAsmError, Sym};

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
    // Gen some code, at location 0x480000, this is where our kernel will place the payload
    let mut asm = DynAsm::new(0x480000);

    // Add x86 to AIS transition header
    asm.gen_header();

    // Clear result register
    asm.gen_load("EAX".into(), 0x0)?;

    // Define pseudo call and return. Return value is place in a register instead of the stack
    fn pseudo_call(asm: &mut DynAsm, function: Sym) -> Result<(), TopError> {
        // forward declare return label
        let ret = asm.new_sym();
        // Load return register
        asm.gen_load_symbol("EBX".into(), ret)?;
        // Jump to the function
        asm.gen_jump(function)?;
        // Resolve retunr label to be just after the jump
        asm.set_sym_here(ret)?;
        Ok(())
    }

    fn pseudo_ret(asm: &mut DynAsm) -> Result<(), TopError> {
        // Jump to the return register
        asm.gen(Instruction::xj("EBX".into()))?;
        Ok(())
    }

    // Forward declare push function
    let push = asm.new_sym();

    // Push some bytes
    asm.gen_load("EDX".into(), 0xB)?;
    pseudo_call(&mut asm, push)?;
    asm.gen_load("EDX".into(), 0xA)?;
    pseudo_call(&mut asm, push)?;
    asm.gen_load("EDX".into(), 0xD)?;
    pseudo_call(&mut asm, push)?;
    asm.gen_load("EDX".into(), 0xC)?;
    pseudo_call(&mut asm, push)?;
    asm.gen_load("EDX".into(), 0x0)?;
    pseudo_call(&mut asm, push)?;
    asm.gen_load("EDX".into(), 0xD)?;
    pseudo_call(&mut asm, push)?;
    asm.gen_load("EDX".into(), 0xE)?;
    pseudo_call(&mut asm, push)?;

    // Done jump to the end
    let end = asm.new_sym();
    asm.gen_jump(end)?;

    // Function that will push a byte in the result
    // EAX = EAX << 8 | EDX
    asm.set_sym_here(push)?;
    asm.gen_load("R4".into(), 4)?;
    asm.gen(Instruction::xalur(SubOpXalu::SHL, DpCntl::Word, "EAX".into(), "EAX".into(), "R4".into()))?;
    asm.gen(Instruction::xalur(
        SubOpXalu::OR,
        DpCntl::Word,
        "EAX".into(),
        "EAX".into(),
        "EDX".into(),
    ))?;
    pseudo_ret(&mut asm)?;

    // The end is here
    asm.set_sym_here(end)?;

    // Append footer and we are done. This is just a return, so it will return from the payload back into the kernel
    asm.gen_footer();

    // Show dynamic assembled instructions
    asm.dump();

    // Write payload to out.bin, the kernel will included this as the payload
    let mut output = File::create("out.bin")?;
    output.by_ref().write_all(asm.memory())?;
    output.flush()?;

    // Show generated disassembly in regular x86 instructions.
    let output = Command::new("objdump")
        .args(["-D", "-bbinary", "-mi386", "-Mintel", "out.bin"])
        .output()?;
    println!("{}", std::str::from_utf8(&output.stdout).unwrap());

    Ok(())
}
