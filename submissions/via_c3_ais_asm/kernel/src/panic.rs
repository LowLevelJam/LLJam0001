use core::panic::PanicInfo;

use crate::asm;
use crate::println;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}\n", info);

    loop {
        asm::halt();
    }
}
