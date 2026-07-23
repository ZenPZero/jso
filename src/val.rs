use crate::{Map, String, Vec};

/// Core json type
#[expect(missing_docs)]
#[derive(Debug, Default, Clone, PartialEq)]
pub enum Val {
  #[default]
  Null,
  Bool(bool),
  Num(f64),
  Str(String),
  Arr(Vec<Self>),
  Obj(Map<String, Self>),
}
