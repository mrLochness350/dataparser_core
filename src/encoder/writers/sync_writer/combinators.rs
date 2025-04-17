use std::io::Write;

use crate::utils::ParseResult;

use super::core::DataWriter;

impl<W: Write> DataWriter<W> {
    pub fn add_between<F>(&mut self, start: &[u8], end: &[u8], build_fn: F) -> ParseResult<()>
    where
        F: FnOnce(&mut DataWriter<W>) -> ParseResult<()>,
    {
        self.add_item(start)?;
        build_fn(self)?;
        self.add_item(end)?;
        Ok(())
    }
}
