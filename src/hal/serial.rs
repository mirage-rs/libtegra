use embedded_hal::blocking::serial::Write;

use crate::uart::Uart;

impl Write<u8> for Uart {
    type Error = ();

    fn bwrite_all(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.write(bytes);

        Ok(())
    }

    fn bflush(&mut self) -> Result<(), Self::Error> {
        Uart::flush(self);

        Ok(())
    }
}
