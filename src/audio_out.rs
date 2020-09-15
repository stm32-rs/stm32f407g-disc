//! CS43L22 DAC with headphone amp.

use crate::hal::prelude::*;

use crate::hal::gpio::gpioa::{self, PA4};
use crate::hal::gpio::gpiob::{self, PB6, PB9};
use crate::hal::gpio::gpioc::{self, PC10, PC12};
use crate::hal::gpio::gpiod::{self, PD4};
use crate::hal::gpio::{Alternate, AlternateOD, Output, PushPull, AF4, AF6};
use crate::hal::i2c::*;
use crate::hal::i2s::*;
use crate::hal::rcc::Clocks;
use crate::hal::stm32::{I2C1, SPI3};

use embedded_hal::blocking::delay::DelayMs;

pub use cs43l22;

pub struct AudioOut {
    pub cs43l22: cs43l22::CS43L22<
        I2c<I2C1, (PB6<AlternateOD<AF4>>, PB9<AlternateOD<AF4>>)>,
        PD4<Output<PushPull>>,
    >,
    pub i2s: I2s<
        SPI3,
        (
            PC10<Alternate<AF6>>,
            PA4<Alternate<AF6>>,
            PC12<Alternate<AF6>>,
            NoSdExt,
        ),
    >,
}

impl AudioOut {
    pub fn new<DELAY: DelayMs<u8>>(
        i2c1: I2C1,
        spi3: SPI3,
        gpioa: gpioa::Parts,
        gpiob: gpiob::Parts,
        gpioc: gpioc::Parts,
        gpiod: gpiod::Parts,
        clocks: Clocks,
        delay: &mut DELAY,
        audio_freq: u32,
        vol: u8,
    ) -> Self {
        // Setup I2C1 using PB6 and PB9 pins at 100kHz bitrate.
        let scl = gpiob.pb6.into_alternate_af4().set_open_drain();
        let sda = gpiob.pb9.into_alternate_af4().set_open_drain();
        let i2c1 = I2c::i2c1(i2c1, (scl, sda), 100.khz(), clocks);

        // CS43L22 reset pin.
        let reset = gpiod.pd4.into_push_pull_output();

        // Set PC7 into AF6 to output the MCLK for I2S3.
        let _mck = gpioc.pc7.into_alternate_af6();

        // Setup I2S3 for 48kHz audio.
        let ck = gpioc.pc10.into_alternate_af6();
        let ws = gpioa.pa4.into_alternate_af6();
        let sd = gpioc.pc12.into_alternate_af6();
        let i2s3 = I2s::i2s3(spi3, (ck, ws, sd, NoSdExt), audio_freq.hz(), clocks);

        let cs43l22 = cs43l22::CS43L22::new(i2c1, reset, delay, vol)
            .expect("could not create CS43L22 driver");

        AudioOut { cs43l22, i2s: i2s3 }
    }
}
