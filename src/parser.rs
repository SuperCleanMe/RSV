use crate::structs::{Content, Entry};
use crate::tokenizer::{Token, Tokens};

pub fn parse(tree: Vec<Token>, delmiter: char, has_headers: bool) -> Content {
    let mut line = String::new();
    let mut body: String = String::new();
    let mut previous: Option<Token> = None;
    let mut index: usize = 0;
    let mut in_head = true;
    let mut headers = Vec::<String>::new();
    let mut row = Vec::<String>::new();
    let mut content = Content::new();
    for token in tree {
        index += 1;
        match token.token_type {
            Tokens::Escape => {}
            Tokens::Delimiter => {
                println!("{}", body);
                line = format!("{}{}", body, token.value);
                if in_head && has_headers {
                    headers.push(body.clone());
                } else {
                    row.push(body.clone());
                }
                body = String::new();
            }
            Tokens::NewLine => {
                println!("{}", line);
                if in_head && has_headers {
                    in_head = false;
                    content.columns = headers.clone();
                } else {
                    content.rows.push(Entry::from_vec(content.headers.clone()))
                }
                content.raw_lines.push(line);
                line = String::new();
                body = String::new();
            }
            _ => {
                // if previous.is_some() {
                //     println!("prv: {} cur: {} escaped: {} type: {:?}", previous.unwrap().value, token.value, token.is_escaped, token.token_type)
                // } else {
                //     println!("prv:   cur: {} escaped: {} type: {:?}", token.value, token.is_escaped, token.token_type)
                // }
                body = format!("{}{}", body, token.value);
                line = format!("{}{}", body, token.value);
            }
        };
        previous = Some(token)
    }
    return content;
}