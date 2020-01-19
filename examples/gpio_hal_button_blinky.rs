// This is an example which lights up the leds when the user button is
// pressed down. Each press also cycles through the leds.

#![no_main]
#![no_std]

use panic_halt as _;

use stm32f407g_disc as board;

use crate::board::{
    button::Button,
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
        let gpioa = p.GPIOA.split();

        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiod);

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        // Create Button
        let button = Button::from(gpioa.pa0.into_pull_down_input().into());

        // Create slice of LEDs
        let led_slice = [
            LedColor::Green,
            LedColor::Orange,
            LedColor::Red,
            LedColor::Blue,
        ];

        // Make LEDs infinitely cyclable
        let mut colors = led_slice.iter().cycle();
        let mut pressed = false;
        let mut actual_color = colors.next().unwrap();

        loop {
            if button.pressed().unwrap() && !pressed {
                pressed = true;
                actual_color = colors.next().unwrap();
                leds[*actual_color].on();
                // A cheap way of debouncing
                delay.delay_ms(10_u16);
            } else if !button.pressed().unwrap() && pressed {
                pressed = false;
                leds[*actual_color].off();
                // A cheap way of debouncing
                delay.delay_ms(10_u16);
            }
        }
    }

    loop {
        continue;
    }
}
