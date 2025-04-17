//! AES-256-CBC encryption and decryption utilities with PKCS7 padding.
//!
//! This module provides helper functions and trait implementations for
//! encrypting and decrypting binary data using AES-256 in CBC mode.
//! These features are conditionally compiled using the `crypto` feature flag.

use crate::encoder::core::DataEncoder;
use crate::errors::DataParseError;
use crate::options::{EncodingOptions, ParseOptions};
use crate::parser::core::DataParser;
use crate::utils::ParseResult;
use aes::Aes256;
use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};

type Aes256CbcEnc = cbc::Encryptor<Aes256>;
type Aes256CbcDec = cbc::Decryptor<Aes256>;

/// Helper function to map crypto-related errors to `DataParseError`.
fn map_crypto_err<E: std::fmt::Display>(e: E) -> DataParseError {
    DataParseError::CryptoError { e: e.to_string() }
}

/// Decrypts the given data in-place using AES-256-CBC with the specified key and IV.
///
/// # Arguments
/// - `raw_data`: The encrypted data to decrypt.
/// - `key`: The 32-byte AES-256 key.
/// - `iv`: The 16-byte initialization vector.
///
/// # Returns
/// The decrypted plaintext as a `Vec<u8>`.
pub(crate) fn aes_decrypt(
    raw_data: &mut [u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, DataParseError> {
    let dc = Aes256CbcDec::new_from_slices(key, iv).map_err(map_crypto_err)?;
    let pt = dc
        .decrypt_padded_mut::<Pkcs7>(raw_data)
        .map_err(map_crypto_err)?;
    Ok(pt.to_vec())
}

/// Encrypts the given data in-place using AES-256-CBC with the specified key and IV.
///
/// # Arguments
/// - `raw_data`: The plaintext data to encrypt.
/// - `key`: The 32-byte AES-256 key.
/// - `iv`: The 16-byte initialization vector.
///
/// # Returns
/// The encrypted ciphertext as a `Vec<u8>`.
pub(crate) fn aes_encrypt(
    raw_data: &mut [u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, DataParseError> {
    let enc = Aes256CbcEnc::new_from_slices(key, iv).map_err(map_crypto_err)?;
    let pt = enc
        .encrypt_padded_mut::<Pkcs7>(raw_data, raw_data.len())
        .map_err(map_crypto_err)?;
    Ok(pt.to_vec())
}

#[cfg(feature = "crypto")]
impl DataParser<'_> {
    /// Encrypts the internal buffer using AES-256-CBC with the configured key and IV.
    pub fn encrypt(&mut self) -> ParseResult<()> {
        aes_encrypt(&mut self.buffer, &self.options.key, &self.options.iv)?;
        Ok(())
    }

    /// Decrypts the internal buffer using AES-256-CBC with the configured key and IV.
    pub fn decrypt(&mut self) -> ParseResult<()> {
        aes_decrypt(&mut self.buffer, &self.options.key, &self.options.iv)?;
        Ok(())
    }
}

#[cfg(feature = "crypto")]
impl DataEncoder {
    /// Encrypts the encoder's internal writer buffer using AES-256-CBC.
    pub fn encrypt(&mut self) -> ParseResult<()> {
        aes_encrypt(&mut self.buffer, &self.options.key, &self.options.iv)?;
        Ok(())
    }

    /// Decrypts the encoder's internal writer buffer using AES-256-CBC.
    pub fn decrypt(&mut self) -> ParseResult<()> {
        aes_decrypt(&mut self.buffer, &self.options.key, &self.options.iv)?;
        Ok(())
    }
}

#[cfg(feature = "crypto")]
impl ParseOptions {
    /// Sets the encryption key and IV, enabling encryption/decryption for `DataParser`.
    ///
    /// # Arguments
    /// - `key`: A 32-byte AES-256 encryption key.
    /// - `iv`: A 16-byte initialization vector.
    ///
    /// # Returns
    /// The updated `ParseOptions` with encryption configured.
    pub fn with_encryption(mut self, key: Vec<u8>, iv: Vec<u8>) -> Self {
        self.key = key;
        self.iv = iv;
        self
    }
}

#[cfg(feature = "crypto")]
impl EncodingOptions {
    /// Sets the encryption key and IV, enabling encryption/decryption for `DataVecEncoder`.
    ///
    /// # Arguments
    /// - `key`: A 32-byte AES-256 encryption key.
    /// - `iv`: A 16-byte initialization vector.
    ///
    /// # Returns
    /// The updated `EncodingOptions` with encryption configured.
    pub fn with_encryption(mut self, key: Vec<u8>, iv: Vec<u8>) -> Self {
        self.key = key;
        self.iv = iv;
        self
    }
}
