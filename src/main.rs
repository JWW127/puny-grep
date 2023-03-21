use std::env;
use std::process;

use puny_grep::Config;

fn main() {
    // get env args and collect
    // need to provide types for collect()
    let args: Vec<String> = env::args().collect();

    // assign/call build method whose arg are env args
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing args: {err}");
        process::exit(1);
    });

    // if run(config) == Err(e) do what is in braces
    if let Err(e) = puny_grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
