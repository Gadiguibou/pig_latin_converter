fn main() {
    let s = "";
    println!("'{}' is '{}' in pig latin.", s, convert_to_pig_latin(s))
}

fn convert_to_pig_latin(s:&str) -> String {
    let split: Vec<&str> = s.split_whitespace().collect();

    let mut final_split: Vec<String> = Vec::new();

    for word in split {
        let chars: Vec<char>  = word.chars().collect();

        let word = 
            if ['a', 'e', 'i', 'o', 'u'].contains(&chars[0]) {
                let mut word = String::new();

                for char in &chars {
                    word.push(*char);
                }
                word.push_str("-hay");

                word
            } else {
                let mut word = String::new();

                if chars[0].is_uppercase() {
                    word.push_str(&chars[1].to_uppercase().to_string());
                } else {
                    word.push(chars[1]);
                }

                for char in &chars[2..] {
                    word.push(*char);
                }
                word.push_str(&["-", &chars[0].to_lowercase().to_string(), "ay"].join(""));

                word
            };

        final_split.push(word);
    }

    final_split.join(" ")
}
