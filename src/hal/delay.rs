use embedded_hal::blocking::delay;

use crate::timer;

pub struct Delay;

impl Delay {
    pub fn new() -> Self {
        Self {}
    }
}

impl delay::DelayMs<u32> for Delay {
    fn delay_ms(&mut self, duration: u32) {
        timer::msleep(duration);
    }
}

impl delay::DelayUs<u32> for Delay {
    fn delay_us(&mut self, duration: u32) {
        timer::usleep(duration);
    }
}

impl delay::DelayMs<u16> for Delay {
    fn delay_ms(&mut self, duration: u16) {
        timer::msleep(duration as u32);
    }
}

impl delay::DelayUs<u16> for Delay {
    fn delay_us(&mut self, duration: u16) {
        timer::usleep(duration as u32);
    }
}

impl delay::DelayMs<u8> for Delay {
    fn delay_ms(&mut self, duration: u8) {
        timer::msleep(duration as u32);
    }
}

impl delay::DelayUs<u8> for Delay {
    fn delay_us(&mut self, duration: u8) {
        timer::usleep(duration as u32);
    }
}
