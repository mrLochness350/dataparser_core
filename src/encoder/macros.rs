macro_rules! impl_endian_serialize {
    ($($t:ty),* $(,)?) => {
        $(
            impl $crate::utils::EndianSerialize for $t {
                fn to_endian_bytes(self, endian: &$crate::utils::Endianness) -> Vec<u8> {
                    match endian {
                        $crate::utils::Endianness::BigEndian => self.to_be_bytes().to_vec(),
                        $crate::utils::Endianness::LittleEndian => self.to_le_bytes().to_vec(),
                        $crate::utils::Endianness::NativeEndian => self.to_ne_bytes().to_vec(),
                    }
                }
            }
        )*
    };
}

impl_endian_serialize!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
#[macro_export]
macro_rules! impl_number {
    ($($t:ty),* $(,)?) => {
        $(
            paste::paste! {
                pub fn [<add_ $t>](&mut self, n: $t) -> $crate::utils::ParseResult<()> {
                    self.add_num(n)
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_encodable {
    ($($t:ty),* $(,)?) => {
        $(
        paste::paste! {
            impl $crate::Encodable for $t {
                fn encode_data(&self, encoder: &mut DataEncoder) -> ParseResult<()> {
                    encoder.[<add_ $t>](*self)
                }
            }
        }
        )*
    };
}
