//! Turns all the user LEDs on
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use f3_r6::{
    hal::{prelude::*, pac},
    led::Leds,
};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let gpioe = p.GPIOE.split(&mut rcc.ahb);

    let mut leds = Leds::new(gpioe);

    for led in leds.iter_mut() {
        led.on();
    }

    loop {}
}
