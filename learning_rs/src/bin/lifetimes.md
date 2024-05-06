(https://practice.course.rs/lifetime/basic.html)<br>

**A reference must never outlive its value**<br>
Basically Lifetimes are used to make sure that there are no dangling pointers in a program.

Lifetime Ellision Rules (i.e automatically inferred by the compiler, no need to explicitly mention lifetimes in these cases)

1. each paramter that is a reference get its own lifetime parameter automatically by the rustc

2. If there is exactly one input lifetime reference parameter, that lifetime is assigned to all output lifetime

3. If there are multiple input lifetime reference parameters, but one of them is &self or &mut self the lifetime of self is assigned to all output lifetime parameters.

So basically rust follows these principles and if it still not able to figure out acc. to these rules
It expects us to explicitly mention the lifetme for the references.

```rs
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
```

Exercise 7

```rs
/* Make it work */

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}

fn main()
{
  /* 'a tied to fn-main stackframe */
  let var_a = 35;
  let example: Example;
  let var_b = NoCopyType {};
  {
    /* Lifetime 'b tied to new stackframe/scope */


    /* fixme */
    example = Example { a: &var_a, b: &var_b };
  }

  println!("(Success!) {:?}", example);
}
```

SOl 7

```rs
/* Make it work */

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}

fn main()
{
  /* 'a tied to fn-main stackframe */
  let var_a = 35;
  let example: Example;
  let var_b = NoCopyType {};
  {
    /* Lifetime 'b tied to new stackframe/scope */


    /* fixme */
    example = Example { a: &var_a, b: &var_b };
    /*
        we cannot use example out of this scope, bcoz the lifetime of 'b has ended.
     */

  println!("(Success!) {:?}", example);
  }


}
```

## 'static lifetime

```rs
    /* The diff btw the two is
     Both of them live for the entire lifetime, but
     when const is used, the value is inlined.
     but the static will have the same memory location throughout the program
    */
    static NUM = 32;
    const NUM = 22;
```

Ex 5

```rs
/* Make it work */
use std::fmt::Debug;

fn print_it<T: Debug + 'static>( input: T) {
    println!( "'static value passed in is: {:?}", input );
}

fn print_it1( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}


fn print_it2<T: Debug + 'static>( input: &T) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(&i);

    print_it1(&i);

    // but this one WORKS !
    print_it2(&i);
}
```

SOL 5

```rs
/* Make it work */
use std::fmt::Debug;

fn print_it<T: Debug + 'static>( input: T) {
    println!( "'static value passed in is: {:?}", input );
}

fn print_it1( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}


fn print_it2<T: Debug + 'static>( input: &T) {
    println!( "'static value passed in is: {:?}", input );
}


fn main() {
    // i is owned and contains no references, thus it's 'static:
    const i = 5; //changed from let to const
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:

    /*
        so to make it work, we must make the i as either const or static.
        so that, its reference will have the static lifetime.
     */
    print_it(&i);
    print_it1(&i);

    // but this one WORKS !
    print_it2(&i);
}
```
