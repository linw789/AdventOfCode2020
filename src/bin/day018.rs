use std::iter::{Enumerate, Peekable};
use std::str::from_utf8;
use std::str::{Chars, Lines};
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
    pub fn precedence_part1(&self) -> u64 {
        match self {
            TokenKind::LeftParen => 1,
            TokenKind::Add => 2,
            TokenKind::Mul => 2,
            TokenKind::RightParen => 4,
            TokenKind::EndOfLine => 99,
            TokenKind::Int(_) => 0,
        }
    }

    pub fn precedence_part2(&self) -> u64 {
        match self {
            TokenKind::LeftParen => 1,
            TokenKind::Add => 2,
            TokenKind::Mul => 3,
            TokenKind::RightParen => 4,
            TokenKind::EndOfLine => 99,
            TokenKind::Int(_) => 0,
        }
    }
}

struct Tokens<'a> {
    stream: &'a str,
    char_iter: Peekable<Enumerate<Chars<'a>>>,
    past_eol: bool,
}

impl<'a> Tokens<'a> {
    pub fn new(s: &'a str) -> Self {
        return Self {
            stream: s,
            char_iter: s.chars().enumerate().peekable(),
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

        loop {
            let c = self.char_iter.next();
            if !c.is_some() {
                if self.past_eol == false {
                    self.past_eol = true;
                    return Some(TokenKind::EndOfLine);
                } else {
                    return None;
                }
            }
            let (c_pos, c) = c.unwrap();

            if !c.is_whitespace() {
                // If token is in-processing, it must have been Int. No need to check character
                // becuase it's done in the last iteration during look-ahead.
                if !token_inprocessing {
                    token_start = c_pos;
                    match c {
                        '+' => res = TokenKind::Add,
                        '*' => res = TokenKind::Mul,
                        '(' => res = TokenKind::LeftParen,
                        ')' => res = TokenKind::RightParen,
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                            res = TokenKind::Int(0)
                        } // Set the value to 0 first.
                        _ => panic!("Invalid character."),
                    }
                }

                if matches!(res, TokenKind::Int(_)) {
                    // Need to look ahead to check if the next character is also numeric.
                    match self.char_iter.peek() {
                        Some((_, next_c)) if next_c.is_numeric() => token_inprocessing = true,
                        _ => {
                            res = TokenKind::Int(
                                self.stream[token_start..(c_pos + 1)]
                                    .parse::<u64>()
                                    .unwrap(),
                            );
                            token_inprocessing = false;
                        }
                    }
                } else {
                    token_inprocessing = false;
                }

                if token_inprocessing == false {
                    return Some(res);
                }
            }
        }
    }
}

fn shunting_yard(line: &str, precedence: fn(&TokenKind, &TokenKind) -> bool) -> u64 {
    let mut operand_stack = Vec::new();
    let mut operator_stack = Vec::new();

    for tok in Tokens::new(line) {
        if matches!(tok, TokenKind::Int(_)) {
            operand_stack.push(tok);
        } else if tok == TokenKind::LeftParen {
            operator_stack.push(tok);
        } else {
            match operator_stack.last() {
                Some(op) => {
                    if *op == TokenKind::LeftParen {
                        operator_stack.push(tok);
                    } else {
                        loop {
                            // println!("current token: {:?}", tok);
                            // println!("operand stack: {:?}", operand_stack);
                            // println!("operator stack: {:?}", operator_stack);

                            if let Some(last_op) = operator_stack.last() {
                                if *last_op == TokenKind::LeftParen {
                                    break;
                                } else if precedence(last_op, &tok) == true {
                                    match *last_op {
                                        TokenKind::Add => {
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
                                        TokenKind::Mul => {
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
                                        _ => {
                                            panic!("Invalid token on operator stack: {:?}", last_op)
                                        }
                                    }
                                } else {
                                    break;
                                }
                            } else {
                                break;
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

    let res = match operand_stack.pop().unwrap() {
        TokenKind::Int(v) => v,
        _ => panic!("The last tok left on operand after evaluation stack is not Int."),
    };

    return res;
}

fn part_1(lines: Lines) -> u64 {
    let precedence = |a: &TokenKind, b: &TokenKind| {
        return a.precedence_part1() <= b.precedence_part1();
    };

    let mut total = 0;
    for line in lines {
        total += shunting_yard(line, precedence);
    }
    return total;
}

fn part_2(lines: Lines) -> u64 {
    let precedence = |a: &TokenKind, b: &TokenKind| {
        return a.precedence_part2() <= b.precedence_part2();
    };

    let mut total = 0;
    for line in lines {
        total += shunting_yard(line, precedence);
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

    println!("Part 1: {}", part_1(lines.clone()));
    println!("Part 2: {}", part_2(lines));
}
