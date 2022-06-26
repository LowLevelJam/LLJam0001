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

#[inline]
pub unsafe fn rdmsr(msr: u32) -> u64 {
    let mut low: u32;
    let mut high: u32;
    asm!("rdmsr", in("ecx") msr, out("edx") high, out("eax") low, options(nomem, nostack));

    (high as u64) << 32 | (low as u64)
}

#[inline]
pub unsafe fn wrmsr(msr: u32, val: u64) {
    let high = (val >> 32) as u32;
    let low = (val & 0xFFFF) as u32;
    asm!("wrmsr", in("ecx") msr, in("edx") high, in("eax") low, options(nomem, nostack));
}

#[inline]
pub fn flags() -> u32 {
    let flags;
    unsafe {
        asm!(
            "pushfd",
            "pop {flags}",
            flags = out(reg) flags,
            options(nomem, preserves_flags));
    };
    flags
}

#[inline]
pub fn cpuid(val: u32) -> [u32; 4] {
    let mut eax;
    let mut ebx;
    let mut ecx;
    let mut edx;

    unsafe {
        asm!("cpuid", 
        inout("eax") val => eax,
        out("ebx") ebx,
        out("ecx") ecx,
        out("edx") edx,
        options(nomem, nostack));
    }

    [eax,ebx,ecx,edx]
}