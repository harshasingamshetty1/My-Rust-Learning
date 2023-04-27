use std::io;
fn main() {
    let mut input_string = String::new();
    io::stdin()
        .read_line(&mut input_string)
        .expect("Provide a valid input");
    // println!("{}", input_string);
    let first_word = extract_first_word(&input_string);
    println!("The first word of the entered input is = {}", first_word)
}
fn extract_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
