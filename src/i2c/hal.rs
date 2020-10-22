use embedded_hal::blocking::i2c;

use super::{Error, I2c};

impl i2c::Write for I2c {
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        self.send_normal(addr as u32, bytes)
    }
}

impl i2c::WriteRead for I2c {
    type Error = Error;

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        assert_eq!(
            bytes.len(),
            1,
            "A single byte (the register) should be sent to a device in order to read data."
        );

        self.read(address as u32, bytes[0], buffer)
    }
}
