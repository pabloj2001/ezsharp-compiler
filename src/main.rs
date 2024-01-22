mod lexical;
mod regular_expressions;

use std::env;
use lexical::lexical_analysis;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No input file provided");
    }

    let filename = &args[1];

    // Perform lexical analysis on the file
    match lexical_analysis(filename) {
        Ok(tokens) => println!("Lexical analysis completed successfully: {:?}", tokens),
        Err(e) => println!("Lexical Error: {:?}", e),
    }
}
