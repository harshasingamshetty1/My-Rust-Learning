// Rust By Example:  https://practice.rs/ownership/ownership.html

//Exercise 1: Make the code work
// Don't modify code in main!
// fn main() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1);

//     println!("{}", s2);
// }

// // Only modify the code below!
// fn take_ownership(s: String) -> String {
//     println!("{}", s);
//     s
// }

//Exercise 2: Make the code work
// Don't modify code in main!
// fn main() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1);

//     println!("{}", s2);
// }

// // Only modify the code below!
// fn take_ownership(s: String) -> String {
//     println!("{}", s);
//     s
// }

//exercise 3
// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }

// // Only modify the code below!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // Convert String to Vec
//     let _s = s.as_bytes(); //into_bytes, consumes the string
//     s
// }

//exercise 4
// Fix the error without removing code line
// fn main() {
//     let s = String::from("hello, world");

//     print_str(&s);

//     println!("{}", s);
// }

// fn print_str(s: &String) {
//     println!("{}", s)
// }

//exercise 5
// Don't use clone ,use copy instead
// fn main() {
//     let x = (1, 2, (), "hello");
//     let y = x;
//     println!("{:?}, {:?}", x, y);
// }

//exercise 6
// fn main() {
//     let s = String::from("hello, ");

//     // Modify this line only !
//     let mut s1 = s;
//     s1.push_str("world");

//     println!("Success!");
// }

//exercise 7
// fn main() {
//     let x = Box::new(5);

//     let mut y = Box::new(3); // implement this line, dont change other lines!

//     *y = 4;

//     assert_eq!(*x, 5);
//     println!("Success, {}", y)
// }

//exercise 8
// fn main() {
//     let t = (String::from("hello"), String::from("world"));

//     let _s = t.0;

//     // Modify this line only, don't use `_s`
//     println!("{:?}", t.1);
// }

//exercise 9
fn main() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (ref s1, ref s2) = t;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
