// https://practice.course.rs/pattern-match/match-iflet.html
// ex  5 
// enum MyEnum {
//     Foo,
//     Bar
// }

// fn main() {
//     let mut count = 0;

//     let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
//     for e in v {
//         if e == MyEnum::Foo { // Fix the error by changing only this line
//             count += 1;
//         }
//     }

//     assert_eq!(count, 2);

//     println!("Success!");
// }

// Sol 5 ()
enum MyEnum {
    Foo,
    Bar
}

// fn main() {
//     let mut count = 0;

//     let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    
//     //usually if stmts cannot be used for pattern matchings, so we use match, matches, if let
//     //match can be used, if have to match all possibilities
//     //matches can be much more concise 
//     for e in v {
//         if matches!(e, MyEnum::Foo) 
//             {count+=1}
        
//     }
    
//     // for e in v {
//     //  match e{ 
//     //      MyEnum::Foo =>count+=1,
//     //      _ => ()
//     //     } ;
//     // }

//     assert_eq!(count, 2);

//     println!("Success!");
// }

////////// ////////// //////////  PATTERNS  ////////// ////////// //////////


//EX 2
/* The @ operator lets us create a variable that holds a value, 
at the same time we are testing that value to see whether it matches a pattern. */


// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     // Fill in the blank to let p match the second arm
//     let p = Point { x: __, y: __ };

//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {}", x),
//         // Second arm
//         Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
//         Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//     }
// }

// SOL 2
// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     // Fill in the blank to let p match the second arm
//     let p = Point { x: 1, y: 20 };
    
//     // we can use the @ operator, so that we can both destrucuture as well as check for a pattern
//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {}", x),
//         // Second arm
//         //here if we try to print x, it wont work bcoz, we have not destructured x and hence it is not is scope.

//         Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
//         Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//     }
// }


// ex 3

// Fix the errors
// enum Message {
//     Hello { id: i32 },
// }

// fn main() {
//     let msg = Message::Hello { id: 5 };

//     match msg {
//         Message::Hello {
//             id:  3..=7,
//         } => println!("Found an id in range [3, 7]: {}", id),
//         Message::Hello { id: newid@10 | 11 | 12 } => {
//             println!("Found an id in another range [10, 12]: {}", newid)
//         }
//         Message::Hello { id } => println!("Found some other id: {}", id),
//     }
// }

//sol 3

// Fix the errors
// enum Message {
//     Hello { id: i32 },
// }

// fn main() {
//     let msg = Message::Hello { id: 5 };

//     match msg {
//         Message::Hello {
//             id: id @ 3..=7,
//         } => println!("Found an id in range [3, 7]: {}", id),

//         Message::Hello { id: newid@(10 | 11 | 12) } => {
//             println!("Found an id in another range [10, 12]: {}", newid)
//         }
//         Message::Hello { id } => println!("Found some other id: {}", id),
//     }
// }


//ex 6


// FIX the error with least changing
// DON'T remove any code line
// fn main() {
//     let mut v = String::from("hello,");
//     let r = &mut v;

//     match r {
//        &mut value => value.push_str(" world!") 
//     }
// }

//sol 6


fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;
    // understand that, while we are matching r, directly, we will be matching the possible values of it.
    // and hence, need not use & , bcoz actually value becomes equal to &mut v, within the match stmt arm.
    // so, we can just mutate the string, using value. (as it holds the mut reference)
    match r {
        value => value.push_str(" world!") 
    }
}