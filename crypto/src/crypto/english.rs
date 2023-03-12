// Scores some text. The higher the score,
// the more likely it is to be english
pub fn english_score(bytes: &[u8]) -> f64 {
    let mut score = 0.0;
    for byte in bytes {
        let char  = char::from(*byte);
        if char.is_ascii_alphabetic() || char == ' ' {
            score += 1.0;
        }
    }
    return score / bytes.len() as f64;
}
