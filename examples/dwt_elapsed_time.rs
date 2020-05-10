#![no_main]
#![no_std]

// Use DWT (Debug Watch and Trace) module to measure elapsed time.
// DWT has a 32-bit running counter that counts CPU clock cycles.
// Benefit of using DWT to measure elapsed time is that, it doesn't require interrupts.

use panic_halt as _;

use stm32f407g_disc as board;

use crate::board::{
    hal::stm32,
    hal::{delay::Delay, prelude::*},
    led::{LedColor, Leds},
};

use cortex_m::peripheral::{Peripherals, DWT};

use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(mut cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let gpiod = p.GPIOD.split();

        cp.DWT.enable_cycle_counter();

        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiod);

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        let begin_clock = DWT::get_cycle_count();

        // delay for 0.1 second
        delay.delay_ms(100_u16);
        
        let end_clock = DWT::get_cycle_count();
        
        // Calculate elapsed clock cycles
        let elapsed_clocks = end_clock - begin_clock;

        // Just to signal that time measurement is finished
        leds[LedColor::Orange].on();
        
        hprintln!("{}", elapsed_clocks).unwrap();
        
        // Ensure that the measured clock cycles are within acceptable bounds
        assert!(
            elapsed_clocks >= (16_800_000 - 1_000)
            && elapsed_clocks < (16_800_000 + 1_000)
        );

        loop {
            continue;
        }
    }

    panic!("Failed to access peripherals");
}
