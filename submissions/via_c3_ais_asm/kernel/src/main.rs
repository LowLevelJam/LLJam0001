#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(asm_sym)]
#![feature(naked_functions)]

mod asm;
mod multiboot;
mod panic;
mod print;
mod uart;

use core::arch::asm;
use crate::print::SERIAL1;

// Include the payload, linker script will place it a 0x48000
#[link_section = ".payload"]
static PAYLOAD: [u8; PAYLOAD_LEN] = *core::include_bytes!("../../ais_asm/out.bin");
const PAYLOAD_LEN: usize = core::include_bytes!("../../ais_asm/out.bin").len();

pub fn multiboot_entry(_: &[u8]) {
    println!("");
    println!("Kernel started");

    // Enable AIS
    unsafe {
        let fcr= asm::rdmsr(0x1107);
        asm::wrmsr(0x1107, fcr | 0x0001);
    }

    // Test Centaur Extended Features Flags
    let flags = asm::cpuid(0xC0000001);
    assert!(
        flags[3] & 3 == 3,
        "AIS is not supported or not enabled"
    );

    println!("AIS is supported and has been enabled");

    println!("Run payload at 0x{:08X}", PAYLOAD.as_ptr() as u32);

    // Flush serial
    while !SERIAL1.lock().tx_empty() {
        core::hint::spin_loop()
    }

    // Run payload
    let payload: extern "C" fn() -> u32 = unsafe { core::mem::transmute(PAYLOAD.as_ptr()) };
    let r = payload();

    // Show result
    println!("Result EAX = 0x{:08X}", r);

    println!("Done");
    loop {
        asm::halt()
    }
}
