// Rust by practise:  https://practice.course.rs/compound-types/slice.html

//////////////////////////////////    SLICES      ///////////////////////////////////////////////////

// Fix the errors, DON'T add new lines!
// fn main() {
//     let arr = [1, 2, 3];
//     let s1: &[i32] = &arr[0..2];

//     let s2: &str = "hello, world";

//     println!("Success!");
// } 

//Ex 2:
// fn main() {
//     let arr: [char; 3] = ['中', '国', '人'];

//     let slice = &arr[..2];
    
//     // Modify '8' to make it work
//     // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
//     assert!(std::mem::size_of_val(&slice) == 8);

//     println!("Success!");
// }

//Soln 2

// it is bcoz the slice isa reference and has 2 u32 size values which are 1. pointer to value, 2. size of slice.
// so the size of slice is 16 bytes.(u32+u32)
// fn main() {
//     let arr: [char; 3] = ['中', '国', '人'];
//     let mut h = "harsha";
//     h = "shetty";
//     let slice = &arr[..2];
    
//     // Modify '8' to make it work
//     // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
//     assert!(std::mem::size_of_val(&slice) == 16);

//     println!("Success!");
// }


//ex 3
// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     // Fill the blanks to make the code work
//     let slice: __ = __;
//     assert_eq!(slice, &[2, 3, 4]);

//     println!("Success!");
// }

//sol3 

// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     // Fill the blanks to make the code work
//     let slice: &[i32] = &arr[1..4];
//     assert_eq!(slice, &[2, 3, 4]);

//     println!("Success!");
// }

//ex 6

// Fix errors
// fn main() {
//     let mut s = String::from("hello world");

//     // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
//     // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`. 
//     let letter = first_letter(&s);

//     s.clear(); // error!

//     println!("the first letter is: {}", letter);
// }
// fn first_letter(s: &str) -> &str {
//     &s[..1]
// }


// sol 6
/* here, wkt we cannot have a mutable ref, if there already exists an immutable ref.
but if we observe, if we use the "letter" before the clear, then we can have a mutable ref.
bcoz "letter" scope dies in that line itself.
 */
// fn main() {
//     let mut s = String::from("hello world");

//     // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
//     // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`. 
//     let letter = first_letter(&s);
//     println!("the first letter is: {}", letter);
 
//     s.clear(); // error!

// }
// fn first_letter(s: &str) -> &str {
//     &s[..1]
// }



//////////////////////////////////    TUPLES      ///////////////////////////////////////////////////

//ex 3

// Fix the error
// fn main() {
//     let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
//     println!("too long tuple: {:?}", too_long_tuple);
// }

//SOL 3 
//*By default debug can implement only upto 12 elements max */
// fn main() {
//     let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
//     println!("too long tuple: {:?}", too_long_tuple);
// }

//////////////////////////////////    STRUCTS      ///////////////////////////////////////////////////


//ex 3 


// Fix the error and fill the blanks
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// fn main() {
//     let v = Point(__, __, __);
//     check_color(v);

//     println!("Success!");
// }   

// fn check_color(p: Color) {
//     let (x, _, _) = p;
//     assert_eq!(x, 0);
//     assert_eq!(p.1, 127);
//     assert_eq!(__, 255);
//  }

//SOL3 

// Fix the error and fill the blanks
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// fn main() {
//     let v = Point(0,127,255);
//     check_color(v);

//     println!("Success!");
// }   
 

// fn check_color(p: Point) {
//     let Point(x, _, _) = p;
//     // let x = 0;
//     assert_eq!(x, 0);
//     assert_eq!(p.1, 127);
//     assert_eq!(p.2, 255);
//  }
 
 
//Partial Move in Structs
// when we move only any part of a struct, then we cannot borrow the struct as a whole again, instead we can use 
// the un moved parts of the struct.
// fn main() {
//     #[derive(Debug)]
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }

//     let person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };

//     // `name` is moved out of person, but `age` is referenced
//     let Person { name, ref age } = person;

//     println!("The person's age is {}", age);

//     println!("The person's name is {}", name);

//     // Error! borrow of partially moved value: `person` partial move occurs
//     //println!("The person struct is {:?}", person);

//     // `person` cannot be used but `person.age` can be used as it is not moved
//     println!("The person's age from person struct is {}", person.age);
// }

//////////////////////////////////    ENUMS      ///////////////////////////////////////////////////

// Ex 1

// Fix the errors
// enum Number {
//     Zero,
//     One,
//     Two,
// }

// enum Number1 {
//     Zero = 0,
//     One,
//     Two,
// }

// // C-like enum
// enum Number2 {
//     Zero = 0.0,
//     One = 1.0,
//     Two = 2.0,
// }


// fn main() {
//     // An enum variant can be converted to a integer by `as`
//     assert_eq!(Number::One, Number1::One);
//     assert_eq!(Number1::One, Number2::One);

//     println!("Success!");
// } 

//sol 1
/* the discriminator default starts with 0, but we can change it to any other integer value */

// enum Number {
//     Zero,
//     One,
//     Two,
// }

// enum Number1 {
//     Zero = 0,
//     One,
//     Two,
// }

// // C-like enum
// enum Number2 {
//     Zero ,
//     One = 1,
//     Two = 2,
// }


// fn main() {
//     // An enum variant can be converted to a integer by `as`
//     assert_eq!(Number::One as u8, Number1::One as u8);
//     assert_eq!(Number1::One as u8, Number2::One as u8);
//     println!("{:?}", Number1::One as u8);
//     println!("Success!");
// } 


// ex 5

// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.
// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     if let Some(n) = six {
//         println!("{}", n);

//         println!("Success!");
//     } 
//       else{panic!("NEVER LET THIS RUN！");}  
    
// } 

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }



//sol 5
/* in Rust, we dont have null which can be genericly used to every type, instead we must use, 
    Option<T> type, which is a generic type, which can be used to represent a value that may or may not be there.
    Understand that, Option<i32> !== i32.
    So, to we must destructure the Option<i32> type, and check if it is Some or None. only then we can use the value.
    and hence, the errors of falling into null will be avoided.
*/
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);

        println!("Success!");
    } 
      else{panic!("NEVER LET THIS RUN!");}  
    
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}