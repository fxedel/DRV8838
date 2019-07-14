#![no_main]
#![no_std]

extern crate panic_halt;

use stm32f1xx_hal::{
    prelude::*,
    pac,
    delay::Delay,
};

use drv8838::{
    DRV8838
};


use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let mut drv = DRV8838::new(
        gpiob.pb0.into_open_drain_output(&mut gpiob.crl),
        gpiob.pb1.into_open_drain_output(&mut gpiob.crl),
        gpiob.pb2.into_open_drain_output(&mut gpiob.crl)
    );
    hprintln!("Initialized DRV8838 motor driver on pins PB0 (nSLEEP), PB1 (PH) and PB2 (EN).").unwrap();

    let mut delay = Delay::new(cp.SYST, clocks);
    let wait_time: u32 = 5000; // milliseconds

    loop {
        hprintln!("Set coast mode (nSleep = 0)").unwrap();
        drv.set_coast_mode();
        delay.delay_ms(wait_time);

        hprintln!("Set forward mode (nSleep = 1, PH = 0, EN = 1)").unwrap();
        drv.set_forward_mode();
        delay.delay_ms(wait_time);

        hprintln!("Set break mode (nSleep = 1, EN = 0)").unwrap();
        drv.set_break_mode();
        delay.delay_ms(wait_time);

        hprintln!("Set reverse mode (nSleep = 1, PH = 1, EN = 1)").unwrap();
        drv.set_reverse_mode();
        delay.delay_ms(wait_time);
    }
}
