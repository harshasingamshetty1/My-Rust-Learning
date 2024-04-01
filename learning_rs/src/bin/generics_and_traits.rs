// https://practice.course.rs/generics-traits/generics.html

////////// ////////// //////////  GENERICS  ////////// ////////// //////////

/* Generics are filled with concrete types at compile time by the compiler.

    Monomorphization is the process of turning generic code into specific code by filling in the concrete types
    that are used when compiled. 
 */

 //EX 1
// // Fill in the blanks to make it work
// struct A;          // Concrete type `A`.
// struct S(A);       // Concrete type `S`.
// struct SGen<T>(T); // Generic type `SGen`.

// fn reg_fn(_s: S) {}

// fn gen_spec_t(_s: SGen<A>) {}

// fn gen_spec_i32(_s: SGen<i32>) {}

// fn generic<T>(_s: SGen<T>) {}

// fn main() {
//     // Using the non-generic functions
//     reg_fn(__);          // Concrete type.
//     gen_spec_t(__);   // Implicitly specified type parameter `A`.
//     gen_spec_i32(__); // Implicitly specified type parameter `i32`.

//     // Explicitly specified type parameter `char` to `generic()`.
//     generic::<char>(__);

//     // Implicitly specified type parameter `char` to `generic()`.
//     generic(__);

//     println!("Success!");
// }

//Sol 1
// struct A;          // Concrete type `A`.
// struct S(A);       // Concrete type `S`.
// struct SGen<T>(T); // Generic type `SGen`.

// fn reg_fn(_s: S) {}

// fn gen_spec_t(_s: SGen<A>) {}

// fn gen_spec_i32(_s: SGen<i32>) {}

// fn generic<T>(_s: SGen<T>) {}

// fn main() {
//     // Using the non-generic functions
//     reg_fn(S(A));          // Concrete type.
//     gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
//     gen_spec_i32(SGen(1)); // Implicitly specified type parameter `i32`.

//     // Explicitly specified type parameter `char` to `generic()`.
//     // here as we explcitly mentioned that the generic is going to be char, so we must pass a char
//     generic::<char>(SGen('z'));

//     // Implicitly specified type parameter `char` to `generic()`.
//     generic(SGen(99.999));

//     println!("Success!");
// }


//Ex 5

// Add generic for Val to make the code work, DON'T modify the code in `main`.
// struct Val {
//     val: f64,
// }

// impl Val {
//     fn value(&self) -> &f64 {
//         &self.val
//     }
// }


// fn main() {
//     let x = Val{ val: 3.0 };
//     let y = Val{ val: "hello".to_string()};
//     println!("{}, {}", x.value(), y.value());
// }


//Sol 5
// struct Val<T> {
//     val: T,
// }

// impl<T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }


// fn main() {
//     let x = Val{ val: 3.0 };
//     let y = Val{ val: "hello".to_string()};
//     println!("{}, {}", x.value(), y.value());
// }

//EX 6

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     // Implement mixup to make it work, DON'T modify other code.
//     fn mixup
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "Hello", y: '中'};

//     let p3 = p1.mixup(p2);

//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');

//     println!("Success!");
// }

//SOL 6

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     // Implement mixup to make it work, DON'T modify other code.
//     fn mixup<A, B>(self, other_point: Point<A, B>) -> Point<T, B> {
//         Point {
//             x: self.x,
//             y: other_point.y,
//         }
//     }
// }




// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "Hello", y: '中'};

//     let p3 = p1.mixup(p2);

//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');

//     println!("Success! ");
// }

//EX 7


// Fix the errors to make the code work.
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn main() {
//     let p = Point{x: 5, y: 10};
//     println!("{}",p.distance_from_origin());
// }


//SOl 7

// struct Point<T> {
//     x: T,
//     y: T,
// }

// // Understand, that the impl is defined only for f32, for all other types, this method does not exist
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn main() {
//     let p = Point{x: 5.0, y: 10.0};
//     println!("{}",p.distance_from_origin());
// }



////////// ////////// ////////// CONST GENERICS  ////////// ////////// //////////

//EX 2
// Fill in the blanks to make it work.
// fn print_array<__>(__) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr = [1, 2, 3];
//     print_array(arr);

//     let arr = ["hello", "world"];
//     print_array(arr);
// }

//SOl 2

// fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T;N] ) {
//     /*  here, we have to use the const keyword to make sure that the array is of a fixed size
//         and we cannpt use just like <T, N> bocz, N will be considered as a type, but in the 
//         arr defn, the size of array is exprected as a value, but not a type.

//         so, we have to use const keyword to make sure that the size of the array is fixed but not a type.
//     */

//     println!("{:?}", arr);
// }
// fn main() {
//     let arr = [1, 2, 3];
//     print_array(arr);

//     let arr = ["hello", "world"];
//     print_array(arr);
// }

////////// ////////// //////////  TRAITSS  ////////// ////////// //////////
