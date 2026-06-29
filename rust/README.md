# Valuand.rs

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/valuand)](https://crates.io/crates/valuand)
[![Documentation](https://img.shields.io/docsrs/valuand?label=docs.rs)](https://docs.rs/valuand)

**The value to be carried in Rust.**

> [!TIP]
> 🚧 _We are building in public. This is presently under heavy construction._

<sub>

[[Features](#-features)] |
[[Prerequisites](#%EF%B8%8F-prerequisites)] |
[[Installation](#%EF%B8%8F-installation)] |
[[Examples](#-examples)] |
[[Reference](#-reference)] |
[[Development](#%E2%80%8D-development)]

</sub>

<br/>

## ✨ Features

- Implements a universal scalar value (aka variant) type for Rust.
- Plays nice with others: interoperates with frameworks such as [Serde].
- 100% pure and safe Rust with minimal dependencies and no bloat.
- Designed for `no_std` environment compatibility from the get-go.
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- Cuts red tape: 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add valuand
```

### Installation in `Cargo.toml`

Enable all default features:

```toml
[dependencies]
valuand = { version = "0" }
```

Enable only specific features:

```toml
[dependencies]
valuand = { version = "0", default-features = false, features = ["number"] }
```

## 👉 Examples

### Importing the Library

```rust
use valuand::{Value, ValueType};
use valuand::{Decimal, Float, Integer, Natural, Rational, Real};
```

### Matching on [`Value`]

```rust,compile_fail
match value {
    Value::Unit() => {}
    Value::Bool(value) => {}
    Value::Number(number) => {}
    Value::Other(t) => {}
}
```

### Matching on [`ValueType`]

```rust,compile_fail
match value.r#type() {
    ValueType::Unit => {}
    ValueType::Bool => {}
    ValueType::Number => {}
    ValueType::Other => {}
}
```

### Matching on [`Real`] Numbers

```rust,compile_fail
match value.as_number().unwrap() {
    Real::Float(r) => {}
    Real::Decimal(r) => {}
    Real::Rational(q) => {}
    Real::Integer(z) => {}
    Real::Natural(n) => {}
}
```

## 📚 Reference

[docs.rs/valuand](https://docs.rs/valuand)

### Variant Type

- [`Value`]
- [`ValueType`]

### Numeric Tower

- [`Complex`]
- [`Real`]
  - [`Float`]
  - [`Decimal`]
  - [`Rational`]
  - [`Integer`]
  - [`Natural`]

### Feature Flags

#### Interoperability

| Feature      | Version | Summary |
| :----------- | :------ | :------ |
| `serde`      | 1.0     | Derives `serde::{Serialize, Deserialize}`

## 👨‍💻 Development

```bash
git clone https://github.com/dryrust/valuand.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/dryrust/valuand&text=Valuand.rs)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/dryrust/valuand&title=Valuand.rs)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/dryrust/valuand&t=Valuand.rs)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/dryrust/valuand)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/dryrust/valuand)

[feature flags]: https://github.com/dryrust/valuand/blob/master/rust/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[Rust]: https://rust-lang.org
[Serde]: https://serde.rs

[`Complex`]: https://docs.rs/valuand/latest/valuand/type.Complex.html
[`Decimal`]: https://docs.rs/valuand/latest/valuand/struct.Decimal.html
[`Float`]: https://docs.rs/valuand/latest/valuand/enum.Float.html
[`Integer`]: https://docs.rs/valuand/latest/valuand/enum.Integer.html
[`Natural`]: https://docs.rs/valuand/latest/valuand/enum.Natural.html
[`Rational`]: https://docs.rs/valuand/latest/valuand/struct.Rational.html
[`Real`]: https://docs.rs/valuand/latest/valuand/enum.Real.html
[`Value`]: https://docs.rs/valuand/latest/valuand/enum.Value.html
[`ValueType`]: https://docs.rs/valuand/latest/valuand/enum.ValueType.html
