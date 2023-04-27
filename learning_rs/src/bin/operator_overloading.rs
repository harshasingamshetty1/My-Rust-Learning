use std::ops::Add;

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

/* Here we are importing the Add trait from the std::ops module.*/
impl Add for Point {
    // this is actually present in the trait Add, so we have to specify it here
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    // The Add trait of std::ops does not have any other functions, so you cant add more functions to it.
    // fn subs(self, other: Point) -> Point {
    //     Point {
    //         x: (self.x - other.x),
    //         y: (self.y - other.y),
    //     }
    // }
}

fn main() {
    let p1 = Point { x: 1.3, y: 4.6 };
    let p2 = Point { x: 3.7, y: 1.4 };
    let p3 = p1 + p2;
    println!("{:?}", p3);
}
