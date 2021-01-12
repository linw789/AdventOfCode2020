use std::str::from_utf8;
use std::vec::Vec;

struct AstNode {
    token_kind: TokenKind,
    children: Vec<AstNode>,
}

#[derive(Debug)]
enum TokenKind {
    Invalid,
    Add,
    Mul,
    LeftParen,
    RightParen,
    Int(i32),
}

struct Tokens<'a> {
    stream: &'a str,
    curr_pos: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(s: &'a str) -> Self {
        return Self {
            stream: s,
            curr_pos: 0,
        };
    }
}

impl Iterator for Tokens<'_> {
    type Item = TokenKind;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = TokenKind::Invalid;

        let mut token_inprocessing = false;
        let mut token_start = 0;

        let mut char_iter = self.stream[self.curr_pos..].chars().peekable();

        loop {
            let c = char_iter.next();
            if !c.is_some() {
                return None;
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
                            res = TokenKind::Int(self.stream[token_start..(self.curr_pos + 1)].parse::<i32>().unwrap());
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

fn tokens<'a>(s: &'a str) -> Tokens {
    return Tokens::new(s);
}

// expr1 -> expr0 { '+' | '*' expr0 }
// expr0 -> Int | '(' expr1 ')'

fn main() {
    let input = include_bytes!("day018.input");
    let mut lines = from_utf8(input).unwrap().lines();
    let token_iter = tokens(lines.next().unwrap());
    for tok in token_iter {
        println!("Token: {:?}", tok);
    }
}
