use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

pub mod parse;

use crate::data::meta_type::*;
use crate::data::equation::*;

pub fn read_types<'a, 'b>(path: &'b str) -> Vec<MetaType> {
    let file = File::open(path).expect("Couldn't open file");
    let reader = BufReader::new(file);
    if let serde_json::Value::Object(m) = serde_json::from_reader(reader).expect("Couldn't make json") {
        for (k, v) in m {
            if let serde_json::Value::Object(m) = v {
                println!("{}", define_type(k, m));
            } else {
                panic!();
            }
        }
    } else {
        panic!();
    }
    vec![]
}

fn define_type(type_name: String, m: serde_json::Map<String, serde_json::Value>) -> MetaType {
    let mut fields: Vec<MetaField> = vec![];

    for (k, v) in m {
        let field_type: Type = match v {
            serde_json::Value::Array(a) => Type::Enum(a.into_iter().fold(vec![], |mut b, v| {
                if let serde_json::Value::String(s) = v {
                    b.push(s);
                    b
                } else {
                    panic!()
                }
            })),
            serde_json::Value::Bool(_) => panic!(),
            serde_json::Value::Null => panic!(),
            serde_json::Value::Number(_) => panic!(),
            serde_json::Value::Object(_) => panic!(),
            serde_json::Value::String(s) => get_type(s),
        };
        fields.push(MetaType::define_field(k, field_type));
    }

    MetaType::new(type_name, fields)
}

fn get_type(s: String) -> Type {
    if s.eq("I32") {
        return Type::Int
    }
    
    if s.eq("String") {
        return Type::String
    }

    if s.contains("List<") && s.contains('>') {
        // TODO: Error handling for wrong format
        let it: Vec<&str> = s.split('<').collect();
        let it: Vec<&str> = it[1].split('>').collect();
        return Type::List(Box::new(get_type(it[0].to_owned())))
    }

    return Type::Meta(s);
}