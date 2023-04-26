#[allow(unused_variables)]
#[allow(unused_assignments)]

static mut global_var: i32 = 0;
fn main() {
    for i in 0..15 {
        println!("{}. I have {} oranges", i, get_oranges(i));
    }

    let points = [(0, 0), (6, 0), (0, 5), (2, 3)];
    for point in points {
        match point {
            (0, 0) => {
                println!("origin")
            }
            (x, 0) => println!("x axis ({}, 0)", x),
            (0, y) => println!("y axis (0, {})", y),
            (x, y) => println!("On x-y plane({}, {})", x, y),
        }
    }

    /* loops */
    for (i, v) in (1..=100).enumerate() {
        println!("{}, {}", i, v);
    }
    // Global vars must be static and can only be used in unsafe blocks

    unsafe {
        global_var = 22;
        println!("prinitng the global var in the unsafe block {}", global_var);
    }
    //global_var = 11 // error, bcoz out of scope of unsafe block
}

fn get_oranges(amount: i32) -> &'static str {
    return match amount {
        0 => "no",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "an even amount of",
        _ => "lots of",
    };
}
