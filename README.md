# dataparser_core

**This crate is not 100% finished, and you _may_ encounter bugs that I missed!**

A flexible and efficient binary parsing and serialization library for Rust.
This crate is intended to serve as the foundation for parsers, encoders, and binary protocol implementations, and is designed to be extensible, testable, and compatible with both synchronous and asynchronous I/O. Inspired by [nom](https://crates.io/crates/nom) <3

---

## Features

- Zero-copy binary parsing
- Declarative combinator-style parser utilities (WIP)
- Procedural macros for `#[derive(StructDeserialize, StructSerialize)]` (Accessible via the `derive` feature)
- Optional async I/O support using `tokio::io::AsyncWrite` (WIP, accessible via the `async` feature)

---

## Crate Overview

The crate is divided into the following modules:

- `parser::core` – The `DataParser` type and parser combinators
- `encoder::core` – The `DataEncoder` and `DataWriter` types for serialization
- `traits` – Contains `Encodable`, `Decodable`, and `AsyncEncodable` traits (If using the `async` feature)
- `errors` – Unified error handling for parser and encoder logic

---

## Crate Features

- `derive`: Enables the usage of `StructDeserialize/StructSerialize` for serializing/deserializing structs
- `async`: (WIP) Enables support for asynchronous readers/writers via tokio
- `crypto`: (WIP) Enables encrypting/decrypting the buffer via `AES-256-CBC` encryption. Working on adding a more dynamic approach to this

---

## Custom Encoding

You can implement custom encoders for types via the `Encodable` trait

```rust

struct MyType {
    id: u32,
    name: String
}

impl Encodable for MyType {
    fn encode_data(&self, encoder: &mut DataEncoder) -> ParseResult<()> {
        encoder.add_u32(self.id)?;
        encoder.add_string(&self.name)?;
        Ok(())
    }
}
```

---

## Example: Deserialization

**This will only work if you have the `derive` feature enabled!**

```rust
use dataparser_core::parser::core::DataParser;
use dataparser_core::parser::traits::Decodable;

#[derive(StructDeserialize, Debug)]
struct Message {
    id: u32,
    name: String,
    values: Vec<u16>,
    is_active: bool,
}

fn main() -> io::Result<()> {
    let raw_data: &[u8] = vec![/* binary input */].as_slice();
    let mut parser = DataParser::new(raw_data);
    let message = Message::from_parser(&mut parser)?;
    Ok(())
}
```

## Example: Serialization

**This will only work if you have the `derive` feature enabled!**

```rust
use dataparser_core::encoder::core::DataEncoder;
use dataparser_core::encoder::helpers::Encodable;

#[derive(StructSerialize)]
struct Payload {
    timestamp: u64,
    payload: Vec<u8>,
}

fn main() -> io::Result<()> {
    let payload = Payload { timestamp: 123456, payload: vec![1, 2, 3] };
    let mut encoder = DataEncoder::default();
    payload.encode_data(&mut encoder)?;
    let bytes = encoder.get_data()?;
    Ok(())
}
```

---

## Installation

Add this to your `Cargo.toml` file:

```toml
dataparser_core = "0.1.0"
```

Or alternatively, using `cargo`:

```shell
cargo add dataparser_core
```

---

## License

MIT
