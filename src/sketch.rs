use arduino_hal::{
    hal::port::{PB0, PB1, PB2, PB3, PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7},
    port::mode::{Input, Output, PullUp},
};

use crate::{
    duration,
    millis::{millis, Millis},
    state::State,
};

macro_rules! decl_sketch {
    {
        $($pin_field:ident : pin!( $mode:ty, $pin:ident : $pin_t:ty ) $({ $init:stmt })?),* $(,)?
        @
        $($field:ident : $field_t:ty = $default:expr),* $(,)?
    } => {
        #[allow(dead_code, unused_variables)]
        pub struct Sketch {
            $($pin_field : ::arduino_hal::port::Pin<$mode, $pin_t>,)*
            $($field : $field_t,)*
        }

        impl Sketch {
            #[allow(unused_mut, clippy::too_many_arguments)]
            #[must_use]
            pub fn new(
                $(mut $pin_field : ::arduino_hal::port::Pin<$mode, $pin_t>,)*
            ) -> Self {
                $($($init)?)*
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
    blender: pin!(Output, d0: PD0),
    heater: pin!(Output, d1: PD1),
    mixer: pin!(Output, d2: PD2),
    input_hatch_lock: pin!(Output, d3: PD3),
    lower_drain_pump: pin!(Output, d4: PD4),
    separator_hatch_direction: pin!(Output, d5: PD5),
    separator_hatch_enable: pin!(Output, d6: PD6),
    start: pin!(Input<PullUp>, d7: PD7),
    stop: pin!(Input<PullUp>, d8: PB0),
    ready: pin!(Output, d9: PB1) { ready.set_high() },
    upper_drain_pump: pin!(Output, d10: PB2),
    water_pump: pin!(Output, d11: PB3),
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
        let _stop = self.stop.is_low();

        macro_rules! transition_to {
            ($state:ident) => {
                self.state = $crate::state::State::$state;
                self.last_ms = curr_ms;
            };
        }

        match self.state {
            State::InitialIdling if self.start.is_low() => {
                transition_to!(InitialLocking);
                self.input_hatch_lock.set_high();
            }
            State::InitialLocking if delta_ms >= duration::LOCKING && self.start.is_low() => {
                transition_to!(InitialSetupSeparatorOpening);
                self.ready.set_low();
                self.separator_hatch_direction.set_low();
                self.separator_hatch_enable.set_high();
            }
            State::InitialLocking if self.start.is_high() => {
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
            State::SoakWaterPumping if delta_ms >= duration::SOAK_WATER_PUMPING => {
                transition_to!(SoakWaterHeating);
                self.water_pump.set_low();
                self.heater.set_high();
            }
            _ => { /* TODO */ }
        }
    }
}
