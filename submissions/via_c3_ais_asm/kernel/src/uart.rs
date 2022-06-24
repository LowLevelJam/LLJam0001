use crate::asm;
use core::fmt::{Error, Write};

pub struct Uart {
    base: u16,
}

pub enum Baudrate {
    B115200 = 1,
    B19200 = 6,
    B9600 = 12,
}

struct Register(u16);

const RHR: Register = Register(0); // read-only
const THR: Register = Register(0); // write-only
const DLL: Register = Register(0); // when DLAB = 1
const DLM: Register = Register(1); // when DLAB = 1
const LCR: Register = Register(3);
const LCR_DLAB: u8 = 0x80;
const LCR_8BITS: u8 = 0x03;
const LSR: Register = Register(5);
const LSR_READY: u8 = 0x01;
const LSR_EMPTY: u8 = 0x20;

impl Uart {
    pub unsafe fn new(base: u16, baudrate: Baudrate) -> Self {
        let this = Self { base };

        this.setup(baudrate);

        this
    }

    fn read(&self, reg: Register) -> u8 {
        unsafe { asm::in8(self.base + (reg.0 as u16)) }
    }

    fn write(&self, reg: Register, value: u8) {
        unsafe { asm::out8(self.base + (reg.0 as u16), value) }
    }

    fn setup(&self, baudrate: Baudrate) {
        // Set DLAB, so DLL and DLM can be accesed
        self.write(LCR, LCR_DLAB);

        // Baudrate enum is encoded with the correct divider value
        let baudrate_divisor: u16 = baudrate as u16;

        // Setup 'prescaler'
        self.write(DLL, (baudrate_divisor & 0x00FF) as u8);
        self.write(DLM, ((baudrate_divisor >> 8) & 0x00FF) as u8);

        // Disable DLAB and setup 8bit, no-parity, 1 stop bit
        self.write(LCR, LCR_8BITS);
    }

    pub fn tx_empty(&self) -> bool {
        self.read(LSR) & LSR_EMPTY == LSR_EMPTY
    }

    fn rx_ready(&self) -> bool {
        self.read(LSR) & LSR_READY == LSR_READY
    }

    pub fn putc(&self, c: u8) {
        // wait for space
        while !self.tx_empty() {
            core::hint::spin_loop()
        }

        self.write(THR, c);
    }

    pub fn getc(&self) -> Option<u8> {
        if self.rx_ready() {
            Some(self.read(RHR))
        } else {
            None
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, string: &str) -> core::result::Result<(), Error> {
        for c in string.bytes() {
            self.putc(c);
        }

        Ok(())
    }
}
