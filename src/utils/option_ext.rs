pub trait OptionExt<T> {
    /// Unsafely unwraps the option.
    ///
    /// If `self` is `None`, then:
    ///  - In debug mode, this will panic with an error.
    ///  - In release modeIf this value is `None`, undefined behavior will occur
    fn unsafe_unwrap(self) -> T;
}

#[cfg(debug_assertions)]
impl<T> OptionExt<T> for Option<T> {
    fn unsafe_unwrap(self) -> T {
        self.expect("Called `unsafe_unwrap` on a `None` value")
    }
}

#[cfg(not(debug_assertions))]
impl<T> OptionExt<T> for Option<T> {
    fn unsafe_unwrap(self) -> T {
        match self {
            Some(s) => s,
            None => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}
