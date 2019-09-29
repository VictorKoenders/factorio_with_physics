#[macro_export]
macro_rules! some_or_continue {
    ($expr:expr) => {
        match $expr {
            Some(val) => val,
            None => continue,
        }
    };
}
