pub fn alpha_ct(s: &str) -> usize {
    s.chars().filter(|a| a.is_ascii() && a.is_alphabetic()).count()
}

pub fn to_str(inp: &Vec<u8>) -> String {
    let mut out = String::new();
    for byte in inp {
        out.push(char::from(*byte));
    }
    out
}
