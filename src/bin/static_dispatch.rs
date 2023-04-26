#[allow(unused_variables)]
#[allow(unused_assignments)]

trait Duplicateable {
    fn dupl(&self) -> String;
}

impl Duplicateable for String {
    fn dupl(&self) -> String {
        format!("{0}{0}", *self)
    }
}

impl Duplicateable for i32 {
    fn dupl(&self) -> String {
        format!("{}", *self * 2)
    }
}
/*
Static dispatch works by monomorphizing generic functions at compile-time.
This means that for each unique set of generic type parameters used in a function call,
Rust will generate a new, specialized version of the function with those type parameters substituted in.
This specialized function is then called directly, without any runtime overhead.

This means that, the fn with a generic type is actually, internally implemented as both i32 and String in our case
so, that, there wont be any overhead, at runtime
its like kind of having same fn 2 times for each type generic usable!
 */
fn duplicate<T: Duplicateable>(x: T) {
    println!("{}", x.dupl());
}

fn main() {
    /*STATIC DISPATCH */
    let a = 42;
    let b = "Hi John ".to_string();
    // here the monomorphized version of duplicate with i32 type is called
    duplicate(a);
    // here the monomorphized version of duplicate with String type is called
    duplicate(b);

    /*DYNAMIC DISPATCH */
    /* Here,we hace created a vector of Trait objects,

    When the speak method is called on each trait object,
    here rust will know which particular implementation to use only at runtime,
    bcoz diffnt trait objects can have diffnt implementation
    So, at the runtime rust checks its vtable to correctly call the implementation

    This is known as the dynamic dispatch
    */
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in animals {
        // here the animal can be any struct, which has implemented the trait Animal
        // so rust cant dispatch this statically.
        // only at the runtime, rust will know which implementation to use
        // and hence known as the dynamic disptach. (It has a little overhead, bcoz of its runtime excn)
        animal.speak();
    }
}

trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}
