//////////////////////////////////////
///     Data Model for Tool        ///
//////////////////////////////////////

use std::{collections::HashMap, ops::{Deref, Index}, rc::Rc};

use serde::{Deserialize, Serialize};

// Restrictions are stored Server-side. 
// Client will not have to handle restrictions as only the data allowed will be sent.
pub struct Restrictions {
    // TODO: Allow defaults of: OnlyMe, OnlyGameMasters, AllPlayers
    wiki_permissions: HashMap<String, Vec<Permission>>,      // Permission to see a wiki page in a game
    type_permissions: HashMap<String, Vec<Permission>>,      // Permission to use a type in a game
    character_permissions: HashMap<String, Vec<Permission>>, // Permission to see a character in a game
}

// Might need a way to go from user_id to Permissions
#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct Permission {
    user_id: uuid::Uuid,
    read_permissions: bool,
    write_permissions: bool, // List of allowed players by User-id
}

pub struct GameMaster {
    user_id: uuid::Uuid,
}

pub struct Character {
    owned_user: uuid::Uuid,
    shared_with: Vec<Permission>, // Might be good to store this in a permission index instead.
    name: String,                   // String of the character
    id: uuid::Uuid,               // ID for database storage
    wiki_pages: WikiIndex,        // Wiki pages the character has made, typically concerning the character
    values: ValueIndex,           // Values for the character, such as characteristics, abilities, etc.
}

pub struct WikiIndex {
    pages: HashMap<String, Rc<WikiPage>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct WikiPage {
    heading: String,
    sub_headings: Vec<String>,  // User can make links by heading and subheading for display. Ex: [[heading#subheading]]
    display_data: String,       // Stored as Markdown text
}

pub struct TypeIndex {
    types: HashMap<String, Rc<Type>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub enum Type { // Important to note. Changing types in-game will be very difficult. Might be best to restrict it to only changing meta, enums, meta-refs, and die-rolls
    Num,
    List(Box<Type>),
    Enum(EnumerationType),
    Meta(MetaRef),
    Equation(EquationType),
    DieRoll(DieRollType),
    MetaRef(MetaRef),
}

pub struct ValueIndex {
    types: HashMap<String, Rc<Value>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum Value {
    Num(Number),
    List(List),
    Enum(Enumeration),
    Meta(MetaInst),
    Equation(Equation),
    DieRoll(DieRoll), 
    MetaRef(MetaRef),
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Number {
    name: String,
    value: f32,
}

impl Named for Number { fn get_name(&self) -> &str { &self.name } }

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct List {
    values: Vec<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Enumeration {
    t: EnumerationType, // Might technically need to be a Rc to an Enumeration Type
    inst: usize,
}

impl Named for Enumeration { fn get_name(&self) -> &str { &self } }

impl Eval for Enumeration {
    fn eval_f32(&self) -> Result<EvalResult<f32>, EvalError> {
        Ok(EvalResult::Value(self.inst as f32))
    }

    fn eval_bool(&self) -> Result<EvalResult<bool>, EvalError> {
        Err(EvalError::ExpectedBool)
    }
}

impl PartialEq for Enumeration {
    fn eq(&self, other: &Self) -> bool {
        self.inst == other.inst
    }
}

impl Deref for Enumeration {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.t[self.inst]
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct EnumerationType {
    name: String,
    types: Vec<String>,
}

impl Named for EnumerationType { fn get_name(&self) -> &str { &self.name } }

impl Index<usize> for EnumerationType {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        &self.types[index]
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Equation {
    t: EquationType,
    inputs: Vec<Input>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct EquationType {
    name: String,
    ast: EvalTree,
}

impl Named for EquationType { fn get_name(&self) -> &str { &self.name } }

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct DieRoll {
    die_results: Vec<u16>,
    value: f32,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub struct DieRollType { // Defined by 1d4, 4d6, etc. format
    num_dice: u8,
    num_sides: u16,
    special_sides: HashMap<u8, Modifier>, // Th
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct Modifier {
    name: String,
    target: String, // Target might be sub-field of a meta-inst, so to address that value:  Creo#1#2
    change: i32,
}

impl Named for Modifier { fn get_name(&self) -> &str { &self.name } }

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Input {
    name: String,  // This helps us pair InputRequest to Input for the evaluation of the EvalTree
    value: Value,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub struct InputRequest {
    name: String,
    requested_type: Type,
}

pub trait Eval {
    fn eval_f32(&self) -> Result<EvalResult<f32>, EvalError>;
    fn eval_bool(&self) -> Result<EvalResult<bool>, EvalError>;
}

pub trait Named {
    fn get_name(&self) -> &str;
}

pub enum EvalResult<T> {
    Value(T),
    InputRequired(InputRequest),
}

pub enum EvalError {
    DivideByZero,
    NonExistantNamedValue,
    ExpectedBool,
    Expectedf32,
    ExpectedMetaInst,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct EvalTree {
    root: EvalNode,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
struct EvalNode {
    // TODO: Probably want to define in a separate file.
}

impl Eval for EvalTree {
    fn eval_f32(&self) -> Result<EvalResult<f32>, EvalError> {
        todo!()
    }

    fn eval_bool(&self) -> Result<EvalResult<bool>, EvalError> {
        todo!()
    }
}


#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Clone)]
pub struct MetaType {
    pub name: String,
    pub fields: Vec<Type>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct MetaInst {
    pub name: String,
    pub fields: Vec<Value>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, Serialize, Clone)]
pub struct MetaRef { // MetaRef could also be MetaInst
    // Hold data on the ruleset / setting it came from?
    pub type_name: String,
    pub ref_name: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Event {
    // TODO: Also define ordering trait based on time
    year: Value,    // Defined specifically by a Year  meta-type required to be placed in the rule-set. Must be a num
    month: Value,   // Defined specifically by a Month meta-type required to be placed in the rule-set. Must be a num
    day: Value,     // Defined specifically by a Day   meta-type required to be placed in the rule-set. Must be a num
    event_type: EventType,  // Defined by a EventType meta-type. The event type holds the reference to the effect
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct EventType {
    name: String,
    effect: Effect,
    // TODO: Restrictions
    // Also, display img?
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Effect {
    target: MetaRef,
    old_value: Value,
    new_value: Value,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Timeline {
    events: Vec<Event>,
}