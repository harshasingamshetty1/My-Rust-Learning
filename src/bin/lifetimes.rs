/* Basically Lifetimes are used to make sure that there are no dangling pointers in a program.

  1. each paramter that is a reference get its own lifetime parameter automatically by the rustc

  2. If there is exactly one input lifetime reference parameter, that lifetime is assigned to all output lifetime

  3. If there are multiple input lifetime reference  parameters, but one of them is &self or &mut self
   the lifetime of self is assigned to all output lifetime parameters.

   So basically rust follows these principles and if it still not able to figure out acc. to these rules
   It expects us to explicitly mention the lifetme for the references.
*/

fn main() {
    let x = 5; // variable x is allocated on the stack
    let y = &x; // y is a reference to x
    let z; // declare a variable z without initializing it
    {
        let w = 10; // allocate variable w on the stack
        z = &w; // z is a reference to w
        println!("z: {}", z); // this line will work
    }
    // println!("z: {}", z); // this line will not compile because the lifetime of w has ended
    println!("y: {}", y); // this line will work

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
        part2: "hello",
    };
}

struct ImportantExcerpt<'a, 'b> {
    part: &'b str,
    part2: &'a str,
}
