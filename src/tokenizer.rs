use nom::Input;
use regex::Regex;
use std::str;

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Operator(char),
    Number(i32),
    StringLitteral(String),
    Type(String, bool),
    OpenBracket,
    CloseBracket,
    OpenParenthesis,
    CloseParenthesis,
    Return,
    Semicolon,
    LineEnd,
}

#[derive(Debug, Clone)]
struct Tokenizer<'a> {
    it: str::Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            it: input.iter_elements(),
            current_char: None,
        }
    }

    fn next(&mut self) -> Option<char> {
        self.current_char = self.it.next();
        self.current_char
    }

    fn parse_word(&mut self) -> Option<Token> {
        let mut str = self.current_char.unwrap().to_string();
        while let Some(c) = self.next() {
            match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' => str.push(c),
                _ => break,
            }
        }
        if str.is_empty() {
            None
        } else {
            Some(if Regex::new(r"int|char|void").unwrap().is_match(&str) {
                if self.current_char == Some('*') {
                    self.next();
                    Token::Type(str, true)
                } else {
                    Token::Type(str, false)
                }
            } else {
                if Regex::new(r"[0-9]+").unwrap().is_match(&str) {
                    Token::Number(str.parse::<i32>().unwrap())
                } else {
                    Token::Identifier(str)
                }
            })
        }
    }

    fn parse_string(&mut self) -> Option<Token> {
        let mut str = String::new();
        while let Some(c) = self.next() {
            match c {
                '"' => break,
                _ => str.push(c),
            }
        }

        if self.current_char.is_none() {
            None
        } else {
            self.skip(Token::StringLitteral(str))
        }
    }

    fn skip(&mut self, token: Token) -> Option<Token> {
        self.next();
        Some(token)
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        self.next();
        while let Some(c) = self.current_char {
            let token = match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' => self.parse_word(),
                '=' | '+' | '-' | '*' | '/' => self.skip(Token::Operator(c)),
                ';' => self.skip(Token::Semicolon),
                '\n' => self.skip(Token::LineEnd),
                '{' => self.skip(Token::OpenBracket),
                '}' => self.skip(Token::CloseBracket),
                '(' => self.skip(Token::OpenParenthesis),
                ')' => self.skip(Token::CloseParenthesis),
                '"' => self.parse_string(),
                ' ' => {
                    self.next();
                    continue;
                }
                _ => None,
            };

            if token.is_some() {
                tokens.push(token.unwrap());
            } else {
                println!(
                    "Error while tokenizing string, current char: '{}', remaining: \"{}\"",
                    self.current_char.unwrap(),
                    self.it.as_str()
                );
                break;
            }
        }
        tokens
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    Tokenizer::new(input).tokenize()
}
