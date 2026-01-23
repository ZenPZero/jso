use crate::Val::{self, *};

macro_rules! case {
  (
    $($in:expr => $out:expr)*
  ) => {
    $(
      assert_eq!(Val::from($in), $out);
    )*
  };
}

#[test]
fn null() {
  case!(() => Null);
}

#[test]
fn bool() {
  case!(
    true  => Bool(true)
    false => Bool(false)
  );
}

#[test]
fn num() {
  case!(
    4_u8   => Num(4.)
    4_u16  => Num(4.)
    4_u32  => Num(4.)
    4_u64  => Num(4.)
    4_u128 => Num(4.)

    4_i8   => Num(4.)
    4_i16  => Num(4.)
    4_i32  => Num(4.)
    4_i64  => Num(4.)
    4_i128 => Num(4.)

    4_f32  => Num(4.)
    4_f64  => Num(4.)
  );
}

#[test]
fn str() {
  case!(
    ""                 => Str(""    .to_string())
    ""    .to_string() => Str(""    .to_string())
    "what"             => Str("what".to_string())
    "what".to_string() => Str("what".to_string())
  );
}

// @todo Arr Obj
