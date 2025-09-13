use std::collections::HashMap;

use rand::prelude::*;

use once_cell::sync::Lazy;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::api::data::{attribute::AttributeSet, equation::Equation, error::ParseError, tag::Tag};

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct DiceSet
{
    dice_types: HashMap<Tag, DieRoll>,
}

impl DiceSet
{
    pub fn new() -> DiceSet
    {
        DiceSet { dice_types: HashMap::new() }
    }

    pub fn define_die_roll(&mut self, d_roll: DieRoll)
    {
        self.dice_types.insert(d_roll.name.clone(), d_roll);
    }

    pub fn get_die_roll(&self, t: &Tag) -> Option<&DieRoll>
    {
        self.dice_types.get(t)
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct DiceRoll
{
    dice_to_roll: Vec<(Tag, i32)>,
}

impl DiceRoll
{
    pub fn roll_dice(&self, set: &DiceSet) -> DiceRollResult
    {
        let mut result = vec![];
        for (die, roll_count) in self.dice_to_roll.iter()
        {
            if let Some(die) = set.get_die_roll(die)
            {
                for _ in 0..*roll_count
                {
                    result.push(die.roll(set));
                }
            }
        } 
        DiceRollResult::new(result)
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct DiceRollResult
{
    results: Vec<DieRollResult>
}

impl DiceRollResult
{
    fn new(results: Vec<DieRollResult>) -> Self
    {
        DiceRollResult { results }
    }

    pub fn process_result(by: DiceRollProcess) -> i32
    {
        todo!()
    }
}

pub enum DiceRollProcess
{
    // Tends to be the default when rolling multiple dice
    // just sum up the results
    SumValues,
    // DnD's Disadvantage
    // take the least and use that value
    Minimum,
    // DnD's Advantage
    // take the greatest and use that value
    Maximum,
    // Ars Magica's Botch Dice
    CountFacesMatching(u16),
    // We count only the dice matching the exact face rolled. 
    SumFacesMatching(u16),
    // Eldritch Horror's dice system.
    // We count sum the results of the dice whose value is greater than the given value.
    RestrictGreaterAndCount(i32),
    // We count only the results of the dice whose value is less than the given value.
    RestrictLessAndCount(i32),
    // We sum only the greatest `x` dice together
    // Some DnD damage abilities do this, where you take 
    // the highest of some number of damage dice.
    SumRestrictNextLargest(u32),
    // Same as above, except we take the least `x` dice
    // and sum them together
    SumRestrictNextLowest(u32)
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct DieRoll
{
    // The type of die roll
    // Ex: Stress Die, Normal Die, Botch Die, Exploding Die
    name: Tag,
    num_sides: u16,
    side_modifiers: HashMap<u16, DieModifier>,
    final_operation: Option<Equation>,
}

impl DieRoll
{
    pub fn new(name: Tag, num_sides: u16) -> DieRoll
    {
        DieRoll { name, num_sides, side_modifiers: HashMap::new(), final_operation: None }
    }

    pub fn set_side_modifier(&mut self, side: u16, modifier: DieModifier)
    {
        self.side_modifiers.insert(side, modifier);
    }

    pub fn set_final_operation(&mut self, e: Equation)
    {
        self.final_operation = Some(e);
    }

    pub fn clear_final_operation(&mut self)
    {
        self.final_operation = None;
    }

    pub fn roll(&self, set: &DiceSet) -> DieRollResult
    {
        self.roll_recursive(set, 0)
    }

    fn roll_recursive(&self, set: &DiceSet, num_of_rolls: u16) -> DieRollResult
    {
        let mut rng = SmallRng::seed_from_u64(1);
        let side = rng.random_range(1..=self.num_sides);
        self.evaluate_side(set, num_of_rolls, side)
    }

    fn evaluate_side(&self, set: &DiceSet, num_of_rolls: u16, side: u16) -> DieRollResult
    {
        static RESULT_TAG: Lazy<Result<Tag, ParseError>> = Lazy::new(|| Tag::from_str("die roll.result"));
        static ROLL_COUNT_TAG: Lazy<Result<Tag, ParseError>> = Lazy::new(|| Tag::from_str("die roll.roll count"));

        let mut value = if let Some(m) = self.side_modifiers.get(&side)
        {
            match m
            {
                DieModifier::ReRoll => self.roll_recursive(set, num_of_rolls + 1),
                DieModifier::RollNew(ex) =>
                {
                    if let Some(die) = set.get_die_roll(ex)
                    {
                        die.roll_recursive(set, num_of_rolls + 1)
                    }
                    else
                    {
                        DieRollResult::new(self.name.clone(), side, side as i32)
                    }
                },
                DieModifier::MapValue(v) => DieRollResult::new(self.name.clone(), side, *v),
            }
        }
        else
        {
            DieRollResult::new(self.name.clone(), side, side as i32)
        };

        // Perform a final operation 
        if let Ok(result_tag) = &*RESULT_TAG
        {
            if let Ok(rolls_tag) = &*ROLL_COUNT_TAG
            {
                if let Some(op) = &self.final_operation
                {
                    let mut atr_set = AttributeSet::new();
                    atr_set.set_attribute(result_tag, value.roll_value as f32);
                    atr_set.set_attribute(rolls_tag, num_of_rolls as f32);
                    if let Ok(v) = op.eval(&(&atr_set).into())
                    {
                        value.roll_value = v.round() as i32;
                    }
                }
            }
        }
        value
    }

    pub fn simulate_roll(&self, set: &DiceSet, side: u16) -> Option<DieRollResult>
    {
        if side > self.num_sides
        {
            None
        }
        else
        {
            Some(self.evaluate_side(set, 0, side))
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct DieRollResult
{
    pub t: Tag,
    pub face_rolled: u16,
    pub roll_value: i32,
}

impl DieRollResult
{
    pub fn new(t: Tag, face_rolled: u16, value: i32) -> DieRollResult
    {
        DieRollResult { t, face_rolled, roll_value: value }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum DieModifier
{
    ReRoll,                 // ReRolls self
    RollNew(Tag),           // Rolls a new provided dice
    MapValue(i32),          // Maps the side value to a new value (example maps 10 to 0 for d10)
}

#[cfg(test)]
mod unit_tests 
{
    use super::*;

    /// Tests a simple roll of a d10
    #[test]
    fn roll_0()
    {
        let die_roll = DieRoll::new(Tag::new("simple").unwrap(), 10);
        let set = DiceSet::new();
        for _ in 0..50
        {
            let result = die_roll.roll(&set);
            assert!(result.roll_value <= 10 && result.roll_value > 0);
        }
        for i in 1..=10
        {
            assert_eq!(die_roll.simulate_roll(&set, i as u16).unwrap().roll_value, i);
        }
    }

    /// Tests simple dice from sizes of d2 to d500
    #[test]
    fn roll_1()
    {
        let set = DiceSet::new();
        for sides in 2..=500
        {
            let die_roll = DieRoll::new(Tag::new("die").unwrap(), sides as u16);
            for _ in 0..50
            {
                let result = die_roll.roll(&set);
                assert!(result.roll_value <= sides && result.roll_value > 0);
            }
            for i in 1..=sides
            {
                assert_eq!(die_roll.simulate_roll(&set, i as u16).unwrap().roll_value, i);
            }
        }
    }

    /// Tests a stress die as defined by ars magica
    /// 10 -> 0
    /// 1 -> Exploding
    #[test]
    fn roll_2()
    {
        let exploding_die_tag = Tag::new("die roll.exploding die").unwrap();
        let stress_die_tag = Tag::new("die roll.stress die").unwrap();
        
        let mut stress_die = DieRoll::new(stress_die_tag, 10);
        stress_die.set_side_modifier(10, DieModifier::MapValue(0));
        stress_die.set_side_modifier(1,DieModifier::RollNew(exploding_die_tag.clone()));
        
        let mut exploding_die = DieRoll::new(exploding_die_tag.clone(), 10);
        exploding_die.set_side_modifier(1, DieModifier::ReRoll);
        exploding_die.set_final_operation(Equation::new(Tag::new("die roll.exploding die.equation").unwrap(), "pow(2, die roll.roll count) * die roll.result").unwrap());

        let mut set = DiceSet::new();
        set.define_die_roll(stress_die.clone());
        set.define_die_roll(exploding_die.clone());

        for i in 2..=9
        {
            assert_eq!(stress_die.simulate_roll(&set, i as u16).unwrap().roll_value, i);
            assert_eq!(exploding_die.simulate_roll(&set, i as u16).unwrap().roll_value, i);
        }

        assert_eq!(stress_die.simulate_roll(&set, 10).unwrap().roll_value, 0);
        assert_eq!(exploding_die.simulate_roll(&set, 10).unwrap().roll_value, 10);

        // Should've exploded
        assert_eq!(stress_die.simulate_roll(&set, 1).unwrap().t, exploding_die_tag);
    }


    /// Tests complex final equation evaluation
    #[test]
    fn roll_3()
    {
        let exploding_die_tag = Tag::new("die roll.exploding die").unwrap();
        
        let mut exploding_die = DieRoll::new(exploding_die_tag.clone(), 10);
        exploding_die.set_final_operation(Equation::new(Tag::new("die roll.exploding die.equation").unwrap(), "pow(2, 2) * die roll.result").unwrap());

        let set = DiceSet::new();

        for i in 1..=10
        {
            assert_eq!(exploding_die.simulate_roll(&set, i as u16).unwrap().roll_value, 4 * i);
        }
    }
}