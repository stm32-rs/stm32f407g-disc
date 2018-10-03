#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_abort;

extern crate stm32f407g_disc as board;

use cortex_m_rt::entry;

use board::hal::prelude::*;
use board::hal::stm32;

#[entry]
fn main() -> ! {
    if let Some(p) = stm32::Peripherals::take() {
        let gpiod = p.GPIOD.split();

        // (Re-)configure PD pins connected to the LEDs as output
        let mut green = gpiod.pd12.into_push_pull_output();
        let mut orange = gpiod.pd13.into_push_pull_output();
        let mut red = gpiod.pd14.into_push_pull_output();
        let mut blue = gpiod.pd15.into_push_pull_output();

        // Endlessly blink the 4 LEDs in a circle, delaying by executing the state write many times
        // in a row
        loop {
            for _ in 0..500_000 {
                orange.set_high();
            }

            for _ in 0..500_000 {
                orange.set_low();
            }

            for _ in 0..500_000 {
                red.set_high();
            }

            for _ in 0..500_000 {
                red.set_low();
            }

            for _ in 0..500_000 {
                blue.set_high();
            }

            for _ in 0..500_000 {
                blue.set_low();
            }

            for _ in 0..500_000 {
                green.set_high();
            }

            for _ in 0..500_000 {
                green.set_low();
            }
        }
    }

    loop {}
}
