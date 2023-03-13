mod crypto;

#[cfg(test)]
mod tests {
    use crate::crypto::cypher::{char_xor, repeating_key_xor};

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
        use crate::crypto::bytes::{hex, plaintext};
        use crate::crypto::crack::decode_single_char_xor;

        let hex_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        let bytes = hex::decode(hex_str);

        let key = decode_single_char_xor(&bytes);
        let result = plaintext::encode(&char_xor(&bytes, key));

        assert_eq!("Cooking MC\'s like a pound of bacon", result);
    }

    #[test]
    fn challenge_4() {
        use crate::crypto::bytes::hex;
        use crate::crypto::bytes::plaintext;
        use crate::crypto::util::load_challenge_data;
        use crate::crypto::crack::decode_single_char_xor;
        use crate::crypto::score::english_score;

        let mut best_score = 0.0;
        let mut best_line = String::new();
        for line in load_challenge_data("4").split("\n") {
            let bytes = hex::decode(line);
            let key = decode_single_char_xor(&bytes);
            let decoded_bytes = char_xor(&bytes, key);
            let score = english_score(&decoded_bytes);
            if score > best_score {
                best_score = score;
                best_line = plaintext::encode(&decoded_bytes);
            }
        }
        assert_eq!("Now that the party is jumping\n", best_line)
    }

    #[test]
    fn challenge_5() {
        use crate::crypto::bytes::plaintext::decode;
        use crate::crypto::bytes::hex::encode;
        use crate::crypto::cypher::repeating_key_xor;

        let bytes = decode("Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal");
        let cypher = decode("ICE");

        assert_eq!("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f",
                   encode(&repeating_key_xor(&bytes, &cypher)));
    }

    #[test]
    fn challenge_6() {
        use crate::crypto::bytes::{base64, plaintext};
        use crate::crypto::crack::break_repeating_key_xor;
        use crate::crypto::util::load_challenge_data;

        let mut bytes = Vec::new();
        for line in load_challenge_data("6").split("\n") {
            for byte in base64::decode(line) {
                bytes.push(byte);
            }
        }
        let key = break_repeating_key_xor(&bytes);
        let output = plaintext::encode(&repeating_key_xor(&bytes, &key));

        assert_eq!("I'm back and I'm ringin' the bell", &output[0..33]);
    }
}


fn main() {}
