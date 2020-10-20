#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! ## What is RustSV?
//! <p>
//! RustSV (referred to as RSV) is a CSV parser, built for the modern age.
//!
//! It focuses on usability, and has the advantage of not requiring the use of [Serde](https://github.com/fatalcenturion/RSV#reference-2-serde-free-serialization) to parse your files into a programmatically readable structure.
//!
//! See the source code [here](https://github.com/fatalcenturion/RSV)
//!
//! Found a bug? [report it!](https://github.com/fatalcenturion/RSV/issues/new)
//!
//! ## Basic usage:
//!
//! ### Parsing a string:
//! ```
//! use rustsv::prelude::*;
//! // Create our input data
//! let input: &str = "surname,initial,address,phone number\
//! Smith,A,\"14 Made up Drive, Made up City, Ohio\",216-235-3744\
//! Doe,J,\"15 Fake Street, Phonyville, Texas\",210-214-5737";
//!
//! // Parse the `input` into `Content`
//! // The parameters are as follows:
//! // 1. Input: String   - The text you wish to parse
//! // 2. Delimiter: Char - The character to delimit by
//! // 3. Headers: Bool   - If the parser should use the first row in the file as headers
//! let content: Content = parse(input, ',', true);
//! ```
//! The above method will provide an instance of `Content`
//!
//! ### Parsing a file:
//! ```no_run
//! // Parse the `input` into `Content`
//! // The parameters are as follows:
//! // 1. Input: String   - The text you wish to parse
//! // 2. Delimiter: Char - The character to delimit by
//! // 3. Headers: Bool   - If the parser should use the first row in the file as headers
//! let content: Content = read("path/to/file.csv", ',', true)?;
//! ```
//! The above method will provide a result containing an error, or `Content`
use std::error::Error;

mod tokenizer;

mod parser;

pub mod structs;

pub mod prelude;

#[cfg(feature = "nostd")]
/// Parses the provided String into an instance of `Content`
///
/// The method will deconstruct the provided data, turning it into a special, serialization free structure [`Content`]: structs.Content
///
/// [`content`] the CSV data to parse
/// [`delimiter`] The delimiter used in the data, for example a pipe (`|`) or a tab (`   `)
/// [`has_headers`] If the data's first line contains the titles of each column or not
///
/// [`content`]: #parse.content
/// [`delimiter`]: #parse.delimiter
/// [`has_headers`]: #parse.has_headers
///
/// # Examples
///
/// Basic usage:
/// ```no_run
/// use rustsv::prelude::*;
/// // Create our input data
/// let input: &str = "surname,initial,address,phone number\
/// Smith,A,\"14 Made up Drive, Made up City, Ohio\",216-235-3744\
/// Doe,J,\"15 Fake Street, Phonyville, Texas\",210-214-5737";
///
/// // Parse the `input` into `Content`
/// let content: Content = parse(input, ',', true);
/// ```
pub fn parse<A>(content: A, delimiter: char, has_headers: bool) -> structs::Content where A: Into<String> {
    let tree = tokenizer::tokenize(delimiter, content.into());
    let body = parser::parse(tree, has_headers);
    body
}

#[cfg(feature = "std")]
/// Reads a file and parses it into an instance of `Content`
///
/// The method takes a path to a file, and then deconstructs the data, turning it into a special, serialization free structure [`Content`]: structs.Content
///
/// `content` the CSV data to parse
/// `delimiter` The delimiter used in the data, for example a pipe (`|`) or a tab (`   `)
/// `has_headers` If the data's first line contains the titles of each column or not
///
/// # Examples
///
/// Basic usage:
/// ```no_run
/// use rustsv::prelude::*;
/// // Parse the `input` into `Content`
/// let content: Content = read("./path/to/file.csv", ',', true)?;
/// ```
pub fn read<A>(path: A, delimiter: char, has_headers: bool) -> Result<structs::Content, Box<dyn Error>> where A: Into<String> {
    let file = std::fs::read_to_string(path.into());
    return if file.is_ok() {
        let c = file.unwrap();
        Ok(parse(c, delimiter, has_headers))
    } else {
        Err(Box::new(file.unwrap_err()))
    };
}


#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    // CSV sample file is provided free of charge by EForExcel (http://eforexcel.com/wp/downloads-18-sample-csv-files-data-sets-for-testing-sales/)
    fn test_parsing() {
        let csv = crate::parse("Region,Country,Item Type,Sales Channel,Order Priority,Order Date,Order ID,Ship Date,Units Sold,Unit Price,Unit Cost,Total Revenue,Total Cost,Total Profit\nAustralia and Oceania,Tuvalu,Baby Food,Offline,H,5/28/2010,669165933,6/27/2010,9925,255.28,159.42,2533654.00,1582243.50,951410.50\nCentral America and the Caribbean,Grenada,Cereal,Online,C,8/22/2012,963881480,9/15/2012,2804,205.70,117.11,576782.80,328376.44,248406.36", ',', true);
        assert_eq!(csv[0]["Region"], String::from("Australia and Oceania"));
    }

    #[test]
    // CSV sample file is provided free of charge by EForExcel (http://eforexcel.com/wp/downloads-18-sample-csv-files-data-sets-for-testing-sales/)
    fn test_parsing_iter() {
        let csv = crate::parse("Region,Country,Item Type,Sales Channel,Order Priority,Order Date,Order ID,Ship Date,Units Sold,Unit Price,Unit Cost,Total Revenue,Total Cost,Total Profit\nAustralia and Oceania,Tuvalu,Baby Food,Offline,H,5/28/2010,669165933,6/27/2010,9925,255.28,159.42,2533654.00,1582243.50,951410.50\nCentral America and the Caribbean,Grenada,Cereal,Online,C,8/22/2012,963881480,9/15/2012,2804,205.70,117.11,576782.80,328376.44,248406.36", ',', true);
        assert_eq!(csv[0]["Region"], String::from("Australia and Oceania"));
        for entry in csv {
            for reference in entry {
                println!("{}: {}", reference.0, reference.1);
            }
        }
    }

    #[test]
    fn test_random_func() {
        let input: String = "first,middle,last\ntom,bob,scott".to_string();
        let content: Content = parse(input, ',', true);
        println!("Hello, world! content = {:?} content2 = {:?}", content.get(2).is_some(), content[0]);
        assert_eq!(1, 1);
    }
}
