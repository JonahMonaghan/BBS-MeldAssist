use std::collections::HashMap;
use serde::Deserialize;
use crate::character::Character;
use crate::commands::Command;

#[derive(Debug, Deserialize, PartialEq, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
pub struct CharacterSpecificResult{
    pub character: Character,
    pub activation_chance: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CommandResult{
    pub result: Command,
    pub chances: Vec<CharacterSpecificResult>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Recipe{
    pub command1: Command,
    pub command2: Command,
    pub results: Vec<CommandResult>,
    pub recipe_type: RecipeType,
}

#[derive(Debug, Deserialize)]
pub struct RecipeCollection{
    pub recipes: Vec<Recipe>
}
