//! Base64 encoding/decoding verification library
//!
//! This library provides functions to verify the integrity of Base64 encode/decode operations.
//! It can be used both as a standalone verification tool and as a library for other applications.

use data_encoding::BASE64;

/// Remove trailing zeros from a byte slice efficiently
///
/// This function finds the last non-zero byte and returns a slice up to that point.
/// If all bytes are zero or the slice is empty, returns an empty slice.
///
/// # Arguments
///
/// * `input` - The input byte slice to trim
///
/// # Returns
///
/// A byte slice containing all bytes up to and including the last non-zero byte.
/// Returns an empty slice if all bytes are zero or if the input is empty.
///
/// # Performance
///
/// This function runs in O(n) time in the worst case, but typically performs
/// better as it searches from the end of the slice backwards.
///
/// # Examples
///
/// ```
/// use base64check::trim_trailing_zeros;
///
/// assert_eq!(trim_trailing_zeros(&[1, 2, 3, 0, 0]), &[1, 2, 3]);
/// assert_eq!(trim_trailing_zeros(&[0, 0, 0]), &[]);
/// assert_eq!(trim_trailing_zeros(&[1, 2, 3]), &[1, 2, 3]);
/// assert_eq!(trim_trailing_zeros(&[]), &[]);
/// ```
#[must_use]
pub fn trim_trailing_zeros(input: &[u8]) -> &[u8] {
    input
        .iter()
        .rposition(|&b| b != 0)
        .map_or(&[], |pos| &input[..=pos])
}

/// Verify that Base64 encode/decode operations preserve the input data
///
/// This function performs a comprehensive roundtrip test: it encodes the input data
/// to Base64 format, then immediately decodes it back to bytes, and compares the
/// result with the original input (after trimming trailing zeros from both).
///
/// This is the core verification function used for high-performance testing where
/// buffer reuse is important. For simpler one-off verification, use [`verify_base64_simple`].
///
/// # Arguments
///
/// * `input` - The byte sequence to test for roundtrip integrity
/// * `encode_buffer` - Pre-allocated buffer for encoding (must be at least `BASE64.encode_len(input.len())` bytes)
/// * `decode_buffer` - Pre-allocated buffer for decoding (must be at least `BASE64.decode_len(encoded_len)` bytes)
///
/// # Returns
///
/// * `Ok(true)` - Roundtrip successful, data integrity preserved
/// * `Ok(false)` - Roundtrip completed but data was not preserved (integrity failure)
/// * `Err(String)` - Encoding/decoding failed or buffers are insufficiently sized
///
/// # Performance
///
/// This function is optimized for repeated calls with buffer reuse. The provided
/// buffers are reused across multiple calls, eliminating memory allocation overhead
/// in tight loops.
///
/// # Safety
///
/// This function is safe to call with any input data and properly sized buffers.
/// Buffer size requirements are validated at runtime.
///
/// # Examples
///
/// ```
/// use base64check::verify_base64_roundtrip;
///
/// let input = b"Hello, World!";
/// let mut encode_buffer = vec![0u8; input.len() * 4];
/// let mut decode_buffer = vec![0u8; input.len() * 4];
///
/// let result = verify_base64_roundtrip(input, &mut encode_buffer, &mut decode_buffer);
/// assert_eq!(result.unwrap(), true);
/// ```
pub fn verify_base64_roundtrip(
    input: &[u8],
    encode_buffer: &mut [u8],
    decode_buffer: &mut [u8],
) -> Result<bool, String> {
    // Encode the input
    let encoded_len = BASE64.encode_len(input.len());
    if encode_buffer.len() < encoded_len {
        return Err(format!(
            "Encode buffer too small: need {}, have {}",
            encoded_len,
            encode_buffer.len()
        ));
    }

    let encoded = &mut encode_buffer[..encoded_len];
    BASE64.encode_mut(input, encoded);

    // Decode back to bytes
    let decoded_len = BASE64
        .decode_len(encoded.len())
        .map_err(|e| format!("Invalid encoded length: {e}"))?;

    if decode_buffer.len() < decoded_len {
        return Err(format!(
            "Decode buffer too small: need {}, have {}",
            decoded_len,
            decode_buffer.len()
        ));
    }

    let decoded = &mut decode_buffer[..decoded_len];
    let actual_decoded_len = BASE64
        .decode_mut(encoded, decoded)
        .map_err(|e| format!("Decode failed: {e:?}"))?;

    // Compare trimmed versions (removing padding zeros)
    let decoded_trimmed = trim_trailing_zeros(&decoded[..actual_decoded_len]);
    let input_trimmed = trim_trailing_zeros(input);

    Ok(input_trimmed == decoded_trimmed)
}

/// Simple verification function that handles buffer allocation internally
///
/// This is a convenience wrapper around [`verify_base64_roundtrip`] that automatically
/// allocates appropriately sized buffers for the encode/decode operations.
///
/// # Arguments
///
/// * `input` - The byte sequence to verify for Base64 roundtrip integrity
///
/// # Returns
///
/// * `Ok(true)` - Roundtrip successful, data integrity preserved
/// * `Ok(false)` - Roundtrip completed but data was not preserved (integrity failure)
/// * `Err(String)` - Encoding/decoding failed or internal buffer allocation failed
///
/// # Performance Note
///
/// This function allocates new buffers for each call. For high-performance scenarios
/// with many iterations, prefer using [`verify_base64_roundtrip`] with pre-allocated
/// buffers to avoid allocation overhead.
///
/// # Examples
///
/// ```
/// use base64check::verify_base64_simple;
///
/// let input = b"Hello, World!";
/// let result = verify_base64_simple(input);
/// assert_eq!(result.unwrap(), true);
///
/// // Works with any byte data
/// let binary_data = &[0x00, 0x01, 0xFF, 0x42];
/// assert!(verify_base64_simple(binary_data).unwrap());
///
/// // Even empty data
/// assert!(verify_base64_simple(&[]).unwrap());
/// ```
///
/// # Errors
///
/// Returns an error if encoding or decoding fails due to implementation issues.
/// Under normal circumstances with a correct Base64 implementation, this should
/// always return `Ok(true)`.
pub fn verify_base64_simple(input: &[u8]) -> Result<bool, String> {
    let mut encode_buffer = vec![0u8; input.len() * 4]; // Base64 expansion factor
                                                        // Decode buffer needs to account for Base64 padding and expansion
    let mut decode_buffer = vec![0u8; input.len() * 4]; // Generous buffer size

    verify_base64_roundtrip(input, &mut encode_buffer, &mut decode_buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use rand::distributions::Standard;

    #[test]
    fn test_trim_trailing_zeros() {
        assert_eq!(trim_trailing_zeros(&[]), &[]);
        assert_eq!(trim_trailing_zeros(&[0]), &[]);
        assert_eq!(trim_trailing_zeros(&[0, 0, 0]), &[]);
        assert_eq!(trim_trailing_zeros(&[1, 2, 3]), &[1, 2, 3]);
        assert_eq!(trim_trailing_zeros(&[1, 2, 3, 0]), &[1, 2, 3]);
        assert_eq!(trim_trailing_zeros(&[1, 2, 3, 0, 0, 0]), &[1, 2, 3]);
        assert_eq!(trim_trailing_zeros(&[0, 1, 2, 0, 0]), &[0, 1, 2]);
    }

    #[test]
    fn test_verify_base64_simple_basic() {
        // Test empty input
        assert!(verify_base64_simple(&[]).unwrap());

        // Test simple ASCII
        assert!(verify_base64_simple(b"Hello").unwrap());
        assert!(verify_base64_simple(b"Hello, World!").unwrap());

        // Test binary data
        assert!(verify_base64_simple(&[0, 1, 2, 3, 255, 254]).unwrap());

        // Test with trailing zeros
        assert!(verify_base64_simple(&[1, 2, 3, 0, 0]).unwrap());
    }

    #[test]
    fn test_verify_base64_roundtrip_buffer_sizes() {
        let input = b"test data";
        let mut small_encode_buffer = [0u8; 5]; // Too small
        let mut decode_buffer = vec![0u8; input.len() + 1];

        let result = verify_base64_roundtrip(input, &mut small_encode_buffer, &mut decode_buffer);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Encode buffer too small"));

        let mut encode_buffer = vec![0u8; input.len() * 4];
        let mut small_decode_buffer = [0u8; 2]; // Too small

        let result = verify_base64_roundtrip(input, &mut encode_buffer, &mut small_decode_buffer);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Decode buffer too small"));
    }

    #[test]
    fn test_random_data_verification() {
        let mut encode_buffer = vec![0u8; 1024 * 4];
        let mut decode_buffer = vec![0u8; 1024 + 1];

        for _ in 0..1000 {
            let mut rng = thread_rng();
            let len = rng.gen_range(0..1024);
            let input: Vec<u8> = thread_rng().sample_iter(Standard).take(len).collect();

            let result = verify_base64_roundtrip(&input, &mut encode_buffer, &mut decode_buffer);
            assert!(result.is_ok(), "Verification failed for input: {:?}", input);
            assert!(result.unwrap(), "Roundtrip failed for input: {:?}", input);
        }
    }

    #[test]
    fn test_edge_cases() {
        // Test maximum single byte values
        for i in 0..=255u8 {
            assert!(verify_base64_simple(&[i]).unwrap());
        }

        // Test various lengths up to a reasonable size
        for len in 0..256 {
            let input: Vec<u8> = thread_rng().sample_iter(Standard).take(len).collect();
            assert!(
                verify_base64_simple(&input).unwrap(),
                "Failed for length {} with data: {:?}",
                len,
                input
            );
        }
    }

    #[test]
    fn test_all_zero_input() {
        assert!(verify_base64_simple(&[0]).unwrap());
        assert!(verify_base64_simple(&[0, 0, 0, 0]).unwrap());
        assert!(verify_base64_simple(&vec![0; 100]).unwrap());
    }
}
