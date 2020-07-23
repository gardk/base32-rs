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
//! Length calculations are checked and will panic on overflow of `usize`.
//!
//! Encoding & decoding with the respective `to_slice` methods will panic if the
//! provided output slice is too small to handle the input, this will not happen
//! if the accompanying size translation methods are used to allocate them properly.
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
    warnings
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

/// Available encoding character sets
#[derive(Copy, Clone, Debug)]
pub enum Alphabet {
    /// The standard character set defined in RFC4648, it uses an alphabet of A–Z, followed
    /// by 2–7. 0 and 1 are skipped due to their similarity with the letters O and I.
    Standard,
    /// The extended hex character set defined in RFC4648. It is based on hexadecimal and
    /// simply extends its set of valid characters.
    ExtendedHex,
    /// An alternative design for base32 created by Douglas Crockford. It excludes the letters
    /// I, L, and O to avoid confusion with digits. It also excludes the letter U to reduce the
    /// likelihood of accidental obscenity. Checksums are currently not supported by this library.
    ///
    /// [https://crockford.com/base32.html](https://crockford.com/base32.html)
    Crockford,
    /// z-base-32 is a base32 encoding designed to be easier for human use and more compact. It
    /// includes 1, 8 and 9 but excludes l, v and 2. It also permutes the alphabet so that the
    /// easier characters are the ones that occur more frequently.
    ///
    /// [https://philzimmermann.com/docs/human-oriented-base-32-encoding.txt](https://philzimmermann.com/docs/human-oriented-base-32-encoding.txt)
    ZBase32,
}

impl Alphabet {
    /// Returns the pre-defined encoding table for the chosen alphabet
    #[inline(always)]
    pub fn encode_table(self) -> &'static [u8; 32] {
        match self {
            Alphabet::Standard => tables::ENCODE_STD,
            Alphabet::ExtendedHex => tables::ENCODE_HEX,
            Alphabet::Crockford => tables::ENCODE_CROCKFORD,
            Alphabet::ZBase32 => tables::ENCODE_ZBASE32,
        }
    }

    /// Returns the pre-defined decoding table for the chosen alphabet
    #[inline(always)]
    pub fn decode_table(self) -> &'static [u8; 256] {
        match self {
            Alphabet::Standard => tables::DECODE_STD,
            Alphabet::ExtendedHex => tables::DECODE_HEX,
            Alphabet::Crockford => tables::DECODE_CROCKFORD,
            Alphabet::ZBase32 => tables::DECODE_ZBASE32,
        }
    }
}

/// The definition of an encoding, and the center of the API.
#[derive(Copy, Clone, Debug)]
pub struct Encoding {
    alpha: Alphabet,
    pad: Option<u8>,
}

impl Encoding {
    /// Changes or disables padding
    pub const fn with_padding(self, pad: Option<u8>) -> Encoding {
        Encoding { pad, ..self }
    }
}

impl Default for Encoding {
    fn default() -> Self {
        STANDARD
    }
}

/// An `Encoding` for the standard format defined in RFC4648 ([details](enum.Alphabet.html#variant.Standard))
/// ```
/// assert_eq!(base32::STANDARD.encode("foobar"), "MZXW6YTBOI======");
/// ```
pub const STANDARD: Encoding = Encoding {
    alpha: Alphabet::Standard,
    pad: Some(b'='),
};

/// An `Encoding` using the extended hex format defined in RFC4648 ([details](enum.Alphabet.html#variant.ExtendedHex))
/// ```
/// assert_eq!(base32::EXTENDED_HEX.encode("foobar"), "CPNMUOJ1E8======");
/// ```
pub const EXTENDED_HEX: Encoding = Encoding {
    alpha: Alphabet::ExtendedHex,
    pad: Some(b'='),
};

/// An `Encoding` using Douglas Crockford's character set ([details](enum.Alphabet.html#variant.Crockford))
/// ```
/// assert_eq!(base32::CROCKFORD.encode("foobar"), "CSQPYRK1E8");
/// ```
pub const CROCKFORD: Encoding = Encoding {
    alpha: Alphabet::Crockford,
    pad: None,
};

/// An `Encoding` using the z-base-32 character set ([details](enum.Alphabet.html#variant.ZBase32))
/// ```
/// assert_eq!(base32::ZBASE32.encode("foobar"), "c3zs6aubqe");
/// ```
pub const ZBASE32: Encoding = Encoding {
    alpha: Alphabet::ZBase32,
    pad: None,
};
