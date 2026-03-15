# 1.3.0 (Mar 14 2026)

- Added escape support to `parse::str()`
  - Unicode escapes (`\uXXXX`) not yet supported

# 1.2.0 (Mar 7 2026)

- Implemented `std::fmt::Display` for `parse::Error`
- Implemented `std::error::Error` for `parse::Error`

# 1.1.0 (Jul 27 2025) (imported Jan 23 2026)

- Added `Val.{is,unwrap,as}_{null,bool,num,str,arr,obj}()`
- Added `Val.{null,bool,num,str,arr,obj}()`
- Added `Unwrap` trait with implementation for `Val` (`Val.unwrap()`)
- `Val` now derives `Default`, the default being `Val::Null`

# 1.0.0 (Apr 20 2025) (imported Jul 24 2025)

- Added number parsing

# 0.9.0 (Mar 7 2025)

- Added parsing

# 0.1.0 (Mar 1 2025)

- Initial release
