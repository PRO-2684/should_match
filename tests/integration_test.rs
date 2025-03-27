#![no_std]

use macro_rules_attr::apply;
use should_match::should_match;

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







