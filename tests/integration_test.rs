#![no_std]

use macro_rules_attr::apply;
use should_match::{should_match, should_ok, should_err, should_some, should_none};

// Testing `should_match!`

should_match! {
    #[test]
    fn test_match_direct() -> Result<(), &'static str> {
        Err("error")
    },
    pattern = Err("error")
}

#[apply(should_match, pattern = Err("error"))]
#[test]
fn test_match_apply_first() -> Result<(), &'static str> {
    Err("error")
}

#[test]
#[apply(should_match, pattern = Err("error"))]
fn test_match_apply_second() -> Result<(), &'static str> {
    Err("error")
}

#[test]
#[apply(should_match, pattern = Err("error"))]
fn test_match_with_return() -> Result<(), &'static str> {
    return Err("error");
}

#[test]
#[apply(should_match, pattern = Err("error"))]
fn test_match_with_question_mark() -> Result<(), &'static str> {
    Err("error")?;
    Ok(())
}

#[test]
#[apply(should_match, pattern = Err("error"), message = "Expected `Err`, but got `Ok`")]
fn test_match_with_message() -> Result<(), &'static str> {
    Err("error")
}

#[apply(should_match, pattern = Err("error"))]
#[test]
#[should_panic(expected = "Expected to match `Err(\"error\")`")]
fn test_match_panic() -> Result<(), &'static str> {
    Ok(())
}

// Testing shortcut macros

#[test]
#[apply(should_ok)]
fn test_ok() -> Result<(), &'static str> {
    Ok(())
}

#[apply(should_ok)]
#[test]
#[should_panic(expected = "Expected `Ok`, but got `Err`")]
fn test_ok_panic() -> Result<(), &'static str> {
    Err("error")
}

#[test]
#[apply(should_err)]
fn test_err() -> Result<(), &'static str> {
    Err("error")
}

#[apply(should_err)]
#[test]
#[should_panic(expected = "Expected `Err`, but got `Ok`")]
fn test_err_panic() -> Result<(), &'static str> {
    Ok(())
}

#[test]
#[apply(should_some)]
fn test_some() -> Option<()> {
    Some(())
}

#[apply(should_some)]
#[test]
#[should_panic(expected = "Expected `Some`, but got `None`")]
fn test_some_panic() -> Option<()> {
    None
}

#[test]
#[apply(should_none)]
fn test_none() -> Option<()> {
    None
}

#[apply(should_none)]
#[test]
#[should_panic(expected = "Expected `None`, but got `Some`")]
fn test_none_panic() -> Option<()> {
    Some(())
}

// Testing custom enums & shortcuts

#[allow(dead_code, reason = "This is a test")]
enum Custom {
    One,
    Two,
    Three,
}

#[test]
#[apply(should_match, pattern = Custom::One)]
fn test_custom_ok() -> Custom {
    Custom::One
}

macro_rules! should_three {(
    $(#[$attr:meta])*
    $vis:vis fn $name:ident() -> $ret_ty:ty $body:block
) => {
    ::should_match::should_match! {
        $(#[$attr])*
        $vis fn $name() -> $ret_ty $body,
        pattern = $crate::Custom::Three,
        message = "Expected `Three`, but got something else"
    }
}}

#[test]
#[apply(should_three)]
fn test_custom_err() -> Custom {
    Custom::Three
}
