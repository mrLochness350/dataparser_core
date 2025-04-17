//! This module provides the [`DataEncoder`] type, which allows efficient serialization of
//! primitives and complex types into a contiguous byte buffer. It supports flexible encoding
//! behavior through [`EncodingOptions`], including:
//!
//! - Configurable endianness (big, little, native)
//! - Optional size prefixing for each field
//! - Support for custom `Encodable` trait implementations
//! - (Optional) AES-256 encryption integration via the `crypto` feature
//!
//! The encoder works by writing to an internal `Vec<u8>`, and is especially well-suited for
//! generating protocol packets, file formats, or network messages.
//!
//! ## Core Traits
//! - [`Encodable`]: A trait for types that can encode themselves into a `DataEncoder`
//! - [`EndianSerialize`]: Trait for primitives that can convert to endianness-aware byte formats
//!
//! ## Example
//! ```rust
//! use libparser::encoder::DataEncoder;
//! use libparser::Encodable;
//!
//! #[derive(Debug)]
//! struct Header {
//!     id: u32,
//!     flag: bool,
//! }
//!
//! impl Encodable for Header {
//!     fn encode_data(&self, encoder: &mut DataEncoder) -> Result<(), libparser::errors::DataParseError> {
//!         encoder.add_u32(self.id)?;
//!         encoder.add_bool(self.flag)?;
//!         Ok(())
//!     }
//! }
//!
//! let mut encoder = DataEncoder::new();
//! let header = Header { id: 42, flag: true };
//! header.encode_data(&mut encoder)?;
//! let bytes = encoder.get_data()?;
//! assert_eq!(bytes, vec![0x00, 0x00, 0x00, 0x2A, 0x01]);
//! ```
//!
//! ## Features
//! - `crypto`: Enables AES-256-CBC encryption for output buffers
//!
//! [`DataEncoder`]: DataEncoder
//! [`EncodingOptions`]: crate::options::EncodingOptions
//! [`Encodable`]: crate::Encodable
//! [`EndianSerialize`]: crate::utils::EndianSerialize
use crate::{
    Encodable, impl_number,
    options::EncodingOptions,
    utils::{EndianSerialize, ParseResult},
};
#[derive(Default)]
pub struct DataEncoder {
    pub(crate) buffer: Vec<u8>,
    pub(crate) options: EncodingOptions,
}

impl DataEncoder {
    /// Creates a new encoder with default options.
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            options: EncodingOptions::default(),
        }
    }

    /// Sets the encoding options for this encoder.
    ///
    /// # Note
    /// This method clones the given `EncodingOptions`. If you want to avoid cloning,
    /// consider redesigning to pass a shared reference or use an `Arc<EncodingOptions>`.
    pub fn set_options(&mut self, options: &EncodingOptions) {
        self.options = options.clone(); //TODO: figure out a way to remove the clone usage
    }

    /// Adds a raw byte slice (or any `AsRef<[u8]>`) to the buffer.
    ///
    /// If `options.prepend_data_size` is `true`, a `u32` length prefix (in the selected endianness)
    /// is prepended to the data.
    ///
    /// # Errors
    /// Returns an error if the internal logic fails (always returns `Ok(())` currently).
    pub(crate) fn add_item<T>(&mut self, data: T) -> ParseResult<()>
    where
        T: AsRef<[u8]>,
    {
        let data = data.as_ref();
        if self.options.prepend_data_size {
            let data_len = data.len() as u32;
            self.buffer
                .extend_from_slice(&data_len.to_endian_bytes(&self.options.endianness));
        }
        self.buffer.extend_from_slice(data);
        Ok(())
    }

    /// Adds a number (integer or float) using the configured endianness.
    ///
    /// Internally calls `to_endian_bytes()` and delegates to `add_item`.
    fn add_num<T: EndianSerialize>(&mut self, n: T) -> ParseResult<()> {
        let data = n.to_endian_bytes(&self.options.endianness);
        self.add_item(data)
    }

    /// Serializes a slice of encodable items.
    ///
    /// The format is:
    /// - A `u32` indicating the number of items
    /// - For each item:
    ///     - A `u32` length prefix
    ///     - The item's serialized bytes
    ///
    /// Internally creates a temporary encoder for each item to isolate its byte representation.
    ///
    /// # Note
    /// Each item is encoded in isolation with the same encoding options.
    /// This allows complex or nested data to be safely serialized.
    pub fn add_slice<T: Encodable>(&mut self, data: &[T]) -> ParseResult<()> {
        let data_len = data.len();
        self.add_u32(data_len as u32)?;
        for item in data {
            let mut temp_encoder = DataEncoder::default();
            temp_encoder.set_options(&self.options);
            item.encode_data(&mut temp_encoder)?;
            let built = temp_encoder.get_data()?;
            self.add_u32(built.len() as u32)?;
            self.add_item(built)?;
        }
        Ok(())
    }

    /// Returns a copy of the encoder’s internal buffer.
    pub fn get_data(&self) -> ParseResult<&Vec<u8>> {
        Ok(&self.buffer)
    }

    impl_number!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
}
