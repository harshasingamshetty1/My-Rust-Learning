struct RustDev {
    awesome: bool,
}

struct JavaDev {
    awesome: bool,
}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {
        println!("Hello world!")
    }
}

impl RustDev {
    fn specific_function(&self) {
        // implementation for RustDev
        println!("This function is only for Rust developers!");
    }
}

impl JavaDev {
    fn specific_function(&self) {
        println!("This function is only for Java developers!");
    }
}
impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome }
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("println!(\"Hello world!\");");
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev { awesome }
    }

    fn language(&self) -> &str {
        "Java 1.8"
    }

    fn say_hello(&self) {
        println!("System.out.println(\"Hello world!\");");
    }
}

fn main() {
    let r = RustDev::new(true);
    let j = JavaDev::new(false);
    println!("{}", r.language());
    r.say_hello();
    println!("{}", j.language());
    j.say_hello();
    r.specific_function(); // call the specific function for RustDev
    j.specific_function(); // call the specific function for JavaDev
}

// So we can have traits implemented on to structs, also we can have separate impl for each structs differently as well
