<!-- cargo-rdme start -->

# `tracing_log_error`

A utility crate to capture an error, and all its key error properties,
in a `tracing` event.

```rust
use tracing_log_error::log_error;

let e = std::io::Error::new(std::io::ErrorKind::Other, "My error");
log_error!(e, "The connection was dropped");
```

The `log_error!` invocation captures:

- The `Display` representation of the error, in the `error.message` field.
- The `Debug` representation of the error, in the `error.details` field.
- The chain of error sources, in the `error.source_chain` field.

Using raw `tracing`, the equivalent would be:

```rust
use tracing::{event, Level};
use tracing_log_error::fields;

let e = std::io::Error::new(std::io::ErrorKind::Other, "My error");
event!(
    Level::ERROR,
    error.message = fields::error_message(&e),
    error.details = fields::error_details(&e),
    error.source_chain = fields::error_source_chain(&e),
    "The connection was dropped"
);
```

## Installation

To use `log_error!`, add both `tracing` and `tracing_log_error` to your `Cargo.toml`:

```toml
[dependencies]
tracing = "0.1"
tracing-log-error = "0.1"
```

## Some errors don't implement the `Error` trait

Some common error reporting types, like `anyhow::Error` or `eyre::Report`
or `Box<dyn std::error::Error>`, don't implement the `Error` trait.
If you try to use `log_error!` with them directly, you'll get a compiler error.

Good news: you can still use `log_error!` with them!
They dereference to a type that implements the `Error` trait, so you can
use `*` to dereference them when passing them to `log_error!`:

```rust
use tracing_log_error::log_error;
use anyhow::anyhow;

let e = anyhow!("Hey");
// Notice the `*` 👇
log_error!(*e, "An error occurred");
```

## Advanced usage

Check out [`log_error!`](https://docs.rs/tracing_log_error/latest/tracing_log_error/macro.log_error.html)'s documentation for more examples and details.
You can customize the log level, add custom fields, and more.

<!-- cargo-rdme end -->
