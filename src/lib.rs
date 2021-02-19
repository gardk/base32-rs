//! Library for encoding & decoding base32 with various character sets.
//!
//! # Encoding
//!
//! *Brief description of how to encode data goes here*
//!
//! # Decoding
//!
//! *Brief description of how to decode data and which errors can occur goes here*
//!
//! # Panics
//!
//! Length calculations are checked and will panic on overflow of `usize`, this
//! applies to any helper function for encoding or decoding data as well as the
//! length calculation functions themselves.
//!
//! Encoding & decoding with the respective `to_slice` methods will panic if the
//! provided output slice is too small to handle the input, this will not happen
//! if the accompanying size translation methods are used to allocate it properly.
#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    variant_size_differences,
    warnings,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

mod decode;
mod encode;
mod tables;

pub use decode::DecodeError;

#[cfg(all(feature = "alloc", not(any(feature = "std", test))))]
extern crate alloc;
#[cfg(any(feature = "std", test))]
extern crate std as alloc;

#[cfg(any(feature = "alloc", feature = "std", test))]
pub use crate::decode::decode;
#[cfg(any(feature = "alloc", feature = "std", test))]
pub use crate::encode::encode;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug)]
enum Alphabet {
    Standard,
    ExtendedHex,
    Crockford,
    ZBase32,
}

impl Alphabet {
    const fn encode_table(self) -> &'static [u8; 32] {
        match self {
            Alphabet::Standard => tables::ENCODE_STD,
            Alphabet::ExtendedHex => tables::ENCODE_HEX,
            Alphabet::Crockford => tables::ENCODE_CROCKFORD,
            Alphabet::ZBase32 => tables::ENCODE_ZBASE32,
        }
    }

    const fn decode_table(self) -> &'static [u8; 256] {
        match self {
            Alphabet::Standard => tables::DECODE_STD,
            Alphabet::ExtendedHex => tables::DECODE_HEX,
            Alphabet::Crockford => tables::DECODE_CROCKFORD,
            Alphabet::ZBase32 => tables::DECODE_ZBASE32,
        }
    }
}

/// An encoding specification.
#[derive(Copy, Clone, Debug)]
pub struct Encoding {
    alpha: Alphabet,
    pad: Option<u8>,
}

impl Encoding {
    /// Changes or disables padding
    #[inline]
    pub const fn with_padding(self, pad: Option<u8>) -> Encoding {
        Encoding { pad, ..self }
    }
}

impl Default for Encoding {
    #[inline]
    fn default() -> Self {
        STANDARD
    }
}

/// The most widely used Base32 alphabet is defined in RFC 4648. It uses
/// an alphabet of A–Z, followed by 2–7. 0 and 1 are skipped due to their
/// similarity with the letters O and I (thus "2" actually has a decimal value of 26).
/// ```
/// assert_eq!(base32::STANDARD.encode("foobar"), "MZXW6YTBOI======");
/// ```
pub const STANDARD: Encoding = Encoding {
    alpha: Alphabet::Standard,
    pad: Some(b'='),
};

/// The extended hex character set defined in RFC4648. It is based
/// on hexadecimal and simply extends its set of valid characters.
/// ```
/// assert_eq!(base32::EXTENDED_HEX.encode("foobar"), "CPNMUOJ1E8======");
/// ```
pub const EXTENDED_HEX: Encoding = Encoding {
    alpha: Alphabet::ExtendedHex,
    pad: Some(b'='),
};

/// An alternative design for base32 created by Douglas Crockford. It excludes the letters
/// I, L, and O to avoid confusion with digits. It also excludes the letter U to reduce the
/// likelihood of accidental obscenity. Checksums are currently not supported by this library.
///
/// [https://crockford.com/base32.html](https://crockford.com/base32.html)
/// ```
/// assert_eq!(base32::CROCKFORD.encode("foobar"), "CSQPYRK1E8");
/// ```
pub const CROCKFORD: Encoding = Encoding {
    alpha: Alphabet::Crockford,
    pad: None,
};

/// z-base-32 is a base32 encoding designed to be easier for human use and more compact. It
/// includes 1, 8 and 9 but excludes l, v and 2. It also permutes the alphabet so that the
/// easier characters are the ones that occur more frequently.
///
/// [https://philzimmermann.com/docs/human-oriented-base-32-encoding.txt](https://philzimmermann.com/docs/human-oriented-base-32-encoding.txt)
/// ```
/// assert_eq!(base32::ZBASE32.encode("foobar"), "c3zs6aubqe");
/// ```
pub const ZBASE32: Encoding = Encoding {
    alpha: Alphabet::ZBase32,
    pad: None,
};
