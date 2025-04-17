//! Configuration module for binary parsing and encoding behavior.
//!
//! This module defines [`ParseOptions`] and [`EncodingOptions`], which allow
//! fine-grained control over how data is read from or written to byte streams.
//!
//! These options allow developers to customize:
//! - Endianness (byte order)
//! - Strict vs. lossy string decoding
//! - Null string trimming
//! - Verbose/custom error output
//! - Length-prefixed field handling
//! - (Optionally) AES-256 encryption keys and IVs
//!
//! These options are passed to the core data processing types:
//! - [`DataParser`]: uses `ParseOptions`
//! - [`DataEncoder`]: uses `EncodingOptions`
//!
//! # Example
//! ```rust
//! use libparser::options::ParseOptions;
//! use libparser::utils::Endianness;
//!
//! let options = ParseOptions::default()
//!     .with_strict_encoding()
//!     .with_trim_null_strings()
//!     .with_length_prefixed_fields()
//!     .with_verbose_errors();
//! ```
//!
//! [`ParseOptions`]: crate::options::ParseOptions
//! [`EncodingOptions`]: crate::options::EncodingOptions
//! [`DataParser`]: crate::parser::core::DataParser
//! [`DataEncoder`]: crate::encoder::core::DataEncoder
use crate::utils::Endianness;
/// Configuration options used when parsing binary data using [`DataParser`].
///
/// `ParseOptions` control how strings, numbers, and structural details are interpreted from
/// a binary buffer. These options are typically passed into a `DataParser` instance to modify
/// decoding behavior.
///
/// You can build up options manually or using a builder-style API:
///
/// # Example
/// ```
/// let options = ParseOptions::default()
///     .with_strict_encoding()
///     .with_trim_null_strings()
///     .with_length_prefixed_fields();
/// ```
///
/// # Features
/// - Endianness control
/// - Strict vs lossy string decoding
/// - Trimming null bytes from strings
/// - Length-prefixed field support
/// - Optional encryption configuration (via `crypto` feature)
///
/// [`DataParser`]: crate::parser::core::DataParser
#[derive(Clone, Debug)]
pub struct ParseOptions {
    /// If `true`, trailing nulls (`\0`) are removed from strings after decoding.
    pub(crate) trim_null_strings: bool,

    /// If `true`, parser returns an error on invalid UTF-8 or UTF-16 input.
    /// If `false`, uses lossy conversion.
    pub(crate) strict_encoding: bool,

    /// Controls the byte order (endianness) used for numbers.
    pub(crate) endianness: Endianness,

    /// If `true`, expects fields to be prefixed with their length.
    pub(crate) length_prefixed_fields: bool,

    /// If `true`, enables verbose, custom error reporting.
    pub(crate) verbose_errors: bool,

    /// AES-256 key for decryption (only available with `crypto` feature).
    #[cfg(feature = "crypto")]
    pub(crate) key: Vec<u8>,

    /// AES-256 IV for decryption (only available with `crypto` feature).
    #[cfg(feature = "crypto")]
    pub(crate) iv: Vec<u8>,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            trim_null_strings: false,
            endianness: Endianness::BigEndian,
            strict_encoding: false,
            length_prefixed_fields: false,
            verbose_errors: false,
            #[cfg(feature = "crypto")]
            key: Vec::new(),
            #[cfg(feature = "crypto")]
            iv: Vec::new(),
        }
    }
}

impl ParseOptions {
    /// Enables trimming of trailing nulls (`\0`) from decoded strings.
    pub fn set_trim_null_strings(&mut self) {
        self.trim_null_strings = true;
    }

    /// Disables trimming of null bytes from decoded strings.
    pub fn unset_trim_null_strings(&mut self) {
        self.trim_null_strings = false;
    }

    /// Enables strict string decoding (returns an error on invalid UTF-8/UTF-16).
    pub fn set_strict_encoding(&mut self) {
        self.strict_encoding = true;
    }

    /// Disables strict decoding (uses lossy string conversion).
    pub fn unset_strict_encoding(&mut self) {
        self.strict_encoding = false;
    }

    /// Sets the endianness (byte order) for reading numbers.
    pub fn set_endianness(&mut self, endian: Endianness) {
        self.endianness = endian;
    }

    /// Enables verbose, custom error formatting.
    pub fn set_verbose_errors(&mut self) {
        self.verbose_errors = true;
    }

    /// Disables verbose error formatting.
    pub fn unset_verbose_errors(&mut self) {
        self.verbose_errors = false;
    }

    /// Enables support for reading length-prefixed fields (e.g. `Vec<T>`, nested blocks).
    pub fn set_length_prefixed_fields(&mut self) {
        self.length_prefixed_fields = true;
    }

    /// Disables length-prefixed field parsing.
    pub fn unset_length_prefixed_fields(&mut self) {
        self.length_prefixed_fields = false;
    }

    /// Enables trimming and returns updated options (builder-style).
    pub fn with_trim_null_strings(mut self) -> Self {
        self.trim_null_strings = true;
        self
    }

    /// Enables strict string parsing and returns updated options.
    pub fn with_strict_encoding(mut self) -> Self {
        self.strict_encoding = true;
        self
    }

    /// Enables verbose error output and returns updated options.
    pub fn with_verbose_errors(mut self) -> Self {
        self.verbose_errors = true;
        self
    }

    /// Enables length-prefixed field parsing and returns updated options.
    pub fn with_length_prefixed_fields(mut self) -> Self {
        self.length_prefixed_fields = true;
        self
    }
}

/// Configuration options used when encoding data using [`DataEncoder`].
///
/// Controls how numbers and fields are written to the internal buffer,
/// including optional size prefixing and endianness settings.
///
/// These options are typically passed to or cloned into a `DataEncoder`.
///
/// # Example
/// ```
/// let options = EncodingOptions::default().with_prepended_data_size().with_endianness(Endianness::LittleEndian);
/// ```
///
/// [`DataEncoder`]: crate::encoder::core::DataEncoder
#[derive(Debug, Clone)]
pub struct EncodingOptions {
    /// Controls the byte order used for numeric encoding.
    pub(crate) endianness: Endianness,

    /// If `true`, every `add_item(...)` call prepends a `u32` size prefix.
    pub(crate) prepend_data_size: bool,

    /// AES-256 key used for encryption (if crypto is enabled).
    #[cfg(feature = "crypto")]
    pub(crate) key: Vec<u8>,

    /// AES-256 IV used for encryption (if crypto is enabled).
    #[cfg(feature = "crypto")]
    pub(crate) iv: Vec<u8>,
}

impl Default for EncodingOptions {
    fn default() -> Self {
        Self {
            endianness: Endianness::BigEndian,
            prepend_data_size: false,
            #[cfg(feature = "crypto")]
            key: Vec::new(),
            #[cfg(feature = "crypto")]
            iv: Vec::new(),
        }
    }
}

impl EncodingOptions {
    /// Enables data size prefixing for all encoded items (as `u32`).
    pub fn set_prepended_data_size(&mut self) {
        self.prepend_data_size = true;
    }

    /// Disables automatic data size prefixing.
    pub fn unset_prepended_data_size(&mut self) {
        self.prepend_data_size = false;
    }

    /// Sets the byte order for all numeric encodings.
    pub fn set_endianness(&mut self, endianness: Endianness) {
        self.endianness = endianness;
    }

    /// Enables size prefixing and returns updated options (builder-style).
    pub fn with_prepended_data_size(mut self) -> Self {
        self.prepend_data_size = true;
        self
    }

    /// Sets endianness and returns updated options (builder-style).
    pub fn with_endianness(mut self, endianness: Endianness) -> Self {
        self.endianness = endianness;
        self
    }
}
