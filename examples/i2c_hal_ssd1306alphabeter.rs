//! This example writes the ASCII alphabet to an external display via I2C.
//!
//! You need to connect an external SSD1306 OLED screen to the I2C bus at
//! PB6(SCL) and PB7(SDA)
#![no_main]
#![no_std]

use panic_halt as _;

use core::fmt::Write;

use stm32f407g_disc as board;

use ssd1306::{displayrotation::DisplayRotation, mode::TerminalMode, Builder, I2CDIBuilder};

use crate::board::{hal::i2c::*, hal::pac, hal::prelude::*};

#[cortex_m_rt::entry]
fn main() -> ! {
    if let Some(p) = pac::Peripherals::take() {
        let gpiob = p.GPIOB.split();
        let rcc = p.RCC.constrain();

        // Set up the clocks, going too fast exhibits some problem so let's take it slow for now
        let clocks = rcc.cfgr.sysclk(40.MHz()).freeze();

        // Setup I2C1 using PB6/PB7 at 400kHz bitrate (fast mode)
        let scl = gpiob.pb6;
        let sda = gpiob.pb7;
        let i2c = I2c::new(p.I2C1, (scl, sda), 400.kHz(), &clocks);

        // Set up the SSD1306 display at I2C address 0x3c
        let interface = I2CDIBuilder::new().init(i2c);
        let mut disp: TerminalMode<_, _> = Builder::new().connect(interface).into();

        // Set display rotation to 180 degrees
        let _ = disp.set_rotation(DisplayRotation::Rotate180);

        // Init and clear the display
        let _ = disp.init();
        let _ = disp.clear();

        // Endless loop rendering ASCII characters all over the place
        loop {
            for c in (97..123).chain(64..91) {
                let _ = disp.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
            }
        }
    }

    loop {
        continue;
    }
}
