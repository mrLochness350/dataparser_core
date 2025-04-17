#[macro_export]
macro_rules! impl_write_encodable {
    ($($t:ty),* $(,)?) => {
        $(
        paste::paste! {
            impl $crate::encoder::writers::sync_writer::helpers::WriteEncodable for $t {
                fn to_writer<W: Write>(&self, encoder: &mut DataWriter<W>) -> ParseResult<()> {
                    encoder.[<add_ $t>](*self)
                }
            }
        }
        )*
    };
}
