//! # Tests
//! `test` tests

use crate::*;

#[test]
fn test_percentage() {
	let percent = Percentage(50.0);
	assert!(50.0 == 100*percent);
}

#[test]
fn test_percent() {
	assert!(50.0 == 100*50.percent());
}