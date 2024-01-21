use std::fmt::Debug;
use std::fs::File;
use std::io::Read;

use crate::regular_expressions::StateTable;
use crate::regular_expressions::init_state_table;

const BUFFER_SIZE: usize = 4;

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
    lines_read: usize,
}

impl TokenBuffer {
    fn new() -> TokenBuffer {
        TokenBuffer {
            buffers: [[0; BUFFER_SIZE]; 2],
            lexeme_begin: 0,
            forward: 0,
            lb_buffer: 0,
            f_buffer: 0,
            lines_read: 0,
        }
    }

    fn get_next_token(&mut self, state_table: &StateTable) -> Result<Token, LexicalError> {
        // Read the next character in the buffer
        let mut c = self.get_forward_char();
    
        // Skip whitespace and comments
        let mut is_comment = false;
        let mut is_block_comment = false;
        while c.is_whitespace() || is_comment || is_block_comment || c == '/' {
            if is_comment {
                if c == '\n' {
                    is_comment = false;
                    self.lines_read += 1;
                }
                c = self.advance_sentinels();
                continue;
            }
            
            if is_block_comment {
                if c == '*' {
                    let next_c = self.get_char_after_forward();
                    if next_c == '/' {
                        is_block_comment = false;
                        c = self.set_sentinels(self.forward + 2); // Skip the asterisk and slash
                    }
                } else {
                    if c == '\n' {
                        self.lines_read += 1;
                    }
                    c = self.advance_sentinels();
                }
                continue;
            }

            if c == '/' {
                // Check next character
                let next_c = self.get_char_after_forward();
                // If comment, move to the character after // or /*
                if next_c == '/' {
                    is_comment = true;
                    c = self.set_sentinels(self.forward + 2); // Skip the two slashes
                    continue;
                } else if next_c == '*' {
                    is_block_comment = true;
                    c = self.set_sentinels(self.forward + 2); // Skip the slash and asterisk
                    continue;
                }
            }

            // If whitespace, skip
            if c == '\n' {
                self.lines_read += 1;
            }
            c = self.advance_sentinels();
        }

        if c == '\0' {
            return Err(LexicalError::EndOfFile);
        }
    
        println!("First c: {}", c);

        let mut states: Vec<usize> = vec![];
        let mut state = 0;
        while let Some(new_state) = state_table.get_next_state(state, c) {
            state = new_state;
            if states.len() == 0 || state != states.last().unwrap().clone() {
                states.push(state);
            }
    
            // Read the next character in the buffer
            c = self.advance_forward();
        }

        println!("Last c: {}", c);

        // Check if the last state is an accepting state
        if state_table.is_accepting(state) {
            let lexeme = self.get_lexeme();
            match state_table.get_token(states) {
                Some(token) => {
                    // Get lexeme and advance sentinels
                    match token {
                        Token::Identifier(_) => {
                            println!("Identifier: {}", lexeme);
                            return Ok(Token::Identifier(lexeme));
                        },
                        Token::Tint(_) => {
                            // Convert lexeme to integer and return as token
                            if let Ok(int) = lexeme.parse::<i32>() {
                                return Ok(Token::Tint(int));
                            } else {
                                return Err(LexicalError::InvalidTokens(vec![InvalidToken { lexeme, line: self.lines_read + 1 }]));
                            }
                        },
                        Token::Tdouble(_) => {
                            // Convert lexeme to double and return as token
                            if let Ok(double) = lexeme.parse::<f64>() {
                                return Ok(Token::Tdouble(double));
                            } else {
                                return Err(LexicalError::InvalidTokens(vec![InvalidToken { lexeme, line: self.lines_read + 1 }]));
                            }
                        },
                        _ => {
                            // Return token as is
                            return Ok(token);
                        },
                    }
                },
                None => {
                    // Invalid token
                    return Err(LexicalError::InvalidTokens(vec![InvalidToken { lexeme, line: self.lines_read + 1 }]));
                }
            }
        } else {
            // Invalid token
            let lexeme = self.get_lexeme();

            if states.len() == 0 {
                self.advance_sentinels();
            }

            return Err(LexicalError::InvalidTokens(vec![InvalidToken { lexeme, line: self.lines_read + 1 }]));
        }
    }

    fn set_sentinels(&mut self, loc: usize) -> char {
        self.lexeme_begin = loc;
        self.forward = loc;
        if self.lexeme_begin >= BUFFER_SIZE {
            self.lexeme_begin = self.lexeme_begin % BUFFER_SIZE;
            self.forward = self.lexeme_begin;
            self.lb_buffer = (self.lb_buffer + 1) % 2;
            self.f_buffer = self.lb_buffer;
        }
        return self.get_forward_char();
    }

    fn advance_sentinels(&mut self) -> char {
        self.set_sentinels(self.forward + 1)
    }

    fn advance_forward(&mut self) -> char {
        self.forward += 1;
        if self.forward == BUFFER_SIZE {
            self.forward = 0;
            self.f_buffer = (self.f_buffer + 1) % 2;
        }
        return self.get_forward_char();
    }

    fn get_forward_char(&self) -> char {
        return self.buffers[self.f_buffer][self.forward] as char;
    }

    fn get_char_after_forward(&self) -> char {
        let mut next_forward = self.forward + 1;
        let mut next_buffer = self.f_buffer;
        if next_forward == BUFFER_SIZE {
            next_forward = 0;
            next_buffer = (next_buffer + 1) % 2;
        }
        return self.buffers[next_buffer][next_forward] as char;
    }

    /**
     * TODO: ERROR HERE
     * Get the lexeme from the lexeme begin to the forward sentinels and set the sentinels to the position of forward.
     */
    fn get_lexeme(&mut self) -> String {
        let mut lexeme: String;
        if self.lb_buffer == self.f_buffer {
            if self.lexeme_begin == self.forward {
                lexeme = String::from(self.get_forward_char());
            } else {
                lexeme = String::from_utf8_lossy(&self.buffers[self.lb_buffer][self.lexeme_begin..self.forward]).to_string();
            }
        } else {
            lexeme = String::from_utf8_lossy(&self.buffers[self.lb_buffer][self.lexeme_begin..]).to_string();
            lexeme.push_str(&String::from_utf8_lossy(&self.buffers[self.f_buffer][..self.forward]).to_string());
        }
        self.set_sentinels(self.forward);
        return lexeme;
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
    let mut token_buffer = TokenBuffer::new();

    // Clear first buffer
    for i in 0..BUFFER_SIZE {
        token_buffer.buffers[0][i] = 0;
    }
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
            // Clear prev buffer
            for i in 0..BUFFER_SIZE {
                token_buffer.buffers[prev_buffer][i] = 0;
            }

            // Read next buffer
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