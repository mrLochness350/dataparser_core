use std::io::Read;

use crate::{impl_get_with_prefix, parser::ParseOptions, utils::ParseResult};

/// A streaming binary reader that wraps any `Read` implementation (e.g. file, socket).
///
/// `DataReader` provides methods to read structured binary data from a stream-like source,
/// such as files or network sockets. It supports configurable parsing options and
/// automatically handles byte alignment and EOF detection.
///
/// # Examples
///
/// ```rust
/// use std::io::Cursor;
/// use dataparser_core::parser::core::DataReader;
/// let bytes = Cursor::new(vec![0x01, 0x00]);
/// let mut reader = DataReader::new(bytes);
/// let value = reader.get_byte().unwrap();
/// assert_eq!(value, 0x01);
/// ```
#[derive(Default)]
pub struct DataReader<R: Read> {
    pub(crate) reader: R,
    pub(crate) options: ParseOptions,
}

impl<R> DataReader<R>
where
    R: Read,
{
    /// Creates a new `DataReader` with default parsing options.
    ///
    /// # Arguments
    /// * `reader` - A stream or source implementing `std::io::Read`.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            options: ParseOptions::default(),
        }
    }

    /// Creates a new `DataReader` with custom parsing options.
    ///
    /// # Arguments
    /// * `reader` - A stream or source implementing `Read`.
    /// * `options` - Parsing options such as verbose error handling.
    pub fn with_options(reader: R, options: ParseOptions) -> Self {
        Self { reader, options }
    }

    /// Updates the parsing options used by this reader.
    ///
    /// Useful for toggling verbose error messages or other runtime behavior.
    pub fn set_options(&mut self, options: ParseOptions) {
        self.options = options;
    }

    /// Reads exactly `N` bytes into a fixed-size array.
    ///
    /// # Errors
    /// Returns an error if the stream ends before `N` bytes are read.
    pub(crate) fn read_array<const N: usize>(&mut self) -> ParseResult<[u8; N]> {
        let mut buf = [0u8; N];
        self.reader.read_exact(&mut buf)?;

        Ok(buf)
    }

    /// Reads `n` bytes from the stream and returns them in a `Vec<u8>`.
    ///
    /// # Errors
    /// Returns an error if not enough bytes are available.
    pub fn get_bytes(&mut self, n: usize) -> ParseResult<Vec<u8>> {
        let mut buf = vec![0u8; n];
        self.reader.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Reads a single byte from the stream.
    ///
    /// # Errors
    /// Returns an error if the stream is empty or unreadable.
    pub fn get_byte(&mut self) -> ParseResult<u8> {
        let byte = self.read_array::<1>()?;
        Ok(byte[0])
    }

    /// Reads a single byte and interprets it as a boolean value.
    ///
    /// Returns `true` if the byte is non-zero, `false` otherwise.
    ///
    /// # Errors
    /// Returns an error if the stream is empty.
    pub fn get_bool(&mut self) -> ParseResult<bool> {
        Ok(self.get_byte()? != 0)
    }

    impl_get_with_prefix!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
}
