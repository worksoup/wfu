# Worksoup's Formatting Utilities

[![Crates.io](https://img.shields.io/crates/v/wfu)](https://crates.io/crates/wfu)
[![Documentation](https://docs.rs/wfu/badge.svg)](https://docs.rs/wfu)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

```toml
[dependencies]
wfu = "0.0.3"
```

```rust
use wfu::*;
use std::ops::Deref;


#[derive(Debug, Clone, Copy, Default)]
struct UppercaseProxy;
impl FmtHandler<&str> for UppercaseProxy {
    #[inline]
    fn fmt(&self, data: &&str, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        for c in data.chars() {
            write!(f, "{}", c.to_uppercase())?;
        }
        Ok(())
    }
}
impl FmtHandler<str> for UppercaseProxy {
    #[inline]
    fn fmt(&self, data: &str, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        for c in data.chars() {
            write!(f, "{}", c.to_uppercase())?;
        }
        Ok(())
    }
}
impl FmtHandler<String> for UppercaseProxy {
    #[inline(always)]
    fn fmt(&self, data: &String, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        <Self as FmtHandler<str>>::fmt(self, data, f)
    }
}

fn main() {
    let s = "hello world";

    let result = format!("{}", s.fmt_as::<UppercaseProxy>());
    assert_eq!(result, "HELLO WORLD");
}
```

## Provided

### 内置

```rust
use wfu::*;
 
let s = "hello";
let display_result = format!("{}", s.fmt_as::<DisplayProxy>());
let debug_result = format!("{}", s.fmt_as::<DebugProxy>());

let vec = vec!["a", "b", "c"];
// Joined
let joined = format!("{}", vec.fmt_by(Joined(", ")));
assert_eq!(joined, "a, b, c");

// Repeat
let stars = format!("{}", "*".fmt_by(Repeat(5)));
assert_eq!(stars, "*****");
```

### 闭包

```rust
use wfu::*;

let value = 42;
let holder = value.fmt_with(&|v, f| write!(f, "Value: {} ({:#x})", v, v));
let result = format!("{}", holder);
assert_eq!(result, "Value: 42 (0x2a)");
```
