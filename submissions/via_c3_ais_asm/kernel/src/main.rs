#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(asm_sym)]
#![feature(naked_functions)]

mod multiboot;
mod panic;
mod print;
mod uart;

pub fn multiboot_entry(_: &[u8]) {
    println!("Hello world!");
}
