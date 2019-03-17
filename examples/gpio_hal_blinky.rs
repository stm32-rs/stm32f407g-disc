#![no_main]
#![no_std]

use panic_halt as _;

use stm32f407g_disc as board;

use crate::board::{
    hal::stm32,
    hal::{delay::Delay, prelude::*},
    led::{LedColor, Leds},
};

use cortex_m::peripheral::Peripherals;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let gpiod = p.GPIOD.split();

        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiod);

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        loop {
            // Turn LEDs on one after the other with 500ms delay between them
            leds[LedColor::Orange].on();
            delay.delay_ms(500_u16);
            leds[LedColor::Red].on();
            delay.delay_ms(500_u16);
            leds[LedColor::Blue].on();
            delay.delay_ms(500_u16);
            leds[LedColor::Green].on();
            delay.delay_ms(500_u16);

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(500_u16);
            delay.delay_ms(500_u16);

            // Turn LEDs off one after the other with 500ms delay between them
            leds[LedColor::Orange].off();
            delay.delay_ms(500_u16);
            leds[LedColor::Red].off();
            delay.delay_ms(500_u16);
            leds[LedColor::Blue].off();
            delay.delay_ms(500_u16);
            leds[LedColor::Green].off();
            delay.delay_ms(500_u16);

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(500_u16);
            delay.delay_ms(500_u16);
        }
    }

    loop {
        continue;
    }
}
