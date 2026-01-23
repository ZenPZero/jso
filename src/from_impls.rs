use std::{collections::HashMap, convert::Into};

use crate::Val::{self, *};

impl From<()> for Val {
  fn from((): ()) -> Self {
    Null
  }
}

impl From<bool> for Val {
  fn from(b: bool) -> Self {
    Bool(b)
  }
}

macro_rules! impl_from_num {
  ($($t:ty)*) => {
    $(
      impl From<$t> for Val {
        fn from(n: $t) -> Self {
          Num(n as f64)
        }
      }
    )*
  };
}

impl_from_num!(
  u8 u16 u32 u64 u128
  i8 i16 i32 i64 i128
  f32 f64
);

impl From<&str> for Val {
  fn from(s: &str) -> Self {
    Str(s.to_string())
  }
}

impl From<String> for Val {
  fn from(s: String) -> Self {
    Str(s)
  }
}

// covers [Val; N]
impl<V, const N: usize> From<[V; N]> for Val
where
  V: Into<Self>,
{
  fn from(a: [V; N]) -> Self {
    // Arr(Vec::from(a.map(|item| item.into())))
    Arr(a.map(Into::into).to_vec()) // requires Val: Clone
  }
}

// covers Vec<Val>
impl<V> From<Vec<V>> for Val
where
  V: Into<Self>,
{
  fn from(a: Vec<V>) -> Self {
    Arr(a.into_iter().map(Into::into).collect())
  }
}

// covers [(String, Val); N]
impl<K, V, const N: usize> From<[(K, V); N]> for Val
where
  K: Into<String>,
  V: Into<Self>,
{
  fn from(o: [(K, V); N]) -> Self {
    // Self::from(HashMap::from(o)) // requires K: Eq + Hash
    Obj(o.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
  }
}

// covers HashMap<String, Val>
impl<K, V> From<HashMap<K, V>> for Val
where
  K: Into<String>,
  V: Into<Self>,
{
  fn from(o: HashMap<K, V>) -> Self {
    Obj(o.into_iter().map(|(k, v)| (k.into(), v.into())).collect())
  }
}
