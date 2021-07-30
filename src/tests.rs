//! # Tests
//! `test` tests

use crate::*;

#[test]
fn test_contructor() {
	let a = Percentage::new(50);
	assert!(100.0 * a == 50.0 as f32);
}

#[test]
fn test_percent_trait() {
	assert!(50.0 as f32 == 100.0*50.percent());
}

#[test]
fn test_eq() {
	let a = Percentage::new(50);
	let b = 50.percent();
	assert!(a == b, "a: {} -- b: {}", a, b);
}

#[test]
fn test_add_percentage() {
	let a = 100.percent() + 1.percent();
	assert!(a == 101.percent());
}

#[test]
fn test_add_number() {
	let a = 49.percent();
	let b = 1;
	assert!(149.percent() == 1+a, "a: {} + b: {} = c: {}", a, b, 1+a);
}

#[test]
fn test_print() {
	let a =	format!("{}", 50.percent());
	assert!(a == "50%", "a: {}", a);
}
