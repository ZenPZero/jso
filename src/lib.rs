#![no_std]
#![warn(missing_docs)]

//! # Example
//! ```
//! let obj = jso::parse(r#"
//! {
//!   "Hello": "World"
//! }
//! "#).unwrap();
//! assert_eq!(obj["Hello"], "World".into());
//! println!("{obj}");
//! ```

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::{
  string::{String, ToString},
  vec::Vec,
};

#[cfg(not(feature = "std"))]
#[expect(missing_docs)]
pub type Map<K, V> = alloc::collections::BTreeMap<K, V>;
#[cfg(feature = "std")]
#[expect(missing_docs)]
pub type Map<K, V> = std::collections::HashMap<K, V>;

#[cfg(test)]
mod tests;

mod traits;
mod val;

pub mod parse;

mod from_impls;
mod impls;
mod trait_impls;

pub use traits::*;
pub use val::Val;

/// Parses a json value, disregarding whitespace.\
/// See the [`mod@parse`] module for more info
pub fn parse(s: &str) -> parse::Result {
  parse::val(&mut s.chars().enumerate().peekable())
}
