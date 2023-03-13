// Scores some text. The higher the score, the more likely it is to be English.
pub fn english_score(bytes: &[u8]) -> f64 {
    let mut score = 0.0;
    for byte in bytes {
        let char = char::from(*byte);
        if char.is_ascii_alphabetic() || char == ' ' {
            score += 1.0;
        }
    }
    return score / bytes.len() as f64;
}

// Take the hamming distance of two byte slices.
pub fn hamming_distance(s1: &[u8], s2: &[u8]) -> u32 {
    assert_eq!(s1.len(), s2.len());

    let mut dist = 0;
    for (c1, c2) in s1.iter().zip(s2) {
        dist += (c1 ^ c2).count_ones();
    }
    dist
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hamming_distance() {
        use crate::crypto::bytes::plaintext;
        use crate::crypto::score::hamming_distance;

        let s1 = &plaintext::decode("this is a test");
        let s2 = &plaintext::decode("wokka wokka!!!");
        let res = hamming_distance(s1, s2);

        assert_eq!(37, res);
    }
}
