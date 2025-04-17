use std::io::{Cursor, Read};

use crate::{impl_stream_deserializer, utils::ParseResult};

use super::core::DataReader;

/// Deserializes an `Option<T>` from a binary stream.
///
/// The format expects:
/// - A leading boolean flag indicating presence (`true` = Some, `false` = None)
/// - If `true`, the inner value `T` is parsed next.
///
/// # Example binary format
/// - `[0x01, ...]`: `Some(T)`
/// - `[0x00]`: `None`  
impl<T: StreamDecodable> StreamDecodable for Option<T> {
    fn from_stream_parser<R: Read>(parser: &mut DataReader<R>) -> ParseResult<Self> {
        let flag = parser.get_bool()?;
        if flag {
            Ok(Some(T::from_stream_parser(parser)?))
        } else {
            Ok(None)
        }
    }
}

/// Deserializes a `Vec<T>` from a binary stream, where each element is length-prefixed.
///
/// The format expects:
/// - A `u32` representing the number of elements
/// - For each element:
///     - A `u32` length prefix (in bytes)
///     - A sub-buffer of that length, which is parsed recursively with a new `DataReader`
///
/// This approach allows safe and isolated parsing of each element, useful for
/// dynamic or self-contained data units.
///
/// # Example binary format
/// ```text
/// [count][len1][item1_bytes][len2][item2_bytes]...
/// ```
///
/// This pattern ensures safe boundary checks and supports nested serialization schemes.
impl<T: StreamDecodable> StreamDecodable for Vec<T> {
    fn from_stream_parser<R: Read>(parser: &mut DataReader<R>) -> ParseResult<Self> {
        let len = parser.get_u32()?;
        let mut out = Vec::with_capacity(len as usize);
        let options = parser.options.clone();
        for _ in 0..len {
            let item_len = parser.get_u32()?;
            let item_bytes = parser.get_bytes(item_len as usize)?.to_vec();
            let mut cursor = Cursor::new(item_bytes);
            let mut temp_parser = DataReader::with_options(&mut cursor, options.clone());
            out.push(T::from_stream_parser(&mut temp_parser)?);
        }
        Ok(out)
    }
}

/// A trait for types that can be deserialized from a binary stream using a [`DataReader`].
///
/// Implementors define how to parse themselves from an input stream that implements [`std::io::Read`].
/// This trait supports compositional deserialization, making it easy to deserialize nested structures.
///
/// Typically, custom types should implement this manually or derive it using a macro.
///
/// # Type Parameters
/// - `R`: A reader type that implements [`Read`], such as `Cursor<&[u8]>` or `TcpStream`.
///
/// # Example
/// ```no_run
/// use dataparser_core::{StreamDecodable, parser::readers::sync_reader::core::DataReader};
/// let mut reader = crate::DataReader::new(std::io::Cursor::new(vec![0x2A]));
/// let value = u8::from_stream_parser(&mut reader).unwrap();
/// assert_eq!(value, 42);
/// ```
///
/// [`DataReader`]: crate::parser::readers::sync_reader::core::DataReader
pub trait StreamDecodable: Sized {
    fn from_stream_parser<R: Read>(parser: &mut DataReader<R>) -> ParseResult<Self>;
}

impl_stream_deserializer!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
