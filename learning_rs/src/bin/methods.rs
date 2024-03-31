// https://practice.course.rs/method.html

/* Methods vs Associated Functions

🌟🌟 Methods are similar to functions: Declare with fn, have parameters and a return value.
 Unlike functions, methods are defined within the context of a struct (or an enum or a trait object),
  and their first parameter is always "self", which represents the instance of the struct the method is being called on.

  used as, rect1.area();



🌟🌟All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl. 
We can define associated functions that don’t have self as their first parameter (and thus are not methods)
 because they don’t need an instance of the type to work with.

 used as, Rectangle::area(&rect1);
*/

// ex 4
// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     // 1. Implement an associated function `new`,
//     // 2. It will return a TrafficLight contains color "red"
//     // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
//     pub fn new() 

//     pub fn get_state(&self) -> &str {
//         &self.color
//     }
// }

// fn main() {
//     let light = TrafficLight::new();
//     assert_eq!(light.get_state(), "red");

//     println!("Success!");
// }

//sol 4
// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {

//     //return type is must in associcated functions
//     pub fn new() -> Self  {
//          return Self{ color: String::from("red")} ;
//     } 

//     pub fn get_state(&self) -> &str {
//         &self.color
//     }
// }

// fn main() {
//     let light = TrafficLight::new();
//     assert_eq!(light.get_state(), "red");

//     println!("Success!");
// }

//ex 6
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self) -> &str {
       match self {
            TrafficLightColor::Red =>  "red",
            TrafficLightColor::Yellow =>  "yellow",
            TrafficLightColor::Green =>  "green",
        }
    }
} 

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}