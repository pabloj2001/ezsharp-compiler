mod ll1_table;
mod productions;
mod first_set;
mod follow_set;
mod symbols;

use crate::{
    lexical_analysis::Token,
    syntax_analysis::ll1_table::generate_ll1_table,
    syntax_analysis::ll1_table::LL1Table
};

pub fn perform_syntax_analysis(tokens: Vec<Token>) {
    println!("Performing syntax analysis");
    let table: LL1Table = generate_ll1_table();
}