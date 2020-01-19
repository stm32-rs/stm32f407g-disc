//! On-board user button

use crate::hal::prelude::*;

use crate::hal::gpio::{
    gpioa::{PA, PA0},
    Input, PullDown,
};

use core::convert::Infallible;

pub type User = PA0<Input<PullDown>>;

pub struct Button {
    pin: PA<Input<PullDown>>,
}

impl Button {
    pub fn pressed(&self) -> Result<bool, Infallible> {
        return self.pin.is_high();
    }

    pub fn released(&self) -> Result<bool, Infallible> {
        return self.pin.is_low();
    }
}

impl Into<Button> for User {
    fn into(self) -> Button {
        Button {
            pin: self.downgrade(),
        }
    }
}
