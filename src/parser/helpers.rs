use super::core::DataParser;
use crate::{errors::DataParseError, impl_deserializer, utils::ParseResult};

pub trait Decodable: Sized {
    fn from_parser(parser: &mut DataParser) -> ParseResult<Self>;
}

impl DataParser<'_> {
    pub fn get_vector<T: Decodable>(&mut self) -> ParseResult<Vec<T>> {
        let len = self.get_u32()? as usize;
        let mut out = Vec::with_capacity(len);
        let options = self.options.clone();

        for _ in 0..len {
            let item_len = self.get_u32()? as usize;
            let item_bytes = self.get_bytes(item_len)?;
            let mut temp_parser = DataParser::with_options(item_bytes, options.clone());
            out.push(T::from_parser(&mut temp_parser)?);
        }

        Ok(out)
    }

    pub fn get_option<T: Decodable>(&mut self) -> ParseResult<Option<T>> {
        let flag = self.get_bool()?;
        if flag {
            Ok(Some(T::from_parser(self)?))
        } else {
            Ok(None)
        }
    }

    pub fn get_raw<T>(&mut self, signed: bool) -> ParseResult<T>
    where
        T: TryFrom<i8> + TryFrom<u8>,
    {
        let byte = self.take(1)?[0] as i8;

        if signed {
            let signed_val = byte;
            T::try_from(signed_val).map_err(|_| {
                if self.options.verbose_errors {
                    DataParseError::InvalidConversion {
                        e: format!("Failed to convert {} (i8) to target type", signed_val),
                    }
                } else {
                    DataParseError::InvalidConversion {
                        e: "TryFrom<i8> failed".into(),
                    }
                }
            })
        } else {
            T::try_from(byte).map_err(|_| {
                if self.options.verbose_errors {
                    DataParseError::InvalidConversion {
                        e: format!("Failed to convert {} (u8) to target type", byte),
                    }
                } else {
                    DataParseError::InvalidConversion {
                        e: "TryFrom<u8> failed".into(),
                    }
                }
            })
        }
    }
}

impl<T: Decodable> Decodable for Option<T> {
    fn from_parser(parser: &mut DataParser) -> ParseResult<Self> {
        let flag = parser.get_bool()?;
        if flag {
            Ok(Some(T::from_parser(parser)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: Decodable> Decodable for Vec<T> {
    fn from_parser(parser: &mut DataParser) -> ParseResult<Self> {
        let len = parser.get_u32()?;
        let mut out = Vec::with_capacity(len as usize);
        let options = parser.options.clone();
        for _ in 0..len {
            let item_len = parser.get_u32()?;
            let mut item_bytes = parser.take(item_len as usize)?.to_vec();
            let mut temp_parser = DataParser::with_options(&mut item_bytes, options.clone());
            out.push(T::from_parser(&mut temp_parser)?);
        }
        Ok(out)
    }
}

impl Decodable for String {
    fn from_parser(parser: &mut DataParser) -> ParseResult<Self> {
        parser.get_string(false)
    }
}

impl_deserializer!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64);
