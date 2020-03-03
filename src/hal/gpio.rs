use crate::gpio::{Gpio, Level};
use embedded_hal::digital::v2;

impl v2::OutputPin for Gpio {
    type Error = ();

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.write(Level::High);
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.write(Level::Low);
        Ok(())
    }
}

#[cfg(feature = "hal-unproven")]
impl v2::InputPin for Gpio {
    type Error = ();

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

#[cfg(feature = "hal-unproven")]
impl v2::StatefulOutputPin for Gpio {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}
