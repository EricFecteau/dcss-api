use crate::Error;
use flate2::{Decompress, FlushDecompress};
use serde_json::{json, Value};
use std::result::Result;
use std::str;

/// Convert keyword to json key or input for the game, or send the key directly. Returns
/// a [serde_json::Value] to be sent to DCSS Webtiles.
///
/// Special keys:
/// * CTRL+char = `key_ctrl_a` to `key_ctrl_z`
/// * Special chars = `key_tab`, `key_esc` and `key_enter`
/// * Cardinal directions: `key_dir_n`, `key_dir_ne`, `key_dir_e`, `key_dir_se`,
///   `key_dir_s`, `key_dir_sw`, `key_dir_w` and `key_dir_nw`
/// * Stairs: `key_stair_down` and `key_stair_up`
///
/// # Arguments
///
/// * `key` - A string slice of the key, or keyword, to be sent.
pub(crate) fn keys(key: &str) -> Value {
    match key {
        "key_ctrl_a" => json!({"msg": "key", "keycode": 1}),
        "key_ctrl_b" => json!({"msg": "key", "keycode": 2}),
        "key_ctrl_c" => json!({"msg": "key", "keycode": 3}),
        "key_ctrl_d" => json!({"msg": "key", "keycode": 4}),
        "key_ctrl_e" => json!({"msg": "key", "keycode": 5}),
        "key_ctrl_f" => json!({"msg": "key", "keycode": 6}),
        "key_ctrl_g" => json!({"msg": "key", "keycode": 7}),
        "key_ctrl_h" => json!({"msg": "key", "keycode": 8}),
        "key_ctrl_i" => json!({"msg": "key", "keycode": 9}),
        "key_tab" => json!({"msg": "key", "keycode": 9}),
        "key_ctrl_j" => json!({"msg": "key", "keycode": 10}),
        "key_ctrl_k" => json!({"msg": "key", "keycode": 11}),
        "key_ctrl_l" => json!({"msg": "key", "keycode": 12}),
        "key_ctrl_m" => json!({"msg": "key", "keycode": 13}),
        "key_ctrl_n" => json!({"msg": "key", "keycode": 14}),
        "key_ctrl_o" => json!({"msg": "key", "keycode": 15}),
        "key_ctrl_p" => json!({"msg": "key", "keycode": 16}),
        "key_ctrl_q" => json!({"msg": "key", "keycode": 17}),
        "key_ctrl_r" => json!({"msg": "key", "keycode": 18}),
        "key_ctrl_s" => json!({"msg": "key", "keycode": 19}),
        "key_ctrl_t" => json!({"msg": "key", "keycode": 20}),
        "key_ctrl_u" => json!({"msg": "key", "keycode": 21}),
        "key_ctrl_v" => json!({"msg": "key", "keycode": 22}),
        "key_ctrl_w" => json!({"msg": "key", "keycode": 23}),
        "key_ctrl_x" => json!({"msg": "key", "keycode": 24}),
        "key_ctrl_y" => json!({"msg": "key", "keycode": 25}),
        "key_ctrl_z" => json!({"msg": "key", "keycode": 26}),
        "key_esc" => json!({"msg": "key", "keycode": 27}),
        "key_dir_n" => json!({"msg": "input", "text": "8"}),
        "key_dir_ne" => json!({"msg": "input", "text": "9"}),
        "key_dir_e" => json!({"msg": "input", "text": "6"}),
        "key_dir_se" => json!({"msg": "input", "text": "3"}),
        "key_dir_s" => json!({"msg": "input", "text": "2"}),
        "key_dir_sw" => json!({"msg": "input", "text": "1"}),
        "key_dir_w" => json!({"msg": "input", "text": "4"}),
        "key_dir_nw" => json!({"msg": "input", "text": "7"}),
        "key_stair_down" => json!({"msg": "input", "text": ">"}),
        "key_stair_up" => json!({"msg": "input", "text": "<"}),
        "key_enter" => json!({"msg": "input", "text": "\r"}),
        _ => json!({"msg": "input", "text": key}),
    }
}

/// Decompresses (deflate) a message from DCSS Webtiles. Returns a [serde_json::Value] object of the data.
///
/// # Arguments
///
/// * `decompressor` - A [flate2::Decompress] decompression object (Deflate) to decompress data received.
/// * `compressed_msg` - the compressed message received from DCSS Webtiles.
pub(crate) fn deflate_to_json(
    decompressor: &mut Decompress,
    compressed_msg: &mut Vec<u8>,
) -> Result<Value, Error> {
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
        .map_err(Error::Decompress)?;

    // Convert to a string slice
    let json_str = str::from_utf8(&decompressed_bytes).map_err(Error::Utf8)?;

    // Convert to json Value
    let json_data: Value = serde_json::from_str(json_str).map_err(Error::JSON)?;

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
