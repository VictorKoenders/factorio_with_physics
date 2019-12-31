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

        impl std::ops::SubAssign<$type> for $type {
            fn sub_assign(&mut self, other: $type) {
                self.0 = self.0 - other.0;
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

        impl std::cmp::PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl std::cmp::PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl std::cmp::Eq for $type {}
        impl std::cmp::Ord for $type {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl std::ops::Div<Self> for $type {
            type Output = R32;

            fn div(self, other: Self) -> R32 {
                self.0 / other.0
            }
        }
        impl std::ops::MulAssign<f32> for $type {
            fn mul_assign(&mut self, other: f32) {
                self.0 *= other;
            }
        }
    };
}
