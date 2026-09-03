# teable-rs

Rust SDK for the [Teable](https://teable.ai) API.

## Status

This crate is under active development and still in an early stage. The current API supports async requests for spaces, bases, tables, fields, and records.

## Requirements

- Rust 1.85 or newer
- A Teable instance
- A Teable personal access token

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
teable = 0.0.1
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The default `async` feature is enabled automatically. The crate also provides a `blocking` feature for blocking reqwest support. (Still in early development)

## Client Setup

```rust
use teable::client::TeableClient;

let client = TeableClient::builder()
    .base_url("https://app.teable.ai/api/")?
    .token(std::env::var("TEABLE_TOKEN")?)
    .build()?;
```

The default base URL is `https://app.teable.ai/api/`, so `.base_url(...)` can be omitted when using Teable Cloud.

For a self-hosted instance, provide its API URL:

```rust
let client = TeableClient::builder()
    .base_url("https://teable.example.com/api/")?
    .token(token)
    .build()?;
```

## Complete Example

The repository includes an end-to-end example at [`examples/base_example/src/main.rs`](examples/base_example/src/main.rs). It creates a base, table, fields, and records, then lists the created resources.

Set the required environment variable before running it:

```sh
export TEABLE_TOKEN="your-personal-access-token"
export TEABLE_SPACE_ID="spc..."
cargo run --example base_example
```

The example creates test data. Run it only against a space where this is acceptable.

## Development

Format the project:

```sh
cargo fmt
```

Check compilation:

```sh
cargo check
```

Run tests:

```sh
cargo test
```

A repository pre-commit hook runs `cargo fmt --all -- --check` when `.githooks` is configured as the Git hooks path.

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).
