use std::collections::HashMap;

pub struct DieIndex {
    // TODO, for special dice types, such as stress die, botch die, simple die. They have special effects when rolled.
    dice_types: Vec<DieRoll>,
}

impl DieIndex {
    pub fn get_dice(die_name: String, num_dice: i32) {
        // Examples: Stress Die
        //           Botch Roll
        //           Simple Die
        //           d3, d10, d22, etc
    }
}

pub struct DieRoll {
    num_dice: i32,
    num_sides: i32,
    special_sides: HashMap<i32, DieModifier>,
}

impl DieRoll {
    pub fn roll(&self, prev_result: Option<DieResult>) -> DieResult {
        todo!()
    }
}

pub enum DieResult {
    Num(i32),
    ChanceToBotch(i32), // Number of botch die to roll
    Botch(i32), // Number of botch dice that turned up zero
    Explode(i32), // Number of cumulative explodes
}

enum DieModifier {
    Exploding(i32), // Multiplier applied when exploding
    Botch(i32) // Number of botch die
}

impl DieModifier {
    fn apply(&self, input: i32) -> i32 {
        todo!()
    }
}