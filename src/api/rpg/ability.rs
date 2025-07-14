use crate::api::data::{context::Context, effect::Effect, tag::Tag};

/// An ability is given to a character
/// It grants modifiers, can alter attributes, equations, conditionals, and state-tags
/// Some abilities have choices for targets or effects.
///     Some of these are chosen at character creation (and thus are templated abilities so no need to worry about that here)
///     Some of these are chosen while the game is active, allowing the player to pick from a range of tag types.
/// - Name
/// - Context (to layer atop the character's context)
/// - Input actions
/// - Maybe? Helper text targets
pub struct Ability
{
    is_active: bool,
    granted_effects: Vec<Effect>,       // Effects on the character's data when this ability is granted
    active_effects: Vec<Effect>,        // Effects on the character's data when this ability is "active"
    input_actions: Vec<PlayerAction>,   // Actions that the player can perform because of this ability.
    ctx: Context,                       // Values specific to this ability. Layered on the player's context
                                        // For example, take an ars magica spell:
                                        // ability.spell.Unseen Arm
                                        // ability.spell.Unseen Arm.range.voice
                                        // ability.spell.Unseen Arm.duration.concentration
                                        // ability.spell.Unseen Arm.target.individual
                                        // ability.spell.Unseen Arm.lvl: spell-equation
                                        // ability.spell.Unseen Arm.lvl.base: 2
                                        // ability.spell.Unseen Arm.lvl.magnitude.voice: 2
                                        // ability.spell.Unseen Arm.lvl.magnitude.concentration: 2
                                        // ability.spell.Unseen Arm.lvl.flat.always active: 3
}


/// Player input actions
/// 1. Take in a &ctx, outputs a ctx that represents what the actions does.
/// 
/// Input actions will act on values of the character as a layered ctx.
/// 
/// It is important to consider what we need these actions to do in terms of abilities and event procedures
///     - Take in numeric input
///     - Take in a choice of tags (available choices filterable)
///     - Perform a roll or rolls and then prompt other actions based on the results
///     - Perform player modification based on the inputs above
pub struct PlayerAction
{

}

pub enum PlayerActionSpec
{
    SelectTag(Option<TagFilter>),
    PerformRoll(Tag),             // Give the player the option to either perform the roll automatically or input the roll result
}

pub struct PlayerRollSpec
{
    die_roll: Tag,

}

pub struct TagFilter
{

}