//! Sends "Hello, world!" through the ITM port 0
//!
//! ITM is much faster than semihosting. Like 4 orders of magnitude or so.
//!
//! **NOTE** Cortex-M0 chips don't support ITM.
//!
//! You'll have to connect the microcontroller's SWO pin to the SWD interface. Note that some
//! development boards don't provide this option.
//!
//! You'll need [`itmdump`] to receive the message on the host plus you'll need to uncomment two
//! `monitor` commands in the `.gdbinit` file.
//!
//! [`itmdump`]: https://docs.rs/itm/0.2.1/itm/
//!
//! ---

#![no_main]
#![no_std]

extern crate panic_itm;
extern crate stm32f407g_disc as board;
extern crate embedded_hal as hal;

use board::hal::prelude::*;
use board::hal::stm32;

use cortex_m::{iprintln, Peripherals};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
	if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
	    // Constrain clock registers
	    let mut rcc = p.RCC.constrain();
	    // Configure clock to 168 MHz (i.e. the maximum) and freeze it
	    rcc.cfgr.sysclk(168.mhz()).freeze();

	    let mut itm = cp.ITM;
	    let stim = &mut itm.stim[0];

	    iprintln!(stim, "Hello, world!");
	}

    loop {}
}
