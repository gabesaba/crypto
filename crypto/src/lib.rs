pub mod hex {
    pub fn encode(bytes: Vec<u8>) -> String {
        String::new()
    }

    pub fn decode(hex_str: &str) -> Vec<u8> {
        vec![]
    }
    fn to_char(byte: u8) -> char {
        'a'
    }

    fn from_char(c: char) -> u8 {
        match c {
            '0'..='9' => c as u8 - ('0' as u8),
            'A'..='F' => c as u8 - ('A' as u8),
            _ => panic!("Char {} is not in Hex alphabet", c)
        }
    }
}

pub mod base64 {
    pub fn encode(bytes: Vec<u8>) -> String {
        let mut b64_encoding = String::new();

        let mut curr_len = 0;
        let mut buf: u32 = 0;

        for byte in bytes {
            curr_len += 8;
            buf <<= 8;
            buf += byte as u32;

            while curr_len >= 6 {
                let shift_size = curr_len - 6;
                let val = (buf >> shift_size) as u8;
                b64_encoding.push(to_char(val));
                curr_len -= 6;
                buf >>= 6;
            }
        }
        if curr_len > 0 {
            assert!(curr_len == 2 || curr_len == 4);
            let shift_size = 6 - curr_len;
            buf <<= shift_size;
            b64_encoding.push(to_char(buf as u8));

            for _ in 0..curr_len / 2 {
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
                println!("Curr buffer is size {} with val {}", curr_len, buf);
            }

            if curr_len >= 8 {
                let shift_size = curr_len - 8;
                let val = (buf >> shift_size) as u8;
                out.push(val);
                println!("Pushing val {}", val);
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
            26..=51 => char::from('a' as u8 + byte),
            52..=61 => char::from('0' as u8 + byte),
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_deserialize_b64() {
        use crate::base64::decode;
        let inp = "SEVMTE8K";
        let out = decode(inp);
        assert_eq!(vec![72, 69, 76, 76, 79], out)
    }
}
