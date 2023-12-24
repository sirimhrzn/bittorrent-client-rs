use clap::builder::ValueParserFactory;

#[derive(Debug)]
pub enum DecodedList {
    Int(i64),
    String(String),
}

pub struct Decoder {
    encoded_value: String,
    integers: Vec<(usize, i64, Option<usize>)>,
}

impl Decoder {
    pub fn new(encoded_value: String) -> Self {
        Decoder {
            encoded_value,
            integers: Vec::new(),
        }
    }
    pub fn decode(&mut self) -> serde_json::Value {
        if self.encoded_value.chars().next().unwrap().is_numeric() {
            let val = self.decode_bencoded_value();
            return val;
        }
        if self.encoded_value.starts_with('l') && self.encoded_value.ends_with('e') {
            let val = self.decode_bencoded_lists();
            return serde_json::Value::String("Decoded list".to_string());
        }
        serde_json::Value::String("No case match".to_string())
    }
    fn decode_bencoded_value(&mut self) -> serde_json::Value {
        let colon_index = self.encoded_value.find(':').unwrap();
        let number_string = &self.encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let string = &self.encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        serde_json::Value::String(string.to_string())
    }

    pub fn decode_bencoded_lists(&mut self) -> Vec<String> {
        let decoded_ints = self.decode_bencoded_integer();
        let mut newString: Vec<String> = Vec::new();
        let till_semi = self.length_before_colon() + 1;
        let value_after_semi = &self.encoded_value[till_semi..];
        let mut int_hit = false;
        let mut int_hit_index = 0;
        let mut char_hit_index = 0;
        let mut num: Vec<String> = Vec::new();
        let mut ints = String::new();
        for (i, char) in value_after_semi.chars().enumerate() {
            if i == value_after_semi.chars().count() - 1 && char_hit_index == 0 {
                let value = value_after_semi[char_hit_index..i].to_string();
                num.push(value);
            }
            if char == 'e' && int_hit {
                let value = value_after_semi[int_hit_index + 1..i].to_string();
                num.push(value);
                int_hit = false;
                int_hit_index = 0;
                char_hit_index = i;
                continue;
            }
            if char == 'i' && value_after_semi[i + 1..i + 2].parse::<i64>().is_ok() {
                if char_hit_index == 0 {
                    num.push(value_after_semi[char_hit_index..i].to_string());
                } else {
                    num.push(value_after_semi[char_hit_index + 1..i].to_string());
                }
                int_hit = true;
                int_hit_index = i;
                continue;
            }
        }
        dbg!(&num);
        num
    }
    // make this length till colon
    fn length_before_colon(&mut self) -> usize {
        let semi = self.encoded_value.find(':').unwrap();
        self.encoded_value[0..semi].len()
    }
    pub fn decode_bencoded_integer(&mut self) -> Vec<i64> {
        let mut previous_char = "";
        let mut current_sequence = String::from("");
        let mut modified = false;
        let mut parsed_ints: Vec<i64> = Vec::new();
        let before_semi = self.length_before_colon();
        dbg!(&before_semi);
        for (i, char) in self.encoded_value.chars().enumerate() {
            if previous_char == "i" && (char.is_numeric() || char == '-') {
                self.integers.push((
                    i - (before_semi + 1),
                    char.to_string().parse::<i64>().unwrap(),
                    None,
                ));
                current_sequence.push(char);
                modified = true;
            }
            if previous_char == "i" && char.to_string().to_lowercase() == "e" && modified {
                if let Some(last_index) = self.integers.len().checked_sub(1) {
                    if let Some(data) = self.integers.get_mut(last_index) {
                        data.2 = Some(i - before_semi);
                    }
                }
                previous_char = "";
                parsed_ints.push(current_sequence.parse::<i64>().unwrap());
                current_sequence = "".to_string();
            }
            if char.to_string().to_lowercase() == "i" && previous_char != "i" {
                previous_char = "i";
            }
        }
        dbg!(&parsed_ints);
        parsed_ints
    }
}
