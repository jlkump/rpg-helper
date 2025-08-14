use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::{data::{context::{Context, TagFilter}, effect::Effect, tag::Tag}, rpg::player::{PlayerInputAction, PlayerInputActionResponse}};

/// An ability is given to a character
/// It grants modifiers, can alter attributes, equations, conditionals, and state-tags
/// Some abilities have choices for targets or effects.
///     Some of these are chosen at character creation (and thus are templated abilities so no need to worry about that here)
///     Some of these are chosen while the game is active, allowing the player to pick from a range of tag types.
/// - Name
/// - Context (to layer atop the character's context)
/// - Input actions
/// - Maybe? Helper text targets
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct Ability
{
    pub id: Tag,
    granted_effects: Vec<Effect>,           // Effects on the character's data when this ability is granted
    active_effects: Vec<Effect>,            // Effects on the character's data when this ability is "active"
    input_actions: Vec<AbilityPlayerInput>, // Actions that the player can perform because of this ability.
    ctx: Context,                           // Values specific to this ability. Layered on the player's context
                                            // For example, take an ars magica spell:
                                            // ability.spell.Unseen Arm
                                            // ability.spell.Unseen Arm.range.voice
                                            // ability.spell.Unseen Arm.duration.concentration
                                            // ability.spell.Unseen Arm.target.individual
                                            // ability.spell.Unseen Arm.lvl: spell.equation // Might be useful to copy equation from containing character to the ability's ctx
                                            // ability.spell.Unseen Arm.lvl.base: 2
                                            // ability.spell.Unseen Arm.lvl.magnitude.voice: 2
                                            // ability.spell.Unseen Arm.lvl.magnitude.concentration: 2
                                            // ability.spell.Unseen Arm.lvl.flat.always active: 3
                                            // NOTE: for the above to work, equations need the ability to querry for all values
                                            //       with a given prefix, then add then perform some operation on them, 
                                            //       such as adding them all together or tallying up values that land on a side
}

pub struct AbilitySet
{
    abilities: HashMap<Tag, Ability>,
}

impl AbilitySet
{
    pub fn get_ability(&self, ability_id: &Tag) -> Option<&Ability>
    {
        self.abilities.get(ability_id)
    }

    pub fn set_ability(&mut self, ability: Ability)
    {
        self.abilities.insert(ability.id.clone(), ability);
    }

    pub fn remove_ability(&mut self, ability_id: &Tag) -> Option<Ability>
    {
        self.abilities.remove(ability_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Ability>
    {
        self.abilities.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Ability>
    {
        self.abilities.values_mut()
    }
}

// pub struct AbilitySpec
// {
//     ability_prefix: Tag,    // All ability's ctxs and names are prefixed with some tag, the leading first subtag being "ability"

// }

// pub struct AbilityTypeSet
// {

// }

// /// An ability type constructs an ability
// /// For example, spells in Ars Magica could be considered a type of ability
// ///     We then construct an ability using a template of what the abiltiy should
// ///     look like. For a spell, this requires having a set of input options
// ///     the player can choose from.
// /// 
// ///     For example, being able to choose the option spell.magnitude.voice,
// ///     which adds the attribute ability.spell.Spell Name.lvl.magnitude.voice: 2
// ///     to the ability.
// /// 
// ///     For this to work, a player input option needs to be able to query for
// ///     tag value pairings in a context based on a given tag prefix. For example,
// ///     querying for prefixes "rules.ability.spell.magnitude.range" and letting the player choose
// ///     from the options given. The option chosen then adds the chosen value as an attribute (with a different tag)
// ///     to the ability ctx.
// pub struct AbilityType
// {

// }

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct AbilityPlayerInput
{
    input_type: PlayerInputAction,
}

pub struct AbilityPlayerInputResponse
{
    response: PlayerInputActionResponse,
}