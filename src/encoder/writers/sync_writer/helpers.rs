use std::io::Write;

use crate::{encoder::core::DataEncoder, impl_write_encodable, utils::ParseResult};

use super::core::DataWriter;

pub trait WriteEncodable {
    fn to_writer<W: Write>(&self, encoder: &mut DataWriter<W>) -> ParseResult<()>;
}

impl<T: WriteEncodable> WriteEncodable for Option<T> {
    fn to_writer<W: Write>(&self, encoder: &mut DataWriter<W>) -> ParseResult<()> {
        match self {
            Some(value) => {
                encoder.add_bool(true)?;
                value.to_writer(encoder)
            }
            None => {
                encoder.add_bool(false)?;
                Ok(())
            }
        }
    }
}

impl<T: WriteEncodable> WriteEncodable for Vec<T> {
    fn to_writer<W: Write>(&self, encoder: &mut DataWriter<W>) -> ParseResult<()> {
        encoder.add_u32(self.len() as u32)?;
        for item in self {
            let mut temp_encoder = DataEncoder::default();
            temp_encoder.set_options(&encoder.options);
            item.to_writer(&mut DataWriter::new(Vec::new()))?;
            let item_data = temp_encoder.get_data()?;
            encoder.add_u32(item_data.len() as u32)?;
            encoder.add_item(item_data)?;
        }
        Ok(())
    }
}

impl<T: WriteEncodable, const N: usize> WriteEncodable for [T; N] {
    fn to_writer<W: Write>(&self, encoder: &mut DataWriter<W>) -> ParseResult<()> {
        for item in self {
            item.to_writer(encoder)?;
        }
        Ok(())
    }
}

impl WriteEncodable for String {
    fn to_writer<W: Write>(&self, encoder: &mut DataWriter<W>) -> ParseResult<()> {
        encoder.add_u32(self.len() as u32)?;
        encoder.add_item(self.as_bytes())
    }
}

impl<W: Write> DataWriter<W> {
    pub fn add_string(&mut self, data: impl Into<String>) -> ParseResult<()> {
        let data: String = data.into();
        self.add_u32(data.len() as u32)?;
        self.add_item(data)
    }

    pub fn add_bool(&mut self, data: bool) -> ParseResult<()> {
        self.add_item(vec![data as u8])
    }
}

impl_write_encodable!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
