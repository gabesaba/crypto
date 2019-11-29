// Scores some text. The higher the score,
// the more likely it is to be english
pub fn english_score(s: &str) -> usize {
    s.chars().filter(|a| a.is_ascii()
        && (a.is_alphabetic() || a.is_whitespace())).count()
}
