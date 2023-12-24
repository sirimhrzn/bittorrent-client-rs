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
        if self.encoded_value.starts_with('l') {
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

    fn decode_bencoded_lists(&mut self) -> Vec<String> {
        let mut list = String::from("");
        let mut dcl: Vec<DecodedList> = Vec::new();
        let colon = self.encoded_value.find(':').unwrap();
        let value = self.encoded_value
            [colon + 1..self.encoded_value.chars().count().checked_sub(1).unwrap() + 1]
            .to_string();

        let decoded_ints = self.decode_bencoded_integer();
        let mut newString: Vec<String> = Vec::new();
        let mut first = true;
        let mut first_int = String::from("");
        let mut changed_int = false;
        let before_semi = self.length_before_colon();
        for (i, num, e) in &self.integers {
            if let Some(e) = e {
                if !first {
                    if self.integers.len() == 1 {
                        break;
                    }
                    let val = value
                        [*e - before_semi - 2..value.chars().count().checked_sub(1).unwrap() + 1]
                        .to_string();
                    newString.push(val);
                }
                first_int.push(num.to_string().parse::<char>().unwrap());
                first_int.push(' ');
            } else {
                if first {
                    newString.push(value[0..*i - 1].to_string());

                    first = false;
                }
                first_int.push(num.to_string().parse::<char>().unwrap());
            }
        }
        dbg!(&first_int);
        dbg!(&newString);
        let newlist: Vec<String> = list.split_whitespace().map(|f| f.to_string()).collect();
        for item in newlist {
            newString.push(item);
        }
        newString
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
