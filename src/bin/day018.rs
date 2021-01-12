use std::str::from_utf8;
use std::str::Lines;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
enum TokenKind {
    Add,
    Mul,
    LeftParen,
    RightParen,
    Int(u64),
    EndOfLine,
}

impl TokenKind {
    pub fn precedence(&self) -> u64 {
        match self {
            TokenKind::LeftParen => 1,
            TokenKind::Add => 2,
            TokenKind::Mul => 2,
            TokenKind::RightParen => 1,
            TokenKind::EndOfLine => 99,
            TokenKind::Int(_) => 0,
        }
    }
}

struct Tokens<'a> {
    stream: &'a str,
    curr_pos: usize,
    past_eol: bool,
}

impl<'a> Tokens<'a> {
    pub fn new(s: &'a str) -> Self {
        return Self {
            stream: s,
            curr_pos: 0,
            past_eol: false,
        };
    }
}

impl Iterator for Tokens<'_> {
    type Item = TokenKind;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = TokenKind::EndOfLine;

        let mut token_inprocessing = false;
        let mut token_start = 0;

        let mut char_iter = self.stream[self.curr_pos..].chars().peekable();

        loop {
            let c = char_iter.next();
            if !c.is_some() {
                if self.past_eol == false {
                    self.past_eol = true;
                    return Some(TokenKind::EndOfLine);
                } else {
                    return None;
                }
            }
            let c = c.unwrap();

            if !c.is_whitespace() {
                // If token is in-processing, it must have been Int. No need to check character
                // becuase it's done in the last iteration during look-ahead.
                if !token_inprocessing {
                    token_start = self.curr_pos;
                    match c {
                        '+' => res = TokenKind::Add,
                        '*' => res = TokenKind::Mul,
                        '(' => res = TokenKind::LeftParen,
                        ')' => res = TokenKind::RightParen,
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => res = TokenKind::Int(0), // Set the value to 0 first.
                        _ => panic!("Invalid character."),
                    }
                }

                if matches!(res, TokenKind::Int(_)) {
                    // Need to look ahead to check if the next character is also numeric.
                    match char_iter.peek() {
                        Some(next_c) if next_c.is_numeric() => { token_inprocessing = true }
                        _ => {
                            res = TokenKind::Int(self.stream[token_start..(self.curr_pos + 1)].parse::<u64>().unwrap());
                            token_inprocessing = false;
                        }
                    }
                } else {
                    token_inprocessing = false;
                }

                if token_inprocessing == false {
                    self.curr_pos += 1;
                    return Some(res);
                }
            }

            self.curr_pos += 1;
        }
    }
}

fn part_1(lines: Lines) -> u64 {
    let mut operand_stack = Vec::new();
    let mut operator_stack = Vec::new();

    let mut total = 0;
    for line in lines {
        for tok in Tokens::new(line) {
            // println!("current token: {:?}", tok);
            if matches!(tok, TokenKind::Int(_)) {
                operand_stack.push(tok);
            } else if tok == TokenKind::LeftParen {
                operator_stack.push(tok);
            } else {
                match operator_stack.last() {
                    Some(last_op) => {
                        if *last_op == TokenKind::LeftParen {
                            operator_stack.push(tok);
                        } else {
                            loop {
                                // println!("operand stack: {:?}", operand_stack);
                                // println!("operator stack: {:?}", operator_stack);

                                match operator_stack.last() {
                                    Some(TokenKind::LeftParen) => {
                                        break;
                                    }
                                    Some(TokenKind::Add) => {
                                        let val0 = match operand_stack.pop().unwrap() {
                                            TokenKind::Int(v) => v,
                                            _ => panic!("Invalid token on operand stack."),
                                        };
                                        let val1 = match operand_stack.pop().unwrap() {
                                            TokenKind::Int(v) => v,
                                            _ => panic!("Invalid token on operand stack."),
                                        };
                                        operand_stack.push(TokenKind::Int(val0 + val1));
                                        operator_stack.pop();
                                    }
                                    Some(TokenKind::Mul) => {
                                        let val0 = match operand_stack.pop().unwrap() {
                                            TokenKind::Int(v) => v,
                                            _ => panic!("Invalid token on operand stack."),
                                        };
                                        let val1 = match operand_stack.pop().unwrap() {
                                            TokenKind::Int(v) => v,
                                            _ => panic!("Invalid token on operand stack."),
                                        };
                                        operand_stack.push(TokenKind::Int(val0 * val1));
                                        operator_stack.pop();
                                    }
                                    None => {
                                        break;
                                    }
                                    Some(t) => {
                                        panic!("Invalid token on operator stack: {:?}", t);
                                    }
                                }
                            }

                            if tok == TokenKind::RightParen {
                                operator_stack.pop();
                            } else if tok != TokenKind::EndOfLine {
                                operator_stack.push(tok);
                            }
                        }
                    }
                    None => {
                        operator_stack.push(tok);
                    }
                }
            }
        }

        total += match operand_stack.pop().unwrap() {
            TokenKind::Int(v) => v,
            _ => panic!("The last tok left on operand after evaluation stack is not Int."), 
        };

        operand_stack.clear();
        operator_stack.clear();
    }

    return total;
}

fn main() {
    let input = include_bytes!("day018.input");
    let lines = from_utf8(input).unwrap().lines();
    /*
    let token_iter = Tokens::new(lines.next().unwrap());
    for tok in token_iter {
        println!("Token: {:?}", tok);
    }
    */

    println!("Part 1: {}", part_1(lines));
}
