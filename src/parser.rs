use crate::structs::{Content, Entry};
use crate::tokenizer::{Token, Tokens};

pub fn parse(tree: Vec<Token>, has_headers: bool) -> Content {
    let mut line = String::new();
    let mut body: String = String::new();
    let mut previous: Option<Token> = None;
    let mut in_head = true;
    let mut in_quote = false;
    let mut headers = Vec::<String>::new();
    let mut row = Vec::<String>::new();
    let mut content = Content::new();
    for token in tree {
        match token.token_type {
            Tokens::Escape => {}
            Tokens::Delimiter => {
                line = format!("{}{}", line, token.value);
                if !in_quote {
                    if in_head && has_headers {
                        headers.push(body.clone());
                    } else {
                        row.push(body.clone());
                    }
                    body = String::new();
                } else {
                    body = format!("{}{}", body, token.value);
                }
            }
            Tokens::NewLine => {
                if !in_quote {
                    if in_head && has_headers {
                        headers.push(body.clone());
                        in_head = false;
                        content.columns = headers.clone();
                    } else {
                        row.push(body.clone());
                        if content.columns.len() > 0 {
                            content.rows.push(Entry::from_vec(row.clone(), Some(content.columns.clone())))
                        } else {
                            content.rows.push(Entry::from_vec(row.clone(), None))
                        }
                    }
                    content.raw_lines.push(line);
                    line = String::new();
                    body = String::new();
                    row = Vec::<String>::new()
                } else {
                    body = format!("{}{}", body, token.value);
                    line = format!("{}{}", line, token.value);
                }
            }
            _ => {
                if token.value != '\r' {
                    if token.value == '"' && previous.is_some() {
                        let prv = previous.unwrap();
                        if prv.value == '"' {
                            body = format!("{}{}", body, token.value);
                            line = format!("{}{}", line, token.value);
                        } else {
                            if in_quote {
                                in_quote = false;
                            } else {
                                in_quote = true;
                            }
                        }
                    }
                    body = format!("{}{}", body, token.value);
                    line = format!("{}{}", line, token.value);
                }
            }
        };
        previous = Some(token)
    }
    if previous.unwrap().value != '\n' {
        row.push(body.clone());
        if content.columns.len() > 0 {
            content.rows.push(Entry::from_vec(row.clone(), Some(content.columns.clone())));
        } else {
            content.rows.push(Entry::from_vec(row.clone(), None));
        };
    }
    return content;
}