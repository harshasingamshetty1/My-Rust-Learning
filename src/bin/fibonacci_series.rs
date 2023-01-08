use std::io;

pub fn main() {
    println!("Enter a value to find the Fibonacci");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read value 'n'");
    let n: u32 = n.trim().parse().expect("Please enter a valid number");

    let mut a = 1;
    let mut b = 1;
    let mut c = a + b;

    if n == 1 {
        println!(
            "{}{} {}",
            n.to_string(),
            "th Fibonacci number is:",
            a.to_string()
        );
    } else if n == 2 {
        println!(
            "{}{} {}",
            n.to_string(),
            "th Fibonacci number is:",
            b.to_string()
        );
    } else if n == 3 {
        println!(
            "{}{} {}",
            n.to_string(),
            "th Fibonacci number is:",
            c.to_string()
        );
    } else {
        let mut count = 3;
        while count < n {
            a = b;
            b = c;
            c = a + b;
            count += 1;
        }
        println!(
            "{}{} {}",
            n.to_string(),
            "th Fibonacci number is:",
            c.to_string()
        );
    }
}
