#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;

extern crate stm32f407g_disc as board;

#[macro_use(block)]
extern crate nb;

use board::hal::prelude::*;
use board::hal::stm32;

use board::hal::serial::{config::Config, Serial};

use cortex_m_rt::entry;

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::Mutex;

// Make the write part of our serial port globally available
static PANIC_SERIAL: Mutex<RefCell<Option<board::serial::Tx<board::USART2>>>> =
    Mutex::new(RefCell::new(None));

use core::fmt::Write;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::free(|cs| {
        // Obtain mutex protected write part of serial port
        if let &mut Some(ref mut tx) = PANIC_SERIAL.borrow(cs).borrow_mut().deref_mut() {
            writeln!(tx, "\r\n{}", info).ok();
        }

        loop {}
    })
}

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
                if let &mut Some(ref mut tx) = PANIC_SERIAL.borrow(cs).borrow_mut().deref_mut() {
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

    loop {}
}
