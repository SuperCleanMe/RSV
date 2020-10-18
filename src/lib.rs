use std::error::Error;

mod tokenizer;

mod parser;

mod structs;

pub mod prelude;

pub fn parse<A>(content: A, delimiter: char, has_headers: bool) -> structs::Content where A: Into<String> {
    let tree = tokenizer::tokenize(delimiter, content.into());
    let body = parser::parse(tree, delimiter, has_headers);
    println!("{:#?}", body);
    body
}

pub fn read<A>(path: A, delimiter: char, has_headers: bool) -> Result<String, Box<dyn Error>> where A: Into<String> {
    let file = std::fs::read_to_string(path.into());
    return if file.is_ok() {
        parse(file.unwrap(), delimiter, has_headers);
        Ok(String::new())
    } else {
        Err(Box::new(file.unwrap_err()))
    };
}


// CSV sample file is provided free of charge by the Government of New Zealand (https://www.stats.govt.nz/)

#[cfg(test)]
mod tests {
    #[test]
    fn test_parsing() {
        let csv = crate::parse("this,is,a,csv,file\nthis,is,the,second,line", ',', false);
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn test_read_parsing() {
    //     let csv = crate::read("./financialYear19Survey.csv", ',', true).unwrap();
    //     assert_eq!(2 + 2, 4);
    // }
}
