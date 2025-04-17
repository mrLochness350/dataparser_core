use crate::parser::buffer::Buffer;
use crate::{
    errors::DataParseError, impl_get_with_prefix, options::ParseOptions, utils::ParseResult,
};

/// A configurable binary data parser that reads structured data from a byte buffer.
///
/// `DataParser` provides methods for consuming and interpreting byte sequences,
/// with support for verbose error reporting and parsing options.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use dataparser_core::parser::DataParser;
/// let data = &[0x01, 0x00, 0x02];
/// let mut parser = DataParser::new(data);
/// let first_byte = parser.get_byte().unwrap();
/// assert_eq!(first_byte, 0x01);
/// ```
#[derive(Default)]
pub struct DataParser<'a> {
    /// The input buffer to parse from.
    pub(crate) buffer: Buffer<'a>,

    /// The current position within the buffer.
    pub(crate) cursor: usize,

    /// Parser configuration options.
    pub(crate) options: ParseOptions,
}
impl<'a> DataParser<'a> {
    /// Creates a new `DataParser` from the given buffer and parsing options.
    ///
    /// # Arguments
    /// * `buffer` - A byte buffer to read from.
    /// * `options` - Parser configuration options (e.g., verbose errors).
    pub fn with_options<B: Into<Buffer<'a>>>(buffer: B, options: ParseOptions) -> Self {
        Self {
            buffer: buffer.into(),
            cursor: 0,
            options,
        }
    }

    /// Sets the parser's options after creation.
    ///
    /// This allows modifying the parser's behavior (e.g., error verbosity) at runtime.
    pub fn set_options(&mut self, options: ParseOptions) {
        self.options = options;
    }

    /// Creates a new `DataParser` with default parsing options.
    ///
    /// # Arguments
    /// * `buffer` - A byte buffer to read from.
    pub fn new<B: Into<Buffer<'a>>>(buffer: B) -> Self {
        Self {
            buffer: buffer.into(),
            cursor: 0,
            options: ParseOptions::default(),
        }
    }

    /// Returns the number of bytes remaining in the buffer.
    pub(crate) fn remaining(&self) -> usize {
        self.buffer.len().saturating_sub(self.cursor)
    }

    /// Consumes and returns the next `n` bytes from the buffer.
    ///
    /// Advances the internal cursor.
    ///
    /// # Errors
    /// Returns an error if there are not enough bytes remaining.
    pub(crate) fn take(&mut self, n: usize) -> ParseResult<&[u8]> {
        if self.remaining() < n {
            return if self.options.verbose_errors {
                let remaining_bytes = &self.buffer[self.cursor..];
                eprintln!(
                    "Failed to take {} bytes at offset {} — only {} bytes left: {:?}",
                    n,
                    self.cursor,
                    self.remaining(),
                    remaining_bytes
                );
                Err(DataParseError::Custom {
                    e: format!(
                        "Not enough bytes at offset {} (needed {}, have {})",
                        self.cursor,
                        n,
                        self.remaining()
                    ),
                })
            } else {
                Err(DataParseError::UnexpectedEOF)
            };
        }

        let start = self.cursor;
        let end = self.cursor + n;
        self.cursor = end;
        Ok(&self.buffer[start..end])
    }

    /// Reads exactly `N` bytes into a fixed-size array.
    ///
    /// # Errors
    /// Returns an error if there are not enough bytes.
    pub(crate) fn read_array<const N: usize>(&mut self) -> ParseResult<[u8; N]> {
        let slice = self.take(N)?;
        let mut array = [0u8; N];
        array.copy_from_slice(slice);
        Ok(array)
    }

    /// Returns the total length of the underlying buffer.
    pub fn current_len(&self) -> usize {
        self.buffer.len()
    }

    /// Peeks at the next `n` bytes in the buffer without advancing the cursor.
    ///
    /// # Errors
    /// Returns an error if not enough bytes are available.
    pub fn peek(&self, n: usize) -> ParseResult<&[u8]> {
        if self.remaining() < n {
            return Err(DataParseError::UnexpectedEOF);
        }
        Ok(&self.buffer[..n])
    }

    /// Reads the next `byte_len` bytes and returns them as a `Vec<u8>`.
    ///
    /// Advances the internal cursor.
    ///
    /// # Errors
    /// Returns an error if not enough bytes are available.
    pub fn get_bytes(&mut self, byte_len: usize) -> ParseResult<Vec<u8>> {
        if self.remaining() < byte_len {
            return Err(DataParseError::UnexpectedEOF);
        }
        let buf = self.take(byte_len)?.to_vec();
        Ok(buf)
    }

    /// Reads a single byte from the buffer.
    ///
    /// # Errors
    /// Returns an error if the buffer is empty.
    pub fn get_byte(&mut self) -> ParseResult<u8> {
        Ok(*self.take(1)?[0..1].first().unwrap())
    }

    /// Reads a single byte and interprets it as a boolean.
    ///
    /// Returns `true` if the byte is non-zero.
    ///
    /// # Errors
    /// Returns an error if there are not enough bytes.
    pub fn get_bool(&mut self) -> ParseResult<bool> {
        Ok(self.get_byte()? != 0)
    }

    impl_get_with_prefix!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
}
