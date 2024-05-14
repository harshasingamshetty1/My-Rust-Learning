````sh
    This is the general structure of a rust project.
    It can have atmost one main.rs and  one lib.rs in src directory
    But can have as many as binary crates possible in the bin directory
.
├── Cargo.toml
├── Cargo.lockp
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests # directory for integrated tests files
│   └── some_integration_tests.rs
├── benches # dir for benchmark files
│   └── simple_bench.rs
└── examples # dir for example files
    └── simple_example.rs```
````

for more in depth info, refer to these examples:
https://practice.course.rs/crate-module/module.html

https://github.com/sunface/rust-by-practice/blob/master/solutions/crate-module/module.md

## Debug trait

EX 3

```rs
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);


fn main() {
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?

    /* Make it print: Now 7 will print! */
    println!("Now {:?} will print!", Deep(Structure(7)));
}

```

Sol 3

```rs

use std::fmt;

struct Structure(i32);

struct Deep(Structure);
/*
    Here, we have implemented our custom derivation for printing.
    The fn signature must be gathered from the rust doc for Debug trait, and then we can easily implement
*/
impl fmt::Debug for Deep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // here, we are accessing the nested struct, and hence self.0.0
        write!(f, "{:?}", self.0.0)
    }
}

fn main() {
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?

    /* Make it print: Now 7 will print! */
    println!("Now {:?} will print!", Deep(Structure(7)));
}
```
