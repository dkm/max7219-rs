//! MAX7219 lib

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(never_type)]
#![no_std]

extern crate embedded_hal as hal;

#[macro_use(block)]
extern crate nb;

//use hal::spi::{FullDuplex};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Max7219Regs {
    NoOp = 0x0,
    Digit0 = 0x1,
    Digit1 = 0x2,
    Digit2 = 0x3,
    Digit3 = 0x4,
    Digit4 = 0x5,
    Digit5 = 0x6,
    Digit6 = 0x7,
    Digit7 = 0x8,
    DecodeMode = 0x9,
    Intensity = 0xa,
    ScanLimit = 0xb,
    Shutdown = 0xc,
    DisplayTest = 0xf,
}

impl From<u8> for Max7219Regs {
    fn from(reg_index: u8) -> Self {
        match reg_index {
            0 => Max7219Regs::Digit0,
            1 => Max7219Regs::Digit1,
            2 => Max7219Regs::Digit2,
            3 => Max7219Regs::Digit3,
            4 => Max7219Regs::Digit4,
            5 => Max7219Regs::Digit5,
            6 => Max7219Regs::Digit6,
            7 => Max7219Regs::Digit7,
            _ => Max7219Regs::NoOp,
        }
    }
}

/// empty
#[derive(Clone, Copy, PartialEq)]
pub struct Max7219<S,P> {
    /// bob
    spi : S,
    cs : P,
}

impl<S,P> Max7219<S,P>
where S: hal::spi::FullDuplex<u8>,
      P: hal::digital::OutputPin {

    /// bob
    pub fn new(spi: S, cs: P) -> Max7219<S,P> {
        Max7219 {
            spi : spi,
            cs : cs,
        }
    }

    fn set_reg(&mut self, reg: Max7219Regs, val: u8) {
        self.cs.set_low();

        // FIXME: looks ugly, need to handle this correctly.

        // using unwrap() does not work as underlying Error do not
        // implement the Debug trait...
        match block!(self.spi.send(reg as u8)) {
            _ => {}
        }
        match block!(self.spi.send(val)) {
            _ => {}
        }

        self.cs.set_high();
    }

    /// bob
    pub fn write_line(&mut self, line: u8, val: u8) {
        self.set_reg(Max7219Regs::from(line), val);
    }

    /// bob
    pub fn init(&mut self) {
        self.cs.set_high();

        // Shutdown
        self.set_reg(Max7219Regs::Shutdown, 0);

        // Midpower intensity
        self.set_reg(Max7219Regs::Intensity, 0x4);

        // Disable test mode
        self.set_reg(Max7219Regs::DisplayTest, 0x0);

        // Disable char decoding
        self.set_reg(Max7219Regs::DecodeMode, 0);

        // Display all digit
        self.set_reg(Max7219Regs::ScanLimit, 0x07);

        // Enable
        self.set_reg(Max7219Regs::Shutdown, 1);
    }
}
