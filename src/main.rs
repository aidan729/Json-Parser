use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LBrace,         // '{'
    RBrace,         // '}'
    LBracket,       // '['
    RBracket,       // ']'
    Colon,          // ':'
    Comma,          // ','
    String(String), // e.g. "hello"
    Number(String), // e.g. "123", "3.14", "-2e10"
    True,           // true
    False,          // false
    Null,           // null
}

#[derive(Debug)]
pub enum LexError {
    InvalidToken(char, usize),  // unrecognized character, position
    UnterminatedString(usize),  // string not closed properly, position
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::InvalidToken(ch, pos) => {
                write!(f, "Invalid token '{}' at position {}", ch, pos)
            }
            LexError::UnterminatedString(pos) => {
                write!(f, "Unterminated string starting at position {}", pos)
            }
        }
    }
}

impl Error for LexError {}

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().enumerate().peekable();

    while let Some((idx, ch)) = chars.next() {
        match ch {
            // whitespace (ignore)
            ' ' | '\n' | '\t' | '\r' => continue,

            // single character tokens
            '{' => tokens.push(Token::LBrace),
            '}' => tokens.push(Token::RBrace),
            '[' => tokens.push(Token::LBracket),
            ']' => tokens.push(Token::RBracket),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),

            // start of a string
            '"' => {
                let mut string_content = String::new();
                let mut terminated = false;

                while let Some((_, c)) = chars.next() {
                    if c == '"' {
                        terminated = true;
                        break;
                    } else if c == '\\' {
                        // handle escape sequences minimally
                        if let Some((_, escaped_char)) = chars.next() {
                            match escaped_char {
                                '"' => string_content.push('"'),
                                '\\' => string_content.push('\\'),
                                'n' => string_content.push('\n'),
                                't' => string_content.push('\t'),
                                'r' => string_content.push('\r'),
                                // for simplicity, handle others as literal
                                other => string_content.push(other),
                            }
                        } else {
                            return Err(LexError::UnterminatedString(idx));
                        }
                    } else {
                        string_content.push(c);
                    }
                }

                if !terminated {
                    return Err(LexError::UnterminatedString(idx));
                }

                tokens.push(Token::String(string_content));
            }

            // could be a boolean literal, 'null', or invalid
            c if c.is_alphabetic() => {
                let mut ident = c.to_string();
                while let Some((_, next_char)) = chars.peek() {
                    if next_char.is_alphabetic() {
                        ident.push(*next_char);
                        chars.next(); // consume
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "true" => tokens.push(Token::True),
                    "false" => tokens.push(Token::False),
                    "null" => tokens.push(Token::Null),
                    _ => return Err(LexError::InvalidToken(c, idx)),
                }
            }

            // number (or minus sign + number)
            c if c.is_ascii_digit() || c == '-' => {
                let mut number_str = c.to_string();

                // check next chars for digits, '.', 'e', 'E', sign in exponent, etc.
                while let Some((_, next_char)) = chars.peek() {
                    if next_char.is_ascii_digit()
                        || *next_char == '.'
                        || *next_char == 'e'
                        || *next_char == 'E'
                        || *next_char == '+'
                        || *next_char == '-'
                    {
                        number_str.push(*next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Number(number_str));
            }

            // anything else is invalid
            other => return Err(LexError::InvalidToken(other, idx)),
        }
    }

    Ok(tokens)
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEndOfTokens,
    UnexpectedToken(Token),
    InvalidNumber(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedEndOfTokens => {
                write!(f, "Unexpected end of tokens (incomplete JSON)")
            }
            ParseError::UnexpectedToken(token) => {
                write!(f, "Unexpected token: {:?}", token)
            }
            ParseError::InvalidNumber(num_str) => {
                write!(f, "Invalid number: {:?}", num_str)
            }
        }
    }
}

impl Error for ParseError {}


pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    pub fn parse_json(&mut self) -> Result<JsonValue, ParseError> {
        let value = self.parse_value()?;
        // should be at end after one top-level value
        if self.position < self.tokens.len() {
            return Err(ParseError::UnexpectedToken(
                self.tokens[self.position].clone(),
            ));
        }
        Ok(value)
    }

    fn parse_value(&mut self) -> Result<JsonValue, ParseError> {
        let token = self.current_token().ok_or(ParseError::UnexpectedEndOfTokens)?;

        match token {
            Token::LBrace => self.parse_object(),
            Token::LBracket => self.parse_array(),
            Token::String(s) => {
                let result = JsonValue::String(s.clone());
                self.advance(); // consume
                Ok(result)
            }
            Token::Number(num_str) => {
                // attempt to parse as f64
                let number = num_str
                    .parse::<f64>()
                    .map_err(|_| ParseError::InvalidNumber(num_str.clone()))?;
                self.advance();
                Ok(JsonValue::Number(number))
            }
            Token::True => {
                self.advance();
                Ok(JsonValue::Bool(true))
            }
            Token::False => {
                self.advance();
                Ok(JsonValue::Bool(false))
            }
            Token::Null => {
                self.advance();
                Ok(JsonValue::Null)
            }
            other => Err(ParseError::UnexpectedToken(other.clone())),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, ParseError> {
        // current token is '{'
        self.advance(); // consume '{'
        let mut map = HashMap::new();

        // if next is '}', it's an empty object
        if let Some(Token::RBrace) = self.current_token() {
            self.advance(); // consume '}'
            return Ok(JsonValue::Object(map));
        }

        // otherwise parse key-value pairs
        loop {
            // expect a string key
            let key_token = self.current_token().ok_or(ParseError::UnexpectedEndOfTokens)?;
            let key = match key_token {
                Token::String(s) => s.clone(),
                _ => return Err(ParseError::UnexpectedToken(key_token.clone())),
            };
            self.advance(); // consume key

            // expect a colon
            match self.current_token() {
                Some(Token::Colon) => self.advance(),
                Some(other) => return Err(ParseError::UnexpectedToken(other.clone())),
                None => return Err(ParseError::UnexpectedEndOfTokens),
            }

            // parse value
            let value = self.parse_value()?;
            map.insert(key, value);

            // next token must be ',' or '}'
            match self.current_token() {
                Some(Token::Comma) => {
                    self.advance(); // consume ','
                }
                Some(Token::RBrace) => {
                    self.advance(); // consume '}'
                    break;
                }
                Some(other) => return Err(ParseError::UnexpectedToken(other.clone())),
                None => return Err(ParseError::UnexpectedEndOfTokens),
            }
        }

        Ok(JsonValue::Object(map))
    }

    fn parse_array(&mut self) -> Result<JsonValue, ParseError> {
        // current token is '['
        self.advance(); // consume '['
        let mut arr = Vec::new();

        // if next is ']', empty array
        if let Some(Token::RBracket) = self.current_token() {
            self.advance(); // consume ']'
            return Ok(JsonValue::Array(arr));
        }

        // otherwise parse elements
        loop {
            let value = self.parse_value()?;
            arr.push(value);

            match self.current_token() {
                Some(Token::Comma) => {
                    self.advance(); // consume ','
                }
                Some(Token::RBracket) => {
                    self.advance(); // consume ']'
                    break;
                }
                Some(other) => return Err(ParseError::UnexpectedToken(other.clone())),
                None => return Err(ParseError::UnexpectedEndOfTokens),
            }
        }

        Ok(JsonValue::Array(arr))
    }
}

// -------------------------

pub fn parse_json_str(input: &str) -> Result<JsonValue, Box<dyn Error>> {
    let tokens = tokenize(input)?;     
    let mut parser = Parser::new(tokens);
    let json_value = parser.parse_json()?; 
    Ok(json_value)
}

fn main() {
    let sample = r#"
    {
        "name": "Alice",
        "age": 30,
        "married": false,
        "children": null,
        "pets": ["Cat", "Dog"],
        "address": {
            "city": "Wonderland",
            "zip": "12345"
        }
    }
    "#;

    match parse_json_str(sample) {
        Ok(json_value) => {
            println!("Successfully parsed JSON!");
            println!("{:#?}", json_value);
        }
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
        }
    }
}