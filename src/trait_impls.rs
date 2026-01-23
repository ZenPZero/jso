use std::{
  collections::HashMap,
  fmt::{self, Display},
  ops::Index,
};

use crate::{
  Unwrap,
  Val::{self, *},
};

impl Display for Val {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Null => write!(f, "null"),
      Bool(b) => write!(f, "{b}"),
      Num(n) => write!(f, "{n}"),
      Str(s) => write!(f, r#""{s}""#),

      Arr(a) => {
        write!(f, "[")?;
        write!(
          f,
          "{}",
          a.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",")
        )?;
        write!(f, "]")
      }

      Obj(o) => {
        write!(f, "{{")?;
        write!(
          f,
          "{}",
          o.iter()
            .map(|(k, v)| format!(r#""{k}":{v}"#))
            .collect::<Vec<_>>()
            .join(",")
        )?;
        write!(f, "}}")
      }
    }
  }
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
