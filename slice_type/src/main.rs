use std::env;

fn main() {
    let mut input: String = env::args()
        .nth(1)
        .expect("Enter string as argument");

    println!("The word is {}", first_word(&input));

    input.clear()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'_' {
            return &s[..i];
        }
    }

    &s[..]
}
