//! This example echoes all communication received via USART back to the sender.
//!
//! **NOTE:** You need to connect your own USART to PA2 (TX) and PA3 (RX) or
//! modify the board as described in the user manual section 6.1.3

#![no_main]
#![no_std]

use panic_halt as _;

use stm32f407g_disc as board;

use nb::block;

use crate::board::{
    hal::prelude::*,
    hal::stm32,
    serial::{config::Config, Serial},
};

#[cortex_m_rt::entry]
fn main() -> ! {
    if let Some(p) = stm32::Peripherals::take() {
        let gpioa = p.GPIOA.split();
        let rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // USART2 at PA2 (TX) and PA3(RX) are connected to ST-Link
        // (well, not really, you're supposed to wire them yourself!)
        let tx = gpioa.pa2.into_alternate_af7();
        let rx = gpioa.pa3.into_alternate_af7();

        // Set up USART 2 configured pins and a baudrate of 115200 baud
        let serial = Serial::usart2(
            p.USART2,
            (tx, rx),
            Config::default().baudrate(115_200.bps()),
            clocks,
        )
        .unwrap();

        // Separate out the sender and receiver of the serial port
        let (mut tx, mut rx) = serial.split();

        loop {
            // Read character and echo it back
            let received = block!(rx.read()).unwrap();
            block!(tx.write(received)).ok();
        }
    }

    loop {
        continue;
    }
}
