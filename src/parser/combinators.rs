use crate::errors::DataParseError;
use crate::parser::core::DataParser;
use crate::utils::ParseResult;

/// Matches and consumes an exact tag (byte sequence) from the input buffer.
///
/// # Arguments
/// - `expected`: The byte slice to match at the current cursor position.
///
/// # Returns
/// Returns a slice of the matched tag if successful, or a `DataParseError` if the tag doesn't match.
///
/// # Example
/// ```
/// let mut binding = vec![0xDE, 0xAD];
/// let mut parser = dataparser_core::parser::DataParser::new(&mut binding);
/// let tag_parser = dataparser_core::parser::combinators::delim_extract(&[0xDE, 0xAD]);
/// assert_eq!(tag_parser(&mut parser).unwrap(), &[0xDE, 0xAD]);
/// ```
pub fn delim_extract(
    expected: &'static [u8],
) -> impl for<'a> Fn(&'a mut DataParser<'a>) -> ParseResult<&'a [u8]> {
    move |parser: &mut DataParser| {
        let actual = parser.take(expected.len())?;
        if actual == expected {
            Ok(actual)
        } else {
            Err(DataParseError::Custom {
                e: format!("Tag mismatch: expected {:?}, got {:?}", expected, actual),
            })
        }
    }
}

/// Transforms the result of a parser using a mapping function.
///
/// # Arguments
/// - `parser`: A parser function that returns `A`.
/// - `f`: A mapping function from `A` to `B`.
///
/// # Returns
/// A new parser that applies the transformation after parsing.
pub fn map<A, B, F, G>(parser: F, f: G) -> impl Fn(&mut DataParser) -> ParseResult<B>
where
    F: Fn(&mut DataParser) -> ParseResult<A>,
    G: Fn(A) -> B,
{
    move |p: &mut DataParser| parser(p).map(&f)
}

/// Parses a value surrounded by specific start and end delimiters.
///
/// # Arguments
/// - `parser`: The parser function for the content between delimiters.
/// - `delim_start`: Expected start delimiter byte.
/// - `delim_end`: Expected end delimiter byte.
///
/// # Returns
/// A parser function that validates and consumes the delimiters around the value.
///
/// # Errors
/// Returns a `DataParseError` if delimiters do not match.
pub fn parse_between<P, T>(
    parser: P,
    delim_start: u8,
    delim_end: u8,
) -> impl Fn(&mut DataParser) -> ParseResult<T>
where
    P: Fn(&mut DataParser) -> ParseResult<T>,
{
    move |input: &mut DataParser| {
        let start = input.get_byte()?;
        if start != delim_start {
            return Err(DataParseError::Custom {
                e: format!(
                    "Expected start delimiter {:?}, found {:?}",
                    delim_start, start
                ),
            });
        }
        let value = parser(input)?;
        let end = input.get_byte()?;
        if end != delim_end {
            return Err(DataParseError::Custom {
                e: format!("Expected end delimiter {:?}, found {:?}", delim_end, end),
            });
        }

        Ok(value)
    }
}

impl DataParser<'_> {
    /// Parses a value from a length-prefixed sub-buffer.
    ///
    /// Reads a `u32` length, then creates a sub-parser scoped to the slice of that length.
    /// Passes the sub-parser to the provided closure.
    ///
    /// Useful for safely parsing encapsulated structures like compressed, encrypted,
    /// or nested payloads.
    pub(crate) fn parse_with_length_prefix<T, F>(&mut self, f: F) -> ParseResult<T>
    where
        F: FnOnce(&mut DataParser) -> ParseResult<T>,
    {
        let options = self.options.clone();
        let len = self.__get_u32()?;
        let mut sub_buffer = self.take(len as usize)?.to_vec();
        let mut sub_parser = DataParser::with_options(&mut sub_buffer, options);
        f(&mut sub_parser)
    }

    /// Applies a parser function directly on this `DataParser`.
    ///
    /// This is a convenience method for calling function-style parsers.
    ///
    /// # Example
    /// ```
    /// let result = parser.parse_with(my_parser_fn)?;
    /// ```
    pub fn parse_with<T, P>(&mut self, parser: P) -> ParseResult<T>
    where
        P: FnOnce(&mut DataParser) -> ParseResult<T>,
    {
        parser(self)
    }

    /// Parses bytes from the stream until a terminating condition is met.
    ///
    /// # Arguments
    /// - `terminator`: A predicate function that returns `true` for the terminating byte.
    /// - `max_len`: An optional limit to prevent infinite parsing.
    ///
    /// # Returns
    /// A `Vec<u8>` of collected bytes (not including the terminator).
    ///
    /// # Errors
    /// Returns an error if the maximum length is exceeded or reading fails.
    pub fn parse_until<F>(&mut self, terminator: F, max_len: Option<usize>) -> ParseResult<Vec<u8>>
    where
        F: Fn(u8) -> bool,
    {
        let mut collected = Vec::new();
        while self.remaining() > 0 {
            if let Some(limit) = max_len {
                if collected.len() >= limit {
                    return Err(DataParseError::Custom {
                        e: "parse_until exceeded max_len".into(),
                    });
                }
            }

            let byte = self.get_byte()?;
            if terminator(byte) {
                break;
            }
            collected.push(byte);
        }
        Ok(collected)
    }
    
    
}

