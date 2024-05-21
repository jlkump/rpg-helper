use util::cli;

mod syntax;
mod data;
mod error;
mod util;
mod network;
mod gui;

pub fn run() {
    gui::display::client::run_app();
}


// trait Value {
//     fn get_value() -> i32;
// }

// pub struct Record {
//     val: i32,
//     src: RecordSource,
// }
// pub enum RecordSource {
//     CharacterCreation,
//     SeasonActivity(u32),
//     Modifier(u32),
//     Other
// }
// // impl Sum<Record> for Record {
// //     fn sum<I: Iterator<Item = Record>>(iter: I) -> Self {
// //         iter.fold(Self { val: 0, src: RecordSource::Other }, | a, b| Self { 
// //             val: a.val + b.val,
// //             src: a.src
// //         })
// //     }
// // }

// impl<'a> Sum<&'a Record> for i32 {
//     fn sum<I>(iter: I) -> i32
//     where
//         I: Iterator<Item = &'a Record>,
//     {
//         iter.fold(0, |a, b| a + b.val)
//     }
// }

// // pub struct SeasonActivity {
// //     id: u32,
// //     effect: Effect,
// // }
// // pub enum Effect {
// //     NewKnowledge,
// //     NewVirtue,
// //     NewFlaw,
// //     Modifier,
// //     ChangeExp,
// //     LabProgress,
// // }


// // pub struct Equation {
// //     // ast: AST,
// //     id: u32,
// // }

// // impl Equation {
// //     pub fn get_inputs<'a>() -> Vec<&'a str> {
// //         todo!()
// //     }
// // }

// struct AST {

// }
// impl AST {
//     pub fn get_inputs<'a>(&self) -> Vec<&'a str> {
//         todo!()
//     }

//     pub fn evaluate(&self, inputs: Vec<i32>) -> Result<i32, EvalErr> {
//         todo!()
//     }
// }
// #[derive(Debug)]
// enum EvalErr {

// }

// // pub struct SingleInputEquation {
// //     ast: AST,
// // }

// // impl SingleInputEquation {
// //     pub fn evaluate(input: i32) -> i32 {
// //         todo!()
// //     }
// // }




// pub struct Modifier<'a> {
//     id: u32,
//     src: ModifierSource,
//     val: i32,
//     target: &'a str,
// }
// pub enum ModifierSource {
//     Virtue(u32),
//     Flaw(u32),
// }

// pub struct Virtue<'a> {
//     id: u32,
//     name: &'a str,
//     r#type: &'a str,
// }
// pub struct Flaw<'a> {
//     id: u32,
//     name: &'a str,
//     r#type: &'a str,
// }
// pub enum VFSource {
//     CharacterCreation,
//     TwilightScar(u32)
// }

// pub trait Id {
//     fn get_id() -> i32;
// }

// impl <'a>From::<Modifier<'a>> for Record {
//     fn from(value: Modifier) -> Self {
//         Record {
//             val: value.val,
//             src: RecordSource::Modifier(value.id)
//         }
//     }
// }

// pub struct Score<'a> {
//     name: &'a str,
//     score_equation: &'a AST,
//     experience: Vec<&'a Record>,
// }
// impl NamedValue for Score<'_> {
//     fn get_value(&self) -> i32 {
//         self.score_equation.evaluate(vec![self.experience.clone().into_iter().sum()]).expect("Evaluation problem")
//     }

//     fn get_name(&self) -> &str {
//         self.name
//     }
// }

// pub struct Art<'a> {
//     name: &'a str,
//     art_equation: &'a AST,
//     experience: Vec<&'a Record>,
// }
// impl NamedValue for Art<'_> {
//     fn get_value(&self) -> i32 {
//         self.art_equation.evaluate(vec![self.experience.clone().into_iter().sum()]).expect("Evaluation problem")
//     }
//     fn get_name(&self) -> &str {
//         self.name
//     }
// }

// pub trait NamedValue {
//     fn get_name(&self) -> &str;
//     fn get_value(&self) -> i32;
// }


// pub struct CharacterData<'a> {
//     modifiers: Map<u32, Modifier<'a>>,
//     experience: Vec<Record>,
//     scores: Map<&'a str, Score<'a>>,
// }

// impl CharacterData<'_> {
    
//     pub fn get_score(name: &str) -> Score {
//         todo!()
//     }
    
//     pub fn get_experience<'a>(score_name: &str) -> &'a Vec<Record> {
//         todo!()
//     }


// }

// pub struct ScoreIndex<'a> {
//     scores: HashMap<&'a str, Score<'a>>,

// }

// impl ScoreIndex<'_> {
//     pub fn get_score(&self, name: &str) -> i32 {
//         if let Some(score) = self.scores.get(name) {
//             score.score_equation.evaluate(self).expect("Failed to eval")
//         }
//         else 
//         {
//             0
//         }
//     }
// }

// pub struct ModifierIndex<'a> {
//     modifiers: Map<u32, Modifier<'a>>,
// }
// pub struct RecordIndex {
// }

// impl RecordIndex {
//     pub fn get_score(name: &str) -> i32 {
//         todo!()
//     }
// }


// impl <'a>ops::Add<Modifier<'a>> for Record {
//     type Output = Record;
//     fn add(self, rhs: Modifier<'a>) -> Self::Output {
//         Record {
//             val: self.val + rhs.val,
//             src: Source::Modifier(rhs.id)
//         }
//     }
// }



// Record for experience in creo
// Record {
//     val: 7,
//     srcs: Source::CharacterCreation()
// }
// Record {
//     val: 2,
//     srcs: Source::SeasonActivity(SeasonActivityID)
// }
// 
// For Creo
// Record {
//     val: 3,
//     srcs: Source::EquationEval(EquationID)
// }
// 
// For Magic Theory
// Record {
//     val: 3,
//     srcs: Source::EquationEval(EquationID)
// }
// 

// pub struct Equation {
//     ast: AST,
// }

// impl Equation {
//     fn Evaluate(vals: &ValueIndex) {

//     }
// }

// pub struct AST {

// }

// pub struct Description {

// }

// pub struct ValueIndex {

// }

// impl ValueIndex {
//     fn get_score<'a, 'b>(&'a self, val_name: &'b str) -> &'a Value {
//         &Value::Val(1)
//     }
// }

// pub struct Color; // TODO: Use external color
// pub enum Link {
//     Empty,
//     Description()
// }

// pub trait Keyword<'a> {
//     fn get_color() -> Color;
//     fn get_phrase() -> &'a str;
//     fn get_link() -> Link;
// }

// pub enum Effect {
//     NewKnowledge(),
//     ExpGain(),

// }