mod ll1_table;
mod productions;
mod first_set;
mod follow_set;
mod symbols;

use crate::{lexical_analysis::Token, syntax_analysis::ll1_table::generate_ll1_table};

pub fn perform_syntax_analysis(tokens: Vec<Token>) {
    println!("Performing syntax analysis");
    generate_ll1_table();
}