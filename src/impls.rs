use std::collections::HashMap;

use crate::Val::{self, *};

const fn variant_name(v: &Val) -> &'static str {
  match v {
    Null => "Null",
    Bool(..) => "Bool",
    Num(..) => "Num",
    Str(..) => "Str",
    Arr(..) => "Arr",
    Obj(..) => "Obj",
  }
}

macro_rules! impl_is {
  (
    $($fn:ident $variant:ident)*
  ) => {
    $(
      #[doc = "Returns `true` if the val is [`"]
      #[doc = ::core::stringify!($variant)]
      #[doc = "`]"]
      #[must_use]
      #[inline]
      pub fn $fn(&self) -> bool {
        matches!(self, $variant(..))
      }
    )*
  };
}

macro_rules! impl_unwrap {
  (
    $($fn:ident $variant:ident $ret:ty)*
  ) => {
    $(
      #[doc = "Returns the contained [`"]
      #[doc = ::core::stringify!($variant)]
      #[doc = "`] value, consuming the `self` value."]
      #[must_use]
      #[inline]
      pub fn $fn(self) -> $ret {
        if let $variant(x) = self {
          x
        } else {
          panic!(
            "called `Val::{}()` on a `{}` value",
            ::core::stringify!($fn),
            variant_name(&self),
          );
        }
      }
    )*
  };
}

macro_rules! impl_as {
  (
    $($fn:ident $variant:ident $ret:ty)*
  ) => {
    $(
      #[expect(missing_docs)]
      #[must_use]
      #[inline]
      pub fn $fn(&self) -> Option<&$ret> {
        if let $variant(x) = self {
          Some(x)
        } else {
          None
        }
      }
    )*
  };
}

macro_rules! impl_getter {
  (
    $($fn:ident $variant:ident $ret:ty)*
  ) => {
    $(
      #[expect(missing_docs)]
      #[must_use]
      #[inline]
      pub fn $fn(self) -> Option<$ret> {
        if let $variant(x) = self {
          Some(x)
        } else {
          None
        }
      }
    )*
  };
}

impl Val {
  /// Returns `true` if the val is [`Null`]
  #[must_use]
  #[inline]
  pub fn is_null(&self) -> bool {
    self == &Null
  }
  impl_is!(
    is_bool Bool
    is_num Num
    is_str Str
    is_arr Arr
    is_obj Obj
  );

  /// Returns the contained [`Null`] value, consuming the `self` value.
  #[inline]
  pub fn unwrap_null(self) {
    assert!(
      self == Null,
      "called `Val::unwrap_null()` on a `{}` value",
      variant_name(&self),
    );
  }
  impl_unwrap!(
    unwrap_bool Bool bool
    unwrap_num Num f64
    unwrap_str Str String
    unwrap_arr Arr Vec<Self>
    unwrap_obj Obj HashMap<String, Self>
  );

  #[expect(missing_docs)]
  #[must_use]
  #[inline]
  pub fn as_null(&self) -> Option<&()> {
    if let Null = self { Some(&()) } else { None }
  }
  impl_as!(
    as_bool Bool bool
    as_num Num f64
    as_str Str String
    as_arr Arr Vec<Self>
    as_obj Obj HashMap<String, Self>
  );

  #[expect(missing_docs)]
  #[must_use]
  #[inline]
  pub fn null(self) -> Option<()> {
    if let Null = self { Some(()) } else { None }
  }
  impl_getter!(
    bool Bool bool
    num Num f64
    str Str String
    arr Arr Vec<Self>
    obj Obj HashMap<String, Self>
  );
}
