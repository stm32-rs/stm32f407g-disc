use accelerometer;
use lis302dl;

use crate::hal::gpio;
use crate::hal::gpio::gpioa;
use crate::hal::gpio::gpioe;
use crate::hal::prelude::*;
use crate::hal::rcc;
use crate::hal::spi;
use crate::hal::stm32;

use embedded_hal;
use embedded_hal::digital::v2::OutputPin;

type Spi1 = spi::Spi<
    stm32::SPI1,
    (
        gpioa::PA5<gpio::Alternate<gpio::AF5>>,
        gpioa::PA6<gpio::Alternate<gpio::AF5>>,
        gpioa::PA7<gpio::Alternate<gpio::AF5>>,
    ),
    spi::TransferModeNormal,
>;

type ChipSelect = gpioe::PE3<gpio::Output<gpio::PushPull>>;

pub struct Accelerometer {
    lis302dl: lis302dl::Lis302Dl<Spi1, ChipSelect>,
}

impl Accelerometer {
    pub fn new(
        gpioa: gpioa::Parts,
        gpioe: gpioe::Parts,
        spi1: stm32::SPI1,
        clocks: rcc::Clocks,
    ) -> Self {
        let sck = gpioa.pa5.into_alternate_af5().internal_pull_up(false);
        let miso = gpioa.pa6.into_alternate_af5().internal_pull_up(false);
        let mosi = gpioa.pa7.into_alternate_af5().internal_pull_up(false);

        let spi_mode = spi::Mode {
            polarity: spi::Polarity::IdleLow,
            phase: spi::Phase::CaptureOnFirstTransition,
        };

        let spi = spi::Spi::spi1(spi1, (sck, miso, mosi), spi_mode, 10.mhz().into(), clocks);

        let mut chip_select = gpioe.pe3.into_push_pull_output();
        chip_select.set_high().ok();

        let config = lis302dl::Config {
            scale: lis302dl::Scale::PlusMinus8G,
            ..Default::default()
        };
        let lis302dl = lis302dl::Lis302Dl::new(spi, chip_select, config);

        Self { lis302dl }
    }
}

impl accelerometer::RawAccelerometer<accelerometer::vector::I8x3> for Accelerometer {
    type Error = spi::Error;
    fn accel_raw(
        &mut self,
    ) -> Result<accelerometer::vector::I8x3, accelerometer::Error<Self::Error>> {
        self.lis302dl.accel_raw()
    }
}

impl accelerometer::Accelerometer for Accelerometer {
    type Error = spi::Error;
    fn sample_rate(&mut self) -> Result<f32, accelerometer::Error<Self::Error>> {
        self.lis302dl.sample_rate()
    }

    fn accel_norm(
        &mut self,
    ) -> Result<accelerometer::vector::F32x3, accelerometer::Error<Self::Error>> {
        self.lis302dl.accel_norm()
    }
}
