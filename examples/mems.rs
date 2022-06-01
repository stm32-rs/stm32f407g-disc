//! This example reads the onboard accelerometer and lights the LEDs which point
//! towards ground
//!
//! Additionally, the current accelleration is printed via itm.
#![no_main]
#![no_std]

use panic_itm as _;

use stm32f407g_disc as board;

use cortex_m_rt::entry;

use board::hal::prelude::*;
use board::hal::stm32;
use board::led::{LedColor, Leds};

use cortex_m::iprintln;
use cortex_m::peripheral::Peripherals;

use accelerometer::orientation::Tracker;
use accelerometer::Accelerometer;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let gpioa = p.GPIOA.split();
        let gpiod = p.GPIOD.split();
        let gpioe = p.GPIOE.split();
        let mut itm = cp.ITM;

        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiod);

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        let mut accelerometer =
            board::accelerometer::Accelerometer::new(gpioa, gpioe, p.SPI1, clocks);
        let mut tracker = Tracker::new(0.2);

        loop {
            let acceleration = accelerometer.accel_norm().unwrap();
            let orientation = tracker.update(acceleration);

            iprintln!(
                &mut itm.stim[0],
                "received {:?} : {}, {}, {}",
                orientation,
                acceleration.x,
                acceleration.y,
                acceleration.z,
            );

            // x+ orange
            // x- blue
            // y+ red
            // y- green

            if acceleration.x > 0.0 {
                leds[LedColor::Orange].on();
                leds[LedColor::Blue].off();
            } else {
                leds[LedColor::Blue].on();
                leds[LedColor::Orange].off();
            }

            if acceleration.y > 0.0 {
                leds[LedColor::Red].on();
                leds[LedColor::Green].off();
            } else {
                leds[LedColor::Green].on();
                leds[LedColor::Red].off();
            }
        }
    }

    loop {}
}
