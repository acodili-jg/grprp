macro_rules! constants {
    ($($name:ident = $expr:expr;)*) => {
        $(pub const $name: u32 = $expr;)*
    };
}

constants! {
    DEFAULT = 1000;
    DEFAULT_SHORT = 250;

    LOCKING = DEFAULT_SHORT;
    SOAK_WATER_PUMPING = DEFAULT;
    SEPARATOR_TRANSITION = DEFAULT;
}
