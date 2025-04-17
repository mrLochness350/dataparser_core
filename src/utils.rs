use crate::errors::DataParseError;

/// A convenient type alias for parse operations throughout the crate.
///
/// This standardizes all fallible operations in encoders and parsers to return
/// a unified error type, [`DataParseError`].
///
/// # Example
/// ```
/// fn parse_u32(input: &[u8]) -> ParseResult<u32> { ... }
/// ```
///
/// [`DataParseError`]: crate::errors::DataParseError
pub type ParseResult<T> = Result<T, DataParseError>;

/// Represents the byte order used for encoding or decoding multi-byte values.
///
/// This enum allows dynamic selection of endianness at runtime and is used by
/// both [`DataEncoder`] and [`DataParser`] when reading/writing numeric fields.
///
/// - `BigEndian`: Most significant byte first (default)
/// - `LittleEndian`: Least significant byte first
/// - `NativeEndian`: Matches the endianness of the current machine
///
/// # Example
/// ```
/// let endianness = Endianness::LittleEndian;
/// let bytes = 42u32.to_endian_bytes(&endianness);
/// ```
///
/// [`DataEncoder`]: crate::encoder::core::DataEncoder
/// [`DataParser`]: crate::parser::core::DataParser
#[derive(Default, Clone, Debug)]
pub enum Endianness {
    /// Big-endian byte order (network order).
    #[default]
    BigEndian,

    /// Little-endian byte order (Intel/x86).
    LittleEndian,

    /// Native system endianness (`cfg(target_endian)`).
    NativeEndian,
}

/// A trait for converting numeric types to their byte representation with a given endianness.
///
/// Used by encoding APIs to abstract away byte order concerns.
///
/// # Example
/// ```
/// use dataparser_core::Endianness;
/// let n: u32 = 0x12345678;
/// let bytes = n.to_endian_bytes(&Endianness::LittleEndian);
/// ```
pub trait EndianSerialize {
    fn to_endian_bytes(self, endianness: &Endianness) -> Vec<u8>;
}

/// A trait for constructing numeric types from bytes, respecting endianness.
///
/// This is the inverse of [`EndianSerialize`] and is primarily used by deserialization APIs.
///
/// # Associated Types
/// - `Number`: The target type produced by deserialization
///
/// # Example
/// ```
/// let bytes = [0x78, 0x56, 0x34, 0x12];
/// let value = <u32 as EndianDeserialize>::from_endian_bytes(&bytes, Endianness::LittleEndian);
/// ```
///
/// [`EndianSerialize`]: crate::utils::EndianSerialize
pub trait EndianDeserialize<'a> {
    type Number;
    fn from_endian_bytes(bytes: &'a [u8], endianness: Endianness) -> Self::Number;
}
