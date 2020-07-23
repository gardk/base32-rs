#[cfg(any(feature = "alloc", feature = "std", test))]
use alloc::{vec, vec::Vec};
use core::fmt;
#[cfg(any(feature = "std", test))]
use std::error;

use crate::tables::INVALID_BYTE;
use crate::Encoding;

/// Decodes data encoded using the standard base32 format
/// ```
/// fn main() -> Result<(), base32::DecodeError> {
///     assert_eq!(base32::decode("MZXW6YTBOI======")?, b"foobar");
///     Ok(())
/// }
/// ```
#[cfg(any(feature = "alloc", feature = "std", test))]
pub fn decode(data: impl AsRef<[u8]>) -> Result<Vec<u8>, DecodeError> {
    crate::STANDARD.decode(data)
}

/// Errors that can occur while decoding
#[derive(Copy, Clone, Debug)]
pub enum DecodeError {
    /// Returned if a byte that isn't present in the decoding table is encountered,
    /// it contains the index at which the offending byte was found as well as its
    /// value.
    InvalidByte(usize, u8),
    /// Returned if the input is of a length that could never produce well formed output.
    InvalidInputLength,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeError::InvalidByte(i, b) => {
                write!(fmt, "invalid input byte at index {}: {}", i, b)
            }
            DecodeError::InvalidInputLength => write!(fmt, "invalid input length"),
        }
    }
}

#[cfg(any(feature = "std", test))]
impl error::Error for DecodeError {}

const OUTPUT_BLOCK_OVERHEAD: usize = 3;

const INPUT_CHUNK_LEN: usize = 8;
const INPUT_BLOCK_LEN: usize = INPUT_CHUNK_LEN * 4;

const OUTPUT_CHUNK_LEN: usize = 5;
const OUTPUT_BLOCK_LEN: usize = OUTPUT_CHUNK_LEN * 4;

impl Encoding {
    /// Decodes valid base32-encoded data according to the configuration,
    /// returning the resulting raw bytes, and bubbling up any errors from
    /// the implementation.
    #[cfg(any(feature = "alloc", feature = "std", test))]
    #[inline]
    pub fn decode(&self, data: impl AsRef<[u8]>) -> Result<Vec<u8>, DecodeError> {
        let data = data.as_ref();

        let mut buf = vec![0; self.decoded_size(data.len())];
        let written = self.decode_to_slice(&mut buf, data)?;

        buf.truncate(written);

        Ok(buf)
    }

    /// Returns an estimate of how many bytes would be required to store the decoded form
    /// of the given amount of encoded bytes. It will sometimes overestimate how many are
    /// needed, but never underestimate. Panics if the result would overflow `usize`.
    pub fn decoded_size(&self, bytes: usize) -> usize {
        if self.pad.is_some() {
            (bytes / 8).checked_mul(5)
        } else {
            bytes.checked_mul(5).and_then(|n| Some(n / 8))
        }
        .expect("Overflow while calculating decoded length")
    }

    /// Takes a slice of encoded data and decodes it into
    /// the output slice according to the configuration.
    pub fn decode_to_slice(&self, output: &mut [u8], input: &[u8]) -> Result<usize, DecodeError> {
        let (remainder, mut chunks) = (
            input.len() % INPUT_CHUNK_LEN,
            input
                .len()
                .checked_add(INPUT_CHUNK_LEN - 1)
                .expect("Calculating amount of input chunks overflowed")
                / INPUT_CHUNK_LEN,
        );
        let trailing_bytes_to_skip = match remainder {
            // invalid unpadded input lengths ...
            1 | 3 | 6 => return Err(DecodeError::InvalidInputLength),
            // if any other length is present, we must ignore
            // the final block plus any bytes present in the
            // remainder
            _ => INPUT_CHUNK_LEN + remainder,
        };

        let mut output_index = 0;
        let mut input_index = 0;
        let decode_table = self.alpha.decode_table();

        {
            let fast_decode_bytes = input.len().saturating_sub(trailing_bytes_to_skip);

            if let Some(max_index) = fast_decode_bytes.checked_sub(INPUT_BLOCK_LEN) {
                while input_index <= max_index {
                    let output_block = &mut output
                        [output_index..(output_index + OUTPUT_BLOCK_LEN + OUTPUT_BLOCK_OVERHEAD)];
                    let input_block = &input[input_index..(input_index + INPUT_BLOCK_LEN)];

                    decode_chunk(
                        decode_table,
                        &mut output_block[0..],
                        &input_block[0..],
                        input_index,
                    )?;
                    decode_chunk(
                        decode_table,
                        &mut output_block[5..],
                        &input_block[8..],
                        input_index,
                    )?;
                    decode_chunk(
                        decode_table,
                        &mut output_block[10..],
                        &input_block[16..],
                        input_index,
                    )?;
                    decode_chunk(
                        decode_table,
                        &mut output_block[15..],
                        &input_block[24..],
                        input_index,
                    )?;

                    output_index += OUTPUT_BLOCK_LEN;
                    input_index += INPUT_BLOCK_LEN;
                    chunks -= 4;
                }
            }

            if let Some(max_index) = fast_decode_bytes.checked_sub(INPUT_CHUNK_LEN) {
                while input_index < max_index {
                    decode_chunk(
                        decode_table,
                        &mut output[output_index..(output_index + OUTPUT_CHUNK_LEN + 3)],
                        &input[input_index..(input_index + INPUT_CHUNK_LEN)],
                        input_index,
                    )?;

                    output_index += OUTPUT_CHUNK_LEN;
                    input_index += INPUT_CHUNK_LEN;
                    chunks -= 1;
                }
            }
        }

        for _ in 1..chunks {
            decode_chunk_precise(
                decode_table,
                &mut output[output_index..(output_index + OUTPUT_CHUNK_LEN)],
                &input[input_index..(input_index + INPUT_CHUNK_LEN)],
                input_index,
            )?;

            output_index += OUTPUT_CHUNK_LEN;
            input_index += INPUT_CHUNK_LEN;
        }

        debug_assert!(input.len() - input_index > 1 || input.is_empty());
        debug_assert!(input.len() - input_index <= INPUT_CHUNK_LEN);

        let mut quintets_leftover = 0;
        let mut data = 0;

        for (i, b) in input[input_index..].iter().enumerate() {
            if let Some(pad) = self.pad {
                if *b == pad {
                    break;
                }
            }

            let quintet = decode_table[*b as usize];
            if quintet == INVALID_BYTE {
                return Err(DecodeError::InvalidByte(input_index + i, *b));
            }
            data |= (quintet as u64) << (64 - (quintets_leftover + 1) * 5);
            quintets_leftover += 1;
        }

        let leftover_bits = match quintets_leftover {
            0 => 0,
            2 => 8,
            4 => 16,
            5 => 24,
            7 => 32,
            8 => 40,
            n => unreachable!("Invalid leftover quintet count: {}", n),
        };

        let mut leftover_bits_appended = 0;
        while leftover_bits_appended < leftover_bits {
            let selected_bits = (data >> (56 - leftover_bits_appended)) as u8;
            output[output_index] = selected_bits;
            output_index += 1;

            leftover_bits_appended += 8;
        }

        Ok(output_index)
    }
}

#[inline(always)]
fn decode_chunk(
    decode_table: &'static [u8; 256],
    output_chunk: &mut [u8],
    input_chunk: &[u8],
    starting_index: usize,
) -> Result<(), DecodeError> {
    let mut data: u64 = 0;

    let quintet = decode_table[input_chunk[0] as usize];
    if quintet == INVALID_BYTE {
        return Err(DecodeError::InvalidByte(starting_index, quintet));
    }
    data |= (quintet as u64) << 59;

    let quintet = decode_table[input_chunk[1] as usize];
    if quintet == INVALID_BYTE {
        return Err(DecodeError::InvalidByte(starting_index + 1, quintet));
    }
    data |= (quintet as u64) << 54;

    let quintet = decode_table[input_chunk[2] as usize];
    if quintet == INVALID_BYTE {
        return Err(DecodeError::InvalidByte(starting_index + 2, quintet));
    }
    data |= (quintet as u64) << 49;

    let quintet = decode_table[input_chunk[3] as usize];
    if quintet == INVALID_BYTE {
        return Err(DecodeError::InvalidByte(starting_index + 3, quintet));
    }
    data |= (quintet as u64) << 44;

    let quintet = decode_table[input_chunk[4] as usize];
    if quintet == INVALID_BYTE {
        return Err(DecodeError::InvalidByte(starting_index + 4, quintet));
    }
    data |= (quintet as u64) << 39;

    let quintet = decode_table[input_chunk[5] as usize];
    if quintet == INVALID_BYTE {
        return Err(DecodeError::InvalidByte(starting_index + 5, quintet));
    }
    data |= (quintet as u64) << 34;

    let quintet = decode_table[input_chunk[6] as usize];
    if quintet == INVALID_BYTE {
        return Err(DecodeError::InvalidByte(starting_index + 6, quintet));
    }
    data |= (quintet as u64) << 29;

    let quintet = decode_table[input_chunk[7] as usize];
    if quintet == INVALID_BYTE {
        return Err(DecodeError::InvalidByte(starting_index + 7, quintet));
    }
    data |= (quintet as u64) << 24;

    output_chunk[..8].copy_from_slice(&data.to_be_bytes());

    Ok(())
}

#[inline(always)]
fn decode_chunk_precise(
    decode_table: &'static [u8; 256],
    output_chunk: &mut [u8],
    input_chunk: &[u8],
    starting_index: usize,
) -> Result<(), DecodeError> {
    let mut buffer = [0; 8];

    decode_chunk(decode_table, &mut buffer, input_chunk, starting_index)?;
    output_chunk[0..5].copy_from_slice(&buffer[0..5]);

    Ok(())
}
