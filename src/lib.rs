#![cfg_attr(feature = "no_std", no_std)]

//! # Abstract Chearmyp Node
//! Please read the README.md for more information.
//!
//! ## Features available
//! - `no_std`: Uses the `core` crate instead of `std` crate.

#[cfg(feature = "no_std")]
extern crate alloc;
