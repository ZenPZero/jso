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
