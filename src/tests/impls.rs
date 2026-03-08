use crate::Val;

macro_rules! case_fn {
  ($fn:ident, [$($init:expr),* $(,)?] $(,)?) => {
    $(
      assert!(Val::from($init).$fn());
    )*
  };
}

#[test]
fn is_() {
  case_fn!(is_null, [()]);

  case_fn!(is_bool, [false, true]);

  case_fn!(
    is_num,
    [
      0, 1, 2, -0, -1, -2, 0., 1., 2., -0., -1., -2, 123, 9234, -123281, -893242, 8923, 239800,
      -46378798,
    ],
  );

  case_fn!(
    is_str,
    ["", "a", "hello", "another string", String::from("wow")],
  );

  case_fn!(
    is_arr,
    [
      [] as [(); 0],
      [1, 2, 3],
      ["this", "is", "an", "array"],
      vec!["and", "so", "is", "this"],
    ],
  );

  case_fn!(
    is_obj,
    [
      [] as [(String, Val); 0],
      [("a", 1), ("b", 2), ("c", 3)],
      [("this", "is"), ("an", "object")],
      {
        use std::collections::HashMap;
        let mut h = HashMap::new();
        h.insert("and", "so");
        h.insert("is", "this");
        h
      },
    ],
  );
}

macro_rules! case_unwrap {
  ($fn:ident, [$($init:expr),* $(,)?] $(,)?) => {
    $(
      _ = Val::from($init).$fn();
    )*
  };
}

#[test]
fn unwrap_() {
  case_unwrap!(unwrap_null, [()]);

  case_unwrap!(unwrap_bool, [false, true]);

  case_unwrap!(
    unwrap_num,
    [
      0, 1, 2, -0, -1, -2, 0., 1., 2., -0., -1., -2, 123, 9234, -123281, -893242, 8923, 239800,
      -46378798,
    ],
  );

  case_unwrap!(
    unwrap_str,
    ["", "a", "hello", "another string", String::from("wow")],
  );

  case_unwrap!(
    unwrap_arr,
    [
      [] as [(); 0],
      [1, 2, 3],
      ["this", "is", "an", "array"],
      vec!["and", "so", "is", "this"],
    ],
  );

  case_unwrap!(
    unwrap_obj,
    [
      [] as [(String, Val); 0],
      [("a", 1), ("b", 2), ("c", 3)],
      [("this", "is"), ("an", "object")],
      {
        use std::collections::HashMap;
        let mut h = HashMap::new();
        h.insert("and", "so");
        h.insert("is", "this");
        h
      },
    ],
  );
}

// @todo as_
// @todo getters
