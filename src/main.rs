use std::io;

fn main() {
    println!("Enter the string to convert.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read stdin");

    let mut chars = input.chars().peekable();

    let mut new_s = String::new();

    while let Some(c) = chars.next() {
        let suffix = match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                new_s.push(c);
                String::from("-hay")
            }
            'a'..='z' | 'A'..='Z' => format!("-{}ay", c.to_lowercase()),
            _ => {
                new_s.push(c);
                continue;
            }
        };

        while let Some(&c) = chars.peek() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    chars.next();
                    new_s.push(c);
                }
                _ => break,
            }
        }

        new_s += &suffix;
    }

    println!("{}", new_s);
}
