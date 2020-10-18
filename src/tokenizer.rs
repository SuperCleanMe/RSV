#[derive(Debug, Copy, Clone)]
pub enum Tokens {
    Escape    = 0,
    Delimiter = 1,
    AlphaNum  = 2,
    NewLine   = 3,
}

#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub value: char,
    pub token_type: Tokens,
    pub is_escaped: bool,
    pub is_whitespace: bool,
    pub id: usize,
}

impl Token {
    pub fn from(delimiter: char, character: char, id: usize, previous: Option<&Token>) -> Token {
        let mut t_type = match character {
            '\\' => Tokens::Escape,
            '\n' => Tokens::NewLine,
            _ => Tokens::AlphaNum,
        };
        if character == delimiter {
            t_type = Tokens::Delimiter
        }
        let mut is_escaped: bool = false;
        if previous.is_none() {
            is_escaped = false;
        } else {
            let previous_token = previous.unwrap();
            if previous_token.value == '\\' && previous_token.is_escaped == false {
                is_escaped = true
            }
        }
        Token {
            value: character,
            token_type: t_type,
            is_escaped,
            is_whitespace: character.is_whitespace(),
            id,
        }
    }
}

pub fn tokenize(delimiter: char, content: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for char in content.chars() {
        tokens.push(Token::from(delimiter, char, tokens.len(), tokens.last()))
    };
    return tokens;
}