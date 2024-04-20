use std::fs::File;
use std::io::BufReader;

use crate::data::{equation::Equation, indexes::{equation_index::EquationIndex, type_index::{TypeIndex, TypeIndexBuilder}, value_index::ValueIndex}, meta_type::*};

pub fn parse_equations(path: &str) -> EquationIndex {
    let mut result = EquationIndex::new();

    let file = File::open(path).expect("The given type file could not be opened");
    let reader = BufReader::new(file);
    if let serde_json::Value::Object(m) =
        serde_json::from_reader(reader).expect("The given file could not be parse into json")
    {
        for (k, v) in m {
            if let serde_json::Value::String(v) = v {
                result = result.define_equation(&k, Equation::new(v).expect("Syntax error for equation")).expect("Redefined for type");
            } else {
                panic!("Expected string definition of equation");
            }
        }
    }
    result.build()
}

pub fn parse_types(path: &str, mut equations: EquationIndex) -> TypeIndex {
    let mut result = TypeIndex::new();

    result = add_defaults(result);

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
                        serde_json::Value::String(s) => {
                            t = t.define_field(k, string_to_type(s)).unwrap();
                        },
                        serde_json::Value::Array(l) => {
                            let mut enum_vals = vec![];
                            for v in l {
                                if let serde_json::Value::String(s) = v {
                                    enum_vals.push(s);
                                } else {
                                    panic!("Array contains a non-string value")
                                }
                            }
                            t = t.define_field(k, Type::Enum(enum_vals)).unwrap();
                        }
                        serde_json::Value::Object(_) => panic!("Sub-object found"),
                    }
                }
            } else {
                panic!("Couldn't find object for type");
            }
            if let Some(e) = equations.get_equation(&k) {
                result = result.define_type(
                    t.define_field("Value".to_string(), Type::Equation(e))
                    .expect("Value already defined for equation").build()
                ).unwrap();
            }
        }
    } else {
        panic!("Could not find root object");
    }
    result.build()
}

fn add_defaults(builder: TypeIndexBuilder) -> TypeIndexBuilder {
    builder
        // .register_type(
        //     MetaType::new("Num".to_string())
        //     .define_field("Value".to_string(), Type::Num).unwrap().build()
        // ).unwrap()
        // .register_type(
        //     MetaType::new("String".to_string())
        //     .define_field("Text".to_string(), Type::Text).unwrap().build()
        // ).unwrap()
        .define_type(
            MetaType::new("Description".to_string())
            .define_field("Text".to_string(), Type::Text).unwrap()
            .define_field("Keywords".to_owned(), Type::List(Box::new(Type::Text))).unwrap()
            .build()
        ).unwrap()
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
            } else if s.contains("Ref<") && s.contains('>') {
                let s = s.as_str()[(s.find(|c: char| c == '<').unwrap() + 1)..s.find(|c: char| c == '>').unwrap()].to_owned();
                Type::MetaRef(s)
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
                if let Some(type_val) = m.get("Type") {
                    if let serde_json::Value::String(type_str) = type_val {
                        if let Some(meta_type) = types.get_type(&type_str) {
                            result = result.insert(&inst_name, build_instance(meta_type, types, m)).unwrap();
                        } else {
                            panic!("Unknown type given: {}", type_str);
                        }
                    } else {
                        panic!("Expected string def of type");
                    }
                } else {
                    panic!("No type def for {}", inst_name);
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
            _ => {
                if let Some(field_type) = meta_type.get_field_type(&k) {
                    val = val.init_field(k, to_value(v, field_type.clone(), types)).expect("Field already exists");
                }
            }
        }
    }
    val.build(&types)
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
            Type::MetaRef(_) => Value::new_meta_ref(s, t),
            Type::Input(_) => todo!(),
            Type::DieRoll(_) => todo!(),
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