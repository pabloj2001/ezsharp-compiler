mod lexical_analysis;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No input file provided");
    }

    let filename = &args[1];

    // Perform lexical analysis on the file
    match lexical_analysis::perform_lexical_analysis(filename) {
        Ok(tokens) => println!("Lexical analysis completed successfully: {:?}", tokens),
        Err(e) => println!("Lexical Error: {:?}", e),
    }
}
