#[macro_export]
macro_rules! impl_get_with_prefix {
    ($($ty:ty),* $(,)?) => {
        $(
            paste::paste! {
                pub fn [<get_ $ty>](&mut self) -> $crate::utils::ParseResult<$ty> {
                    if self.options.length_prefixed_fields {
                        self.parse_with_length_prefix(|p| p.[<__get_ $ty>]())
                    } else {
                        self.[<__get_ $ty>]()
                    }
                }
                pub(crate) fn [<__get_ $ty>](&mut self) -> $crate::utils::ParseResult<$ty> {
                    let bytes = self.read_array::<{ std::mem::size_of::<$ty>() }>()?;
                    Ok(match self.options.endianness {
                        $crate::utils::Endianness::BigEndian => <$ty>::from_be_bytes(bytes),
                        $crate::utils::Endianness::LittleEndian => <$ty>::from_le_bytes(bytes),
                        $crate::utils::Endianness::NativeEndian => <$ty>::from_ne_bytes(bytes),
                    })
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_deserializer {
    ($($t:ty),* $(,)?) => {
        $(
        paste::paste! {
            impl $crate::Decodable for $t {
                fn from_parser(parser: &mut $crate::parser::core::DataParser) -> $crate::utils::ParseResult<Self> {
                    parser.[<get_ $t:lower>]()
                }
            }
        }
        )*
    };
}

#[macro_export]
macro_rules! impl_stream_deserializer {
    ($($t:ty),* $(,)?) => {
        $(
        paste::paste! {
            impl $crate::StreamDecodable for $t {
                fn from_stream_parser<R: Read>(reader: &mut $crate::parser::readers::sync_reader::core::DataReader<R>) -> ParseResult<Self> {
                    reader.[<get_ $t:lower>]()
                }
            }
        }
        )*
    };
}
