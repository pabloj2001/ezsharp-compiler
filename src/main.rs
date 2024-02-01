mod lexical_analysis;
mod logger;

use std::env;

use crate::logger::FileLogAttributes;

fn main() {
    let mut log_folder = String::from("logs");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1].contains(&String::from("--")) {
        panic!("No input file provided");
    }

    let filename = &args[1];
    if !filename.contains(&String::from(".cp")) {
        panic!("Unknown file type. Only .cp files are supported.");
    }

    if args.len() > 2 {
        if args.contains(&String::from("--log-folder")) {
            let log_folder_pos = args.iter().position(|arg| arg == "--log-folder").unwrap() + 1;
            if log_folder_pos >= args.len() {
                panic!("No log folder provided");
            }
            log_folder = args[log_folder_pos].clone();
        }
    }

    // Perform lexical analysis on the file
    match lexical_analysis::perform_lexical_analysis(filename) {
        Ok((tokens, errors)) => {
            logger::log_to_file(
                &tokens,
                &FileLogAttributes::new((log_folder.clone() + "/tokens.log").to_string(), false),
            ).unwrap();
            logger::log_to_file(
                &errors,
                &FileLogAttributes::new((log_folder.clone() + "/lexical_errors.log").to_string(), false),
            ).unwrap();
        },
        Err(e) => println!("Lexical Error: {:?}", e),
    }
}
