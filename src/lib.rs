#![doc = include_str!("../README.md")]
#![no_std]

/// Wraps a function that takes nothing, panicking if the result does not match the expected pattern.
///
/// See the [crate-level documentation](crate) for more information.
#[macro_export]
macro_rules! should_match {
(
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
        let message = concat!("Expected to match `", stringify!($pattern), "`");
        // Shadow the message if it was provided
        $(let message = $message;)?
        assert!(is_match, "{message}");
    }
};
}
