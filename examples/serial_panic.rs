//! This example shows a custom panic handler, which outputs the panic message
//! via USART. It receives 20 characters via USART and panics on the 21st.
//!
//! **NOTE:** You need to connect your own USART to PA2 (TX) and PA3 (RX) or
//! modify the board as described in the user manual section 6.1.3

#![no_main]
#![no_std]

use stm32f407g_disc as board;

use nb::block;

use crate::board::{
    hal::pac,
    hal::prelude::*,
    hal::serial::{config::Config, Serial, Tx},
};

use cortex_m::interrupt::Mutex;

use core::{cell::RefCell, fmt::Write, ops::DerefMut};

// Make the write part of our serial port globally available
static PANIC_SERIAL: Mutex<RefCell<Option<Tx<pac::USART2>>>> = Mutex::new(RefCell::new(None));

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::free(|cs| {
        // Obtain mutex protected write part of serial port
        if let Some(ref mut tx) = *PANIC_SERIAL.borrow(cs).borrow_mut().deref_mut() {
            writeln!(tx, "\r\n{}", info).ok();
        }

        loop {
            continue;
        }
    })
}

#[cortex_m_rt::entry]
fn main() -> ! {
    if let Some(p) = pac::Peripherals::take() {
        let gpioa = p.GPIOA.split();
        let rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // USART2 at PA2 (TX) and PA3(RX) are connected to ST-Link
        // (well, not really, you're supposed to wire them yourself!)
        let tx = gpioa.pa2;
        let rx = gpioa.pa3;

        // Set up USART 2 configured pins and a baudrate of 115200 baud
        let serial = Serial::new(
            p.USART2,
            (tx, rx),
            Config::default().baudrate(115_200.bps()),
            &clocks,
        )
        .unwrap();

        // Separate out the sender and receiver of the serial port
        let (tx, mut rx) = serial.split();

        // Transfer write part of serial port into Mutex
        cortex_m::interrupt::free(|cs| {
            *PANIC_SERIAL.borrow(cs).borrow_mut() = Some(tx);
        });

        let mut counter = 0;
        loop {
            // Read character and echo it back
            let received = block!(rx.read()).unwrap();

            // Obtain write part of serial port via Mutex
            cortex_m::interrupt::free(|cs| {
                if let Some(ref mut tx) = *PANIC_SERIAL.borrow(cs).borrow_mut().deref_mut() {
                    block!(tx.write(received)).ok();
                }
            });

            // Increment counter
            counter += 1;

            // Panic after 20 bytes written
            if counter > 20 {
                panic!("Too many bytes written!");
            }
        }
    }

    loop {
        continue;
    }
}
