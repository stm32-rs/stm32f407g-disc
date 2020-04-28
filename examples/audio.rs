//! Plays a test tone through the DAC with headphone amp.

#![no_main]
#![no_std]

extern crate panic_itm;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

use stm32f407g_disc as board;

use crate::board::{audio_out::AudioOut, hal::delay::Delay, hal::prelude::*, hal::stm32};

use cortex_m_log::println;
use cortex_m_log::{destination::Itm, printer::itm::InterruptSync as InterruptSyncItm};

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();
    let cp = Peripherals::take().unwrap();

    let mut log = InterruptSyncItm::new(Itm::new(cp.ITM));

    let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();
    let gpioc = p.GPIOC.split();
    let gpiod = p.GPIOD.split();

    // TODO: Use frequency when setting up clocks.
    let audio_freq = 48000;

    // Set clock to 64 MHz and freeze.
    let rcc = p.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(168.mhz())
        .plli2sclk(audio_freq.hz())
        .freeze();

    // Get delay provider.
    let mut delay = Delay::new(cp.SYST, clocks);

    println!(log, "\nSetup audio output");
    let mut audio_out = AudioOut::new(
        p.I2C1, p.SPI3, gpioa, gpiob, gpioc, gpiod, clocks, &mut delay, audio_freq, 128,
    );

    println!(log, "Starting audio");
    let mut s: u16 = 0;
    let mut y: u16 = 100;
    loop {
        // Send both left and right word.
        audio_out.i2s.send(s).unwrap();
        audio_out.i2s.send(s).unwrap();

        // Sawtooth with incrementing pitch each cycle.
        if s >= (65535 - y) {
            s = 0;
            y += 1;
            if y > 400 {
                y = 100
            }
        }
        s += y;
    }
}
