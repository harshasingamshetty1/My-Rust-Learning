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
//EX 5

// Implement `fn summary` to make the code work.
// Fix the errors without removing any code line
// trait Summary {
//     fn summarize(&self) -> String;
// }

// #[derive(Debug)]
// struct Post {
//     title: String,
//     author: String,
//     content: String,
// }

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
// }

// #[derive(Debug)]
// struct Weibo {
//     username: String,
//     content: String,
// }

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} published a weibo {}", self.username, self.content)
//     }
// }

// fn main() {
//     let post = Post {
//         title: "Popular Rust".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust is awesome!".to_string(),
//     };
//     let weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "Weibo seems to be worse than Tweet".to_string(),
//     };

//     summary(post);
//     summary(weibo);

//     println!("{:?}", post);
//     println!("{:?}", weibo);
// }

// // Implement `fn summary` below.


//SOL 5

// Implement `fn summary` to make the code work.
// Fix the errors without removing any code line
// trait Summary {
//     fn summarize(&self) -> String;
// }

// #[derive(Debug)]
// struct Post {
//     title: String,
//     author: String,
//     content: String,
// }

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
// }

// #[derive(Debug)]
// struct Weibo {
//     username: String,
//     content: String,
// }

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} published a weibo {}", self.username, self.content)
//     }
// }

// fn main() {
//     let post = Post {
//         title: "Popular Rust".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust is awesome!".to_string(),
//     };
//     let weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "Weibo seems to be worse than Tweet".to_string(),
//     };

//     summary(&post);
//     summary(&weibo);

//     println!("{:?}", post);
//     println!("{:?}", weibo);
// }

// // Implement `fn summary` below.
// fn summary(a: &impl Summary)-> String{
//     a.summarize()
// }


/* TRAIT OBJECTS

    Using "impl Trait" for return type does not work, when there are mutiple possibilities for the return type.
    For ex, if return type is impl Animal.
    Now, our function might return Cat or DOg based on some conditions, so for these cases it does not work
    It is bcoz, the compiler must know the size of return type at compile time itself, and 
    As, the size of Cat, Dog can be different, so the compiler does not know the size of return type at compile time.
*/

/* 
    BOX!
    Here comes the concept of Box, which is a wrapper around the value, which is stored on the heap.
    It is a smart pointer, that allows to store data on the heap, rather than on the stack.

    We can use BOX, whenever, you have a type whose size is unknown at compile time.
    It returns a pointer to the data stored on the heap
*/
/* 
    Diff btw &(a normal refernce ) and BOX
    A Box "allocates" data on heap and owns it, and it is also responsible for deallocating it.
    A refernce (&) just points to the data that is already present in the memory.

    Box can be cloned, but a refernce cannot be cloned.
    Box can me used in Pattern matching, but a refernce cannot be used.

    Box can be passed across scopes, while a refernce has a limited lifetime.
*/
// EX 6


// struct Sheep {}
// struct Cow {}

// trait Animal {
//     fn noise(&self) -> String;
// }

// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         "baaaaah!".to_string()
//     }
// }

// impl Animal for Cow {
//     fn noise(&self) -> String {
//         "moooooo!".to_string()
//     }
// }

// // Returns some struct that implements Animal, but we don't know which one at compile time.
// // FIX the errors here, you can make a fake random, or you can use trait object.
// fn random_animal(random_number: f64) -> impl Animal {
//     if random_number < 0.5 {
//         Sheep {}
//     } else {
//         Cow {}
//     }
// }

// fn main() {
//     let random_number = 0.234;
//     let animal = random_animal(random_number);
//     println!("You've randomly chosen an animal, and it says {}", animal.noise());
// }


//SOL 6

// struct Sheep {}
// struct Cow {}

// trait Animal {
//     fn noise(&self) -> String;
// }

// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         "baaaaah!".to_string()
//     }
// }

// impl Animal for Cow {
//     fn noise(&self) -> String {
//         "moooooo!".to_string()
//     }
// }

// // Returns some struct that implements Animal, but we don't know which one at compile time.
// // FIX the errors here, you can make a fake random, or you can use trait object.
// fn random_animal(random_number: f64) -> Box<dyn Animal>  {
//     if random_number < 0.5 {
//         Box::new(Sheep{})
//     } else {
//         Box::new(Cow {})
//     }
// }

// fn main() {
//     let random_number = 0.234;
//     let animal = random_animal(random_number);
//     println!("You've randomly chosen an animal, and it says {}", animal.noise());
// }


////////// ////////// //////////  TRAIT OBJECTS  ////////// ////////// //////////
//EX 1

// trait Bird {
//     fn quack(&self) -> String;
// }

// struct Duck;
// impl Duck {
//     fn swim(&self) {
//         println!("Look, the duck is swimming")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) -> String{
//         "duck duck".to_string()
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) -> String{
//         "swan swan".to_string()
//     }
// }

// fn main() {
//     // FILL in the blank.
//     let duck = __;
//     duck.swim();

//     let bird = hatch_a_bird(2);
//     // This bird has forgotten how to swim, so below line will cause an error.
//     // bird.swim();
//     // But it can quak.
//     assert_eq!(bird.quack(), "duck duck");

//     let bird = hatch_a_bird(1);
//     // This bird has forgotten how to fly, so below line will cause an error.
//     // bird.fly();
//     // But it can quak too.
//     assert_eq!(bird.quack(), "swan swan");

//     println!("Success!");
// }   

// // IMPLEMENT this function.
// fn hatch_a_bird...


//SOl 1
// trait Bird {
//     fn quack(&self) -> String;
// }

// struct Duck;
// impl Duck {
//     fn swim(&self) {
//         println!("Look, the duck is swimming")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) -> String{
//         "duck duck".to_string()
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) -> String{
//         "swan swan".to_string()
//     }
// }

// fn main() {
//     // FILL in the blank.
//     let duck = Duck {};
//     duck.swim();

//     let bird = hatch_a_bird(2);
//     // This bird has forgotten how to swim, so below line will cause an error.
//     /* Here it cant swim, as it is not implemented in trait object 
//         Although, it might be a duck which is returned, but the compiler does not know it at the compile time.
//         and hence it wont allow the fns specific to the concrete type Duck.
//         THe only methods allowed are those which are implemented in trait object.
//     */
//     // bird.swim();
//     // But it can quak.
//     assert_eq!(bird.quack(), "duck duck");

//     let bird = hatch_a_bird(1);
//     // This bird has forgotten how to fly, so below line will cause an error.
//     // bird.fly();
//     // But it can quak too.
//     assert_eq!(bird.quack(), "swan swan");

//     println!("Success!");
// }   

// // IMPLEMENT this function.
// fn hatch_a_bird(a: u32)-> Box<dyn Bird> {
//     if(a == 1){
//        Box::new(Swan{}) 
//     }
//     else{
//         Box::new(Duck{})
//     }
// }


//EX 2
// trait Bird {
//     fn quack(&self);
// }

// struct Duck;
// impl Duck {
//     fn fly(&self) {
//         println!("Look, the duck is flying")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) {
//         println!("{}", "duck duck");
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) {
//         println!("{}", "swan swan");
//     }
// }

// fn main() {
//     // FILL in the blank to make the code work.
//     let birds __;

//     for bird in birds {
//         bird.quack();
//         // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
//         // So, the code below will cause an error.
//         // bird.fly();
//     }
// }



//SOL 2
// trait Bird {
//     fn quack(&self);
// }

// struct Duck;
// impl Duck {
//     fn fly(&self) {
//         println!("Look, the duck is flying")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) {
//         println!("{}", "duck duck");
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) {
//         println!("{}", "swan swan");
//     }
// }

// fn main() {
    
//     /*WE can either use Box or the &
    
//     its bocz, if dont use any of that, then we would not know the size of the array at compile time.
//     but when we use references of any type, we know the size of the array at compile time.
//     The size would be usize = 8bytes for each pointer.

//       */    
    
//     let birds: [&dyn Bird;2] = [&(Duck), &(Swan)];

//     for bird in birds {
//         bird.quack();
//         // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
//         // So, the code below will cause an error.
//         // bird.fly();
//     }
// }



/*  STATIC AND DYNAMIC DISPATCH

When we use trait bounds on generics, the compiler generates nongeneric implementations of functions and methods for each concrete type that we use in place of a generic type parameter. The code that results from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile time.

When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the types that might be used with the code that is using trait objects, so it doesn’t know which method implemented on which type to call. Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call. There is a runtime cost when this lookup happens that doesn’t occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations.

However, we do get extra flexibility when using dynamic dispatch.

*/

//EX 4

// trait Foo {
//     fn method(&self) -> String;
// }

// impl Foo for u8 {
//     fn method(&self) -> String { format!("u8: {}", *self) }
// }

// impl Foo for String {
//     fn method(&self) -> String { format!("string: {}", *self) }
// }

// // IMPLEMENT below with generics.
// fn static_dispatch...

// // Implement below with trait objects.
// fn dynamic_dispatch...

// fn main() {
//     let x = 5u8;
//     let y = "Hello".to_string();

//     static_dispatch(x);
//     dynamic_dispatch(&y);

//     println!("Success!");
// }

//SOL 4

// trait Foo {
//     fn method(&self) -> String;
// }

// impl Foo for u8 {
//     fn method(&self) -> String { format!("u8: {}", *self) }
// }

// impl Foo for String {
//     fn method(&self) -> String { format!("string: {}", *self) }
// }

// // IMPLEMENT below with generics.
// fn static_dispatch<T: Foo>(x:T)-> String{
//     println!("done static");
//     x.method()
// }

// // Implement below with trait objects.
// fn dynamic_dispatch(x: &dyn Foo)-> String {
//     x.method()
// }

// fn main() {
//     let x = 5u8;
//     let y = "Hello".to_string();

//     static_dispatch(x);
//     dynamic_dispatch(&y);

//     println!("Success!");
// }


//EX 5


// Use at least two approaches to make it work.
// DON'T add/remove any code line.
// trait MyTrait {
//     fn f(&self) -> Self;
// }

// impl MyTrait for u32 {
//     fn f(&self) -> Self { 42 }
// }

// impl MyTrait for String {
//     fn f(&self) -> Self { self.clone() }
// }

// fn my_function(x: Box<dyn MyTrait>)  {
//     x.f()
// }

// fn main() {
//     my_function(Box::new(13_u32));
//     my_function(Box::new(String::from("abc")));

//     println!("Success!");
// }

/* 

    OBJECT SAFE TRAITS

    You can only make object-safe traits into trait objects. A trait is object safe if all the methods defined in the trait have the following properties:

    The return type isn’t 
    1. Self.
    2. There are no generic type parameters.

    This is bcoz, again the same reason, i.e the compiler must know the size of the return value at compile time.
    
    But if we use trait functions with return type as either Self or generic type parameters, then the compiler cannot know the size 
    of the return value at compile time.

    This is bcoz, any structs can implement the traits and hence, when the return type is Self, it means any struct could be a possible return value, which means complier cannot know the size of the return value at compile time.

    and hence, to be able to use the trait object, the trait must be object safe.

*/

//SOl 5

/* USing Dynamic Dispatch */

trait MyTrait {
    fn f(&self) -> Box<dyn MyTrait>;
}

impl MyTrait for u32 {
    fn f(&self) ->Box<dyn MyTrait> { Box::new(42) } 
}

impl MyTrait for String {
    fn f(&self) -> Box<dyn MyTrait>{ Box::new(self.clone()) }
}

fn my_function(x: Box<dyn MyTrait>)-> Box<dyn MyTrait>  {
    x.f()
}

fn main() {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));

    println!("Success!");
}