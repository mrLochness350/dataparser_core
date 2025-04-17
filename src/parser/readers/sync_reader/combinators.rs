use super::core::DataReader;
use crate::utils::ParseResult;
use std::io::{Cursor, Read};

impl<R: Read> DataReader<R> {
    /// Parses a value using a length-prefixed sub-buffer from the current stream.
    ///
    /// This method reads a `u32` length prefix, then extracts that many bytes from the stream,
    /// and creates a new [`DataReader`] scoped to only that sub-buffer. The provided closure `f`
    /// receives this sub-reader to deserialize the contained value.
    ///
    /// This pattern is commonly used to encapsulate isolated structures or elements
    /// that are prefixed with their own byte size (e.g., nested structs, vectors, encrypted segments).
    ///
    /// # Type Parameters
    /// - `T`: The return type of the parsing function.
    /// - `F`: A closure that accepts a sub-`DataReader` and returns a parsed value.
    ///
    /// # Parameters
    /// - `f`: A closure that takes a `&mut DataReader<Cursor<Vec<u8>>>` and returns a parsed result.
    ///
    /// # Returns
    /// A `ParseResult<T>` containing the parsed value or an error if reading or parsing fails.
    ///
    /// # Example
    /// ```rust
    /// use dataparser_core::parser::readers::sync_reader::core::DataReader;
    /// let mut reader = DataReader::new(std::io::Cursor::new(input_bytes));
    /// let my_struct = reader.parse_with_length_prefix(|sub| MyStruct::from_stream_parser(sub))?;
    /// ```
    ///
    /// This example assumes that `input_bytes` contains a `u32` length followed by a serialized `MyStruct`.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The length prefix is invalid or too large
    /// - The underlying stream fails to provide the required number of bytes
    /// - The inner parsing logic fails
    ///
    /// [`DataReader`]: crate::parser::core::DataReader
    pub(crate) fn parse_with_length_prefix<T, F>(&mut self, f: F) -> ParseResult<T>
    where
        F: FnOnce(&mut DataReader<Cursor<Vec<u8>>>) -> ParseResult<T>,
    {
        let options = self.options.clone();
        let len = self.__get_u32()?;
        let buf = self.get_bytes(len as usize)?;
        let cursor = Cursor::new(buf);
        let mut sub_parser = DataReader::with_options(cursor, options);
        f(&mut sub_parser)
    }
}
