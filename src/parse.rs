//! This module contains functions for parsing json.\
//! See [`val()`] and [`Error`]

use std::{
  collections::HashMap,
  fmt::{self, Display},
  iter::{Enumerate, Peekable},
  num::ParseFloatError,
  str::Chars,
};

use crate::Val::{self, *};

use Error::*;

/// Return type of all parsing functions except [`str()`] which returns
/// <code>[Result]<[String]></code>
pub type Result<T = Val, E = Error> = std::result::Result<T, E>;

/// A json parsing error
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
  /// Input ended in the middle of parsing
  UnexpectedEof,
  /// Encountered an illegal character
  UnexpectedChar(usize, char),
  /// Encountered an invalid number
  InvalidNum(ParseFloatError),
}

impl Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      UnexpectedEof => write!(f, "unexpected end of file"),

      UnexpectedChar(i, c) => write!(f, "unexpected character {c:?} at index {i}"),

      InvalidNum(err) => write!(f, "invalid number: {err}"),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      InvalidNum(err) => Some(err),

      _ => None,
    }
  }
}

/// Parses a json value, disregarding whitespace
pub fn val(chars: &mut Peekable<Enumerate<Chars>>) -> Result {
  skip_ws(chars);
  match chars.peek() {
    Some((_, 'n')) => null(chars),
    Some((_, 't' | 'f')) => bool(chars),

    Some(&(_, c)) if c.is_ascii_digit() || c == '-' => num(chars),

    Some((_, '"')) => Ok(Str(str(chars)?)),
    Some((_, '[')) => arr(chars),
    Some((_, '{')) => obj(chars),

    Some(&(i, c)) => Err(UnexpectedChar(i, c)),
    None => Err(UnexpectedEof),
  }
}

macro_rules! expect {
  ($expr:expr => $char:literal) => {
    match $expr {
      Some((_, $char)) => {}

      Some((i, c)) => return Err(UnexpectedChar(i, c)),
      None => return Err(UnexpectedEof),
    }
  };

  ($expr:expr => $($char:literal)*) => {
    $(
      expect!($expr => $char);
    )*
  };
}

/// Parses a json null value (`null`), disregarding whitespace
pub fn null(chars: &mut Peekable<Enumerate<Chars>>) -> Result {
  skip_ws(chars);
  expect!(chars.next() => 'n' 'u' 'l' 'l');
  Ok(Null)
}

/// Parses a json boolean value (`true`/`false`), disregarding whitespace
pub fn bool(chars: &mut Peekable<Enumerate<Chars>>) -> Result {
  skip_ws(chars);
  match chars.peek() {
    Some((_, 't')) => {
      expect!(chars.next() => 't' 'r' 'u' 'e');
      Ok(Bool(true))
    }

    Some((_, 'f')) => {
      expect!(chars.next() => 'f''a''l''s''e');
      Ok(Bool(false))
    }

    Some(&(i, c)) => Err(UnexpectedChar(i, c)),
    None => Err(UnexpectedEof),
  }
}

/// Parses a json number value, disregarding whitespace
pub fn num(chars: &mut Peekable<Enumerate<Chars>>) -> Result {
  let mut n = String::new();

  skip_ws(chars);
  loop {
    match chars.peek() {
      Some(&(_, c)) if c.is_ascii_digit() => {
        n.push(c);
        chars.next();
      }

      Some(&(_, c @ ('-' | '+' | '.' | 'e' | 'E'))) => {
        n.push(c);
        chars.next();
      }

      _ => break,
    }
  }

  n.parse::<f64>().map(Num).map_err(InvalidNum)
}

/// Parses a json string, disregarding whitespace
///
/// <div class="warning">
///
/// This function returns a [`String`] rather than a [`Val`] so it can be used to parse both string
/// values and object keys
///
/// </div>
pub fn str(chars: &mut Peekable<Enumerate<Chars>>) -> Result<String> {
  let mut s = String::new();

  skip_ws(chars);
  expect!(chars.next() => '"');
  loop {
    match chars.next() {
      Some((_, '"')) => break,

      Some((_, c)) => s.push(c),

      None => return Err(UnexpectedEof),
    }
  }

  Ok(s)
}

/// Parses a json array value, disregarding whitespace
pub fn arr(chars: &mut Peekable<Enumerate<Chars>>) -> Result {
  let mut arr = Vec::new();

  expect!(chars.next() => '[');
  loop {
    skip_ws(chars);
    match chars.peek() {
      Some((_, ']')) => {
        chars.next();
        break;
      }

      Some((_, ',')) => {
        chars.next();
        skip_ws(chars);
      }

      Some(_) => arr.push(val(chars)?),

      None => return Err(UnexpectedEof),
    }
  }

  Ok(Arr(arr))
}

/// Parses a json object value, disregarding whitespace
pub fn obj(chars: &mut Peekable<Enumerate<Chars>>) -> Result {
  let mut obj = HashMap::new();

  expect!(chars.next() => '{');
  loop {
    skip_ws(chars);
    match chars.peek() {
      Some((_, '}')) => {
        chars.next();
        break;
      }

      Some((_, ',')) => {
        chars.next();
        skip_ws(chars);
      }

      Some(_) => {
        let k = str(chars)?;
        skip_ws(chars);
        expect!(chars.next() => ':');
        let v = val(chars)?;
        obj.insert(k, v);
      }

      None => return Err(UnexpectedEof),
    }
  }

  Ok(Obj(obj))
}

fn skip_ws(chars: &mut Peekable<Enumerate<Chars>>) {
  while let Some((_, c)) = chars.peek() {
    if c.is_whitespace() {
      chars.next();
    } else {
      break;
    }
  }
}
