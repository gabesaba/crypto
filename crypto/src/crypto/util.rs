use std::fs::File;
use std::io::{self, Read};

// Load utf-8 encoded challenge data as lines.
pub fn load_challenge_data(challenge: &str) -> Vec<String> {
    let path = format!("data/{}.txt", challenge);
    let file = File::open(path).unwrap();

    let mut buf = String::new();
    io::BufReader::new(file).read_to_string(&mut buf).unwrap();
    let mut lines = Vec::new();
    for line in buf.split('\n') {
        lines.push(String::from(line));
    }
    return lines;
}
