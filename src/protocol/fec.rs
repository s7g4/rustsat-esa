#![allow(dead_code)]

use crate::error::RustSatError;
use heapless::Vec;

/// SECDED (Single Error Correction, Double Error Detection) Hamming(8,4) encoder.
/// Each 4-bit nibble of data is encoded into an 8-bit byte.
/// Thus, the encoded payload is exactly 2x the original size.
pub struct Hamming84;

impl Hamming84 {
    /// Encodes a single 4-bit nibble into an 8-bit Hamming(8,4) codeword.
    const fn encode_nibble(nibble: u8) -> u8 {
        let d1 = (nibble >> 3) & 1;
        let d2 = (nibble >> 2) & 1;
        let d3 = (nibble >> 1) & 1;
        let d4 = nibble & 1;

        // Parity bits
        let p1 = d1 ^ d2 ^ d4;
        let p2 = d1 ^ d3 ^ d4;
        let p3 = d2 ^ d3 ^ d4;

        // Construct 7-bit codeword: p1, p2, d1, p3, d2, d3, d4
        let code7 = (p1 << 6) | (p2 << 5) | (d1 << 4) | (p3 << 3) | (d2 << 2) | (d3 << 1) | d4;

        // Overall parity p4 for double error detection
        let mut p4 = 0;
        let mut temp = code7;
        while temp > 0 {
            p4 ^= temp & 1;
            temp >>= 1;
        }

        (code7 << 1) | p4
    }

    /// Decodes an 8-bit Hamming(8,4) codeword back to a 4-bit nibble.
    /// Returns Ok(nibble) if no errors or 1 error (which it corrects).
    /// Returns Err if 2 errors are detected.
    fn decode_nibble(byte: u8) -> Result<u8, RustSatError> {
        let _p4 = byte & 1;
        let code7 = byte >> 1;

        let p1 = (code7 >> 6) & 1;
        let p2 = (code7 >> 5) & 1;
        let d1 = (code7 >> 4) & 1;
        let p3 = (code7 >> 3) & 1;
        let d2 = (code7 >> 2) & 1;
        let d3 = (code7 >> 1) & 1;
        let d4 = code7 & 1;

        // Syndrome calculation
        let s1 = p1 ^ d1 ^ d2 ^ d4;
        let s2 = p2 ^ d1 ^ d3 ^ d4;
        let s3 = p3 ^ d2 ^ d3 ^ d4;

        let syndrome = (s3 << 2) | (s2 << 1) | s1;

        // Overall parity check
        let mut overall_parity = 0;
        let mut temp = byte;
        while temp > 0 {
            overall_parity ^= temp & 1;
            temp >>= 1;
        }

        let mut corrected7 = code7;

        if syndrome != 0 {
            if overall_parity == 0 {
                // Double error detected (syndrome non-zero but overall parity is even)
                return Err(RustSatError::DataCorruption);
            }

            // Single error correction
            let error_pos = match syndrome {
                1 => 6, // p1
                2 => 5, // p2
                3 => 4, // d1
                4 => 3, // p3
                5 => 2, // d2
                6 => 1, // d3
                7 => 0, // d4
                _ => return Err(RustSatError::DataCorruption),
            };
            corrected7 ^= 1 << error_pos;
        }

        let corrected_d1 = (corrected7 >> 4) & 1;
        let corrected_d2 = (corrected7 >> 2) & 1;
        let corrected_d3 = (corrected7 >> 1) & 1;
        let corrected_d4 = corrected7 & 1;

        Ok((corrected_d1 << 3) | (corrected_d2 << 2) | (corrected_d3 << 1) | corrected_d4)
    }

    /// Encodes a payload of bytes into a robust Hamming(8,4) byte array.
    pub fn encode<const CAP: usize>(data: &[u8]) -> Result<Vec<u8, CAP>, RustSatError> {
        let mut encoded = Vec::new();
        for &byte in data {
            let high_nibble = (byte >> 4) & 0x0F;
            let low_nibble = byte & 0x0F;

            if encoded.push(Self::encode_nibble(high_nibble)).is_err()
                || encoded.push(Self::encode_nibble(low_nibble)).is_err()
            {
                return Err(RustSatError::SystemError); // Capacity exceeded
            }
        }
        Ok(encoded)
    }

    /// Decodes a robust Hamming(8,4) payload back into the original bytes.
    /// Will correct any single bit-flip per nibble.
    pub fn decode<const CAP: usize>(encoded_data: &[u8]) -> Result<Vec<u8, CAP>, RustSatError> {
        if encoded_data.len() % 2 != 0 {
            return Err(RustSatError::InvalidFormat);
        }

        let mut decoded = Vec::new();
        for chunk in encoded_data.chunks_exact(2) {
            let high_nibble = Self::decode_nibble(chunk[0])?;
            let low_nibble = Self::decode_nibble(chunk[1])?;

            let original_byte = (high_nibble << 4) | low_nibble;
            if decoded.push(original_byte).is_err() {
                return Err(RustSatError::SystemError);
            }
        }
        Ok(decoded)
    }
}
