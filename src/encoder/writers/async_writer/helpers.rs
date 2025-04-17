use crate::encoder::core::DataEncoder;
use crate::impl_async_serializer;
use crate::utils::ParseResult;
use async_trait::async_trait;
use tokio::io::AsyncWrite;

use super::core::AsyncDataWriter;

impl<W: AsyncWrite + Unpin> AsyncDataWriter<W> {
    pub async fn add_string(&mut self, data: impl Into<String>) -> ParseResult<()> {
        let data = data.into();
        self.add_u32(data.len() as u32).await?;
        self.add_item(data).await
    }

    pub async fn add_bool(&mut self, data: bool) -> ParseResult<()> {
        self.add_item(vec![data as u8]).await
    }
}

#[async_trait]
pub trait AsyncEncodable {
    async fn async_to_writer<W: AsyncWrite + Unpin + Send>(
        &self,
        writer: &mut AsyncDataWriter<W>,
    ) -> ParseResult<()>;
}

#[async_trait]
impl AsyncEncodable for u8 {
    async fn async_to_writer<W: AsyncWrite + Unpin + Send>(
        &self,
        writer: &mut AsyncDataWriter<W>,
    ) -> ParseResult<()> {
        writer.add_u8(*self).await
    }
}

#[async_trait]
impl<T: AsyncEncodable + Send + Sync> AsyncEncodable for Option<T> {
    async fn async_to_writer<W: AsyncWrite + Unpin + Send>(
        &self,
        writer: &mut AsyncDataWriter<W>,
    ) -> ParseResult<()> {
        match self {
            Some(val) => {
                writer.add_bool(true).await?;
                val.async_to_writer(writer).await
            }
            None => writer.add_bool(false).await,
        }
    }
}

// For now, Vec<T> has to have the size prepended to it
#[async_trait]
impl<T: AsyncEncodable + Send + Sync> AsyncEncodable for Vec<T> {
    async fn async_to_writer<W: AsyncWrite + Unpin + Send>(
        &self,
        encoder: &mut AsyncDataWriter<W>,
    ) -> ParseResult<()> {
        encoder.add_u32(self.len() as u32).await?;
        for item in self {
            let mut temp_encoder = DataEncoder::default();
            temp_encoder.set_options(&encoder.options);
            item.async_to_writer(&mut AsyncDataWriter::new(Vec::new()))
                .await?;
            let item_data = temp_encoder.get_data()?;
            encoder.add_u32(item_data.len() as u32).await?;
            encoder.add_item(item_data).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl<T: AsyncEncodable + Send + Sync, const N: usize> AsyncEncodable for [T; N] {
    async fn async_to_writer<W: AsyncWrite + Unpin + Send>(
        &self,
        encoder: &mut AsyncDataWriter<W>,
    ) -> ParseResult<()> {
        for item in self {
            item.async_to_writer(encoder).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl AsyncEncodable for String {
    async fn async_to_writer<W: AsyncWrite + Unpin + Send>(
        &self,
        encoder: &mut AsyncDataWriter<W>,
    ) -> ParseResult<()> {
        let len = self.len();
        encoder.add_u32(len as u32).await?;
        encoder.add_item(self).await
    }
}

impl_async_serializer!(i16, isize);
