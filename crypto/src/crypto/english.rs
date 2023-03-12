// Scores some text. The higher the score,
// the more likely it is to be english
pub fn english_score(bytes: &[u8]) -> u32 {
    let mut score = 0;
    for byte in bytes {
        let char  = char::from(*byte);
        if char.is_ascii_alphabetic() || char == ' ' {
            score += 1;
        }
    }
    return score;
}
