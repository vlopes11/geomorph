#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
extern crate alloc;

mod coord;
mod datum;
mod math;
mod mgrs;
mod utm;

pub use coord::Coord;
pub use datum::Datum;
pub use mgrs::{Mgrs, MgrsPrecision};
pub use utm::Utm;
