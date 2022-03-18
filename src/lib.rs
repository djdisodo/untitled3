use core::ops::*;
use std::fmt::{Formatter, Write};

#[cfg(test)]
mod tests {
    use crate::Percentage;

    #[test]
    fn mul() {
        let a = 100;
        let p = Percentage(140);

        println!("{}", a * p);
    }

    #[test]
    fn div() {
        let a = 100;
        let p = Percentage(140);

        println!("{}", a / p);
    }

    #[test]
    fn add() {
        let a = Percentage(58.99);
        let b = Percentage(32.96);
        let c = Percentage(23.74);
        let d = Percentage(19.41);
        let e = Percentage(9.32);
        println!("{}", a + b + c + d + e);
    }
}


#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Percentage<T>(pub T);

impl<T: core::fmt::Display> core::fmt::Display for Percentage<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)?;
        f.write_char('%')
    }
}



macro_rules! impl_percentage {
    ($t: ty) => {
        impl Mul<Percentage<$t>> for $t {
            type Output = $t;

            fn mul(self, rhs: Percentage<$t>) -> Self::Output {
                (self * rhs.0) / (100 as $t)
            }
        }
        impl Mul for Percentage<$t> {
            type Output = Self;

            fn mul(mut self, rhs: Self) -> Self::Output {
                self.0 *= rhs.0;
                self.0 /= 100 as $t;
                self
            }
        }

        impl Div<Percentage<$t>> for $t {
            type Output = $t;

            fn div(self, rhs: Percentage<$t>) -> Self::Output {
                (self * (100 as $t)) / rhs.0
            }
        }
        impl Div for Percentage<$t> {
            type Output = Self;

            fn div(mut self, rhs: Self) -> Self::Output {
                self.0 *= 100 as $t;
                self.0 /= rhs.0;
                self
            }
        }

        impl Add for Percentage<$t> {
            type Output = Self;

            fn add(mut self, rhs: Self) -> Self::Output {
                self.0 += rhs.0;
                self
            }
        }

        impl Sub for Percentage<$t> {
            type Output = Self;

            fn sub(mut self, rhs: Self) -> Self::Output {
                self.0 -= rhs.0;
                self
            }
        }
    }
}


//TODO support types by feature
impl_percentage!(u8);
impl_percentage!(i8);
impl_percentage!(u16);
impl_percentage!(i16);
impl_percentage!(u32);
impl_percentage!(i32);
impl_percentage!(f32);
impl_percentage!(u64);
impl_percentage!(i64);
impl_percentage!(f64);
impl_percentage!(usize);
impl_percentage!(isize);
