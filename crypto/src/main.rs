mod crypto;

#[cfg(test)]
mod tests {

    #[test]
    fn challenge_1() {
        use crate::crypto::bytes::base64::encode;
        use crate::crypto::bytes::hex::decode;
        let inp = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", encode(decode(inp).as_slice()))
    }

    #[test]
    fn challenge_2() {
        use crate::crypto::bytes::hex::{encode, decode};
        use crate::crypto::cypher::vec_xor;
        let inp1 = "1c0111001f010100061a024b53535009181c";
        let inp2 = "686974207468652062756c6c277320657965";
        let inp1 = decode(inp1);
        let inp2 = decode(inp2);

        assert_eq!("746865206b696420646f6e277420706c6179",
                   encode(vec_xor(&inp1, &inp2).as_slice()));
    }

    #[test]
    fn challenge_3() {
        use crate::crypto::cypher::char_xor;
        use crate::crypto::bytes::hex;
        use crate::crypto::util;

        let hex_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        let bytes = hex::decode(hex_str);
        let mut best_ct = 0;
        let mut best= "".to_owned();

        for i in 0..255 {
            let as_str = util::to_str(&char_xor(&bytes, i));
            let ct = util::alpha_ct(&as_str);
            if ct > best_ct {
                best_ct = ct;
                best = as_str;
            }
        }
        assert_eq!("Cooking MC\'s like a pound of bacon", best);
    }
}
