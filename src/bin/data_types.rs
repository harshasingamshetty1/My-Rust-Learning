// ARRAYS
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
use crate::Colors::Red;
use crate::Human::Name;
use crate::Human::Surname;

fn main() {
    // arrays are fixed sizes, and are not dynamic
    // let mut arr = [7; 10];
    // arr[2] = 9;
    // println!("{:?}", arr)

    /* VECTORS */

    // let mut vec = vec![0.0, 1.0];
    // println!("{:?}", vec);
    // const DEFAULT: f64 = 3.33;
    // vec = vec![DEFAULT; 7];
    // println!("{:?}", vec);

    // for v in vec.iter() {
    //     println!("{}", v);
    // }

    /* TUPLES*/
    /* Max 12 elements
    Static size
    accessing => var.0
    */

    // let person = ("Singam", 23, true);
    // println!("{:?}", person.2);

    /* STRUCTS */
    // let h = Person {
    //     name: String::from("Harsha"),
    //     age: 23,
    //     gender: true,
    // };

    // h.get_details();
    // Person::overall_details();

    /* ENUMS */

    // let my_color = Colors::Green;
    // println!("{:?}", my_color);
    // let my_color = Red;
    // println!("{:?}", my_color);

    // let person = Name(String::from("Harsha"));
    // println!("{:?}", person);
    // let p2 = Surname(String::from("Singamshetty"));
    // println!("{:?}", p2);

    /*Generics */
    let p = Point { x: 12, y: 7.99999 };
    let p2 = Point {
        x: String::from("Hey"),
        y: true,
    };
    println!("{:?}", p);

    println!("{:?}", p2);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    gender: bool,
}
impl Person {
    fn get_details(&self) {
        println!("{:?}", self.name)
    }
    fn overall_details() {
        println!("Hello Person!");
    }
}

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum Human {
    Name(String),
    Surname(String),
    Age(u32),
}

#[derive(Debug)]
struct Point<T, V> {
    x: T,
    y: V,
}
