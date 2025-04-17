use crate::{errors::DataParseError, utils::ParseResult};

use super::core::DataParser;

impl DataParser<'_> {
    fn _get_string(&mut self, str_len: usize, utf16: bool) -> ParseResult<String> {
        let strict = self.options.strict_encoding;
        let trim_nulls = self.options.trim_null_strings;
        let bytes = self.take(str_len)?;
        if utf16 {
            if bytes.len() % 2 != 0 {
                return Err(DataParseError::InvalidConversion {
                    e: "UTF-16 input length must be even".into(),
                });
            }

            let utf_16_vec: Vec<u16> = bytes
                .chunks_exact(2)
                .map(|pair| u16::from_ne_bytes([pair[0], pair[1]]))
                .collect();

            let mut string = if strict {
                String::from_utf16(&utf_16_vec)
                    .map_err(|e| DataParseError::InvalidConversion { e: e.to_string() })?
            } else {
                String::from_utf16_lossy(&utf_16_vec)
            };

            if trim_nulls {
                string = string.trim_end_matches('\0').to_string();
            }

            Ok(string)
        } else {
            let mut string = if strict {
                std::str::from_utf8(bytes)
                    .map_err(|e| DataParseError::InvalidConversion { e: e.to_string() })?
                    .to_string()
            } else {
                String::from_utf8_lossy(bytes).into()
            };

            if trim_nulls {
                string = string.trim_end_matches('\0').to_string();
            }

            Ok(string)
        }
    }

    fn _get_string_raw(&mut self, utf16: bool) -> ParseResult<String> {
        if utf16 {
            let mut utf16_buf = Vec::new();
            while self.remaining() >= 2 {
                let bytes = self.take(2)?;
                let val = u16::from_ne_bytes([bytes[0], bytes[1]]);
                if val == 0 {
                    break;
                }
                utf16_buf.push(val);
            }
            let mut s = String::from_utf16_lossy(&utf16_buf);
            if self.options.trim_null_strings {
                s = s.trim_end_matches('\0').to_string();
            }
            Ok(s)
        } else {
            let mut string_buf = Vec::new();
            while self.remaining() > 0 {
                let byte = self.get_byte()?;
                if byte == 0 {
                    break;
                }
                string_buf.push(byte);
            }

            let result = if self.options.strict_encoding {
                String::from_utf8(string_buf)
                    .map_err(|e| DataParseError::InvalidConversion { e: e.to_string() })?
            } else {
                String::from_utf8_lossy(&string_buf).to_string()
            };

            Ok(result)
        }
    }

    /// Parses a length-prefixed string from the input stream.
    ///
    /// This method expects a `u32` length prefix followed by a UTF-8 or UTF-16 encoded string.
    /// The parsing behavior can be adjusted via the following options:
    ///
    /// - [`strict_encoding`]: If `true`, parsing will return an error on invalid encoding.
    /// - [`trim_null_strings`]: If `true`, trailing null bytes (`\0`) will be removed.
    ///
    /// # Parameters
    /// - `utf16`: If `true`, the string is parsed as UTF-16. Otherwise, UTF-8 is used.
    ///
    /// # Returns
    /// A `String` if parsing succeeds, or a [`DataParseError`] on failure.
    ///
    /// # Format
    /// ```
    /// [length: u32][string_bytes...]
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// - The length prefix is invalid
    /// - The encoding is invalid (under `strict_encoding`)
    ///
    /// # Example
    /// ```rust
    /// let mut parser = DataParser::new(Cursor::new(vec![0x05, 0x00, 0x00, 0x00, b'H', b'e', b'l', b'l', b'o']));
    /// let s = parser.get_string(false).unwrap();
    /// assert_eq!(s, "Hello");
    /// ```
    ///
    /// [`strict_encoding`]: crate::options::ParseOptions
    /// [`trim_null_strings`]: crate::options::ParseOptions
    pub fn get_string(&mut self, utf16: bool) -> ParseResult<String> {
        let str_len = self.get_u32()?;
        // Strings also have to prepend the size to the data
        self._get_string(str_len as usize, utf16)
    }
}
