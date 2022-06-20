use core::panic::PanicInfo;

use core::arch::asm;
use crate::println;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}\n", info);

    loop {
        unsafe {
            asm!("hlt", options(nomem, nostack));
        }
    }
}
