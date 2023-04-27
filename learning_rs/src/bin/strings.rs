//Rust by Examples https://practice.rs/compound-types/string.html
//ex 1

// Fix error without adding new line
// fn main() {
//     let s: &str = "hello, world";

//     println!("Success!");
// }

//ex 2

// Fix the error with at least two solutions
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(&s)
// }

// fn greetings(s: &str) {
//     println!("{}", s)
// }

//ex 4

// Fix all errors without adding newline
// fn main() {
//     let mut s = String::from("hello");
//     s.push(',');
//     s.push_str(" world");
//     s += "!";

//     println!("{}", s);
// }

//ex 5

// Fill the blank
// fn main() {
//     let s = String::from("I like dogs");
//     // Allocate new memory and store the modified string there
//     let s1 = s.replace("dogs", "cats");

//     assert_eq!(s1, "I like cats");
//     let mut s3 = "oi hoi";
//     let s3: String = s3.replace("oi ", "no ");
//     println!("{}", s3);
//     println!("Success!");
// }

//ex 6

// Fix errors without removing any line
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     let s3 = s1.clone() + &s2;
//     assert_eq!(s3, "hello,world!");
//     println!("{}", s1);
// }

//ex 7

// Fix error with at least two solutions
// fn main() {
//     let s = "hello, world";
//     greetings(s.to_string());
// }

// fn greetings(s: String) {
//     println!("{}", s)
// }

//ex 8

// Use two approaches to fix the error and without adding a new line
// fn main() {
//     let s = "hello, world".to_string();
//     let s1: &str = &s;

//     println!("Success!");
// }

//ex 9
fn main() {
    // You can use escapes to write bytes by their hexadecimal values
    // Fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}
