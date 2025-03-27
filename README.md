# `should_match`

Pass a test if the output matches a pattern.

## Setup

This crate is primarily intended for use in tests, so add it to your `dev-dependencies` instead of `dependencies`:

```shell
cargo add --dev should_match
```

Recommended to work with [`macro_rules_attr`](https://crates.io/crates/macro_rules_attr), which provides nice syntactic sugar:

```shell
cargo add --dev macro_rules_attr should_match
```

## The `should_match` macro

### With `macro_rules_attr`

Simply `apply` the `should_match` macro, and specify your pattern (the order of `apply` and `should_match` does not matter):

```rust
use macro_rules_attr::apply;
use should_match::should_match;

// This test will pass
#[apply(should_match, pattern = Err("error"))]
#[test]
fn test_apply_first() -> Result<(), &'static str> {
    Err("error")
}

// This test will also pass
#[test]
#[apply(should_match, pattern = Err("error"))]
fn test_apply_second() -> Result<(), &'static str> {
    Err("error")
}
```

To specify a custom panic message when it fails, pass an additional `message` argument:

```rust
# use macro_rules_attr::apply;
# use should_match::should_match;
#
#[test]
#[apply(should_match, pattern = Err(_), message = "Expected `Err`, but got `Ok`")]
fn test_with_custom_message() -> Result<(), &'static str> {
    Err("error")
}
```

### Direct usage

You can also use the `should_match` macro directly, but note that `#[test]` must be wrapped inside the macro:

```rust
use should_match::should_match;

// Without custom message
should_match! {
    #[test]
    fn test_match_direct() -> Result<(), &'static str> {
        Err("error")
    },
    pattern = Err("error")
}

// With custom message
should_match! {
    #[test]
    fn test_match_direct_custom_message() -> Result<(), &'static str> {
        Err("error")
    },
    pattern = Err("error"),
    message = "Expected `Err`, but got `Ok`"
}
```

## Shortcuts

### Predefined shortcuts

`should_match` provides some shortcuts for common patterns:

| Macro | Pattern | Message |
| --- | --- | --- |
| `should_ok` | `Ok(_)` | `` Expected `Ok`, but got `Err` `` |
| `should_err` | `Err(_)` | `` Expected `Err`, but got `Ok` `` |
| `should_none` | `None` | `` Expected `None`, but got `Some` `` |
| `should_some` | `Some(_)` | `` Expected `Some`, but got `None` `` |

An example of using `should_err`:

```rust
use macro_rules_attr::apply;
use should_match::should_err;

#[test]
#[apply(should_err)]
fn test_should_err() -> Result<(), &'static str> {
    Err("error")
}
```

Other shortcuts can be used in the same way.

### Define custom shortcuts

```rust
use macro_rules_attr::apply;
use should_match::should_match;

macro_rules! should_three {(
    $(#[$attr:meta])*
    $vis:vis fn $name:ident() -> $ret_ty:ty $body:block
) => {
    ::should_match::should_match! {
        $(#[$attr])*
        $vis fn $name() -> $ret_ty $body,
        pattern = $crate::Custom::Three, // Specify the pattern
        message = "Expected `Three`, but got something else" // Specify the message (optional)
    }
}}

#[test]
#[apply(should_three)]
fn test_custom_err() -> Custom {
    Custom::Three
}
```
