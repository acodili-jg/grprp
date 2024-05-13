use crate::millis::Millis;

macro_rules! constants {
    ($($name:ident = $expr:expr;)*) => {
        $(pub const $name: $crate::millis::Millis = $expr;)*
    };
}

constants! {
    DEFAULT_LONG = Millis(5000);
    DEFAULT = Millis(2000);
    DEFAULT_SHORT = Millis(500);

    DRAINING = DEFAULT_LONG;
    HEATED_MIXING = DEFAULT;
    HEATING = DEFAULT_LONG;
    LOCKING = DEFAULT_SHORT;
    MIXING = DEFAULT_LONG;
    RINSING = DEFAULT_LONG;
    SEPARATOR_HOLDING = DEFAULT;
    SEPARATOR_TRANSITION = DEFAULT;
    SOAK_WATER_PUMPING = DEFAULT_LONG;
}
