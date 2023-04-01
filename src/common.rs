use anyhow::{anyhow, Result};
use flate2::{Decompress, FlushDecompress};
use serde_json::{json, Value};
use std::str;

/// Convert keyword to json key or input for the game, or send the key directly. Returns
/// a [serde_json::Value] to be sent to DCSS Webtiles.
///
/// /// # Arguments
///
/// * `key` - A string slice of the key, or keyword, to be sent.
pub(crate) fn keys(key: &str) -> Value {
    match key {
        "tab" => json!({"msg": "key", "keycode": 9}),
        "ctrl_a" => json!({"msg": "key", "keycode": 1}),
        "esc" => json!({"msg": "key", "keycode": 27}),
        "Dir_N" => json!({"msg": "input", "text": "8"}),
        "Dir_NE" => json!({"msg": "input", "text": "9"}),
        "Dir_E" => json!({"msg": "input", "text": "6"}),
        "Dir_SE" => json!({"msg": "input", "text": "3"}),
        "Dir_S" => json!({"msg": "input", "text": "2"}),
        "Dir_SW" => json!({"msg": "input", "text": "1"}),
        "Dir_W" => json!({"msg": "input", "text": "4"}),
        "Dir_NW" => json!({"msg": "input", "text": "7"}),
        "Down" => json!({"msg": "input", "text": ">"}),
        "Up" => json!({"msg": "input", "text": "<"}),
        "enter" => json!({"msg": "input", "text": "\r"}),
        _ => json!({"msg": "input", "text": key}),
    }
}

/// Decompresses (deflate) a message from DCSS Webtiles. Returns a [serde_json::Value] object of the data.
///
/// # Arguments
///
/// * `decompressor` - A [flate2::Decompress] decompression object (Deflate) to decompress data received
/// * `compressed_msg` - the compressed message received from DCSS Webtiles.
pub(crate) fn deflate_to_json(
    decompressor: &mut Decompress,
    compressed_msg: &mut Vec<u8>,
) -> Result<Value> {
    // DCSS Removes 4 bytes that have to be re-added
    compressed_msg.append(&mut vec![0u8, 0, 255, 255]);

    // Decompress (Deflate)
    let bufsize = 1024 * 1024; // Needs a buffer size to work (1mb) - known to fail at 124kb (too small)
    let mut decompressed_bytes = Vec::with_capacity(bufsize); //capacity necessary, unclear why
    decompressor
        .decompress_vec(
            &compressed_msg[..],
            &mut decompressed_bytes,
            FlushDecompress::Sync,
        )
        .map_err(|e| anyhow!(e))?;
    let json_str = str::from_utf8(&decompressed_bytes).map_err(|e| anyhow!(e))?;

    let json_data: Value = serde_json::from_str(json_str).map_err(|e| anyhow!(e))?;

    Ok(json_data)
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::json;

    #[test]
    fn test_deflate_to_json() {
        // Set up the data #1 ("{\"msg\":\"ping\"}")
        let mut data_1_bin = vec![
            170, 86, 202, 45, 78, 47, 86, 178, 138, 174, 6, 49, 148, 172, 20, 148, 10, 50, 243,
            210, 149, 106, 99, 107, 1, 0,
        ];
        let data_1_solution = json!({"msgs":[{"msg": "ping"}]});

        // Set up the data #2 ("{\"msg\":\"lobby_clear\"}")
        // Data 2 depends on data 1 to work, can't be decompressed alone
        let mut data_2_bin = vec![
            170, 198, 144, 201, 201, 79, 74, 170, 140, 79, 206, 73, 77, 44, 82, 170, 213, 65, 23,
            206, 207, 45, 200, 73, 45, 73, 5, 105, 5, 0,
        ];
        let data_2_solution = json!({"msgs":[{"msg": "lobby_clear"}, {"msg": "lobby_complete"}]});

        // Set up decompressor
        let wbits = 15; // Windows bits fixed (goes to -15 in flate2 because of zlib_header = false)
        let mut decompressor = Decompress::new_with_window_bits(false, wbits);

        // Test it working correctly one after the other
        let decode_1 = deflate_to_json(&mut decompressor, &mut data_1_bin).unwrap();
        assert_eq!(decode_1, json!(data_1_solution));
        let decode_2 = deflate_to_json(&mut decompressor, &mut data_2_bin).unwrap();
        assert_eq!(decode_2, json!(data_2_solution));

        // Try only the second one, after resting the decompressor
        let wbits = 15; // Windows bits fixed (goes to -15 in flate2 because of zlib_header = false)
        let mut decompressor = Decompress::new_with_window_bits(false, wbits);
        let decode_2 = deflate_to_json(&mut decompressor, &mut data_2_bin);
        assert!(decode_2.is_err());
    }
}
