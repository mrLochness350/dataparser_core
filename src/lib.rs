//! # `dataparser` – A configurable binary serialization and deserialization framework
//!
//! `dataparser` provides a flexible and extensible framework for working with structured binary
//! formats. It supports both serialization and deserialization, including streaming decoding,
//! custom encoding logic, endianness control, optional encryption, and macro-derived implementations for serializing structs.
//!
//! ## Features
//! - Custom binary format parsing and writing
//! - Configurable options (`strict_encoding`, `trim_nulls`, `endianness`, etc.)
//! - Optional length-prefixed fields
//! - Zero-copy or stream-based decoding
//! - AES-256-CBC encryption (behind the `crypto` feature)
//! - `#[derive(StructSerialize, StructDeserialize)]` support (behind the `derive` feature)
//!
//! ## Example: Encode and Decode a Struct
//! ```rust
//! use libparser::{DataEncoder, DataParser, Encodable, Decodable, ParseResult};
//!
//! #[derive(Debug)]
//! struct Header {
//!     id: u32,
//!     flag: bool,
//! }
//!
//! impl Encodable for Header {
//!     fn encode_data(&self, encoder: &mut DataEncoder) -> ParseResult<()> {
//!         encoder.add_u32(self.id)?;
//!         encoder.add_bool(self.flag)?;
//!         Ok(())
//!     }
//! }
//!
//! impl Decodable for Header {
//!     fn decode_data(parser: &mut DataParser) -> ParseResult<Self> {
//!         let id = parser.get_u32()?;
//!         let flag = parser.get_bool()?;
//!         Ok(Header { id, flag })
//!     }
//! }
//! ```
//!
//! ## Features
//! - `derive`: Enables `#[derive(StructSerialize, StructDeserialize)]`
//! - `crypto`: Enables AES-256 encryption with PKCS7 padding
//! - `async` : Enables async stream reader/writer support via the tokio crate
//!
//! ## Modules
//! - [`encoder`]: Binary serialization
//! - [`parser`]: Binary deserialization
//! - [`options`]: Runtime configuration for encoding/parsing
//! - [`crypto`]: AES encryption support (optional)
//! - [`utils`]: Shared helpers, endian utilities
//!
//! ## Trait Overview
//! - [`Encodable`] — custom serialization
//! - [`Decodable`] — custom deserialization
//! - [`StreamDecodable`] — streaming-compatible deserialization
//!
//! [`encoder`]: crate::encoder
//! [`parser`]: crate::parser
//! [`options`]: crate::options
//! [`crypto`]: crate::crypto
//! [`utils`]: crate::utils
//! [`Encodable`]: crate::Encodable
//! [`Decodable`]: crate::Decodable
//! [`StreamDecodable`]: crate::StreamDecodable
// Core modules
pub mod encoder;
pub mod errors;
pub mod options;
pub mod parser;
pub mod utils;

#[cfg(feature = "crypto")]
pub mod crypto;

#[cfg(feature = "derive")]
pub use dataparser_derive::{StructDeserialize, StructSerialize};

pub use encoder::helpers::Encodable;
pub use parser::helpers::Decodable;
pub use parser::readers::sync_reader::helpers::StreamDecodable;

pub use encoder::core::DataEncoder;
pub use errors::DataParseError;
pub use options::{EncodingOptions, ParseOptions};
pub use parser::core::DataParser;
pub use utils::Endianness;
pub use utils::ParseResult;
