use crate::{duration, millis::Millis};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum State {
    #[default]
    InitialDraining,
    InitialIdling,
    InitialLocking,
    InitialUnlocking,
    InitialSetupSeparatorOpening,
    InitialSetupWaterPumping,
    InitialSetupSeparatorClosing,
    SoakWaterPumping,
    SoakWaterHeating,
    SoakWaterHeatedMixing,
    SoakWaterMixing,
    SoakWaterDraining,
    RinseWaterPumping,
    RinseWaterDraining,
    SeparatorOpening,
    SeparatorHolding,
    SeparatorClosing,
    Blending,
    PulpDraining,
    SetupSeparatorOpening,
    SetupWaterPumping,
    SetupSeparatorClosing,
    Idling,
    Locking,
    Unlocking,
}

#[allow(clippy::cognitive_complexity)]
#[must_use]
pub fn override_for(curr: State, delta: Millis, starting: bool, stopping: bool) -> Option<State> {
    Some(match curr {
        State::InitialDraining if delta >= duration::DRAINING => State::InitialIdling,

        State::InitialIdling if starting => State::InitialLocking,

        State::InitialLocking if !starting => State::InitialUnlocking,
        State::InitialLocking if delta >= duration::LOCKING => State::InitialSetupSeparatorOpening,

        State::InitialUnlocking if delta >= duration::LOCKING => State::InitialIdling,

        State::InitialSetupSeparatorOpening if delta >= duration::SEPARATOR_TRANSITION => {
            State::InitialSetupWaterPumping
        }

        State::InitialSetupWaterPumping if delta >= duration::WATER_PUMPING => {
            State::InitialSetupSeparatorClosing
        }

        State::InitialSetupSeparatorClosing if delta >= duration::SEPARATOR_TRANSITION => {
            State::SoakWaterPumping
        }

        State::SoakWaterPumping
        | State::SoakWaterHeating
        | State::SoakWaterHeatedMixing
        | State::SoakWaterMixing
            if stopping =>
        {
            State::SoakWaterDraining
        }

        State::SoakWaterPumping if delta >= duration::WATER_PUMPING => State::SoakWaterDraining,
        State::SoakWaterHeating if delta >= duration::HEATING => State::SoakWaterHeatedMixing,
        State::SoakWaterHeatedMixing if delta >= duration::HEATED_MIXING => State::SoakWaterMixing,
        State::SoakWaterMixing if delta >= duration::MIXING => State::SoakWaterDraining,

        State::SoakWaterDraining if delta >= duration::DRAINING => {
            if stopping {
                State::Idling
            } else {
                State::RinseWaterPumping
            }
        }

        State::RinseWaterPumping if stopping || delta >= duration::RINSING => {
            State::RinseWaterDraining
        }

        State::RinseWaterDraining if delta >= duration::DRAINING => {
            if stopping {
                State::Idling
            } else {
                State::SeparatorOpening
            }
        }

        State::SeparatorOpening if delta >= duration::SEPARATOR_TRANSITION => {
            State::SeparatorHolding
        }

        State::SeparatorHolding if delta >= duration::SEPARATOR_HOLDING => State::SeparatorClosing,

        State::SeparatorClosing if delta >= duration::SEPARATOR_TRANSITION => State::Blending,

        State::Blending if delta >= duration::BLENDING => State::PulpDraining,

        State::PulpDraining if delta >= duration::DRAINING => State::SetupSeparatorOpening,

        State::SetupSeparatorOpening if delta >= duration::SEPARATOR_TRANSITION => {
            State::SetupWaterPumping
        }

        State::SetupWaterPumping if delta >= duration::WATER_PUMPING => {
            State::SetupSeparatorClosing
        }

        State::SetupSeparatorClosing if delta >= duration::SEPARATOR_TRANSITION => State::Idling,

        State::Idling if starting => State::Locking,

        State::Locking if !starting => State::Unlocking,
        State::Locking if delta >= duration::LOCKING => State::SoakWaterPumping,

        State::Unlocking if delta >= duration::LOCKING => State::Idling,

        _ => return None,
    })
}
