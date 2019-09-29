#[macro_export]
macro_rules! conversion {
    ($a:tt / $b:tt => $out:tt) => {
        impl std::ops::Div<$b> for $a {
            type Output = $out;
            fn div(self, rhs: $b) -> $out {
                $out(self.0 / rhs.0)
            }
        }
    };
    ($a:tt * $b:tt => $out:tt) => {
        impl std::ops::Mul<$b> for $a {
            type Output = $out;
            fn mul(self, rhs: $b) -> $out {
                $out(self.0 * rhs.0)
            }
        }
    };
}

#[macro_export]
macro_rules! unit {
    ($type:ty: $unit:expr) => {
        impl std::fmt::Debug for $type {
            fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
                use std::io::Write;
                write!(fmt, "{} {}", self.0, $unit)
            }
        }

        impl std::ops::AddAssign<$type> for $type {
            fn add_assign(&mut self, other: $type) {
                self.0 += other.0;
            }
        }

        impl std::ops::Sub<$type> for $type {
            type Output = Self;
            fn sub(self, other: $type) -> Self::Output {
                Self(self.0 - other.0)
            }
        }

        impl std::ops::Neg for $type {
            type Output = Self;
            fn neg(self) -> Self {
                Self(-self.0)
            }
        }
        impl Copy for $type {}
        impl Clone for $type {
            fn clone(&self) -> Self {
                Self(self.0)
            }
        }
    };
}
