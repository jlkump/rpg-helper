use std::fs::File;
use std::io::BufReader;

use crate::data::{equation::Equation, meta_type::*, EquationIndex, TypeIndex, ValueIndex, ValueIndexBuilder};

pub fn parse_equations(path: &str) -> EquationIndex {
    let mut result = EquationIndex::new();

    let file = File::open(path).expect("The given type file could not be opened");
    let reader = BufReader::new(file);
    if let serde_json::Value::Object(m) =
        serde_json::from_reader(reader).expect("The given file could not be parse into json")
    {
        for (k, v) in m {
            if let serde_json::Value::String(v) = v {
                result.insert_equation(&k, Equation::new(v).expect("Syntax error for equation"));
            } else {
                panic!("Expected string definition of equation");
            }
        }
    }
    result.build()
}

pub fn parse_types(path: &str, mut equations: EquationIndex) -> TypeIndex {
    let mut result = TypeIndex::new();

    let file = File::open(path).expect("The given type file could not be opened");
    let reader = BufReader::new(file);
    if let serde_json::Value::Object(m) =
        serde_json::from_reader(reader).expect("The given file could not be parse into json")
    {
        for (k, v) in m {
            let mut t = MetaType::new(k.clone());
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
                        }
                        serde_json::Value::Object(_) => panic!("Sub-object found"),
                    }
                }
            } else {
                panic!("Couldn't find object for type");
            }
            if let Some(e) = equations.get_equation(&k) {
                if t.has_field_defined("Value") {
                    panic!("Equation for type that already has a value defined");
                } else {
                    t.define_field("Value".to_string(), Type::Equation(e));
                }
            }
            result
                .register_type(t.build())
                .expect("Failed to register type, repeat definition of a type");
        }
    } else {
        panic!("Could not find root object");
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
                let s = s.as_str()
                    [(s.find(|c: char| c == '<').unwrap() + 1)..s.find(|c: char| c == '>').unwrap()]
                    .to_owned();
                Type::List(Box::new(string_to_type(s)))
            } else if super::string_contains_op(&s) {
                Type::Equation(Equation::new(s).expect("Syntax error in equation"))
            } else {
                Type::Meta(s)
            }
        }
    }
}

pub fn parse_values<'a>(types: &'a TypeIndex, path: &str) -> ValueIndex<'a> {
    let mut result = ValueIndex::new();

    let file = File::open(path).expect("The given type file could not be opened");
    let reader = BufReader::new(file);
    if let serde_json::Value::Object(m) = serde_json::from_reader(reader)
            .expect("The given file could not be parse into json")
    {
        for (inst_name, v) in m {
            if let serde_json::Value::Object(m) = v {
                if let Some(meta_type) = types.get_type(
                    m.get("Type")
                        .expect("No type definition for value")
                        .as_str()
                        .expect("Expected string definition of type"),
                ) {
                    result.register_value(&inst_name, build_instance(meta_type, types, m));
                } else {
                    panic!("Type for object not registered");
                }
            }
        }
    }
    result.build()
}

fn build_instance<'a>(meta_type: &'a MetaType, types: &'a TypeIndex, m: serde_json::Map<String, serde_json::Value>) -> MetaTypeInstance<'a> {
    let mut val = MetaTypeInstance::new(meta_type);
    for (k, v) in m {
        match k.as_str() {
            "Type" => {}
            _ => {
                if let Some(field_type) = meta_type.get_field_type(&k) {
                    val.init_field(k, to_value(v, field_type.clone(), types)).expect("Field already exists");
                }
            }
        }
    }
    val.build()
}

fn to_value<'a>(val: serde_json::Value, t: Type, types: &'a TypeIndex) -> Value<'a> {
    match val {
        serde_json::Value::Null => panic!(),
        serde_json::Value::Bool(_) => panic!(),
        serde_json::Value::Number(n) => Value::new_num(n.as_f64().unwrap() as f32),
        serde_json::Value::String(s) => match &t {
            Type::Num => Value::new_num(s.parse().expect("Expected number, found string")),
            Type::Text => Value::new_text(s),
            Type::List(_) => panic!("Found list with a string value"),
            Type::Enum(_) => Value::new_enum(s, t).expect("Enum variant not allowed"),
            Type::Equation(_) => panic!("Found string value for equation"),
            Type::Meta(_) => panic!("Expected meta type, found string"),
        },
        serde_json::Value::Array(l) => if let Type::List(sub_type) = &t {
            Value::new_list(l.into_iter().map(|f| to_value(f, sub_type.as_ref().clone(), types)).collect(), t).expect("List values not able to be parsed")
        } else {
            panic!("Found list for field of different type")
        },
        serde_json::Value::Object(m) => {
            if let Type::Meta(meta_type_name) = t {
                let result = build_instance(types.get_type(&meta_type_name).expect("Unknown type"), types, m);
                Value::new_meta_instance(meta_type_name, result)
            } else {
                println!("Type mis-match, expected meta found {} for value {:?}", t, m);
                panic!()
            }
        },
    }
}

// fn matches_type(val: &serde_json::Value, t: &Type) -> bool {
//     match val {
//         serde_json::Value::Null => false,
//         serde_json::Value::Bool(_) => false,
//         serde_json::Value::Number(_) => t == &Type::Num,
//         serde_json::Value::String(s) => {
//             match t {
//                 Type::Num => false,
//                 Type::Text => true,
//                 Type::List(_) => false,
//                 Type::Enum(l) => l.contains(&s),
//                 Type::Equation(_) => true,
//                 Type::Meta(_) => true, // TODO: Check type index
//             }
//         }
//         serde_json::Value::Array(a) => {
//             if let Type::List(l) = t {
//                 if let Some(f) = a.first() {
//                     matches_type(f, l.as_ref())
//                 } else {
//                     true
//                 }
//             } else {
//                 false
//             }
//         }
//         serde_json::Value::Object(_) => panic!(), // Invalid
//     }
// }