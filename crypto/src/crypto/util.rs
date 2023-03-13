use std::fs::File;
use std::io::{self, Read};

// Load utf-8 encoded challenge data as a String
pub fn load_challenge_data(challenge: &str) -> String {
    let path = format!("data/{}.txt", challenge);
    let file = File::open(path).unwrap();

    let mut buf = String::new();
    io::BufReader::new(file).read_to_string(&mut buf).unwrap();
    return buf;
}
