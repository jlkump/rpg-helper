use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::api::{data::{context::{Context, ContextTemplate, CtxValue, TagFilter}, effect::Effect, tag::Tag, template::TemplateValue}, rpg::input::InputAction};

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
    id: Tag,
    // Effects on the character's data when this ability is granted. Think of them as always-on effects.
    passive_effects: Vec<Effect>,
    // Effects on the character's data when a condition is true. This is evaluated from the character's root ctx and thus can check for values
    // outside just this ability's subctx.
    conditional_effects: HashMap<Tag, Vec<Effect>>,
    // Actions that the player can perform because of this ability.
    // It could be as simple as a toggle which grants the "active" tag to this ability
    // or could be a die roll that places the result value in this ability to be processed
    // by other values in this ability or elsewhere in the character.
    // input_actions: Vec<AbilityPlayerInput>,
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

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
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

    pub fn set_ability(&mut self, ability: Ability) -> Option<Ability>
    {
        self.abilities.insert(ability.id.clone(), ability)
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

/// This is used to create abilities and is defined by the ruleset.
/// For example, the spells in Ars Magica would be defined as abilities.
/// The ruleset would define a spec for spells. The spec would contain:
///     prefix: spell
///     ctx: []
///     required values: []
/// NOTE: Spec requirements are not expressed as inputs, but rather can be converted
///       into input actions.
/// A "Spec" is a value defined in a ruleset for which the players can fill in options in
/// order to create the associated value defined by the spec. 
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct AbilitySpec
{
    /// The prefix is what specifies "Type"
    /// Such as "spell" or "virtue"
    /// 
    /// As an example, the spec for a virtue is incredibly bare-bones.
    /// All it does is define that a virtue has the prefix "virtue".
    /// The player defines what it does and what effect it has.
    /// 
    /// The ruleset contains all already pre-existing virtues.
    /// When a player chooses the "Grant Virtue" event in the timeline to build out the
    /// virtue event, they are given the option of choosing a pre-existing virtue
    /// in the ruleset or the ability to create their own from this spec.
    pub prefix: Tag,
    // TODO: Templated effects? Probably
    default_passive_effects: Vec<Effect>,
    default_conditional_effects: HashMap<Tag, Vec<Effect>>,
    // For any templates, [name] uses the input of the name of the ability for ability builder.
    template_ctx: ContextTemplate,
    /// These are the required inputs from the spec
    /// that must be filled out in order to create the ability.
    /// 
    /// It is possible that the list is empty, as in the case of Virtues and Flaws in Ars Magica.
    /// Thus, the only required input from the player to finish the creation would be the name / id
    /// of the ability.
    requirements: Vec<AbilityCreationRequirement>,
    // When allowing the player to create their own abilities from specs, we also give them the
    // option of adding in their own passive and conditional effects, as well was defining their
    // own ctx values.
    // For all of this configuration, the root spec may have some restrictions on what can be configured.
    // passive_effect_restrictions: Vec<EffectRestriction>,
    
    /// These values are shared across all abilities derived from this spec.
    /// They exist at the root of the character's ctx under
    /// ability.[prefix].spec
    /// For example, for spells in ars magica, the spec values include range.voice.magnitude: 2, spell.equation
    spec_values: Context,
}

impl AbilitySpec
{
    pub fn make_ability(&self) -> AbilityBuilder
    {
        todo!()
    }
}


/// Example types of requirements
///     - Pick a tag option from those available in the root-type
///       subtags. For example, the ars magica spell ability will
///       have the ability.spell.spec.range.* subtags which the user can choose
///       (based on some restrictions applied by the spec).
///       The choice of this tag is used to fill in the ability's template [range] tag
///       for the ability.spell.[name].range.[range] tag.
/// 
///       Additionally, it is used to fill in other templated values for attributes, equations, and conditionals.
///       The root spell definition will have a "magnitude" attribute under ability.spell.spec.range.[range].magnitude
///       This value will be referenced in the ability.spell.[name].lvl.magnitude.[range] as an equation with a single value ability.spell.spec.range.[range].magnitude. This allows a root configuration of the [range]'s magnitude while also allowing targeted change on the spell's individual magnitude on range.
///       Ability spell configuration:
///       ability.spell.[name].lvl = SumWithPrefix(ability.spell.[name].lvl.magnitude) * 5 + SumWithPrefix(ability.spell.[name].lvl.flat)
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum AbilityCreationRequirement
{
    PickTag(PickRootTag),
}

impl AbilityCreationRequirement
{
    pub fn as_input(&self, character_ctx: Context) -> InputAction
    {
        todo!()
    }

    pub fn fufill_requirement(&self)
    {
        todo!()
    }
}

/// Used to fill-in a template option in the ability
#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct PickRootTag
{
    /// Prefixed with the spec's prefix + "spec"
    suffix: Tag,
    /// The template option to be filled by the choice of this tag.
    template_filled: String,
    // Cascade from chosen tag value:
    // - Add new tag, attribute, equation, or conditional based on the chosen tag value
    added_template_values: TemplateValue,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub struct AbilityBuilder
{
    prefix: Tag,
    /// id must be a single tag. It is the "name" of the ability.
    /// This is the first thing filled in when creating an ability from a spec.
    /// Reserved name is "spec", which is used to define data shared across all of the same abilities.
    id: Option<Tag>,
    passive_effects: Vec<Effect>,
    conditional_effects: HashMap<Tag, Vec<Effect>>,
    template_ctx: ContextTemplate,
    requirements: Vec<AbilityCreationRequirement>,
}

impl AbilityBuilder
{
    pub fn with_id(mut self, id: Tag) -> Self
    {
        self.id = Some(id);
        self
    }

    pub fn fufill_requirement()
    {

    }

    // Either build the ability or return the builder and an error if not built.
    pub fn build(self) -> Result<Ability, (AbilityBuilder, AbilityBuildError)>
    {
        todo!()
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
pub enum AbilityBuildError
{
    MissingRequirements(Vec<AbilityCreationRequirement>),
}
// /// A value requirement is something that the player must provide some
// /// input for. For example, what is the name of the ability is a required
// /// value which is a tag input. It is used to create the full prefix of the ability.
// /// 
// /// Another required value might be a tag choice of some list of tags based on the player's
// /// ctx. For example, spell.range, where the player chooses a range that is in the character's
// /// state tags for ability.spell.range. All the player's subtags, (ability.spell.range.voice, ability.spell.range.touch, etc)
// /// are queried for and used in the available choices of the player. This value is placed in the intermediate ctx, along with all
// /// all other required values.
// /// 
// /// When the ability is constructed, the prefix + id are added as a prefix to all values in the intermediate ctx to get their final tag values.
// /// 
// /// The player can add additional ctx values, conditional effects, and input actions as part of ability construction (if the spec allows it).
// /// 
// /// Some values in the AbilitySpec are already defined and known by the spec (for example, virtues and flaws already know their modifier effects
// /// on character values). These are stored in the ContextTemplate and are filled in by the required ctx values. (Method TBD). If the ContextTemplate has no templated values, then it is fine to be turned directly into the ability without player input.