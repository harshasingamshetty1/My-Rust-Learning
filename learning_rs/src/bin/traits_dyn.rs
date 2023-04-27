#[allow(unused_variables)]
#[allow(unused_assignments)]

struct Dog {}
struct Cat {}

trait Animal {
    fn make_noise(&self) -> &'static str;
}

impl Animal for Dog {
    fn make_noise(&self) -> &'static str {
        "woof"
    }
}

impl Animal for Cat {
    fn make_noise(&self) -> &'static str {
        "meow"
    }
}

// we cant directly return a trait object, we need to use Box<dyn Animal>
// it is bcoz, rust wants to know the memory beforehand, with out the Box<dyn >,
//  rust wont knw the size, the dyn was introduced lately, it was not present in legacy code.

fn get_animal(rand_number: f64) -> Box<dyn Animal> {
    if rand_number < 1.0 {
        Box::new(Dog {})
    } else {
        Box::new(Cat {})
    }
}

/* we can also use impl Trait syntax but with this,
we can only return one type of Animal, i.e if we return a Dog, we cant return a Cat
therefore we use the box dyn syntax to reeturn any type of animal
*/

// fn get_animal(rand_number: f64) -> impl Animal {
//     if rand_number < 1.0 {
//         Box::new(Dog {})
//     } else {
//         Box::new(Cat {}) // error here
//     }
// }

/* Implementing traits for structs we havent created */

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum = 0;
        for i in self {
            sum += *i;
        }
        sum
    }
}
fn main() {
    // as we are able to retrun a trait, we can acceess all the methods of that trait here
    println!("The animal says {}", get_animal(0.5).make_noise());
    println!("The animal says {}", get_animal(2.0).make_noise());
    let v = vec![1, 2, 3];
    println!("The sum is {}", v.sum());
    let v = vec![1.0, 2.0, 3.0];
    // the sum on this doesnot work bcoz, we have implemented the trait only for the i32 type
    // println!("The sum is {}", v.sum()); //error
}
