use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
use crate::crypto::cypher::char_xor;
use crate::crypto::english::english_score;

pub fn to_str(inp: &Vec<u8>) -> String {
    let mut out = String::new();
    for byte in inp {
        out.push(char::from(*byte));
    }
    out
}

// Load utf-8 encoded challenge data as a String
pub fn load_challenge_data(challenge: &str) -> String {
    let path = format!("data/{}.txt", challenge);
    let file = File::open(path).unwrap();

    let mut buf = String::new();
    io::BufReader::new(file).read_to_string(&mut buf);
    buf
}

// Returns the result of a char_xor that's
// most likely to be English, as well as its score
pub fn decode_single_char_xor(inp: &Vec<u8>) -> (String, usize){
    let mut best_score = 0;
    let mut best = String::new();
    for i in 0..255 {
            let as_str = to_str(&char_xor(&inp, i));
            let score = english_score(&as_str);
            if score > best_score {
                best_score = score;
                best = as_str;
            }
    }
    (best, best_score)
}

fn char_dist(c1: char, c2: char) -> u32 {
    let mismatching_bits = (c1 as u8) ^ (c2 as u8);

    // Could use u8.count_ones(), but since I'm a Rust newbie,
    // I'm biasing towards writing everything myself.
    let mut dist = 0;
    for shift in 0..8 {
        if mismatching_bits >> shift & 1 == 1 {
            dist += 1
        }
    }
    dist
}

pub fn hamming_distance(s1: &str, s2: &str) -> u32 {
    assert_eq!(s1.len(), s2.len());

    let mut dist = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        dist += char_dist(c1, c2)
    }
    dist
}
