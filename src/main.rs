// Used to read CLI arguments
use std::env;
// Used to access the filesystem
use std::process;

// We can import as
use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        // Instead of exiting the program with a panic error, we use a nonzero error code which is
        // convention to signal that a process that called our program had the program exited with
        // an error state
        println!("{}", err);
        process::exit(1)
    });
    if let Err(e) = run(config) {
        println!("{}", e);
        process::exit(1)
    }
}
