//! # percentage-rs
//! `percentage-rs` Get the percentage of any number

#[cfg(test)] mod tests;

use std::{fmt, ops::{Add, AddAssign, Mul, MulAssign}};

use num_traits::NumCast;

/// Transform any number into [`Percentage`]
pub trait Percent {
	/// Transform any number into [`Percentage`]
	/// # Examples
	/// 
	/// ```
	/// use percentage_rs::Percent;
	///
	/// let percent = 50.percent();
	/// println!("{}", percent);
	/// ```
	fn percent(self) -> Percentage;
}

macro_rules! percent_impl {
	($($ty:ty)*) => {
		$(
			impl Percent for $ty {
				fn percent(self) -> Percentage {
					Percentage::new(self)
				}
			}
		)*
	};
}

percent_impl!{
	usize u8 u16 u32 u64 u128
	isize i8 i16 i32 i64 i128
	f32 f64
}

/// Allows to calculate the percentage of any number
/// # Examples
/// 
/// ### Create a [`Percentage`] with the default constructor
/// ```
/// use percentage_rs::Percentage;
/// 
/// let p = Percentage::new(50);
/// assert!("50%" == format!("{}", p)); // the result is "50%"
/// ```
/// 
/// ### Create a [`Percentage`] with the trait [`Percent`]
/// ```
/// use percentage_rs::Percent;
/// 
/// assert!(format!("{}", 50.percent()) == "50%"); // the result is "50%"
/// ```
/// 
/// ### Calculate the percentage
/// To do that you just need to multiply the percentage by your number
/// ```
/// use percentage_rs::Percent;
/// 
/// let p = 1234*50.percent(); // 50% of 1234
/// assert!(p == 617.0 as f32); // the result is "617"
/// ```
#[derive(Default, Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, serde::Deserialize))]
pub struct Percentage(f32);

impl Percentage {
	/// Create a percentage with the given value into percen
	pub fn new<T: NumCast>(value: T) -> Percentage {
		Self(value.to_f32().unwrap() / 100.0)
	}
}

impl fmt::Display for Percentage {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}%", self.0*100.0)
	}
}

impl Add for Percentage {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0)
	}
}

impl AddAssign for Percentage {
	fn add_assign(&mut self, rhs: Percentage) {
		*self = *self + rhs;
	}
}

impl<T: NumCast> AddAssign<T> for Percentage {
	fn add_assign(&mut self, rhs: T) {
		*self = Self(self.0 + rhs.to_f32().unwrap());
	}
}

impl Mul for Percentage {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		Self(rhs.0*self.0)
	}
}

impl MulAssign for Percentage {
	fn mul_assign(&mut self, rhs: Self) {
		*self = *self * rhs;
	}
}

macro_rules! ops_percentage_impl {
	($($type: ty)*) => {
		$(
			impl Add<Percentage> for $type {
				type Output = Percentage;
				fn add(self, rhs: Percentage) -> Self::Output {
					Percentage(rhs.0 + self as f32)
				}
			}

			impl Mul<Percentage> for $type {
				type Output = f32;
				fn mul(self, rhs: Percentage) -> Self::Output {
					self as f32 * rhs.0
				}
			}

			impl MulAssign<Percentage> for $type {
				fn mul_assign(&mut self, rhs: Percentage) {
					*self = (*self * rhs) as $type;
				}
			}
		)*
	};
}

ops_percentage_impl!{
	usize u8 u16 u32 u64 u128
	isize i8 i16 i32 i64 i128
	f32 f64
}
