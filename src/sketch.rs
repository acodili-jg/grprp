use arduino_hal::{
    hal::port::{PB0, PB1, PB2, PB3, PB4, PD0, PD1, PD2, PD3, PD4, PD5, PD7},
    port::mode::{Floating, Input, Output},
};

use crate::{
    duration,
    millis::{millis, Millis},
    state::State,
};

macro_rules! decl_sketch {
    {
        $($pin_field:ident : pin!( $mode:ty, $pin:ident : $pin_t:ty )),* $(,)?
        @
        $($field:ident : $field_t:ty = $default:expr),* $(,)?
    } => {
        #[allow(dead_code, unused_variables)]
        pub struct Sketch {
            $($pin_field : ::arduino_hal::port::Pin<$mode, $pin_t>,)*
            $($field : $field_t,)*
        }

        impl Sketch {
            #[allow(clippy::too_many_arguments)]
            #[must_use]
            pub fn new(
                $($pin_field : ::arduino_hal::port::Pin<$mode, $pin_t>,)*
            ) -> Self {
                Sketch {
                    $($pin_field,)*
                    $($field : $default,)*
                }
            }
        }

        #[macro_export]
        macro_rules! sketch {
            ($pins:expr) => {
                sketch::Sketch::new(
                    $(pin_mode!($pins.$pin, $mode),)*
                )
            };
        }
    };
}

decl_sketch! {
    lower_drain_pump: pin!(Output, d0: PD0),
    blender: pin!(Output, d1: PD1),
    separator_hatch_direction: pin!(Output, d2: PD2),
    separator_hatch_enable: pin!(Output, d3: PD3),
    upper_drain_pump: pin!(Output, d4: PD4),
    heater: pin!(Output, d5: PD5),
    start: pin!(Input<Floating>, d7: PD7),
    stop: pin!(Input<Floating>, d8: PB0),
    ready: pin!(Output, d9: PB1),
    input_hatch_lock: pin!(Output, d10: PB2),
    mixer: pin!(Output, d11: PB3),
    water_pump: pin!(Output, d12: PB4),
    @
    state: State = State::InitialIdling,
    last_ms: Millis = millis(),
}

impl Sketch {
    #[must_use]
    pub const fn state(&self) -> State {
        self.state
    }

    pub fn invoke(&mut self) {
        let curr_ms = millis();
        let delta_ms = curr_ms.wrapping_sub(self.last_ms);
        let _stop = self.stop.is_high();

        macro_rules! transition_to {
            ($state:ident) => {
                self.state = $crate::state::State::$state;
                self.last_ms = curr_ms;
            };
        }

        match self.state {
            State::InitialIdling if self.start.is_high() => {
                transition_to!(InitialLocking);
                self.input_hatch_lock.set_high();
            }
            State::InitialLocking if delta_ms >= duration::LOCKING && self.start.is_high() => {
                transition_to!(InitialSetupSeparatorOpening);
                self.separator_hatch_direction.set_low();
                self.separator_hatch_enable.set_high();
            }
            State::InitialLocking if self.start.is_low() => {
                transition_to!(InitialIdling);
                self.input_hatch_lock.set_low();
            }
            State::InitialSetupSeparatorOpening if delta_ms >= duration::SEPARATOR_TRANSITION => {
                transition_to!(InitialSetupWaterPumping);
                self.separator_hatch_enable.set_low();
                self.water_pump.set_high();
            }
            State::InitialSetupWaterPumping if delta_ms >= duration::SOAK_WATER_PUMPING => {
                transition_to!(InitialSetupSeparatorClosing);
                self.water_pump.set_low();
                self.separator_hatch_direction.set_high();
                self.separator_hatch_enable.set_high();
            }
            State::InitialSetupSeparatorClosing if delta_ms >= duration::SEPARATOR_TRANSITION => {
                transition_to!(SoakWaterPumping);
                self.separator_hatch_enable.set_low();
                self.water_pump.set_high();
            }
            _ => { /* TODO */ }
        }
    }
}
