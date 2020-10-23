use embedded_hal::PwmPin;

use super::PwmChannel;

impl PwmPin for PwmChannel {
    type Duty = f32;

    fn disable(&mut self) {
        PwmChannel::disable(self);
    }

    fn enable(&mut self) {
        PwmChannel::enable(self);
    }

    fn get_duty(&self) -> f32 {
        PwmChannel::get_duty(self)
    }

    fn get_max_duty(&self) -> f32 {
        PwmChannel::get_max_duty(self)
    }

    #[allow(unused_must_use)]
    fn set_duty(&mut self, duty: f32) {
        PwmChannel::set_pulse_width(self, duty);
    }
}
