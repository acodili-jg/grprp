#![no_std]
#![feature(abi_avr_interrupt)]

pub mod duration;
pub mod millis;
pub mod sketch;
pub mod state;

pub use millis::millis;

#[macro_export]
macro_rules! pin_mode {
    ($pin:expr, Input) => {
        $pin.into_floating_input().forget_imode()
    };
    ($pin:expr, InputFloating) => {
        $pin.into_floating_input()
    };
    ($pin:expr, InputPullUp) => {
        $pin.into_pull_up_input()
    };
    ($pin:expr, OpenDrain) => {
        $pin.into_opendrain()
    };
    ($pin:expr, OpenDrainHigh) => {
        $pin.into_opendrain_high()
    };
    ($pin:expr, Output) => {
        $pin.into_output()
    };
    ($pin:expr, OutputHigh) => {
        $pin.into_output_high()
    };
}
