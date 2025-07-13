use crate::api::data::effect::Effect;

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
}


/// Player input actions
/// 1. Take in a &ctx, outputs a ctx that represents what the actions does.
/// 
/// Input actions will act on values of the character as a layered ctx.
pub struct PlayerAction
{

}