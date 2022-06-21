use core::arch::asm;

#[inline]
pub unsafe fn out8(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
}

#[inline]
pub unsafe fn in8(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack));
    value
}

#[inline]
pub fn halt() {
    unsafe {
        asm!("hlt", options(nomem, nostack));
    }
}

pub unsafe fn rdmsr(msr: u32) -> u64 {
    let mut low: u32;
    let mut high: u32;
    asm!("rdmsr", in("ecx") msr, out("edx") high, out("eax") low, options(nomem, nostack));

    (high as u64) << 32 | (low as u64)
}

pub unsafe fn wrmsr(msr: u32, val: u64) {
    let high = (val >> 32) as u32;
    let low = (val & 0xFFFF) as u32;
    asm!("wrmsr", in("ecx") msr, in("edx") high, in("eax") low, options(nomem, nostack));
}
