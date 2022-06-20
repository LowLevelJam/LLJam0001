use core::arch::asm;

const HEADER_SIZE: u32 = 4 + 2;

#[repr(C, align(8))]
struct Multiboot2Header {
    data: [u32; HEADER_SIZE as usize],
}

#[used]
#[link_section = ".rodata.keep"]
#[no_mangle]
static MULTIBOOT2_HEADER: Multiboot2Header = Multiboot2Header {
    data: [
        0xE852_50D6,
        0,
        HEADER_SIZE * 4,
        0x17AD_AF2A - HEADER_SIZE * 4,
        // terminal tag
        0,
        8,
    ],
};

pub const STACK_SIZE: usize = 64 * 1024;

#[repr(C, align(16))]
pub struct Stack([u8; STACK_SIZE]);

#[no_mangle]
pub static mut STACK: Stack = Stack([0; STACK_SIZE]);

#[naked]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // multiboot calling convention
    // EAX: magic
    // EBX: info

    unsafe {
        asm!(
            // setup stack
            "mov esp, offset {stack}",
            "add esp, {stack_size}",
            // copy magic and info
            "push ebx",
            "push eax",
            // jump to entry point
            "call {main}",
            stack = sym crate::multiboot::STACK,
            stack_size = const crate::multiboot::STACK_SIZE,
            main = sym multiboot_entry,
            options(noreturn)
        )
    };
}

#[no_mangle]
pub extern "C" fn multiboot_entry(magic: u32, ptr: *const u32) -> ! {
    assert!(magic == 0x36d76289, "Expected Multiboot2 magic value");

    let total_size = unsafe { *ptr } as usize;
    let boot_info = unsafe { core::slice::from_raw_parts(ptr as *const u8, total_size) };

    crate::multiboot_entry(boot_info);

    panic!("multiboot_entry returned");
}
