use regex::{Regex, Error};

pub enum Type {
    Varchar,
    Integer,
    Float,
}

pub struct Schema {
    pub table_name: String,
    pub fields: Vec<(String, Type, i32)>,  // (field name, type, length)
    pub offsets: Vec<(String, i32)>, // (field name, offset from start of row)
    pub row_length: i32
}

impl Schema {
    pub fn new(table_name: String, fields: Vec<(String, Type, i32)>) -> Self {
        Self {
            table_name: table_name,
            row_length: calc_row_length(&fields),
            offsets: calc_offsets(&fields),
            fields: fields
        }
    }

    pub fn insert_regex(&self) -> Result<Regex, Error> {
        let mut re_str = String::from("insert ");
        for (field_name, t, _) in self.fields.iter() {
            re_str.push_str(format!("{}: ", field_name).as_str());
            let type_re = match t {
                Type::Varchar => r"/'\w*/'",
                Type::Integer => r"\d*",
                Type::Float => r"\d*\.\d*"
            };
            re_str.push_str(format!("{}, ", type_re).as_str());
        }

        re_str.pop(); // remove the last ", " 
        re_str.pop();
        re_str.push_str(format!(" into {};", self.table_name).as_str());
        Regex::new(&re_str)
    }

}

fn calc_offsets(fields: &Vec<(String, Type, i32)>) -> Vec<(String, i32)> {
    let mut offsets: Vec<(String, i32)> = Vec::new();
    let mut counter: i32 = 0;
     for (field_name, _, len) in fields.iter() {
        offsets.push((field_name.clone(), counter));
        counter += *len;
    }
    offsets
}

fn calc_row_length(fields: &Vec<(String, Type, i32)>) -> i32 {
    let lengths: Vec<i32> = fields.iter().map(|a| a.2).collect();
    lengths.iter().sum::<i32>()
}




