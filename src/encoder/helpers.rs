use super::core::DataEncoder;
use crate::impl_encodable;
use crate::utils::ParseResult;

impl DataEncoder {
    /// Adds a string value to the encoder.
    ///
    /// This method converts the input into a `String`, writes its length (`u32`),
    /// then writes the raw bytes.
    ///
    /// # Example
    /// ```rust
    /// encoder.add_string("hello")?;
    /// ```
    ///
    /// This is equivalent to calling `String::encode_data(...)` directly.
    pub fn add_string(&mut self, data: impl Into<String>) -> ParseResult<()> {
        let data: String = data.into();
        self.add_u32(data.len() as u32)?;
        self.add_item(data)
    }

    /// Adds a single boolean value to the encoder.
    ///
    /// Encoded as a single byte: `0x01` for `true`, `0x00` for `false`.
    pub fn add_bool(&mut self, data: bool) -> ParseResult<()> {
        self.add_item(vec![data as u8])
    }
}

/// A trait for types that can be serialized using a [`DataEncoder`].
///
/// Types implementing `Encodable` define how to write their binary representation
/// into an output encoder. This is the core trait used by the framework's derive macros
/// and container support.
///
/// # Example
/// ```rust
/// struct MyData {
///     id: u32,
///     flag: bool,
/// }
///
/// impl Encodable for MyData {
///     fn encode_data(&self, encoder: &mut DataEncoder) -> ParseResult<()> {
///         encoder.add_item(self.id)?;
///         encoder.add_bool(self.flag)
///     }
/// }
/// ```
///
/// [`DataEncoder`]: crate::encoder::core::DataEncoder
pub trait Encodable {
    fn encode_data(&self, encoder: &mut DataEncoder) -> ParseResult<()>;
}

/// Implements `Encodable` for `Option<T>` by writing a boolean flag followed by the value (if present).
///
/// Format:
/// - `0x01` followed by encoded `T` if `Some`
/// - `0x00` if `None`
impl<T: Encodable> Encodable for Option<T> {
    fn encode_data(&self, writer: &mut DataEncoder) -> ParseResult<()> {
        match self {
            Some(value) => {
                writer.add_bool(true)?;
                value.encode_data(writer)
            }
            None => {
                writer.add_bool(false)?;
                Ok(())
            }
        }
    }
}

/// Implements `Encodable` for `Vec<T>` by prefixing the length (as `u32`), and encoding each element.
///
/// Format:
/// - `[length: u32][item1][item2]...[itemN]`
///
/// Note: Internally uses `add_slice`.
impl<T: Encodable> Encodable for Vec<T> {
    fn encode_data(&self, encoder: &mut DataEncoder) -> ParseResult<()> {
        encoder.add_slice(self)
    }
}

/// Implements `Encodable` for arrays `[T; N]` by encoding each element sequentially.
///
/// Does **not** include a length prefix; assumes caller knows the array size.
///
/// Format:
/// - `[item1][item2]...[itemN]`
impl<T: Encodable, const N: usize> Encodable for [T; N] {
    fn encode_data(&self, encoder: &mut DataEncoder) -> ParseResult<()> {
        for item in self {
            item.encode_data(encoder)?;
        }
        Ok(())
    }
}

impl Encodable for String {
    fn encode_data(&self, encoder: &mut DataEncoder) -> ParseResult<()> {
        encoder.add_u32(self.len() as u32)?;
        encoder.add_item(self.as_bytes())
    }
}

impl_encodable!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
