#[allow(unused_variables)]
#[allow(unused_assignments)]
/*
   Implementing a trait for a any type that satisfies another trait, is called blanket implementaion
   ex:

   impl<T: Display> ToString for T {}

   this means that, we are implementing the ToString trait for any type that satisfies the Display trait.
   This is called a blanket implementation.
*/
trait Bark {
    fn bark(&self) -> String;
}

struct Dog {
    species: &'static str,
}
struct Dog2 {
    species: &'static str,
}
struct Cat {
    color: &'static str,
}

impl Bark for Dog {
    fn bark(&self) -> String {
        return format!("{} barking", self.species);
    }
}

impl Bark for Dog2 {
    fn bark(&self) -> String {
        return format!("{} barking", self.species);
    }
}
// we can give specific impl only the structs that have implemented the Bark trait will
//BE able to use this function, bcoz of the generic we used.
fn bark_it<T: Bark>(b: T) {
    println!("{}", b.bark())
}

fn main() {
    let dog = Dog2 {
        species: "retriever",
    };
    let cat = Cat { color: "black" };
    bark_it(dog);
    // bark_it(cat);
}
