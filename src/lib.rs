#![doc = include_str!("../README.md")]
#![no_std]

/// Wraps a function that takes nothing, panicking if the result does not match the expected pattern.
///
/// See the [crate-level documentation](crate) for more information.
#[macro_export]
macro_rules! should_match {(
    $(#[$attr:meta])*
    $vis:vis fn $name:ident() -> $ret_ty:ty $body:block,
    pattern = $pattern:pat
    $(, message = $message:literal)?
) => {
    $(#[$attr])*
    $vis fn $name() {
        fn inner() -> $ret_ty $body
        let result = inner();
        let is_match = ::core::matches!(result, $pattern);
        let message = ::core::concat!("Expected to match `", stringify!($pattern), "`");
        // Shadow the message if it was provided
        $(let message = $message;)?
        ::core::assert!(is_match, "{message}");
    }
}}

// Shortcuts

/// Wraps a function that takes nothing, panicking if the result is not `Ok`. Shortcut for [`should_match!`].
///
/// ```rust ignore
/// // This macro invocation:
/// should_ok! { ... }
/// // Is equivalent to:
/// should_match! { ..., pattern = Ok(_), message = "Expected `Ok`, but got `Err`" }
/// ```
///
/// You probably don't need this, since this is supported by `#[test]` directly. It exists solely for consistency.
#[macro_export]
macro_rules! should_ok {(
    $(#[$attr:meta])*
    $vis:vis fn $name:ident() -> $ret_ty:ty $body:block
) => {
    $crate::should_match! {
        $(#[$attr])*
        $vis fn $name() -> $ret_ty $body,
        pattern = ::core::result::Result::Ok(_),
        message = "Expected `Ok`, but got `Err`"
    }
}}

/// Wraps a function that takes nothing, panicking if the result is not `Err`. Shortcut for [`should_match!`].
///
/// ```rust ignore
/// // This macro invocation:
/// should_err! { ... }
/// // Is equivalent to:
/// should_match! { ..., pattern = Err(_), message = "Expected `Err`, but got `Ok`" }
/// ```
#[macro_export]
macro_rules! should_err {(
    $(#[$attr:meta])*
    $vis:vis fn $name:ident() -> $ret_ty:ty $body:block
) => {
    $crate::should_match! {
        $(#[$attr])*
        $vis fn $name() -> $ret_ty $body,
        pattern = ::core::result::Result::Err(_),
        message = "Expected `Err`, but got `Ok`"
    }
}}

/// Wraps a function that takes nothing, panicking if the result is not `Some`. Shortcut for [`should_match!`].
///
/// ```rust ignore
/// // This macro invocation:
/// should_some! { ... }
/// // Is equivalent to:
/// should_match! { ..., pattern = Some(_), message = "Expected `Some`, but got `None`" }
/// ```
#[macro_export]
macro_rules! should_some {(
    $(#[$attr:meta])*
    $vis:vis fn $name:ident() -> $ret_ty:ty $body:block
) => {
    $crate::should_match! {
        $(#[$attr])*
        $vis fn $name() -> $ret_ty $body,
        pattern = ::core::option::Option::Some(_),
        message = "Expected `Some`, but got `None`"
    }
}}

/// Wraps a function that takes nothing, panicking if the result is not `None`. Shortcut for [`should_match!`].
///
/// ```rust ignore
/// // This macro invocation:
/// should_none! { ... }
/// // Is equivalent to:
/// should_match! { ..., pattern = None, message = "Expected `None`, but got `Some`" }
/// ```
#[macro_export]
macro_rules! should_none {(
    $(#[$attr:meta])*
    $vis:vis fn $name:ident() -> $ret_ty:ty $body:block
) => {
    $crate::should_match! {
        $(#[$attr])*
        $vis fn $name() -> $ret_ty $body,
        pattern = ::core::option::Option::None,
        message = "Expected `None`, but got `Some`"
    }
}}
