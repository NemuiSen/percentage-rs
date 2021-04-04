//! # percentage-rs
//! `percentage-rs` Get the percentage of any number

#[cfg(test)] mod tests;

use std::{
	fmt,
	ops::{
		Add,
		Mul,
		MulAssign
	},
};

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
					Percentage(self as f64)
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
/// let percent = Percentage(50.0);
/// println!("{}", percent); // the result is "50%"
/// ```
/// 
/// ### Create a [`Percentage`] with the trait [`Percent`]
/// ```
/// use percentage_rs::Percent;
/// 
/// let percent = 50.percent();
/// println!("{}", percent); // the result is "50%"
/// ```
/// 
/// ### Calculate the percentage
/// To do that you just need to multiply the percentage by your number
/// ```
/// use percentage_rs::Percent;
/// 
/// let res_num = 1234*50.percent(); // 50% of 1234
/// println!("{}", res_num); // the result is "617"
/// ```
#[derive(Clone, Copy)]
pub struct Percentage(pub f64);

impl Percentage {
	/// This function returns the value of the percentage divided by 100, this is because the percentage formula is `x*(p/100)` where:
	/// * `x` is the value from you want extract the percentage
	/// * `p` is the percentage
	pub fn get(&self) -> f64 {
		self.0 / 100.0
	}
}

impl Default for Percentage {
	fn default() -> Self {
		Self(0.0)
	}
}

impl fmt::Display for Percentage {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}%", self.0)
	}
}

impl Add for Percentage {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0)
	}
}

macro_rules! ops_percentage_impl {
	($($type: ty)*) => {
		$(
			impl Mul<Percentage> for $type {
				type Output = f64;
				fn mul(self, rhs: Percentage) -> Self::Output {
					self as f64 * rhs.get()
				}
			}

			impl Add<Percentage> for $type {
				type Output = Percentage;
				fn add(self, rhs: Percentage) -> Self::Output {
					((rhs.get() + self as f64)*100.0).percent()
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
