use tokio::io::AsyncWrite;

use crate::utils::ParseResult;

use super::core::AsyncDataWriter;

impl<W: AsyncWrite + Unpin> AsyncDataWriter<W> {
    pub async fn add_between<F>(&mut self, start: &[u8], end: &[u8], build_fn: F) -> ParseResult<()>
    where
        F: FnOnce(&mut AsyncDataWriter<W>) -> ParseResult<()>,
    {
        self.add_item(start).await?;
        build_fn(self)?;
        self.add_item(end).await?;
        Ok(())
    }
}
