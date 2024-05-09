use arduino_hal::{
    hal::port::{PB0, PB1, PB2, PB3, PB4, PD0, PD1, PD2, PD3, PD4, PD5, PD7},
    port::mode::{Floating, Input, Output},
};

use crate::{millis, state::State};

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
                $crate::sketch::Sketch::new(
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
    last_ms: u32 = millis(),
}
