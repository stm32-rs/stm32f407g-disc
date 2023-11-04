#![no_main]
#![no_std]

use panic_halt as _;

use stm32f407g_disc as board;

use crate::board::{
    hal::prelude::*,
    hal::pac,
    led::{LedColor, Leds},
};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let Some(p) = pac::Peripherals::take() {
        let gpiod = p.GPIOD.split();

        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiod);

        // Endlessly blink the 4 LEDs in a circle, delaying by executing the state write many times
        // in a row
        loop {
            for _ in 0..10_000 {
                leds[LedColor::Orange].on();
            }

            for _ in 0..10_000 {
                leds[LedColor::Orange].off();
            }

            for _ in 0..10_000 {
                leds[LedColor::Red].on();
            }

            for _ in 0..10_000 {
                leds[LedColor::Red].off();
            }

            for _ in 0..10_000 {
                leds[LedColor::Blue].on();
            }

            for _ in 0..10_000 {
                leds[LedColor::Blue].off();
            }

            for _ in 0..10_000 {
                leds[LedColor::Green].on();
            }

            for _ in 0..10_000 {
                leds[LedColor::Green].off();
            }
        }
    }

    loop {
        continue;
    }
}
