#![cfg_attr(feature = "substr", feature(wrapping_int_impl))]

mod sorts;
pub use sorts::*;

pub mod binary_tree;

#[cfg(feature = "substr")]
pub mod substr;
