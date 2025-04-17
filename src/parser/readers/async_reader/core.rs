use crate::parser::buffer::Buffer;
use crate::parser::{DataParser, ParseOptions};
use crate::utils::ParseResult;
use tokio::io::{AsyncRead, AsyncReadExt};

#[allow(unused)]
pub struct AsyncDataReader<R: AsyncRead + Unpin> {
    pub(crate) reader: R,
    pub(crate) options: ParseOptions,
}

impl<R> AsyncDataReader<R>
where
    R: AsyncRead + Unpin,
{
    pub async fn new(reader: R) -> ParseResult<Self> {
        Ok(Self {
            reader,
            options: ParseOptions::default(),
        })
    }

    pub async fn with_options(reader: R, options: ParseOptions) -> ParseResult<Self> {
        Ok(Self { reader, options })
    }
}

impl DataParser<'_> {
    pub async fn with_async_reader<R: AsyncRead + Unpin>(mut reader: R) -> ParseResult<Self> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await?;
        Ok(Self {
            buffer: Buffer::from(buf),
            cursor: 0,
            options: ParseOptions::default(),
        })
    }

    pub async fn with_options_async_reader<R: AsyncRead + Unpin>(
        mut reader: R,
        options: ParseOptions,
    ) -> ParseResult<Self> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await?;
        Ok(Self {
            buffer: Buffer::from(buf),
            cursor: 0,
            options,
        })
    }
}
