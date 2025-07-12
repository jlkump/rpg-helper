use crate::api::{data::{context::Context, effect::Effect, tag::Tag}, rpg::character::{Character, CharacterModification}};

pub struct CharacterTemplate
{
    name: String,
    tag: Tag,
    defaults: Character,
    creation_schema: CreationSchema,
}

pub struct CreationSchema
{
    stages: Vec<CreationStage>,
}


pub struct CreationContext
{
    ctx: Context,
    character: Character,   // Default character built from template, modified in this creation context to reach final character
    schema: CreationSchema,
}

pub struct CreationStage
{
    ctx: Context,               // Values specific to this stage of creation (such as exp points to spend)
    options: Vec<CreationOption>,
    restrictions: Vec<CreationRestriction>, // Conditionals targeting values of the character or creation context that ensure the stage can progress forward
    warnings: Vec<CreationRestriction>,     // Warnings of unused creation values (such as having more available exp points to spend that will be lost)
}

pub struct CreationOption
{
    creation_changes: Vec<Effect>,
    character_changes: Vec<CharacterModification>,
}

pub struct CreationRestriction
{
    cond_tag: Tag,
    display_error: String, // What we display to the user if this conditional fails
}