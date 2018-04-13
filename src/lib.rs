//! MAX7219 lib

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(never_type)]
#![no_std]

extern crate embedded_hal as hal;

#[macro_use(block)]
extern crate nb;

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

/// Nada
pub struct PixArray {
    pixels : [u8;8]
}

impl PixArray {

    /// Creates new pix
    pub fn new() -> PixArray {
        PixArray { pixels: [0; 8] }
    }

    /// From char
    pub fn from(c : char) -> PixArray {
        match c {
            'a' => PixArray { pixels : [0b00000000,
                                        0b00011000,
                                        0b00100100,
                                        0b01111110,
                                        0b10000001,
                                        0b00000000,
                                        0b00000000,
                                        0b00000000]},
            _ => PixArray { pixels: [0;8] }
        }
    }

    /// set pixel
    pub fn set_pixel(&mut self, line : usize, col : usize , v : bool) {
        if v {
            self.pixels[line] |= 1<<col;
        } else {
            self.pixels[line] &= !(1<<col );
        }
    }

    /// get pixel
    pub fn get_pixel(&self, line : usize, col : usize) -> u8 {
        self.pixels[line] & (1<<col) as u8
    }

    /// Get line
    pub fn get_pixel_line(&self, l : usize) -> u8 {
        self.pixels[l]
    }

    /// Set line
    pub fn set_pixel_line(&mut self, l : usize, v: u8) {
        self.pixels[l] = v;
    }
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

/// Device descriptor
#[derive(Clone, Copy, PartialEq)]
pub struct Max7219<S,P> {
    /// bob
    spi : S,
    cs : P,
    num: u8
}

impl<S,P> Max7219<S,P>
where S: hal::spi::FullDuplex<u8>,
      P: hal::digital::OutputPin {

    /// Creates a new device descriptor
    pub fn new(spi: S, cs: P, num: u8) -> Max7219<S,P> {
        Max7219 {
            spi : spi,
            cs : cs,
            num : num
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

    /// Writes a line
    pub fn write_lines(&mut self, line_index: u8, vals: &[u8]) {
        for i in 0..self.num {
            self.set_reg(Max7219Regs::from(line_index), vals[i as usize]);
        }
    }

    /// Writes a pixel buffer
    pub fn write_pixbufs(&mut self, pixbufs: &[PixArray]) {
        for l in 0..8 {
            let line = Max7219Regs::from(l);
            for i in (0..self.num).rev() {
                self.set_reg(line, pixbufs[i as usize].get_pixel_line(l as usize));
            }
        }
    }

    /// Initializes the device
    pub fn init(&mut self) {
        for _i in 0..self.num {
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
}
