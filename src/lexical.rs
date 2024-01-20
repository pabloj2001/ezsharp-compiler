use std::fmt::Debug;
use std::fs::File;
use std::io::Read;

use crate::regular_expressions::StateTable;
use crate::regular_expressions::init_state_table;

const BUFFER_SIZE: usize = 1024;

#[derive(Debug)]
pub struct InvalidToken {
    lexeme: String,
    line: usize,
}

#[derive(Debug)]
pub enum LexicalError {
    FileOpenError(String),
    FileReadError(String),
    InvalidTokens(Vec<InvalidToken>),
    EndOfFile,
}

struct TokenBuffer {
    buffers: [[u8; BUFFER_SIZE]; 2],
    lexeme_begin: usize,
    forward: usize,
    lb_buffer: usize,
    f_buffer: usize,
}

impl TokenBuffer {
    fn get_next_token(&mut self, state_table: &StateTable) -> Result<Token, LexicalError> {
        // Read the next character in the buffer
        let mut c = self.advance_forward();
    
        // Skip whitespace and comments
        let mut is_comment = false;
        let mut is_block_comment = false;
        while c.is_whitespace() || is_comment || is_block_comment || c == '/' {
            if is_comment {
                if c == '\n' {
                    is_comment = false;
                    self.set_sentinels(self.forward);
                }
                c = self.advance_forward();
                continue;
            }
            
            if is_block_comment {
                if c == '*' {
                    let next_c = self.buffers[self.f_buffer][self.forward] as char;
                    if next_c == '/' {
                        is_block_comment = false;
                        self.set_sentinels(self.forward + 1);
                    }
                }
                c = self.advance_forward();
                continue;
            }

            if c == '/' {
                let next_c = self.buffers[self.f_buffer][self.forward] as char;
                if next_c == '/' {
                    is_comment = true;
                    self.advance_forward();
                    c = self.advance_forward();
                    continue;
                } else if next_c == '*' {
                    is_block_comment = true;
                    self.advance_forward();
                    c = self.advance_forward();
                    continue;
                }
            }

            c = self.buffers[self.f_buffer][self.forward] as char;
            self.set_sentinels(self.forward);
        }
    
        let mut states: Vec<usize> = vec![0];
        let mut state = 0;
        while let Some(new_state) = state_table.get_next_state(state, c) {
            state = new_state;
            states.push(state);
    
            // Read the next character in the buffer
            c = self.buffers[self.f_buffer][self.forward] as char;
            self.forward += 1;
        }
    
        Ok(Token::Identifier("".to_string()))
    }

    fn set_sentinels(&mut self, loc: usize) {
        self.lexeme_begin = loc;
        self.forward = loc;

        if self.lexeme_begin / BUFFER_SIZE != self.lb_buffer {
            self.lb_buffer = (self.lb_buffer / BUFFER_SIZE) % 2;
            self.f_buffer = self.lb_buffer;
        }
    }

    /**
     * Get character at current forward position and advance forward
     */
    fn advance_forward(&mut self) -> char {
        let c = self.buffers[self.f_buffer][self.forward] as char;
        self.forward += 1;
        if self.forward == BUFFER_SIZE {
            self.forward = 0;
            self.f_buffer = (self.f_buffer + 1) % 2;
        }
        return c;
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Tint(i32),
    Tdouble(f64),
    Kif,
    Kelse,
    Kfi,
    Kwhile,
    Kdo,
    Kod,
    Kreturn,
    Kand,
    Kor,
    Knot,
    Kint,
    Kdouble,
    Oplus,
    Ominus,
    Omultiply,
    Odivide,
    Omod,
    Oassign,
    Oequal,
    Olt,
    Olte,
    Ogt,
    Ogte,
    Onot,
    Scomma,
    Ssemicolon,
    Speriod,
    Slparen,
    Srparen,
}

pub fn lexical_analysis(filename: &String) -> Result<(), LexicalError> {    
    // Initialize state table
    let state_table = init_state_table().map_err(|e| dbg!(e)).expect("State table initialization failed");

    // Open file
    let mut file = File::open(filename).map_err(|e| LexicalError::FileOpenError(e.to_string()))?;

    // Create double buffer
    let mut token_buffer = TokenBuffer {
        buffers: [[0; BUFFER_SIZE]; 2],
        lexeme_begin: 0,
        forward: 0,
        lb_buffer: 0,
        f_buffer: 0,
    };

    // Read first buffer
    let read_size = file.read(&mut token_buffer.buffers[0]).map_err(|e| LexicalError::FileReadError(e.to_string()))?;
    if read_size == 0 {
        // Empty file
        return Err(LexicalError::EndOfFile);
    }

    let mut invalid_tokens: Vec<InvalidToken> = Vec::new();

    let mut prev_buffer = 1;
    loop {
        // Check if next buffer should be read
        if token_buffer.lb_buffer == token_buffer.f_buffer && prev_buffer != token_buffer.lb_buffer {
            file.read(&mut token_buffer.buffers[prev_buffer]).map_err(|e| LexicalError::FileReadError(e.to_string()))?;
            prev_buffer = token_buffer.lb_buffer;
        }

        match token_buffer.get_next_token(&state_table) {
            Ok(token) => {
                dbg!(token);
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

    if invalid_tokens.len() > 0 {
        return Err(LexicalError::InvalidTokens(invalid_tokens));
    }

    Ok(())
}