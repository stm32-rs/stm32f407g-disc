#![no_main]
#![no_std]

extern crate cortex_m_rt;
extern crate panic_abort;

extern crate stm32f407g_disc as board;

#[macro_use(block)]
extern crate nb;

use board::hal::prelude::*;
use board::hal::stm32;

use board::hal::serial::{config::Config, Serial};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let Some(p) = stm32::Peripherals::take() {
        let gpioa = p.GPIOA.split();
        let mut rcc = p.RCC.constrain();
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

    loop {}
}
