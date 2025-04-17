#[macro_export]
macro_rules! impl_async_number {
    ($($t:ty),* $(,)?) => {
        $(
            paste::paste! {
                pub async fn [<add_ $t>](&mut self, n: $t) -> $crate::utils::ParseResult<()> {
                    self.add_num(n).await
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_async_serializer {
    ($($t:ty),* $(,)?) => {
        $(
        paste::paste! {
            #[async_trait::async_trait]
            impl $crate::encoder::writers::async_writer::helpers::AsyncEncodable for $t {
                async fn async_to_writer<W: AsyncWrite + Unpin + Send>(
                    &self,
                    writer: &mut AsyncDataWriter<W>,
                ) -> $crate::utils::ParseResult<()> {
                    writer.[<add_ $t>](*self).await
                }
            }
        }
        )*
    };
}
