pub mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        let mut out = String::new();
        for byte in bytes {
            let c1 = to_char(byte >> 4);
            let c2 = to_char(byte & 0b1111);
            out.push(c1);
            out.push(c2);
        }
        out
    }

    pub fn decode(hex_str: &str) -> Vec<u8> {
        let mut out = Vec::new();
        for (c1, c2) in hex_str.chars().step_by(2)
            .zip(hex_str.chars().skip(1).step_by(2)) {
            let mut byte = 0;
            byte += from_char(c1) << 4;
            byte += from_char(c2);
            out.push(byte)
        }
        out
    }

    fn to_char(byte: u8) -> char {
        match byte {
            0..=9 => char::from('0' as u8 + byte),
            10..=15 => char::from('a' as u8 + byte - 10),
            _ => panic!("Invalid input {}", byte)
        }
    }

    fn from_char(c: char) -> u8 {
        match c {
            '0'..='9' => c as u8 - ('0' as u8),
            'a'..='f' => c as u8 - ('a' as u8) + 10,
            _ => panic!("Char {} is not in Hex alphabet", c)
        }
    }
}

pub mod base64 {
    pub fn encode(bytes: &[u8]) -> String {
        let mut b64_encoding = String::new();

        let mut curr_len = 0;
        let mut buf: u32 = 0;

        for byte in bytes {
            curr_len += 8;
            buf <<= 8;
            buf += *byte as u32;
            while curr_len >= 6 {
                let shift_size = curr_len - 6;
                let val = (buf >> shift_size) as u8;
                b64_encoding.push(to_char(val));
                buf -= (val as u32) << shift_size;
                curr_len -= 6;
            }
        }

        // Deal with final quantum when not integral multiple of 24 bits.
        if curr_len > 0 {
            assert!(curr_len == 2 || curr_len == 4);

            buf <<= 6 - curr_len;
            b64_encoding.push(to_char(buf as u8));

            // 8 bit final quantum
            if curr_len == 2 {

                b64_encoding.push_str("==");
            }
            // 16 bit final quantum
            else if curr_len == 4 {

                b64_encoding.push('=');
            }
        }
        b64_encoding
    }

    pub fn decode(b64_str: &str) -> Vec<u8> {
        let mut out = Vec::new();

        let mut curr_len = 0;
        let mut buf: u32 = 0;
        for c in b64_str.chars() {
            if let '=' = c {} else {
                let enc = from_char(c);
                buf <<= 6;
                buf += u32::from(enc);
                curr_len += 6;

            }

            if curr_len >= 8 {
                let shift_size = curr_len - 8;
                let val = (buf >> shift_size) as u8;
                out.push(val);
                buf >>= 8;
                curr_len -= 8;
            }
        }
        println!("Curr len {}", curr_len);
        return out;
    }

    fn to_char(byte: u8) -> char {
        match byte {
            0..=25 => char::from('A' as u8 + byte),
            26..=51 => char::from('a' as u8 + byte - 26),
            52..=61 => char::from('0' as u8 + byte - 52),
            62 => '+',
            63 => '/',
            _ => panic!()
        }
    }

    fn from_char(c: char) -> u8 {
        match c {
            'A'..='Z' => c as u8 - ('A' as u8),
            'a'..='z' => c as u8 - ('a' as u8) + 26,
            '0'..='9' => c as u8 - ('0' as u8) + 52,
            '+' => 62,
            '/' => 63,
            _ => panic!()
        }
    }
}

fn xor(v1: &Vec<u8>, v2: &Vec<u8>) -> Vec<u8> {
    assert_eq!(v1.len(), v2.len());

    let mut out = Vec::new();
    for (b1, b2) in v1.iter().zip(v2.iter()) {
        out.push(b1 ^ b2)
    }
    out
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_edge() {
        use crate::base64::encode;
        let inp = b"";
        let out = encode(inp);
        assert_eq!("", out)
    }

    #[test]
    fn test_encode_b64() {
        use crate::base64::encode;
        assert_eq!("", encode(b""));
        assert_eq!("QQ==", encode(b"A"));
        assert_eq!("Wg==", encode(b"Z"));
        assert_eq!("SEVMTE8=", encode(b"HELLO"));
        assert_eq!("SGVsbG8gV29ybGQxIFpaWg==", encode(b"Hello World1 ZZZ"));

    }
    #[test]
    fn test_decode_b64() {
        use crate::base64::decode;
        let inp = "SEVMTE8=";
        let out = decode(inp);
        assert_eq!(vec![72, 69, 76, 76, 79], out)
    }

    #[test]
    fn test_encode_decode_hex() {
        use crate::hex::{decode, encode};

        let inp = vec![1, 5, 4, 3];

        assert_eq!(inp, decode(
            &encode(inp.as_slice())).as_slice())
    }

    #[test]
    fn challenge_1() {
        use crate::base64::encode;
        use crate::hex::decode;
        let inp = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", encode(decode(inp).as_slice()))
    }

    #[test]
    fn challenge_2() {
        use crate::hex::{encode, decode};
        use crate::xor;
        let inp1 = "1c0111001f010100061a024b53535009181c";
        let inp2 = "686974207468652062756c6c277320657965";
        let inp1 = decode(inp1);
        let inp2 = decode(inp2);

        assert_eq!("746865206b696420646f6e277420706c6179",
                   encode(xor(&inp1, &inp2).as_slice()));
    }
}
