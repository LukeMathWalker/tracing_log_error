//! # `tracing_log_error`
//!
//! A utility crate to capture an error, and all its key error properties,
//! in a `tracing` event.
//!
//! ```rust
//! use tracing_log_error::log_error;
//!
//! let e = std::io::Error::new(std::io::ErrorKind::Other, "My error");
//! log_error!(e, "The connection was dropped");
//! ```
//!
//! The `log_error!` invocation captures:
//!
//! - The `Display` representation of the error, in the `error.message` field.
//! - The `Debug` representation of the error, in the `error.details` field.
//! - The chain of error sources, in the `error.source_chain` field.
//!
//! Using raw `tracing`, the equivalent would be:
//!
//! ```rust
//! use tracing::{event, Level};
//! use tracing_log_error::fields;
//!
//! let e = std::io::Error::new(std::io::ErrorKind::Other, "My error");
//! event!(
//!     Level::ERROR,
//!     error.message = fields::error_message(&e),
//!     error.details = fields::error_details(&e),
//!     error.source_chain = fields::error_source_chain(&e),
//!     "The connection was dropped"
//! );
//! ```
//!
//! ## Installation
//!
//! To use `log_error!`, add both `tracing` and `tracing_log_error` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tracing = "0.1"
//! tracing-log-error = "0.1"
//! ```
//!
//! ## Some errors don't implement the `Error` trait
//!
//! Some common error reporting types, like `anyhow::Error` or `eyre::Report`
//! or `Box<dyn std::error::Error>`, don't implement the `Error` trait.
//! If you try to use `log_error!` with them directly, you'll get a compiler error.
//!
//! Good news: you can still use `log_error!` with them!
//! They dereference to a type that implements the `Error` trait, so you can
//! use `*` to dereference them when passing them to `log_error!`:
//!
//! ```rust
//! use tracing_log_error::log_error;
//! use anyhow::anyhow;
//!
//! let e = anyhow!("Hey");
//! // Notice the `*` 👇
//! log_error!(*e, "An error occurred");
//! ```
//!
//! ## Advanced usage
//!
//! Check out [`log_error!`](crate::log_error)'s documentation for more examples and details.
//! You can customize the log level, add custom fields, and more.
pub mod fields;

/// A macro that desugars to an invocation of `tracing::event!` with all
/// error-related fields (the ones in [the `fields` module](crate::fields))
/// pre-populated.
///
/// # Basic invocation
///
/// ```rust
/// use tracing::{event, Level};
/// use tracing_log_error::{fields, log_error};
///
/// let e = std::io::Error::new(std::io::ErrorKind::Other, "My error");
/// // This 👇
/// log_error!(e, "The connection was dropped");
/// // is equivalent to this 👇
/// event!(
///     Level::ERROR,
///     error.message = fields::error_message(&e),
///     error.details = fields::error_details(&e),
///     error.source_chain = fields::error_source_chain(&e),
///     "The connection was dropped"
/// );
/// ```
///
/// # Custom fields
///
/// You can add custom fields to the log event by prepending them ahead of the
/// error:
///
/// ```rust
/// use tracing::{event, Level};
/// use tracing_log_error::{fields, log_error};
///
/// let e = std::io::Error::new(std::io::ErrorKind::Other, "My error");
/// // This 👇
/// log_error!(e, custom_field = "value", "The connection was dropped");
/// // is equivalent to this 👇
/// event!(
///     Level::ERROR,
///     custom_field = "value",
///     error.message = fields::error_message(&e),
///     error.details = fields::error_details(&e),
///     error.source_chain = fields::error_source_chain(&e),
///     "The connection was dropped"
/// );
/// ```
///
/// # Custom level
///
/// It may be useful, in some cases, to log an error at a level other than
/// `ERROR`. You can do this by specifying the level as a named argument:
///
/// ```rust
/// use tracing::{event, Level};
/// use tracing_log_error::{fields, log_error};
///
/// let e = std::io::Error::new(std::io::ErrorKind::Other, "My error");
/// // This 👇
/// log_error!(e, level: Level::WARN, "The connection was dropped");
/// // is equivalent to this 👇
/// event!(
///     Level::WARN,
///     error.message = fields::error_message(&e),
///     error.details = fields::error_details(&e),
///     error.source_chain = fields::error_source_chain(&e),
///     "The connection was dropped"
/// );
/// ```
///
#[macro_export]
macro_rules! log_error {
    // ...
    ($err:expr, level: $lvl:expr, { $($fields:tt)* }) => (
        ::tracing::event!(
            $lvl,
            {{ $crate::fields::ERROR_MESSAGE }} = $crate::fields::error_message(&$err),
            {{ $crate::fields::ERROR_DETAILS }} = $crate::fields::error_details(&$err),
            {{ $crate::fields::ERROR_SOURCE_CHAIN }} = $crate::fields::error_source_chain(&$err),
            $($fields)*
        )
    );
    ($err:expr, level: $lvl:expr, { $($fields:tt)* }, $($arg:tt)+) => (
        ::tracing::event!(
            $lvl,
            {{ $crate::fields::ERROR_MESSAGE }} = $crate::fields::error_message(&$err),
            {{ $crate::fields::ERROR_DETAILS }} = $crate::fields::error_details(&$err),
            {{ $crate::fields::ERROR_SOURCE_CHAIN }} = $crate::fields::error_source_chain(&$err),
            { $($fields)* },
            $($arg)+
        )
    );
    ($err:expr, level: $lvl:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::log_error!(
            $err,
            level: $lvl,
            { $($k).+ = $($field)*}
        )
    );
    ($err:expr, level: $lvl:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::log_error!(
            $err,
            level: $lvl,
            { ?$($k).+, $($field)*}
        )
    );
    ($err:expr, level: $lvl:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::log_error!(
            $err,
            level: $lvl,
            { %$($k).+, $($field)*}
        )
    );
    ($err:expr, level: $lvl:expr, ?$($k:ident).+) => (
        $crate::log_error!($err, level: $lvl, ?$($k).+,)
    );
    ($err:expr, level: $lvl:expr, %$($k:ident).+) => (
        $crate::log_error!($err, level: $lvl, %$($k).+,)
    );
    ($err:expr, level: $lvl:expr, $($k:ident).+) => (
        $crate::log_error!($err, level: $lvl, $($k).+,)
    );
    ($err:expr, level: $lvl:expr, $($arg:tt)*) => (
        $crate::log_error!($err, level: $lvl, { $($arg)* })
    );
    ($err:expr, level: $lvl:expr) => (
        $crate::log_error!($err, level: $lvl, { })
    );
    ($err:expr, { $($fields:tt)* }, $($arg:tt)+) => (
        $crate::log_error!($err, level: ::tracing::Level::ERROR, { $($fields)* }, $($arg)+)
    );
    ($err:expr, $($k:ident).+ = $($field:tt)*) => (
        $crate::log_error!(
            $err,
            level: ::tracing::Level::ERROR,
            $($k).+ = $($field)*
        )
    );
    ($err:expr, ?$($k:ident).+, $($field:tt)*) => (
        $crate::log_error!(
            $err,
            level: ::tracing::Level::ERROR,
            ?$($k).+,
            $($field)*
        )
    );
    ($err:expr, %$($k:ident).+, $($field:tt)*) => (
        $crate::log_error!(
            $err,
            level: ::tracing::Level::ERROR,
            $($field)*
        )
    );
    ($err:expr, ?$($k:ident).+) => (
        $crate::log_error!($err, level: ::tracing::Level::ERROR, ?$($k).+)
    );
    ($err:expr, %$($k:ident).+) => (
        $crate::log_error!($lvl, level: ::tracing::Level::ERROR, %$($k).+)
    );
    ($err:expr, $($k:ident).+) => (
        $crate::log_error!($err, level: ::tracing::Level::ERROR, $($k).+)
    );
    ($err:expr, $($arg:tt)*) => (
        $crate::log_error!($err, level: ::tracing::Level::ERROR, $($arg)*)
    );
    ($err:expr) => (
        $crate::log_error!($err,)
    );
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::log_error;

    #[test]
    fn my_test() {
        let e = std::io::Error::new(std::io::ErrorKind::Other, "My error");
        // Most common usage
        log_error!(e, "Yay");
        // Passing a reference to the error rather than an owned error
        log_error!(&e, "Yay");
        // Just the error
        log_error!(e);
        // An error report that doesn't implement the `Error` trait, but
        // dereferences to a type that does
        log_error!(*anyhow::anyhow!("Hey"));
        let y: Box<dyn std::error::Error> =
            Box::new(std::io::Error::new(std::io::ErrorKind::Other, "My error"));
        log_error!(*y);
        // Formatting in the message
        let a = "friend";
        log_error!(e, "Here I am, {}", a);
        log_error!(e, "Here I am, {} {}", "my", "friend");
        // Custom level
        log_error!(e, level: tracing::Level::WARN, "Yay");
        // Custom level, no message
        log_error!(e, level: tracing::Level::WARN);
        // Custom level, formatted message
        log_error!(e, level: tracing::Level::WARN, "Here I am, {}", a);
        // Custom fields
        log_error!(e, custom_field = "value", "Yay");
        // Custom fields with a message
        log_error!(e, custom_field1 = "value1", custom_field2 = "value2", "Yay");
        // Custom fields with a formatted message
        log_error!(
            e,
            custom_field1 = "value1",
            custom_field2 = "value2",
            "Here I am, {}",
            a
        );
        // Custom fields with a custom level
        log_error!(e, level: tracing::Level::INFO, custom_field1 = "value1", "Yay");
        // Custom fields with a custom level and formatted message
        log_error!(e, level: tracing::Level::INFO, custom_field1 = "value1", custom_field2 = "value2", "Here I am, {} {}", "my", "friend");
        // Using % and ? to log fields using their Display and Debug representations
        let a = PathBuf::from("a path");
        let b = "A string".to_string();
        log_error!(e, custom_field = ?a, custom_field2 = %b, ?a, %b, "Hello");
        // Using {{ }} to log fields using a constant as their name
        const FIELD: &str = "field";
        log_error!(
            e,
            {
                {
                    FIELD
                }
            } = "value",
            "Yay"
        );
    }
}
