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

    fn decode_bencoded_lists(&mut self) -> Vec<DecodedList> {
        let mut list = String::from("");
        let mut dcl: Vec<DecodedList> = Vec::new();
        let colon = self.encoded_value.find(':').unwrap();
        let value = self.encoded_value
            [colon + 1..self.encoded_value.chars().count().checked_sub(1).unwrap()]
            .to_string();

        let decoded_ints = self.decode_bencoded_integer();
        let mut newString: Vec<String> = Vec::new();
        let mut first = true;
        dbg!(&self.integers);
        for (i, num, e) in &self.integers {
            if let Some(e) = e {
                if !first {
                    dbg!(&value[*e - 2..*i - 1]);
                    newString.push(value[*e..*i].to_string());
                }
            } else {
                continue;
            }
            if first {
                newString.push(value[0..*i - 3].to_string());
                first = false;
                continue;
            }
        }
        dbg!(&newString);
        for int in decoded_ints {
            list.push(' ');
            for (i, n) in int.to_string().chars().enumerate() {
                list.push(n);
            }
        }
        let newlist: Vec<String> = list.split_whitespace().map(|f| f.to_string()).collect();
        dbg!(&newlist);
        for f in newlist {
            if let Ok(parsed_int) = f.parse::<i64>() {
                dcl.push(DecodedList::Int(parsed_int));
            }
            if let Ok(str) = f.parse::<String>() {
                dcl.push(DecodedList::String(str));
            }
        }
        for data in dcl.iter() {}
        dcl
    }
    fn length_before_colon(&mut self) -> usize {
        let semi = self.encoded_value.find(':').unwrap();
        self.encoded_value[0..semi].len()
    }
    fn decode_bencoded_integer(&mut self) -> Vec<i64> {
        let mut previous_char = "";
        let mut current_sequence = String::from("");
        let mut modified = false;
        let mut parsed_ints: Vec<i64> = Vec::new();
        let before_semi = self.length_before_colon();
        for (i, char) in self.encoded_value.chars().enumerate() {
            if previous_char == "i" && (char.is_numeric() || char.to_string() == "-") {
                self.integers.push((
                    i - before_semi,
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
                parsed_ints.push(current_sequence.parse::<i64>().unwrap());
            }
            if char.to_string().to_lowercase() == "i" {
                previous_char = "i";
            }
        }
        parsed_ints
    }
}
