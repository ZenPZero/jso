# jso <!-- omit from toc -->

No-BS, no-bloat json library.

[crates.io](https://crates.io/crates/jso) |
[API Docs (docs.rs)](https://docs.rs/jso) |
[Changelog](CHANGELOG.md) |
[License](LICENSE)

- [Installation](#installation)
- [Examples](#examples)
  - [Parsing](#parsing)

## Installation

```shell
cargo add jso@=1.3.1
```

or

`Cargo.toml`

```toml
[dependencies]
jso = "=1.3.1"
```

or

```toml
[dependencies.jso]
version = "=1.3.1"
```

## Examples

For complete examples, see the [`examples`](examples/) directory.

### Parsing

```rust
let obj = jso::parse(r#"
{
  "Hello": "World"
}
"#).unwrap();
assert_eq!(obj["Hello"], "World".into());
println!("{obj}");
```
