#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Make, Set, When, Repeat, Times, Return, Show, Extern, New, Import,
    Ident(String),
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Equal, DoubleEqual, Greater, Less, Plus, Minus, Star, Slash,
    LParen, RParen, Comma, Dot,
    Newline, Indent, Dedent,
    Eof,
}

pub struct Lexer;

impl Lexer {
    pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut indent_stack = vec![0];
        let lines = input.lines();

        for (line_idx, line) in lines.enumerate() {
            let line_num = line_idx + 1;
            let trimmed = line.trim_end();
            if trimmed.is_empty() || trimmed.trim_start().starts_with('#') {
                continue;
            }

            
            let mut indent_spaces = 0;
            for c in line.chars() {
                if c == ' ' {
                    indent_spaces += 1;
                } else if c == '\t' {
                    return Err(format!(
                        "Line {}: Tabs are not allowed. Please use standard spaces for indentation.",
                        line_num
                    ));
                } else {
                    break;
                }
            }

            let current_indent = *indent_stack.last().unwrap();
            if indent_spaces > current_indent {
                indent_stack.push(indent_spaces);
                tokens.push(Token::Indent);
            } else if indent_spaces < current_indent {
                while let Some(&top) = indent_stack.last() {
                    if indent_spaces < top {
                        indent_stack.pop();
                        tokens.push(Token::Dedent);
                    } else if indent_spaces == top {
                        break;
                    } else {
                        return Err(format!(
                            "Line {}: Indentation level mismatch. Ensure it lines up with parent blocks.",
                            line_num
                        ));
                    }
                }
            }

            
            let mut chars = trimmed[indent_spaces..].chars().peekable();
            while let Some(&c) = chars.peek() {
                match c {
                    ' ' => { chars.next(); }
                    '(' => { tokens.push(Token::LParen); chars.next(); }
                    ')' => { tokens.push(Token::RParen); chars.next(); }
                    ',' => { tokens.push(Token::Comma); chars.next(); }
                    '.' => { tokens.push(Token::Dot); chars.next(); }
                    '=' => {
                        chars.next();
                        if chars.peek() == Some(&'=') {
                            tokens.push(Token::DoubleEqual);
                            chars.next();
                        } else {
                            tokens.push(Token::Equal);
                        }
                    }
                    '>' => { tokens.push(Token::Greater); chars.next(); }
                    '<' => { tokens.push(Token::Less); chars.next(); }
                    '+' => { tokens.push(Token::Plus); chars.next(); }
                    '-' => { tokens.push(Token::Minus); chars.next(); }
                    '*' => { tokens.push(Token::Star); chars.next(); }
                    '/' => { tokens.push(Token::Slash); chars.next(); }
                    '"' => {
                        chars.next();
                        let mut s = String::new();
                        let mut closed = false;
                        while let Some(sc) = chars.next() {
                            if sc == '"' {
                                closed = true;
                                break;
                            }
                            s.push(sc);
                        }
                        if !closed {
                            return Err(format!("Line {}: Unterminated string literal.", line_num));
                        }
                        tokens.push(Token::Str(s));
                    }
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let mut ident = String::new();
                        while let Some(&ic) = chars.peek() {
                            if ic.is_alphanumeric() || ic == '_' {
                                ident.push(ic);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        match ident.as_str() {
                            "make" => tokens.push(Token::Make),
                            "set" => tokens.push(Token::Set),
                            "when" => tokens.push(Token::When),
                            "repeat" => tokens.push(Token::Repeat),
                            "times" => tokens.push(Token::Times),
                            "return" => tokens.push(Token::Return),
                            "show" => tokens.push(Token::Show),
                            "extern" => tokens.push(Token::Extern),
                            "new" => tokens.push(Token::New),
                            "import" => tokens.push(Token::Import),
                            "true" => tokens.push(Token::Bool(true)),
                            "false" => tokens.push(Token::Bool(false)),
                            _ => tokens.push(Token::Ident(ident)),
                        }
                    }
                    '0'..='9' => {
                        let mut num_str = String::new();
                        let mut is_float = false;
                        while let Some(&nc) = chars.peek() {
                            if nc.is_ascii_digit() {
                                num_str.push(nc);
                                chars.next();
                            } else if nc == '.' {
                                is_float = true;
                                num_str.push(nc);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        if is_float {
                            let val: f64 = num_str.parse().map_err(|_| format!("Invalid float value: {}", num_str))?;
                            tokens.push(Token::Float(val));
                        } else {
                            let val: i64 = num_str.parse().map_err(|_| format!("Invalid integer value: {}", num_str))?;
                            tokens.push(Token::Int(val));
                        }
                    }
                    '#' => break,
                    _ => return Err(format!("Line {}: Unexpected token sequence starting with '{}'", line_num, c)),
                }
            }
            tokens.push(Token::Newline);
        }

        while indent_stack.len() > 1 {
            indent_stack.pop();
            tokens.push(Token::Dedent);
        }
        tokens.push(Token::Eof);
        Ok(tokens)
    }
}