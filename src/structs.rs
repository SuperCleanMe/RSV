use std::ops::Index;
use std::collections::HashMap;
use std::collections::hash_map::{Iter, IntoIter};

#[derive(Debug, Clone)]
pub struct Entry {
    index: usize,
    value_iter: Vec<(String, String)>,
    null_val: String,
    pub values: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Content {
    index: usize,
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

        return if v.is_some() {
            v.unwrap()
        } else {
            &self.null_val
        }
    }
}

impl Iterator for Content {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.index;
        self.index += 1;
        return if self.rows.len() - 1 > i {
            Some(self.rows[i].clone())
        } else {
            None
        };
    }
}

impl Iterator for Entry {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.index;
        self.index += 1;
        return if self.value_iter.len() - 1 > i {
            Some(self.value_iter[i].clone())
        } else {
            None
        };
    }
}

impl Content {
    pub fn new() -> Content {
        Content {
            index: 0,
            columns: Vec::new(),
            raw_lines: Vec::new(),
            rows: Vec::new(),
        }
    }
}

impl Entry {
    pub fn from_vec(columns: Vec<String>, headers: Option<Vec<String>>) -> Entry {
        let mut v = Entry {
            index: 0,
            value_iter: Vec::new(),
            null_val: String::new(),
            values: HashMap::new(),
        };
        let h = headers.unwrap_or(Vec::new());
        let mut index = 0;
        for column in columns {
            if h.len() > 0 {
                v.values.insert(h[index].clone(), column.clone());
                v.value_iter.push((h[index].clone(), column));
            } else {
                v.values.insert(index.to_string(), column.clone());
                v.value_iter.push((index.to_string(), column));
            }
            index += 1;
        };
        v
    }
}