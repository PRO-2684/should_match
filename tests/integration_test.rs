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

// Testing shortcut macros

#[test]
#[apply(should_ok)]
fn test_ok() -> Result<(), &'static str> {
    Ok(())
}

#[test]
#[apply(should_err)]
fn test_err() -> Result<(), &'static str> {
    Err("error")
}

#[test]
#[apply(should_some)]
fn test_some() -> Option<()> {
    Some(())
}

#[test]
#[apply(should_none)]
fn test_none() -> Option<()> {
    None
}

// Testing custom enums

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
