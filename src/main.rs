extern crate tokens;
use std::env;
use std::fs;
use tokens::{Token, TokenCategory};

fn format(filename: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let data = fs::read_to_string(filename)?;
    let token = Token::from(data.as_str());
    let tokens = token.get_tokens();
    let mut indent = 0;
    let mut last: Option<Token> = None;

    for token in tokens.into_iter() {
        if token.string.ends_with("::") {
            print!("{}", token.string);
            println!("");
            indent = 6;
            last = None;
        } else if token.string.ends_with(":") {
            print!("{}", token.string);
            println!("");
            indent = 4;
            last = None;
        } else if token.string.starts_with(";") {
            print!("{}", token.string);
            println!("");
            indent = 2;
            last = None;
        } else if token.string.starts_with("{") || token.string.starts_with("}") {
            let increase = token.string.starts_with("{");
            let decrease = token.string.starts_with("}");

            if decrease {
                indent = 0;
            } else {
                println!("");
            }

            for _ in 0..indent {
                print!(" ");
            }
            print!("{}", token.string);

            if increase {
                indent = 2;
            }
            println!("");
            last = None;
        } else {
            match &last {
                Some(previous) => match (&previous.category, &token.category) {
                    (TokenCategory::Identifier, TokenCategory::Identifier) => {
                        print!(" ");
                    }
                    (TokenCategory::StringLiteral, TokenCategory::Identifier) => {
                        print!(" ");
                    }
                    (TokenCategory::Sequence, _) => {
                        print!(" ");
                    }
                    (_, TokenCategory::Sequence) => {
                        print!(" ");
                    }
                    (_, _) => {}
                },
                None => {
                    if token.category == TokenCategory::StringLiteral {
                        indent = 6;
                    }
                    for _ in 0..indent {
                        print!(" ");
                    }
                }
            };
            print!("{}", token.string);

            last = Some(token);
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = env::args().collect();
    return match args[1].as_str() {
        "format" => format(&args[2]),
        _ => Ok(()),
    };
}
