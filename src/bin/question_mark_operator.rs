use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

#[allow(unused_variables)]
#[allow(unused_assignments)]

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("src/username.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

/* all the above code can be written as
using the ? operator, instead of handling the match stmt, for ok or err
the ? operator returns the value of the Ok or Err

 */
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("src/username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let a = read_username_from_file();
    println!("{:?}", a);
    /*
    The unwrap operator basically, unwraps the result type, i.e if it Ok, then result is returned,
    else, if the result type is Err, then Panic is called.

    So, we use expect  method, just to provide custom error message for the panic
    */
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}
