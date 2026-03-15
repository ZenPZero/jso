use std::{
  collections::HashMap,
  fmt::{self, Display, Write},
  ops::Index,
};

use crate::{
  Unwrap,
  Val::{self, *},
};

impl Display for Val {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Null => f.write_str("null"),
      Bool(b) => write!(f, "{b}"),
      Num(n) => write!(f, "{n}"),
      Str(s) => escape_str(f, s),

      Arr(a) => {
        f.write_char('[')?;
        for (i, v) in a.iter().enumerate() {
          if i > 0 {
            f.write_char(',')?;
          }
          write!(f, "{v}")?;
        }
        f.write_char(']')
      }

      Obj(o) => {
        f.write_char('{')?;
        for (i, (k, v)) in o.iter().enumerate() {
          if i > 0 {
            f.write_char(',')?;
          }
          escape_str(f, k)?;
          write!(f, ":{v}")?;
        }
        f.write_char('}')
      }
    }
  }
}

fn escape_str(f: &mut fmt::Formatter, s: &str) -> fmt::Result {
  f.write_char('"')?;
  for c in s.chars() {
    match c {
      '"' => f.write_str("\\\"")?,
      '\\' => f.write_str("\\\\")?,
      '\x08' => f.write_str("\\b")?,
      '\x0c' => f.write_str("\\f")?,
      '\n' => f.write_str("\\n")?,
      '\r' => f.write_str("\\r")?,
      '\t' => f.write_str("\\t")?,

      c if c.is_control() => write!(f, "\\u{:04x}", c as u32)?,

      _ => f.write_char(c)?,
    }
  }
  f.write_char('"')
}

impl Index<&str> for Val {
  type Output = Self;

  fn index(&self, k: &str) -> &Self::Output {
    if let Obj(o) = self {
      o.get(k).unwrap_or(&Null)
    } else {
      &Null
    }
  }
}

macro_rules! impl_unwrap_trait {
  (
    $($fn:ident $ret:ty)*
  ) => {
    $(
      impl Unwrap<$ret> for Val {
        fn unwrap(self) -> $ret {
          self.$fn()
        }
      }
    )*
  };
}

impl_unwrap_trait!(
  unwrap_null ()
  unwrap_bool bool
  unwrap_num f64
  unwrap_str String
  unwrap_arr Vec<Self>
  unwrap_obj HashMap<String, Self>
);
