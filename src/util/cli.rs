use std::{io::{self, Write}, slice::Iter};

use crate::{data::{indexes::{type_index::TypeIndex, value_index::ValueIndex}, meta_type::{MetaType, MetaTypeInstance, Type, Value}}, syntax::parse};

// pub fn run_basic_test() -> Result<(), io::Error> {
//     let equations = parse::json_parser::parse_equations("C:\\Users\\lando\\OneDrive\\Documents\\code_projects\\rpg-helper\\data\\test\\basic\\setting\\equations.json");
//     println!("##### Equations ######\n{}", equations);
//     let types = parse::json_parser::parse_types("C:\\Users\\lando\\OneDrive\\Documents\\code_projects\\rpg-helper\\data\\test\\basic\\setting\\types.json", equations);
//     println!("##### Types ######\n{}", types);
//     let mut values = parse::json_parser::parse_values(&types, "C:\\Users\\lando\\OneDrive\\Documents\\code_projects\\rpg-helper\\data\\test\\basic\\character.json");
//     println!("##### Values ######\n{}", values);
    
//     let mut buffer = String::new();
//     let mut should_loop = true;
//     while should_loop {
//         print!(">");
//         io::stdout().flush().unwrap();
//         io::stdin().read_line(&mut buffer)?;
//         let temp = buffer.split(|c: char| c.is_whitespace()).collect::<Vec<&str>>();
//         // let temp = buffer.split_off(buffer.find(|c: char| c.is_whitespace()).unwrap_or(buffer.len()));
//         let mut tokens = temp.iter();
//         if let Some(token) = tokens.next() {
//             match *token {
//                 "get" => {
//                     if let Some(meta_inst) = tokens.next() {
//                         if let Some(field_name) = tokens.next() {
//                             if let Some(v) = values.get_instance(meta_inst) {
//                                 if let Some(fv) = v.get_field_value(field_name) {
//                                     if let Some(num) = fv.as_f32(&v, &values) {
//                                         println!("{}.{} = {}", meta_inst, field_name, num);
//                                     } else if let Some(text) = fv.as_f32(&v, &values) {
//                                         println!("{}.{} = {}", meta_inst, field_name, text);
//                                     } else {
//                                         println!("{}.{} = {}", meta_inst, field_name, fv);
//                                     }
//                                 } else {
//                                     println!("No field \"{}\" on instance", field_name);
//                                 }
//                             } else {
//                                 println!("No instance \"{}\"", meta_inst);
//                             }
//                         } else {
//                             println!("No field query supplied");
//                         }
//                     } else {
//                         println!("No inst query supplied");
//                     }
//                 },
//                 "add" => values = add(tokens, &types, values),
//                 "quit" => should_loop = false,
//                 _ => println!("Unknown command \"{}\"", token),
//             }
//         }
//         buffer.clear();
//     }
//     Ok(())
// }

// enum AddOption {
//     None,
//     AddNum,
//     AddListElem(Type),
// }

// fn add<'a, 'b>(mut tokens: Iter<'b, &str>, types: &'a TypeIndex, mut values: ValueIndex<'a>) -> ValueIndex<'a> {
//     if let Some(meta_inst_name) = tokens.next() {
//         if let Some(field_name) = tokens.next() {
//             let mut add_option = AddOption::None;
//             if let Some(v) = values.get_instance(meta_inst_name).unwrap().get_field_value(field_name) {
//                 add_option = match v.get_type() {
//                     Type::Num => AddOption::AddNum,
//                     Type::Text => {
//                         println!("Can not add text");
//                         AddOption::None
//                     },
//                     Type::List(t) => AddOption::AddListElem(t.as_ref().clone()),
//                     _ => {
//                         println!("Can not add {}", v.get_type());
//                         AddOption::None
//                     }
//                 }
//             } else {
//                 println!("{}.{} does not exist", meta_inst_name, field_name);
//             }
//             match add_option {
//                 AddOption::None => {},
//                 AddOption::AddNum => {
//                     if let Some(old) = values.get_mut_instance(meta_inst_name).unwrap().get_mut_field_value(field_name)
//                             .unwrap()
//                             .as_mut_f32() {
//                         let mut val_str = String::new();
//                         if let Some(val) = tokens.next() {
//                             val_str = val.to_string();
//                         } else {
//                             println!("Input num to add: ");
//                             io::stdin().read_line(&mut val_str).unwrap();
//                         }
//                         while let Err(_) = val_str.parse::<f32>() {
//                             println!("Input must be a numeric value");
//                             println!("Input num to add: ");
//                             io::stdin().read_line(&mut val_str).unwrap();
//                         }
//                         if let Ok(val) = val_str.parse::<f32>() {
//                             *old = *old + val;
//                         }
//                     } else {
//                         println!("Field {} is not a numeric type", field_name);
//                     }
//                 },
//                 AddOption::AddListElem(t) => {
//                     println!("Enter List Elem: ");
//                     let mut elem = None;
//                     while elem.is_none() {
//                         elem = create_value(&t, types);
//                     }
//                     if let Some(mut_list) = values.get_mut_instance(meta_inst_name).unwrap().get_mut_field_value(field_name)
//                             .unwrap()
//                             .as_mut_list() {
//                         mut_list.push(elem.unwrap());
//                     }
//                 },
//             }
//         }
//     } else {
//         println!("Expected target for add");
//     }
//     values
// }

// fn create_value<'a>(t: &Type, types: &'a TypeIndex) -> Option<Value<'a>> {
//     match t {
//         Type::Num => {
//             let mut buffer = String::new();
//             if io::stdin().read_line(&mut buffer).is_ok() {
//                 if let Ok(num) = buffer.parse() {
//                     Some(Value::new_num(num))
//                 } else {
//                     println!("Error reading input, expected numeric value");
//                     None
//                 }
//             } else {
//                 println!("Error reading input");
//                 None
//             }
//         },
//         Type::Text => {
//             let mut buffer = String::new();
//             if io::stdin().read_line(&mut buffer).is_ok() {
//                 Some(Value::new_text(buffer))
//             } else {
//                 println!("Error reading input");
//                 None
//             }
//         },
//         Type::List(_) => todo!(),
//         Type::Enum(var) => {
//             println!("   Available types:");
//             for v in var {
//                 println!("   {}", v);
//             }
//             let mut buffer = String::new();
//             if io::stdin().read_line(&mut buffer).is_ok() {
//                 if let Ok(v) = Value::new_enum(buffer, t.clone()) {
//                     Some(v)
//                 } else {
//                     println!("Invalid variant of enum");
//                     None
//                 }
//             } else {
//                 println!("Error reading input");
//                 None
//             }
//         },
//         Type::Meta(s) => Some(Value::new_meta_instance(
//                     s.to_string(), 
//                     create_meta_instance(types.get_type(&s).unwrap(), 
//                     types))),
//         Type::Equation(_) => todo!(),
//         Type::MetaRef(_) => todo!(),
//         Type::Input(_) => todo!(),
//         Type::DieRoll(_) => todo!(),
//     }
// }

// fn create_meta_instance<'a>(meta_type: &'a MetaType, types: &'a TypeIndex) -> MetaTypeInstance<'a> {
//     let mut m = MetaTypeInstance::new(meta_type);
//     let needed_fields = m.get_needed_fields();
//     println!("Constructing type: {}", meta_type.get_type_name());
//     for (f, t) in needed_fields {
//         let mut final_value = None;
//         while final_value == None {
//             println!("Input value for {}: ", f);
//             let mut buffer = String::new();
//             if io::stdin().read_line(&mut buffer).is_ok() {
//                 final_value = match t {
//                     Type::Num => {
//                         buffer.retain(|c: char| c.is_numeric());
//                         if let Ok(num) = buffer.parse::<f32>() {
//                             Some(Value::new_num(num))
//                         } else {
//                             println!("Input string {} could not be parsed into a number", &buffer);
//                             None
//                         }
//                     },
//                     Type::Text => Some(Value::new_text(buffer.clone())),
//                     Type::List(_) => todo!(),
//                     Type::Enum(_) => todo!(),
//                     Type::Meta(_) => todo!(),
//                     Type::Equation(_) => todo!(),
//                     Type::MetaRef(_) => todo!(),
//                     Type::Input(_) => todo!(),
//                     Type::DieRoll(_) => todo!(),
//                 };
//             } else {
//                 println!("Error while reading input");
//             }
//             buffer.clear();
//         }
//         m = m.init_field(f, final_value.unwrap()).unwrap();
//     }
//     m.build(types)
// }