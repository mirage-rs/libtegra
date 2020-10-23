use embedded_hal::blocking::spi::{Transfer, Write};

use super::Spi;

impl Write<u8> for Spi {
    type Error = ();

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        Spi::write(self, words)
    }
}

impl Transfer<u8> for Spi {
    type Error = ();

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        Spi::write(self, words)?;
        Spi::read(self, words)?;
        Ok(words)
    }
}
