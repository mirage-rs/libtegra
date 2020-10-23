use embedded_hal::blocking::serial::Write;

use super::Uart;

impl Write<u8> for Uart {
    type Error = ();

    fn bwrite_all(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        self.write(buffer);

        Ok(())
    }

    fn bflush(&mut self) -> Result<(), Self::Error> {
        self.flush();

        Ok(())
    }
}
