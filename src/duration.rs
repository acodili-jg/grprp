use crate::millis::Millis;

macro_rules! constants {
    ($($name:ident = $expr:expr;)*) => {
        $(pub const $name: $crate::millis::Millis = $expr;)*
    };
}

constants! {
    DEFAULT_LONG = Millis(5000);
    DEFAULT = Millis(2000);
    DEFAULT_SHORT = Millis(250);

    DRAINING = DEFAULT_LONG;
    LOCKING = DEFAULT_SHORT;
    SOAK_WATER_PUMPING = DEFAULT_LONG;
    SEPARATOR_TRANSITION = DEFAULT;
}
