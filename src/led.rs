//! On-board user LEDs

use crate::hal::prelude::*;

use crate::hal::gpio::gpiod::{self, PD, PD12, PD13, PD14, PD15};
use crate::hal::gpio::{Output, PushPull};

/// Top LED (orange)
pub type LD3 = PD12<Output<PushPull>>;

/// Left LED (green)
pub type LD4 = PD13<Output<PushPull>>;

/// Right LED (red)
pub type LD5 = PD15<Output<PushPull>>;

/// Bottom LED (blue)
pub type LD6 = PD14<Output<PushPull>>;

/// User LED colors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedColor {
    /// Green LED / LD4
    Green,
    /// Orange LED / LD3
    Orange,
    /// Red LED / LD5
    Red,
    /// Blue LED / LD6
    Blue,
}

// Array of the on-board user LEDs
pub struct Leds {
    leds: [Led; 4],
}

impl Leds {
    pub fn new(gpiod: gpiod::Parts) -> Self {
        let top = gpiod.pd12.into_push_pull_output();
        let left = gpiod.pd13.into_push_pull_output();
        let right = gpiod.pd14.into_push_pull_output();
        let bottom = gpiod.pd15.into_push_pull_output();

        Leds {
            leds: [top.into(), left.into(), right.into(), bottom.into()],
        }
    }
}

impl core::ops::Deref for Leds {
    type Target = [Led];

    fn deref(&self) -> &[Led] {
        &self.leds
    }
}

impl core::ops::DerefMut for Leds {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.leds
    }
}

impl core::ops::Index<usize> for Leds {
    type Output = Led;

    fn index(&self, i: usize) -> &Led {
        &self.leds[i]
    }
}

impl core::ops::Index<LedColor> for Leds {
    type Output = Led;

    fn index(&self, c: LedColor) -> &Led {
        &self.leds[c as usize]
    }
}

impl core::ops::IndexMut<usize> for Leds {
    fn index_mut(&mut self, i: usize) -> &mut Led {
        &mut self.leds[i]
    }
}

impl core::ops::IndexMut<LedColor> for Leds {
    fn index_mut(&mut self, c: LedColor) -> &mut Led {
        &mut self.leds[c as usize]
    }
}

/// One of the on-board user LEDs
pub struct Led {
    pin: PD<Output<PushPull>>,
}

macro_rules! ctor {
	($($ldx:ident),+) => {
		$(
			impl Into<Led> for $ldx {
				fn into(self) -> Led {
					Led {
						pin: self.downgrade(),
					}
				}
			}
		)+
	}
}

ctor!(LD3, LD4, LD5, LD6);

impl Led {
    /// Turns the LED off
    pub fn off(&mut self) {
        self.pin.set_low().unwrap();
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        self.pin.set_high().unwrap();
    }

    /// Toggles the LED
    pub fn toggle(&mut self) {
        if let Ok(true) = self.pin.is_low() {
            self.pin.set_high().unwrap();
        } else {
            self.pin.set_low().unwrap();
        }
    }
}
