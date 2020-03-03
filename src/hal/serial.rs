use embedded_hal::serial;
use crate::uart::Uart;

impl serial::Read<u8> for Uart {
    type Error = ();
    fn read(&mut self) -> Result<u8, nb::Error<Self::Error>> {
        Ok(self.read_byte())
    }
}

impl serial::Write<u8> for Uart {
    type Error = ();

    fn write(&mut self, byte: u8) -> Result<(), nb::Error<Self::Error>> {
        self.write_byte(byte);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), nb::Error<Self::Error>> {
        Uart::flush(self);
        Ok(())
    }
}
