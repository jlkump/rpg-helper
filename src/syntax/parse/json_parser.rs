use std::fs::File;
use std::io::BufReader;

use crate::data::{equation::Equation, meta_type::*, TypeIndex};

pub fn parse_types(path: &str) -> TypeIndex {
    let mut result = TypeIndex::new();
    let file = File::open(path).expect("The given type file could not be opened");
    let reader = BufReader::new(file);
    if let serde_json::Value::Object(m) = serde_json::from_reader(reader).expect("The given file could not be parse into json") {
        for (k, v) in m {
            let mut t = MetaType::new(k);
            if let serde_json::Value::Object(m) = v {
                for (k, v) in m {
                    match v {
                        serde_json::Value::Null => panic!("Null type found"),
                        serde_json::Value::Bool(_) => panic!("Bool type found"),
                        serde_json::Value::Number(_) => panic!("Numeric type found"),
                        serde_json::Value::String(s) => t.define_field(k, string_to_type(s)),
                        serde_json::Value::Array(l) => {
                            let mut enum_vals = vec![];
                            for v in l {
                                if let serde_json::Value::String(s) = v {
                                    enum_vals.push(s);
                                } else {
                                    panic!("Array contains a non-string value")
                                }
                            }
                            t.define_field(k, Type::Enum(enum_vals));
                        },
                        serde_json::Value::Object(_) => panic!("Sub-object found"),
                    }
                }
            } else {
                panic!("Couldn't find object for type");
            }
            result.register_type(t.build()).expect("Failed to register type, repeat definition of a type");
        }
    }
    result.build()
}

fn string_to_type(s: String) -> Type {
    match s.as_str() {
        "Num" => Type::Num,
        "Number" => Type::Num,
        "Int" => Type::Num,
        "Float" => Type::Num,
        "String" => Type::Text,
        "Text" => Type::Text,
        _ => {
            if s.contains("List<") && s.contains('>') {
                let s = s.as_str()[s.find(|c: char| c == '<').unwrap() ..s.find(|c: char| c == '>').unwrap()].to_owned();
                Type::List(Box::new(string_to_type(s)))
            } else {
                Type::Equation(Equation::new(s).expect("Syntax error in equation"))
            }
        }
    }
} 

// pub fn read_types<'a, 'b>(path: &'b str) -> Vec<MetaType> {
//     let file = File::open(path).expect("Couldn't open file");
//     let reader = BufReader::new(file);
//     if let serde_json::Value::Object(m) = serde_json::from_reader(reader).expect("Couldn't make json") {
//         for (k, v) in m {
//             if let serde_json::Value::Object(m) = v {
//                 println!("{}", define_type(k, m));
//             } else {
//                 panic!();
//             }
//         }
//     } else {
//         panic!();
//     }
//     vec![]
// }

// fn define_type(type_name: String, m: serde_json::Map<String, serde_json::Value>) -> MetaType {
//     let mut fields: Vec<MetaField> = vec![];

//     for (k, v) in m {
//         let field_type: Type = match v {
//             serde_json::Value::Array(a) => Type::Enum(a.into_iter().fold(vec![], |mut b, v| {
//                 if let serde_json::Value::String(s) = v {
//                     b.push(s);
//                     b
//                 } else {
//                     panic!()
//                 }
//             })),
//             serde_json::Value::Bool(_) => panic!(),
//             serde_json::Value::Null => panic!(),
//             serde_json::Value::Number(_) => panic!(),
//             serde_json::Value::Object(_) => panic!(),
//             serde_json::Value::String(s) => get_type(s),
//         };
//         fields.push(MetaType::define_field(k, field_type));
//     }

//     MetaType::new(type_name, fields)
// }

// fn get_type(s: String) -> Type {
//     if s.eq("I32") {
//         return Type::Int
//     }
    
//     if s.eq("String") {
//         return Type::String
//     }

//     if s.contains("List<") && s.contains('>') {
//         // TODO: Error handling for wrong format
//         let it: Vec<&str> = s.split('<').collect();
//         let it: Vec<&str> = it[1].split('>').collect();
//         return Type::List(Box::new(get_type(it[0].to_owned())))
//     }

//     return Type::Meta(s);
// }