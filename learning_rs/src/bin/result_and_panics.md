By default the stack unwinding will only give something like this:

thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

Though there is the reason of panic and the line of the code is showing where the panic has occured, sometimes we want to get more info about the call stack.

Then we can use, the environment variable `RUST_BACKTRACE = 1 ` to get the call stack.

### Unwinding and Abort

By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters.

But this walk back and clean up is a lot of work. The alternative is to immediately abort the program without cleaning up.

If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting by adding below content to Cargo.toml:

[profile.release]
panic = 'abort'

# result and ?

EX 1

```rs

// FILL in the blanks and FIX the errors
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> __ {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn main() {
    let result = multiply("10", "2");
    assert_eq!(result, __);

    let result = multiply("t", "2");
    assert_eq!(result.__, 8);

    println!("Success!");
}
```

SOL 1

```rs

use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn main() {
    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply("4", "2");
    assert_eq!(result.unwrap(), 8);

    println!("Success!");
}
```

# ?

? is almost exactly equivalent to unwrap, but ? returns instead of panic on Err.

EX 2

```rs

use std::num::ParseIntError;

// IMPLEMENT multiply with ?
// DON'T use unwrap here
fn multiply(n1_str: &str, n2_str: &str) -> __ {
}

fn main() {
    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!");
}
```

Sol 2

```rs

use std::num::ParseIntError;

// IMPLEMENT multiply with ?
// DON'T use unwrap here
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let a = n1_str.parse::<i32>()?;
    let b = n2_str.parse::<i32>()?;
    Ok(a*b)
}

fn main() {
    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!");
}
```

EX 3

```rs

use std::fs::File;
use std::io::{self, Read};

fn read_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// FILL in the blanks with one code line
// DON'T change any code lines
fn read_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    __;

    Ok(s)
}

fn main() {
    assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
    println!("Success!");
}

```

SOL 3

```rs

use std::fs::File;
use std::io::{self, Read};

fn read_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// FILL in the blanks with one code line
// DON'T change any code lines
fn read_file2() -> Result<String, io::Error> {
    let mut s = String::new();
    /*
        Here, understand that entire match stmts can be consolidated into "?" operator.
        Bcoz, the "?" operator will return the value of the Ok variant, and will return the Err value if it encounters an Err value.
     */
    File::open("hello.txt")?.read_to_string(&mut s)?;
    /*
        here, we are returing Ok(s) though we have already unwrapped bcoz,
        the return type expected is Result, and hence if everything works out Ok(s) is returned
        else, the err is returned from the '?' operator
     */
    Ok(s)
}

fn main() {
    assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
    println!("Success!");
}
```

### map, and_then

Map,
Maps a Result<T, E> to Result<U, E> by applying a function to a contained Ok value, leaving an Err value untouched.

This function can be used to compose the results of two functions.

Ex:

```rs

let line = "1\n2\n3\n4\n";

for num in line.lines() {
    match num.parse::<i32>().map(|i| i * 2) {
        Ok(n) => println!("{n}"),
        Err(..) => {}
    }
}
```

and_then is almost similar but, just within the closure, it returns a Result<T, E> instead of T.
So,

```rs
let line = "1\n2\n3\n4\n";

for num in line.lines() {
    match num.parse::<i32>().map(|i| Ok(i * 2)) {
        Ok(n) => println!("{n}"),
        Err(..) => {}
    }
}
```

## type alias

Using std::result::Result<T, ParseIntError> everywhere is verbose and tedious, we can use alias for this purpose.

At a module level, creating aliases can be particularly helpful. Errors found in a specific module often has the same Err type, so a single alias can succinctly defined all associated Results. This is so useful even the std library supplies one: io::Result.

```rs

use std::num::ParseIntError;

    /*
        Here, we have defined the alias.
        So it can be used across the program
     */
type Res<i32> = Result<i32, ParseIntError>;

// Use the above alias to refer to our specific `Result` type.
fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// Here, the alias again allows us to save some space.
fn print(result: Res<i32>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));

    println!("Success!");
}
```
