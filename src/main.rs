mod recipes;
mod abilities;

use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader};
use std::ops::Deref;
use serde_json::{json, Result};
use crate::recipes::{Command, RecipeCollection, RecipeType, Recipe};

fn normalize_commands(command1: &Command, command2: &Command) -> (Command, Command){
    if command1 < command2 {
        (command1.clone(), command2.clone())
    }else{
        (command2.clone(), command1.clone())
    }
}

fn add_recipes_to_hashmap(json_path: &str) -> Result<HashMap<(Command, Command), Recipe>> {
    let json = fs::read_to_string(json_path).expect("Unable to read file");
    let recipe_collection: RecipeCollection = serde_json::from_str(&json)?;

    let mut recipes_map: HashMap<(Command, Command), Recipe> = HashMap::new();

    for recipe in recipe_collection.recipes{
        let normalized_key = normalize_commands(&recipe.command1, &recipe.command2);
        recipes_map.insert(normalized_key, recipe);
    }

    Ok(recipes_map)
}

fn main() -> Result<()> {
    let recipes_map = add_recipes_to_hashmap("src/recipes.json")?;

    // Example of accessing a recipe
    let key = normalize_commands(&Command::Blitz, &Command::Quake);
    if let Some(recipe) = recipes_map.get(&key) {
        println!("{:?}", recipe);
    } else {
        println!("Recipe not found.");
    }

    Ok(())
}
