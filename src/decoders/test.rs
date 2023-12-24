mod tests {
    use crate::decoders::decoders::Decoder;

    #[test]
    fn decode_bencoded_value() {
        let value = "5:hello";
        let decode = Decoder::new(value.to_string()).decode();
        assert_eq!(decode, "hello");
    }

    #[test]
    fn decode_one_bencoded_integer() {
        let value = "l5:i64e".to_string();
        let decoded_value = Decoder::new(value.to_string()).decode_bencoded_integer();
        assert_eq!(decoded_value, [64]);
    }
    #[test]
    fn decode_multiple_bencoded_integer() {
        let value = "l5:i64esiri19e".to_string();
        let decoded_value = Decoder::new(value.to_string()).decode_bencoded_integer();
        dbg!(&decoded_value);
        assert_eq!(decoded_value, [64, 19]);
    }
}
