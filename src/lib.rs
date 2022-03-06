#![cfg_attr(feature = "no_std", no_std)]

//! # Feo Template
//! Please read the README.md for more information.

#[cfg(feature = "no_std")]
#[macro_use]
extern crate alloc;

mod native {
	#[cfg(feature = "no_std")]
	pub use alloc::vec::Vec;

	#[cfg(not(feature = "no_std"))]
	pub use std::{
		vec::Vec
	};
}
