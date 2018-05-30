//! MAX7219 lib

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(never_type)]
#![no_std]
#![feature(reverse_bits)]

extern crate embedded_hal as hal;

use hal::blocking::spi;
use hal::digital::OutputPin;

//#[macro_use(block)]
extern crate nb;

/// Registers
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Max7219Regs {
    /// No op
    NoOp = 0x0,
    /// Digit0
    Digit0 = 0x1,
    /// Digit1
    Digit1 = 0x2,
    /// Digit2
    Digit2 = 0x3,
    /// Digit3
    Digit3 = 0x4,
    /// Digit4
    Digit4 = 0x5,
    /// Digit5
    Digit5 = 0x6,
    /// Digit6
    Digit6 = 0x7,
    /// Digit7
    Digit7 = 0x8,
    /// DecodeMode
    DecodeMode = 0x9,
    /// Intensity
    Intensity = 0xa,
    /// ScanLimit
    ScanLimit = 0xb,
    /// Shutdown
    Shutdown = 0xc,
    /// DisplayTest
    DisplayTest = 0xf,
}


// macro_rules! maxv {
//     ($v1:expr, $v2:expr) =>(if $v1 > $v2 { $v1 } else { $v2 })
// }
// macro_rules! minv {
//     ($v1:expr, $v2:expr) =>(if $v1 < $v2 { $v1 } else { $v2 })
// }


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


/// Errors
#[derive(Debug)]
pub enum Error<E> {
    /// SPI bus error
    Spi(E),
}

/// Device descriptor
#[derive(Clone, Copy, PartialEq)]
pub struct Max7219<SPI,P>
{
    spi : SPI,

    /// ChipSelect
    pub cs : P,
    nums: usize
}

impl<SPI,P,E> Max7219<SPI,P>
where SPI: spi::Write<u8,Error = E>,
      P: OutputPin {

    /// Creates a new device descriptor
    pub fn new(spi: SPI, cs: P, numbers: usize) -> Result<Self, E> {
        let dev = Max7219 {
            spi : spi,
            cs : cs,
            nums: numbers
        };
        Ok(dev)
    }

    /// Set register
    pub fn set_reg(&mut self, reg: Max7219Regs, val: u8) -> Result<(), E> {
        // FIXME: looks ugly, need to handle this correctly.

        // using unwrap() does not work as underlying Error do not
        // implement the Debug trait...
        self.spi.write(&[reg as u8])?;
        self.spi.write(&[val])?;

        Ok(())
    }

    /// Initializes the device
    pub fn init(&mut self) -> Result<(),E>{
        self.cs.set_low();
        for _i in 0..self.nums {
            // Shutdown
            self.set_reg(Max7219Regs::Shutdown, 0)?;
        }
        self.cs.set_high();

        self.cs.set_low();
        for _i in 0..self.nums {
            // Midpower intensity
            self.set_reg(Max7219Regs::Intensity, 0x4)?;
        }
        self.cs.set_high();

        self.cs.set_low();
        for _i in 0..self.nums {
            // Disable test mode
            self.set_reg(Max7219Regs::DisplayTest, 0x0)?;
        }
        self.cs.set_high();

        self.cs.set_low();
        for _i in 0..self.nums {
            // Disable char decoding
            self.set_reg(Max7219Regs::DecodeMode, 0)?;
        }
        self.cs.set_high();

        self.cs.set_low();
        for _i in 0..self.nums {
            // Display all digit
            self.set_reg(Max7219Regs::ScanLimit, 0x07)?;
        }
        self.cs.set_high();

        self.cs.set_low();
        for _i in 0..self.nums {
            // Enable
            self.set_reg(Max7219Regs::Shutdown, 1)?;
        }
        self.cs.set_high();

        Ok(())
    }
}
   
