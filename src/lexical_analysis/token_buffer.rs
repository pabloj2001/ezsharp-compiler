use super::constants::BUFFER_SIZE;
use super::token::{Token, InvalidToken, LexicalError};
use super::transition_table::TransitionTable;

pub struct TokenBuffer {
    pub buffers: [[u8; BUFFER_SIZE]; 2],
    lexeme_begin: usize,
    forward: usize,
    pub lb_buffer: usize,
    pub f_buffer: usize,
    lines_read: usize,
}

impl TokenBuffer {
    pub fn new() -> TokenBuffer {
        TokenBuffer {
            buffers: [[0; BUFFER_SIZE]; 2],
            lexeme_begin: 0,
            forward: 0,
            lb_buffer: 0,
            f_buffer: 0,
            lines_read: 0,
        }
    }

    pub fn get_next_token(&mut self, transition_table: &TransitionTable) -> Result<Token, LexicalError> {
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

        let mut states: Vec<usize> = vec![];
        let mut state = 0;
        while let Some(new_state) = transition_table.get_next_state(state, c) {
            state = new_state;
            if states.len() == 0 || state != states.last().unwrap().clone() {
                states.push(state);
            }
            
            // Read the next character in the buffer
            c = self.advance_forward();
        }

        // Check if the last state is an accepting state
        if transition_table.is_accepting(state) {
            let lexeme = self.get_lexeme();
            match transition_table.get_token(states) {
                Some(token) => {
                    // Get lexeme and advance sentinels
                    match token {
                        Token::Identifier(_) => {
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
        if loc < self.lexeme_begin {
            self.lexeme_begin = loc;
            self.forward = loc;
            self.lb_buffer = (self.lb_buffer + 1) % 2;
            self.f_buffer = self.lb_buffer;
        } else {
            self.lexeme_begin = loc;
            self.forward = loc;
            if self.lexeme_begin >= BUFFER_SIZE {
                self.lexeme_begin = self.lexeme_begin % BUFFER_SIZE;
                self.forward = self.lexeme_begin;
                self.lb_buffer = (self.lb_buffer + 1) % 2;
                self.f_buffer = self.lb_buffer;
            }
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