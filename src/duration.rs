use crate::millis::Millis;

macro_rules! constants {
    ($($name:ident = $expr:expr;)*) => {
        $(pub const $name: $crate::millis::Millis = $expr;)*
    };
}

constants! {
    DEFAULT = Millis(2000);
    DEFAULT_SHORT = Millis(250);

    LOCKING = DEFAULT_SHORT;
    SOAK_WATER_PUMPING = DEFAULT;
    SEPARATOR_TRANSITION = DEFAULT;
}
