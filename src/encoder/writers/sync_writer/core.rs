use crate::{
    Encodable,
    encoder::core::DataEncoder,
    errors::DataParseError,
    impl_number,
    parser::EncodingOptions,
    utils::{EndianSerialize, ParseResult},
};

#[derive(Default, Clone)]
pub struct DataWriter<W: std::io::Write> {
    pub(crate) options: EncodingOptions,
    pub(crate) writer: W,
}

impl<W: std::io::Write> DataWriter<W> {
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

    pub fn flush(&mut self) -> ParseResult<()> {
        self.writer
            .flush()
            .map_err(|e| DataParseError::IoError { e })?;
        Ok(())
    }

    pub(crate) fn add_item<T>(&mut self, data: T) -> ParseResult<()>
    where
        T: AsRef<[u8]>,
    {
        let data = data.as_ref();
        if self.options.prepend_data_size {
            let data_len = data.len() as u32;
            self.writer
                .write_all(&data_len.to_be_bytes())
                .map_err(|e| DataParseError::IoError { e })?;
        }
        self.writer
            .write_all(data)
            .map_err(|e| DataParseError::IoError { e })?;
        Ok(())
    }

    fn add_num<T: EndianSerialize>(&mut self, n: T) -> ParseResult<()> {
        let data = n.to_endian_bytes(&self.options.endianness);
        self.add_item(data)
    }

    pub fn add_slice<T: Encodable>(&mut self, data: &[T]) -> ParseResult<()> {
        let data_len = data.len();
        self.add_u32(data_len as u32)?;
        for item in data {
            let mut temp_encoder = DataEncoder::default();
            temp_encoder.set_options(&self.options);
            item.encode_data(&mut temp_encoder)?;
            let built = temp_encoder.get_data()?;
            self.add_u32(built.len() as u32)?;
            self.add_item(built)?;
        }
        Ok(())
    }

    impl_number!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
}
