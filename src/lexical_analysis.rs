mod token_buffer;
mod transition_table;
mod dfa;
mod token;
mod constants;

use std::fs::File;
use std::io::Read;

use transition_table::init_transition_table;
pub use token::{Token, InvalidToken, LexicalError, ParsedToken};
use token_buffer::TokenBuffer;

pub fn perform_lexical_analysis(filename: &String) -> Result<Vec<ParsedToken>, LexicalError> {    
    // Initialize state table
    let state_table = init_transition_table();

    // Open file
    let mut file = File::open(filename).map_err(|e| LexicalError::FileOpenError(e.to_string()))?;

    // Create double buffer
    let mut token_buffer = TokenBuffer::new();

    // Clear first buffer
    for i in 0..constants::BUFFER_SIZE {
        token_buffer.buffers[0][i] = 0;
    }
    // Read first buffer
    let read_size = file.read(&mut token_buffer.buffers[0]).map_err(|e| LexicalError::FileReadError(e.to_string()))?;
    if read_size == 0 {
        // Empty file
        return Err(LexicalError::EmptyFile);
    }

    let mut tokens: Vec<ParsedToken> = Vec::new();
    let mut invalid_tokens: Vec<InvalidToken> = Vec::new();

    let mut prev_buffer = 1;
    loop {
        // Check if next buffer should be read
        if token_buffer.lb_buffer == token_buffer.f_buffer && prev_buffer != token_buffer.lb_buffer {
            // Clear prev buffer
            for i in 0..constants::BUFFER_SIZE {
                token_buffer.buffers[prev_buffer][i] = 0;
            }

            // Read next buffer
            file.read(&mut token_buffer.buffers[prev_buffer]).map_err(|e| LexicalError::FileReadError(e.to_string()))?;
            prev_buffer = token_buffer.lb_buffer;
        }

        match token_buffer.get_next_token(&state_table) {
            Ok(token) => {
                tokens.push(token);
            },
            Err(e) => {
                match e {
                    LexicalError::EndOfFile => break,
                    LexicalError::InvalidTokens(inv_token) => {
                        invalid_tokens.extend(inv_token);
                    },
                    _ => return Err(e),
                }
            }
        };
    }

    if !invalid_tokens.is_empty() {
        return Err(LexicalError::InvalidTokens(invalid_tokens));
    }

    if tokens.is_empty() {
        return Err(LexicalError::NoValidTokens);
    }
    Ok(tokens)
}