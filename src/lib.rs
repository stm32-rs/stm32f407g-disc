#![no_std]
#![allow(non_camel_case_types)]

pub use stm32f4xx_hal as hal;

pub use crate::hal::stm32::interrupt::*;
pub use crate::hal::stm32::Interrupt as interrupt;
pub use crate::hal::stm32::Peripherals;
pub use crate::hal::stm32::*;
pub use crate::hal::*;
pub use cortex_m::*;
pub use cortex_m_rt::*;

pub mod led;
