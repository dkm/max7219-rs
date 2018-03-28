//! MAX7219 lib

#![deny(missing_docs)]
//#![deny(warnings)]
#![feature(never_type)]
#![no_std]

extern crate embedded_hal as hal;

use hal::spi::{FullDuplex};

/// empty
#[derive(Clone, Copy, PartialEq)]
pub struct Max7219<S: hal::spi::FullDuplex<u8>> {
    /// bob
    spi : S,
}


enum Commands {

}

impl<S> Max7219<S>
        where S: hal::spi::FullDuplex<u8> {

    /// bob
    pub fn new(spi: S) -> Max7219<S> {
        Max7219 {
            spi : spi,
        }
    }

    /// bob
    pub fn init(&mut self) {
        self.spi.send(0x8 as u8);
    }
}
