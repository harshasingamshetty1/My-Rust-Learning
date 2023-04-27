use crate::Suit::{Club, Diamond, Heart, Spade};

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    print_choice(Heart);
    print_choice(Club);
    print_choice(Diamond);
    print_choice(Spade);

    country(44);
    country(34);
    country(125);
    country(-15);
}

fn country(code: i32) {
    let country = match code {
        44 => "UK",
        34 => "Spain",
        1..=999 => "unknown",
        _ => "invalid",
    };
    println!("Country is {}", country);
}

enum Suit {
    Heart,
    Spade,
    Club,
    Diamond,
}

fn print_choice(choice: Suit) {
    match choice {
        Heart => {
            println!("\u{2665}")
        }
        Spade => {
            println!("\u{2660}")
        }
        Club => {
            println!("\u{2663}")
        }
        Diamond => {
            println!("\u{2666}")
        }
    }
}
