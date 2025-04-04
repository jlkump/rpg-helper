use std::{collections::{BTreeMap, HashMap}, fmt};

use serde::{Deserialize, Serialize};

use crate::model::{core::Reference, database::entity::EntityID};

use super::{Storable, StorableBuilder};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Type
{
    container: EntityID,
    name: String,
    data: EType,
}

impl Storable for Type 
{
    fn to_ref(&self) -> crate::model::core::Reference
    {
        Reference::new(self.container.clone(), self.name.clone())
    }
}

impl Type
{
    pub fn new(name: &str) -> TypeBuilder
    {
        TypeBuilder 
        { 
            name: name.to_string(), 
            data: EType::Number, // Default to number, allow the type to change
            enum_list: Vec::new(),
            struct_map: HashMap::new(),
        }
    }

    pub fn into_builder(self) -> TypeBuilder
    {
        let enum_list;
        if let EType::Enum(e) = &self.data
        {
            enum_list = e.clone();
        }
        else
        {
            enum_list = Vec::new();
        }
        let struct_map;
        if let EType::Struct(s) = &self.data
        {
            struct_map = s.clone();
        }
        else
        {
            struct_map = HashMap::new();
        }
        TypeBuilder { name: self.name, data: self.data, enum_list, struct_map }
    }

    pub fn get_pretty_string(&self, space_prefix: u8) -> String
    {
        let s = match &self.data
        {
            EType::Number => "Number".to_owned(),
            EType::Boolean => "Boolean".to_owned(),
            EType::List(s) => format!("List<{}>", s),
            EType::Enum(types) => format!("Enum {:?}", types),
            EType::Struct(_) => "Struct".to_owned(),
            EType::DieRoll() => "DieRoll".to_owned(),
            EType::Modifier() => "Modifier".to_owned(),
            EType::Equation() => "Equation".to_owned(),
            EType::Reference(s) => format!("Ref<{}>", s),
        };
        let mut res = format!("{}: {}", self.name, s);
        self.data.pretty_string(space_prefix, &mut res);
        res
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum EType
{
    Number,             // Simply a number value, such as Experience
    Boolean,            // Simply a boolean value, such as InAura
    List(String),       // A list of another type
    Enum(Vec<String>),  // A list of available types for a value to take, such as Technique or Form for a subtype of Art
    Struct(HashMap<String, String>), // A collection of types addressable by field-name
    DieRoll(),          // A number that requires input by the user to be calculated
    Modifier(),         // A number that is added to a referenced Value when the condition is true
    Equation(),         // A number or boolean that is calculated based on a given equation, which can reference other Values
    Reference(String),  // A wrapper for the Reference<T>, specifically only targeting Values by a given Type
                        //      For example, the "Spell" Struct type has "Range" as a field, which is a Reference type
                        //      that references specifically a "Range" Struct type.
                        //      All this type does is restrict the Value type's Reference
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct DieRoll
{
    pub num_dice: u8,
    pub num_sides: u16,
    pub special_sides: BTreeMap<u16, DieRollEffect>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct DieRollEffect
{

}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct TypeBuilder
{
    pub name: String,
    data: EType,
    enum_list: Vec<String>,
    struct_map: HashMap<String, String>,
}

impl StorableBuilder<Type> for TypeBuilder
{
    fn build(self, container_id: EntityID, path: String) -> Type 
    {
        Type
        {
            container: container_id,
            name: path,
            data: self.get_data()
        }
    }
}

impl TypeBuilder
{
    pub fn rename(&mut self, new_name: String)
    {
        self.name = new_name;
    }

    pub fn as_number(&mut self)
    {
        self.data = EType::Number;
    }

    pub fn as_boolean(&mut self)
    {
        self.data = EType::Boolean;
    }

    pub fn as_list(&mut self)
    {
        self.data = EType::List("".to_owned());
    }

    pub fn set_list_type(&mut self, s: String)
    {
        self.data = EType::List(s);
    }

    pub fn as_enum(&mut self)
    {
        self.data = EType::Enum(self.enum_list.clone())
    }

    pub fn add_to_enums(&mut self, e: String)
    {
        if !self.enum_list.contains(&e)
        {
            self.enum_list.push(e);
        }
    }

    pub fn remove_from_enums(&mut self, e: String)
    {
        if let Some(index) = self.enum_list.iter().position(|x| *x == e)
        {
            self.enum_list.remove(index);
        }
    }

    pub fn as_struct(&mut self)
    {
        self.data = EType::Struct(self.struct_map.clone())
    }

    pub fn set_struct_field(&mut self, name: String, value: String)
    {
        self.struct_map.insert(name, value);
    }

    pub fn remove_struct_field(&mut self, name: &str)
    {
        self.struct_map.remove(name);
    }

    pub fn as_reference(&mut self)
    {
        self.data = EType::Reference("".to_string());
    }

    pub fn set_reference(&mut self, t: String)
    {
        self.data = EType::Reference(t);
    }

    /// Used for build()
    /// Ensures that the data of EType is what we actually configured
    pub fn get_data(&self) -> EType
    {
        match &self.data
        {
            EType::Number | EType::Boolean | EType::List(_) => self.data.clone(),
            EType::Enum(_) => EType::Enum(self.enum_list.clone()),
            EType::Struct(_) => EType::Struct(self.struct_map.clone()),
            EType::DieRoll() => todo!(),
            EType::Modifier() => todo!(),
            EType::Equation() => todo!(),
            EType::Reference(_) => todo!(),
        }
    }

    pub fn get_pretty_string(&self, space_prefix: u8) -> String
    {
        let s = match &self.data
        {
            EType::Number => "Number".to_owned(),
            EType::Boolean => "Boolean".to_owned(),
            EType::List(s) => format!("List<{}>", s),
            EType::Enum(types) => format!("Enum {:?}", types),
            EType::Struct(_) => "Struct".to_owned(),
            EType::DieRoll() => "DieRoll".to_owned(),
            EType::Modifier() => "Modifier".to_owned(),
            EType::Equation() => "Equation".to_owned(),
            EType::Reference(s) => format!("Ref<{}>", s),
        };
        let mut res = format!("{}: {}", self.name, s);
        self.data.pretty_string(space_prefix, &mut res);
        res
    }
}

impl EType
{
    fn pretty_string(&self, space_prefix: u8, res: &mut String)
    {
        
        if let EType::Struct(d) = &self
        {
            let mut prefix = String::from('\n');
            for _ in 0..space_prefix
            {
                prefix.push(' ');
            }
            for (k, v) in d.iter()
            {
                res.push_str(&format!("{}{}: {}", prefix, k, v));
            }
        }
    }
}

impl fmt::Display for Type
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let s = match &self.data
        {
            EType::Number => "Number".to_owned(),
            EType::Boolean => "Boolean".to_owned(),
            EType::List(s) => format!("List<{}>", s),
            EType::Enum(types) => format!("Enum {:?}", types),
            EType::Struct(_) => "Struct".to_owned(),
            EType::DieRoll() => "DieRoll".to_owned(),
            EType::Modifier() => "Modifier".to_owned(),
            EType::Equation() => "Equation".to_owned(),
            EType::Reference(s) => format!("Ref<{}>", s),
        };
        write!(f, "{}: {}", &self.name, s)
    }
}

impl fmt::Display for TypeBuilder
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let s = match &self.data
        {
            EType::Number => "Number".to_owned(),
            EType::Boolean => "Boolean".to_owned(),
            EType::List(s) => format!("List<{}>", s),
            EType::Enum(types) => format!("Enum {:?}", types),
            EType::Struct(_) => "Struct".to_owned(),
            EType::DieRoll() => "DieRoll".to_owned(),
            EType::Modifier() => "Modifier".to_owned(),
            EType::Equation() => "Equation".to_owned(),
            EType::Reference(s) => format!("Ref<{}>", s),
        };
        write!(f, "{}: {}", &self.name, s)
    }
}