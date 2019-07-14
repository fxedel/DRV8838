#![no_std]

use embedded_hal::{
    digital::v2::OutputPin,
};

pub struct DRV8838<SLEEP, PH, EN> {
    sleep_pin: SLEEP,
    phase_pin: PH,
    enable_pin: EN
}

impl <SLEEP, PH, EN> DRV8838<SLEEP, PH, EN> where SLEEP: OutputPin, PH: OutputPin, EN: OutputPin {
    pub fn new(sleep_pin: SLEEP, phase_pin: PH, enable_pin: EN) -> Self {
        return DRV8838 {
            sleep_pin,
            phase_pin,
            enable_pin
        }
    }

    pub fn set_coast_mode(&mut self) {
        self.sleep_pin.set_low();
        // phase pin's and enable pin's states don't matter
    }

    pub fn set_break_mode(&mut self) {
        self.sleep_pin.set_high();
        // phase pin's state doesn't matter
        self.enable_pin.set_low();
    }

    pub fn set_forward_mode(&mut self) {
        self.sleep_pin.set_high();
        self.phase_pin.set_low();
        self.enable_pin.set_high();
    }

    pub fn set_reverse_mode(&mut self) {
        self.sleep_pin.set_high();
        self.phase_pin.set_high();
        self.enable_pin.set_high();
    }
}