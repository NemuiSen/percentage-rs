//! # Tests
//! `test` tests

use crate::*;

#[test]
fn test_partial_eq() {
	assert!(50.percent() == 50.percent());
}

#[test]
fn test_percentage() {
	let percent = Percentage(50.0);
	assert!(50.0 == 100*percent);
}

#[test]
fn test_percent() {
	assert!(50.0 == 100*50.percent());
}

#[test]
fn test_add() {
	let mut a = 50.percent();
	a += 50.percent();
	assert!(a == 50.percent() + 50.percent());
}

#[test]
fn test_add_2() {
	println!("{}", 1+50.percent())
}
