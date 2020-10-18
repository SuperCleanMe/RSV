use std::ops::Index;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Entry {
    pub values: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Content {
    pub columns: Vec<String>,
    pub raw_lines: Vec<String>,
    pub rows: Vec<Entry>,
}

impl Index<usize> for Content {
    type Output = Entry;

    fn index(&self, field: usize) -> &Entry {
        &self.rows[field]
    }
}

impl Index<&str> for Entry {
    type Output = String;

    fn index(&self, field: &str) -> &Self::Output {
        let v = self.values.get(field);
        return v.unwrap();
    }
}

impl Content {
    pub fn new() -> Content {
        Content {
            columns: Vec::new(),
            raw_lines: Vec::new(),
            rows: Vec::new(),
        }
    }
}

impl Entry {
    pub fn from_vec(columns: Vec<String>, headers: Option<Vec<String>>) -> Entry {
        let mut v = Entry {
            values: HashMap::new()
        };
        let mut index = 0;
        for column in columns {

        };

    }
}