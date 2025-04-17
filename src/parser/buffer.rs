use std::ops::{Deref, DerefMut};

/// A flexible, owned-or-borrowed buffer abstraction for in-place encoding and decoding.
///
/// `Buffer` allows you to either borrow a mutable buffer (`&'a mut [u8]`) or
/// own the buffer outright (`Box<[u8]>`). This is useful for encoding APIs
/// that want to support zero-copy or heap-allocated storage interchangeably.
///
/// ## Example
/// ```
/// let mut scratch = [0u8; 32];
/// let buf = Buffer::from(&mut scratch[..]); // Borrowed
/// let buf = Buffer::from(vec![0u8; 32]);    // Owned
/// ```
pub enum Buffer<'a> {
    /// A mutable borrowed buffer slice.
    Borrowed(&'a mut [u8]),
    /// An owned heap-allocated buffer.
    Owned(Box<[u8]>),
}

impl Buffer<'_> {
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Buffer::Borrowed(buf) => buf,
            Buffer::Owned(buf) => buf,
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        match self {
            Buffer::Borrowed(buf) => buf,
            Buffer::Owned(buf) => buf,
        }
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }
}

impl Deref for Buffer<'_> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for Buffer<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl From<Vec<u8>> for Buffer<'_> {
    fn from(value: Vec<u8>) -> Self {
        Buffer::Owned(value.into_boxed_slice())
    }
}
impl<'a> From<&'a mut [u8]> for Buffer<'a> {
    fn from(value: &'a mut [u8]) -> Self {
        Buffer::Borrowed(value)
    }
}

impl<'a> From<&'a mut Vec<u8>> for Buffer<'a> {
    fn from(value: &'a mut Vec<u8>) -> Self {
        Buffer::Borrowed(value.as_mut_slice())
    }
}

impl Default for Buffer<'_> {
    fn default() -> Self {
        Self::Owned(Box::default())
    }
}
