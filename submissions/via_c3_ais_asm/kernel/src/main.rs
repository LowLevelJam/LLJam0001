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

const PAYLOAD_LEN: usize = core::include_bytes!("../../ais_asm/out.bin").len();

#[link_section = ".payload"]
static PAYLOAD: &[u8; PAYLOAD_LEN] = core::include_bytes!("../../ais_asm/out.bin");

pub fn multiboot_entry(_: &[u8]) {
    println!("Hello world!");

    let fcr = unsafe { asm::rdmsr(0x1107) };
    println!("FCR: {:16X}", fcr);

    let fcr = fcr | 0x0001;
    unsafe {
        asm::wrmsr(0x1107, fcr);
    }

    let fcr = unsafe { asm::rdmsr(0x1107) };
    println!("FCR: {:16X}", fcr);

    // assert!(
    //     fcr & 0x0001 != 0,
    //     "This processor doens't have support for VIA C3 AIS"
    // );

    println!("r = {:8X}", PAYLOAD.as_ptr() as u32);

    let payload: extern "C" fn() -> u32 = unsafe { core::mem::transmute(PAYLOAD.as_ptr()) };
    let r = payload();


    println!("r = {:8X}", r);

}
