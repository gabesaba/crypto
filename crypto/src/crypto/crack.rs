use std::collections::HashSet;
use crate::crypto::score::{english_score, hamming_distance};
use crate::crypto::cypher::{char_xor, repeating_key_xor};

fn get_hamming_distance_for_segments(bytes: &[u8], key_size: usize, num_trials: usize) -> f64 {
    let mut distance = 0.0;
    let mut num_trials_completed: usize = 0;
    for trial in (0..num_trials * 2).step_by(2) {
        let lower = trial * key_size;
        let mid = (trial + 1) * key_size;
        let upper = (trial + 2) * key_size;

        // Make sure we didn't walk off the end of the array.
        if upper > bytes.len() {
            println!("Breaking!!!");
            break;
        }

        let slice1 = &bytes[lower..mid];
        let slice2 = &bytes[mid..upper];
        distance += hamming_distance(slice1, slice2) as f64;
        num_trials_completed += 1;
    }

    // Don't add key if we weren't able to score it. Also, since subsequent keys will be greater,
    // we can simply break.
    if num_trials_completed == 0 {
        return f64::MAX;
    }

    distance /= num_trials_completed as f64; // normalize by num trials.
    return distance;
}

fn get_key_sizes(bytes: &[u8]) -> Vec<(f64, usize)> {
    let mut minimum_distance = f64::MAX;
    let mut scores = Vec::new();

    for key_size in 2..40 {
        let mut distance = 0.0;
        distance += get_hamming_distance_for_segments(bytes, key_size, 10);
        distance /= key_size as f64; // normalize by key size.

        if distance < minimum_distance {
            minimum_distance = distance;
        }
        scores.push((distance, key_size))
    }
    scores.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    return scores;
}

fn transpose(bytes: &[u8], key_size: usize) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for i in 0..key_size {
        let mut row = Vec::new();
        for byte in bytes.iter().skip(i).step_by(key_size) {
            row.push(*byte);
        }
        out.push(row);
    }
    return out;
}

// Returns the key that's most likely to produce English text.
pub fn decode_single_char_xor(inp: &[u8]) -> u8 {
    let mut best_score = 0.0;
    let mut best_key = 0;
    for key in 0..255 {
        let decrypted = char_xor(inp, key);
        let score = english_score(&decrypted);
        if score > best_score {
            best_score = score;
            best_key = key;
        }
    }
    return best_key;
}

// Returns the best key.
pub fn break_repeating_key_xor(bytes: &[u8]) -> Vec<u8> {
    let key_sizes = get_key_sizes(bytes);

    let mut best_score = 0.0;
    let mut best_key = Vec::new();

    // try the 5 best key-lengths.
    for i in 0..5 {
        let (_, key_size) = key_sizes[i];
        let transposed = transpose(bytes, key_size);
        let mut key = Vec::new();
        for block in transposed {
            let chr = decode_single_char_xor(&block);
            key.push(chr);
        }
        let decrypted = repeating_key_xor(bytes, &key);
        let score = english_score(&decrypted);
        if score > best_score {
            best_key = key.clone();
            best_score = score;
        }
    }
    return best_key;
}

// Determine if there is a duplicate 16-byte segment in encrypted aes-ecb bytes.
pub fn aes_ecb_has_duplicate(bytes: &[u8]) -> bool {
    let mut duplicate = HashSet::new();

    let mut key: u128 = 0;
    let mut curr_size = 0;
    for byte in bytes {
        key += *byte as u128;
        curr_size += 8;

        if curr_size < 128 {
            key <<= 8;
            continue;
        }

        if duplicate.contains(&key) {
            return true;
        }
        duplicate.insert(key);
        key = 0;
        curr_size = 0;
    }
    return false;
}
