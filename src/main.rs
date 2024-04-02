mod lexical_analysis;
mod syntax_semantic_analysis;
mod logger;

use crate::logger::FileLogAttributes;

use std::env;
use lexical_analysis::ParsedToken;

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
    let lexical_result = lexical_analysis::perform_lexical_analysis(filename);
    let tokens: Vec<ParsedToken> = match lexical_result {
        Ok(tokens) => {
            logger::log_to_file(
                &tokens,
                &FileLogAttributes::new((log_folder.clone() + "/tokens.log").to_string(), false),
            ).unwrap();
            logger::clear_log_file((log_folder.clone() + "/lexical_errors.log").to_string()).unwrap();
            println!("Lexical analysis completed successfully");
            tokens
        },
        Err(e) => {
            logger::log_to_file(
                &e,
                &FileLogAttributes::new((log_folder.clone() + "/lexical_errors.log").to_string(), false),
            ).unwrap();
            logger::clear_log_file((log_folder.clone() + "/tokens.log").to_string()).unwrap();
            panic!("Lexical errors found. Check logs for more information.");
        },
    };

    // Perform syntax analysis on the file
    let syntax_result = syntax_semantic_analysis::perform_syntax_semantic_analysis(tokens);
    match syntax_result {
        Ok(table) => {
            logger::log_to_file(
                &table,
                &FileLogAttributes::new((log_folder.clone() + "/symbol_table.log").to_string(), false),
            ).unwrap();
            logger::clear_log_file((log_folder.clone() + "/syntax_errors.log").to_string()).unwrap();
            logger::clear_log_file((log_folder.clone() + "/semantic_errors.log").to_string()).unwrap();
            println!("Syntax and Semantic analysis completed successfully");
        },
        Err(e) => {
            if !e.syntax_errors.is_empty() {
                logger::log_to_file(
                    &e.syntax_errors.into_boxed_slice(),
                    &FileLogAttributes::new((log_folder.clone() + "/syntax_errors.log").to_string(), false),
                ).unwrap();
                println!("Syntax errors found. Check logs for more information.");
            } else if !e.semantic_errors.is_empty() {
                logger::log_to_file(
                    &e.semantic_errors.into_boxed_slice(),
                    &FileLogAttributes::new((log_folder.clone() + "/semantic_errors.log").to_string(), false),
                ).unwrap();
                println!("Semantic errors found. Check logs for more information.");
            }
            logger::clear_log_file((log_folder.clone() + "/symbol_table.log").to_string()).unwrap();
        }
    }
}
