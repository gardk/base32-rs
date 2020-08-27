#[cfg(any(feature = "alloc", feature = "std", test))]
use alloc::{string::String, vec};

use crate::Encoding;

/// Encodes the input bytes using the standard base32 format
/// ```
/// assert_eq!(base32::encode("foobar"), "MZXW6YTBOI======");
/// ```
#[cfg(any(feature = "alloc", feature = "std", test))]
#[inline]
pub fn encode(data: impl AsRef<[u8]>) -> String {
    crate::STANDARD.encode(data)
}

// Each fast loop reads four 40-bit (5 bytes) blocks of input as u64. So we
// need 24 bits (3 bytes) of trailing information available each iteration.
const INPUT_BLOCK_OVERHEAD: usize = 3;

const INPUT_CHUNK_LEN: usize = 5;
const INPUT_BLOCK_LEN: usize = INPUT_CHUNK_LEN * 4;

const OUTPUT_CHUNK_LEN: usize = 8;
const OUTPUT_BLOCK_LEN: usize = OUTPUT_CHUNK_LEN * 4;

impl Encoding {
    /// Encodes arbitrary input bytes according to the
    /// chosen configuration, returning it as a `String`.
    #[cfg(any(feature = "alloc", feature = "std", test))]
    #[inline]
    pub fn encode(&self, data: impl AsRef<[u8]>) -> String {
        let data = data.as_ref();

        let mut buf = vec![0; self.encoded_size(data.len())];
        let mut written = self.encode_to_slice(&mut buf, data);
        written += self.add_padding(&mut buf[written..], data.len());

        debug_assert_eq!(written, buf.len());

        // SAFETY: Since all possible encoding tables are defined statically with valid
        //         ASCII characters, the encoding procedure can't produce invalid UTF-8
        unsafe { String::from_utf8_unchecked(buf) }
    }

    /// Calculates the bytes required to store the encoded form of an
    /// amount of bytes. Panics if the result would overflow `usize`.
    #[inline]
    pub fn encoded_size(&self, bytes: usize) -> usize {
        if self.pad.is_some() {
            bytes
                .checked_add(4)
                .map(|n| n / 5)
                .and_then(|n| n.checked_mul(8))
        } else {
            bytes
                .checked_mul(8)
                .and_then(|n| n.checked_add(4))
                .map(|n| n / 5)
        }
        .expect("Overflow while calculating encoded length")
    }

    /// Takes a slice of arbitrary bytes and encodes it according to the
    /// configuration, writing the resulting data into the output slice.
    pub fn encode_to_slice(&self, output: &mut [u8], input: &[u8]) -> usize {
        let mut output_index = 0;
        let mut input_index = 0;
        let encode_table = self.alpha.encode_table();

        let last_fast_index = input
            .len()
            .saturating_sub(INPUT_BLOCK_LEN + INPUT_BLOCK_OVERHEAD);

        if last_fast_index > 0 {
            while input_index <= last_fast_index {
                let output_block = &mut output[output_index..output_index + OUTPUT_BLOCK_LEN];
                let input_block =
                    &input[input_index..input_index + INPUT_BLOCK_LEN + INPUT_BLOCK_OVERHEAD];

                let batch = [
                    read_u64(&input_block[0..]),
                    read_u64(&input_block[5..]),
                    read_u64(&input_block[10..]),
                    read_u64(&input_block[15..]),
                ];

                const LOW_FIVE_BITS: u64 = 0b11111;

                output_block[0] = encode_table[((batch[0] >> 59) & LOW_FIVE_BITS) as usize];
                output_block[1] = encode_table[((batch[0] >> 54) & LOW_FIVE_BITS) as usize];
                output_block[2] = encode_table[((batch[0] >> 49) & LOW_FIVE_BITS) as usize];
                output_block[3] = encode_table[((batch[0] >> 44) & LOW_FIVE_BITS) as usize];
                output_block[4] = encode_table[((batch[0] >> 39) & LOW_FIVE_BITS) as usize];
                output_block[5] = encode_table[((batch[0] >> 34) & LOW_FIVE_BITS) as usize];
                output_block[6] = encode_table[((batch[0] >> 29) & LOW_FIVE_BITS) as usize];
                output_block[7] = encode_table[((batch[0] >> 24) & LOW_FIVE_BITS) as usize];

                output_block[8] = encode_table[((batch[1] >> 59) & LOW_FIVE_BITS) as usize];
                output_block[9] = encode_table[((batch[1] >> 54) & LOW_FIVE_BITS) as usize];
                output_block[10] = encode_table[((batch[1] >> 49) & LOW_FIVE_BITS) as usize];
                output_block[11] = encode_table[((batch[1] >> 44) & LOW_FIVE_BITS) as usize];
                output_block[12] = encode_table[((batch[1] >> 39) & LOW_FIVE_BITS) as usize];
                output_block[13] = encode_table[((batch[1] >> 34) & LOW_FIVE_BITS) as usize];
                output_block[14] = encode_table[((batch[1] >> 29) & LOW_FIVE_BITS) as usize];
                output_block[15] = encode_table[((batch[1] >> 24) & LOW_FIVE_BITS) as usize];

                output_block[16] = encode_table[((batch[2] >> 59) & LOW_FIVE_BITS) as usize];
                output_block[17] = encode_table[((batch[2] >> 54) & LOW_FIVE_BITS) as usize];
                output_block[18] = encode_table[((batch[2] >> 49) & LOW_FIVE_BITS) as usize];
                output_block[19] = encode_table[((batch[2] >> 44) & LOW_FIVE_BITS) as usize];
                output_block[20] = encode_table[((batch[2] >> 39) & LOW_FIVE_BITS) as usize];
                output_block[21] = encode_table[((batch[2] >> 34) & LOW_FIVE_BITS) as usize];
                output_block[22] = encode_table[((batch[2] >> 29) & LOW_FIVE_BITS) as usize];
                output_block[23] = encode_table[((batch[2] >> 24) & LOW_FIVE_BITS) as usize];

                output_block[24] = encode_table[((batch[3] >> 59) & LOW_FIVE_BITS) as usize];
                output_block[25] = encode_table[((batch[3] >> 54) & LOW_FIVE_BITS) as usize];
                output_block[26] = encode_table[((batch[3] >> 49) & LOW_FIVE_BITS) as usize];
                output_block[27] = encode_table[((batch[3] >> 44) & LOW_FIVE_BITS) as usize];
                output_block[28] = encode_table[((batch[3] >> 39) & LOW_FIVE_BITS) as usize];
                output_block[29] = encode_table[((batch[3] >> 34) & LOW_FIVE_BITS) as usize];
                output_block[30] = encode_table[((batch[3] >> 29) & LOW_FIVE_BITS) as usize];
                output_block[31] = encode_table[((batch[3] >> 24) & LOW_FIVE_BITS) as usize];

                output_index += OUTPUT_BLOCK_LEN;
                input_index += INPUT_BLOCK_LEN;
            }
        }

        // I know this part is very ugly, but I'm in no mood to do anything about it >:(

        const LOW_FIVE_BITS: u8 = 0b11111;

        while input.len() - input_index >= 5 {
            let output_chunk = &mut output[output_index..output_index + OUTPUT_CHUNK_LEN];
            let input_chunk = &input[input_index..input_index + INPUT_CHUNK_LEN];

            output_chunk[0] = encode_table[(input_chunk[0] >> 3) as usize];
            output_chunk[1] = encode_table
                [((input_chunk[0] << 2 | input_chunk[1] >> 6) & LOW_FIVE_BITS) as usize];
            output_chunk[2] = encode_table[((input_chunk[1] >> 1) & LOW_FIVE_BITS) as usize];
            output_chunk[3] = encode_table
                [((input_chunk[1] << 4 | input_chunk[2] >> 4) & LOW_FIVE_BITS) as usize];
            output_chunk[4] = encode_table
                [((input_chunk[2] << 1 | input_chunk[3] >> 7) & LOW_FIVE_BITS) as usize];
            output_chunk[5] = encode_table[((input_chunk[3] >> 2) & LOW_FIVE_BITS) as usize];
            output_chunk[6] = encode_table
                [((input_chunk[3] << 3 | input_chunk[4] >> 5) & LOW_FIVE_BITS) as usize];
            output_chunk[7] = encode_table[(input_chunk[4] & LOW_FIVE_BITS) as usize];

            output_index += OUTPUT_CHUNK_LEN;
            input_index += INPUT_CHUNK_LEN;
        }

        match input.len() - input_index {
            4 => {
                let output_chunk = &mut output[output_index..output_index + 7];
                let input_chunk = &input[input_index..input_index + 4];

                output_chunk[0] = encode_table[(input_chunk[0] >> 3) as usize];
                output_chunk[1] = encode_table
                    [((input_chunk[0] << 2 | input_chunk[1] >> 6) & LOW_FIVE_BITS) as usize];
                output_chunk[2] = encode_table[((input_chunk[1] >> 1) & LOW_FIVE_BITS) as usize];
                output_chunk[3] = encode_table
                    [((input_chunk[1] << 4 | input_chunk[2] >> 4) & LOW_FIVE_BITS) as usize];
                output_chunk[4] = encode_table
                    [((input_chunk[2] << 1 | input_chunk[3] >> 7) & LOW_FIVE_BITS) as usize];
                output_chunk[5] = encode_table[((input_chunk[3] >> 2) & LOW_FIVE_BITS) as usize];
                output_chunk[6] = encode_table[((input_chunk[3] << 3) & LOW_FIVE_BITS) as usize];

                output_index += 7;
            }
            3 => {
                let output_chunk = &mut output[output_index..output_index + 5];
                let input_chunk = &input[input_index..input_index + 3];

                output_chunk[0] = encode_table[(input_chunk[0] >> 3) as usize];
                output_chunk[1] = encode_table
                    [((input_chunk[0] << 2 | input_chunk[1] >> 6) & LOW_FIVE_BITS) as usize];
                output_chunk[2] = encode_table[((input_chunk[1] >> 1) & LOW_FIVE_BITS) as usize];
                output_chunk[3] = encode_table
                    [((input_chunk[1] << 4 | input_chunk[2] >> 4) & LOW_FIVE_BITS) as usize];
                output_chunk[4] = encode_table[((input_chunk[2] << 1) & LOW_FIVE_BITS) as usize];

                output_index += 5;
            }
            2 => {
                let output_chunk = &mut output[output_index..output_index + 4];
                let input_chunk = &input[input_index..input_index + 2];

                output_chunk[0] = encode_table[(input_chunk[0] >> 3) as usize];
                output_chunk[1] = encode_table
                    [((input_chunk[0] << 2 | input_chunk[1] >> 6) & LOW_FIVE_BITS) as usize];
                output_chunk[2] = encode_table[((input_chunk[1] >> 1) & LOW_FIVE_BITS) as usize];
                output_chunk[3] = encode_table[((input_chunk[1] << 4) & LOW_FIVE_BITS) as usize];

                output_index += 4;
            }
            1 => {
                let output_chunk = &mut output[output_index..output_index + 2];
                let input_byte = input[input_index];

                output_chunk[0] = encode_table[(input_byte >> 3) as usize];
                output_chunk[1] = encode_table[((input_byte << 2) & LOW_FIVE_BITS) as usize];

                output_index += 2;
            }
            0 => {}
            _ => unreachable!(
                "Input data should never have more than four bytes left at remainder stage"
            ),
        }

        output_index
    }

    /// Writes padding characters to the output slice according to the
    /// input length, does nothing if the configuration disables padding. You
    /// don't need to call this function unless directly calling `encode_to_slice`.
    #[inline]
    pub fn add_padding(&self, output: &mut [u8], input_len: usize) -> usize {
        if let Some(pad) = self.pad {
            let mut output_index = 0;
            match input_len % 5 {
                4 => {
                    output[output_index] = pad;
                    output_index += 1;
                }
                3 => {
                    output[output_index] = pad;
                    output[output_index + 1] = pad;
                    output[output_index + 2] = pad;
                    output_index += 3;
                }
                2 => {
                    output[output_index] = pad;
                    output[output_index + 1] = pad;
                    output[output_index + 2] = pad;
                    output[output_index + 3] = pad;
                    output_index += 4;
                }
                1 => {
                    output[output_index] = pad;
                    output[output_index + 1] = pad;
                    output[output_index + 2] = pad;
                    output[output_index + 3] = pad;
                    output[output_index + 4] = pad;
                    output[output_index + 5] = pad;
                    output_index += 6;
                }
                _ => {}
            }
            output_index
        } else {
            0
        }
    }
}

#[inline(always)]
fn read_u64(bytes: &[u8]) -> u64 {
    use core::convert::TryInto;
    u64::from_be_bytes(bytes[..8].try_into().unwrap())
}
