use crate::errors::DataParseError;
use crate::impl_async_number;
use crate::options::EncodingOptions;
use crate::utils::{EndianSerialize, ParseResult};
use tokio::io::{AsyncWrite, AsyncWriteExt};

use super::helpers::AsyncEncodable;

#[derive(Default, Clone)]
pub struct AsyncDataWriter<W: AsyncWrite + Unpin> {
    pub(crate) options: EncodingOptions,
    pub(crate) writer: W,
}

impl<W: AsyncWrite + Unpin> AsyncDataWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            options: EncodingOptions::default(),
        }
    }
    pub fn with_options(writer: W, options: EncodingOptions) -> Self {
        Self { writer, options }
    }

    pub fn set_options(&mut self, options: EncodingOptions) {
        self.options = options;
    }

    pub async fn add_item<T: AsRef<[u8]>>(&mut self, data: T) -> ParseResult<()> {
        let data = data.as_ref();
        if self.options.prepend_data_size {
            let len = data.len() as u32;
            self.writer
                .write_all(&len.to_be_bytes())
                .await
                .map_err(DataParseError::from)?;
        }
        self.writer
            .write_all(data)
            .await
            .map_err(DataParseError::from)?;
        Ok(())
    }

    async fn add_num<T: EndianSerialize>(&mut self, n: T) -> ParseResult<()> {
        let data = n.to_endian_bytes(&self.options.endianness);
        self.add_item(data).await
    }

    pub async fn add_slice<T: AsyncEncodable>(&mut self, items: &[T]) -> ParseResult<()> {
        self.add_u32(items.len() as u32).await?;
        for item in items {
            let mut vec = Vec::new();
            let mut temp = AsyncDataWriter::new(&mut vec);
            temp.set_options(self.options.clone());
            item.async_to_writer(&mut temp).await?;
            self.add_u32(vec.len() as u32).await?;
            self.add_item(vec).await?;
        }
        Ok(())
    }
    impl_async_number!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
}
