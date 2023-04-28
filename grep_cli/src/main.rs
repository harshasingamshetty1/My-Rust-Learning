use grep_cli::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // using eprintln, sends to the error stream instead of output stream
        // this is helpful, when we are sending all the output into a file using
        // cargo run > output.txt
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });
    let contents = grep_cli::run(config).unwrap_or_else(|err| {
        eprintln!("Problem reading contents of file: {}", err);
        process::exit(1)
    });
}
