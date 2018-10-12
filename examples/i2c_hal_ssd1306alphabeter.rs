#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate panic_abort;

extern crate embedded_hal;
extern crate ssd1306;

extern crate stm32f407g_disc as board;

use cortex_m_rt::entry;
use ssd1306::displayrotation::DisplayRotation;
use ssd1306::mode::TerminalMode;
use ssd1306::Builder;

use board::hal::i2c::*;
use board::hal::prelude::*;
use board::hal::stm32;

use core::fmt::Write;

#[entry]
fn main() -> ! {
    if let Some(p) = stm32::Peripherals::take() {
        let gpiob = p.GPIOB.split();
        let mut rcc = p.RCC.constrain();

        // Set up the clocks, going to fast exhibits some problem so let's take it slow for now
        let clocks = rcc.cfgr.sysclk(40.mhz()).freeze();

        // Set up the SCL pin of the I2C bus at PB6
        let scl = gpiob
            .pb6
            .into_alternate_af4()
            .internal_pull_up(true)
            .set_open_drain();

        // Set up the SDA pin of the I2C bus at PB7
        let sda = gpiob
            .pb7
            .into_alternate_af4()
            .internal_pull_up(true)
            .set_open_drain();

        // Setup I2C1 using the above defined pins at 400kHz bitrate (fast mode)
        let mut i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks);

        // Set up the SSD1306 display at I2C address 0x3c
        let mut disp: TerminalMode<_> = Builder::new().with_i2c_addr(0x3c).connect_i2c(i2c).into();

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

    loop {}
}
