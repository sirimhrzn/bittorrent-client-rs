mod tests {
    use crate::decoders::decoders::Decoder;

    #[test]
    fn decode_bencoded_value() {
        let value = "5:hello".to_string();
        let decode = Decoder::new(value).decode();
        assert_eq!(decode, "hello");
    }

    #[test]
    fn decode_one_bencoded_integer() {
        let value = "l5:i64e".to_string();
        let decoded_value = Decoder::new(value).decode_bencoded_integer();
        assert_eq!(decoded_value, [64]);
    }
    #[test]
    fn decode_multiple_bencoded_integer() {
        let value = "l5:i64esiri19e".to_string();
        let decoded_value = Decoder::new(value).decode_bencoded_integer();
        assert_eq!(decoded_value, [64, 19]);
    }
    #[test]
    fn decode_with_integer_ending_bencoded_list() {
        let value = "l5:neveri2048egonnai4096ee".to_string();
        let decoded_value = Decoder::new(value).decode_bencoded_lists();
        assert_eq!(decoded_value, ["never", "2048", "gonna", "4096"]);
    }
    #[test]
    fn decode_with_string_ending_bencoding_list() {
        let value = "l5:ilovei732ewafflese".to_string();
        let decoded_value = Decoder::new(value).decode_bencoded_lists();
        assert_eq!(decoded_value, ["ilove", "732", "waffles"]);
    }
    #[test]
    fn decode_string_only_bencoding_list() {
        let value = "l5:neovimgoate".to_string();
        let decoded_value = Decoder::new(value).decode_bencoded_lists();
        assert_eq!(decoded_value, ["neovimgoat"]);
    }
}
