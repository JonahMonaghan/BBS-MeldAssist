use std::str::FromStr;
use serde::Deserialize;
use crate::character::Character;
use crate::commands::Command;

#[derive(Debug, Deserialize)]
pub enum RecipeType{
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
}

#[derive(Debug, Deserialize)]
pub struct CharacterSpecificResult{
    pub character: Character,
    pub activation_chance: f32,
}

#[derive(Debug, Deserialize)]
pub struct CommandResult{
    pub result: Command,
    pub chances: Vec<CharacterSpecificResult>,
}

#[derive(Debug, Deserialize)]
pub struct Recipe{
    pub command1: Command,
    pub command2: Command,
    pub results: Vec<CommandResult>,
    pub command_type: RecipeType,
}

#[derive(Debug, Deserialize)]
pub struct RecipeCollection{
    pub recipes: Vec<Recipe>
}
