use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
use crate::crypto::cypher::char_xor;
use crate::crypto::english::english_score;

#[cfg(test)]
mod tests {
    use crate::crypto::bytes::plaintext;

    #[test]
    fn test_hamming_distance() {
        use crate::crypto::bytes::plaintext;
        use crate::crypto::util::hamming_distance;

        let s1 = &plaintext::decode("this is a test");
        let s2 = &plaintext::decode("wokka wokka!!!");
        let res = hamming_distance(s1, s2);

        assert_eq!(37, res);
    }
}

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

fn char_dist(c1: u8, c2: u8) -> u32 {
    let mismatching_bits = c1 ^ c2;

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

pub fn hamming_distance(s1: &[u8], s2: &[u8]) -> u32 {
    assert_eq!(s1.len(), s2.len());

    let mut dist = 0;
    for (c1, c2) in s1.iter().zip(s2) {
        dist += char_dist(*c1, *c2)
    }
    dist
}
