mod recipes;
mod abilities;
mod character;
mod commands;
mod meld;
mod utilities;

use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use serde_json::{Result};
use crate::character::*;
use crate::commands::*;
use crate::meld::*;
use crate::recipes::*;
use crate::utilities::*;

fn main() {
    launch_banner();
    let recipes = add_recipes_to_hashmap("src/recipes.json");
    let mut command1 : Command = Command::Unknown;
    let mut command2 : Command = Command::Unknown;
    let mut character : Character = Character::Unknown;
    loop {
        let user_input = get_user_input();
        let args: Vec<String> = split_args_with_quotes(&user_input);

        process_user_command(&args, &mut character, &mut command1, &mut command2, &recipes);
    }
}

fn process_user_command(args: &[String], character: &mut Character, command1: &mut Command, command2: &mut Command, recipes: &Result<HashMap<(Command, Command), Recipe>>){
    if args.is_empty(){
        return;
    }

    match args[0].to_lowercase().as_str(){
        "char" => {
            if args.len() < 2{
                println!("\x1b[38;5;196mERROR\x1b[0m: No sub-commands provided for \x1b[1mchar\x1b[0m. Try using \x1b[1mchar help\x1b[0m for more info.");
                return;
            }
            match args[1].to_lowercase().as_str(){
                "change" => change_character(&args[2..], character),
                "status" => display_character_status(character.clone()),
                "help" => display_character_help(),
                _ => println!("\x1b[38;5;196mERROR\x1b[0m: Unknown sub-command for \x1b[1mchar\x1b[0m. Try using \x1b[1mchar help\x1b[0m for more info.")
            }
        }
        "commands" => {
            if args.len() < 2{
                println!("\x1b[38;5;196mERROR\x1b[0m: No sub-commands provided for \x1b[1mcommands\x1b[0m. Try using \x1b[1mcommands help\x1b[0m for more info.");
                return;
            }
            match args[1].to_lowercase().as_str(){
                "change" => change_commands(&args[2..], command1, command2),
                "status" => display_commands_status(command1.clone(), command2.clone()),
                "help" => display_commands_help(),
                _ => println!("\x1b[38;5;196mERROR\x1b[0m: Unknown sub-command for \x1b[1mcommands\x1b[0m. Try using \x1b[1mcommands help\x1b[0m for more info.")
            }
        }
        "meld" => pre_meld_menu(&character, &command1, &command2, recipes),
        "help" => display_help(),
        _ => println!("\x1b[38;5;196mERROR\x1b[0m: Unknown command {}. Use \x1b[1mhelp\x1b[0m for a detailed list of commands.", args[0])
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

//Banner
fn launch_banner(){
    println!("*************************");
    println!("***   WELCOME TO...   ***");
    println!("**************************************************************************************************");
    println!("***   ____________  _____  ___  ___ _____ _    ______    ___   _____ _____ _____ _____ _____   ***");
    println!("***   | ___ \\ ___ \\/  ___| |  \\/  ||  ___| |   |  _  \\  / _ \\ /  ___/  ___|_   _/  ___|_   _|  ***");
    println!("***   | |_/ / |_/ /\\ `--.  | .  . || |__ | |   | | | | / /_\\ \\\\ `--.\\ `--.  | | \\ `--.  | |    ***");
    println!("***   | ___ \\ ___ \\ `--. \\ | |\\/| ||  __|| |   | | | | |  _  | `--. \\`--. \\ | |  `--. \\ | |    ***");
    println!("***   | |_/ / |_/ //\\__/ / | |  | || |___| |___| |/ /  | | | |/\\__/ /\\__/ /_| |_/\\__/ / | |    ***");
    println!("***   \\____/\\____/ \\____/  \\_|  |_/\\____/\\_____/___/   \\_| |_/\\____/\\____/ \\___/\\____/  \\_/    ***");
    println!("**************************************************************************************************\n\n");
    println!("Welcome to BBS-MeldAssist! The defacto tool for all of your command melding needs.");
    println!("If this is your first time using this tool it is \x1b[38;5;196mhighly recommended\x1b[0m that you start by using the \x1b[38;5;196mhelp\x1b[0m command")
}

fn display_help() {
    println!("Available commands:");
    println!("\x1b[1mchar\x1b[0m - Manage your character settings");
    println!("    \x1b[1mchar change [character]\x1b[0m - Change the current character. Available options: Terra, Aqua, Ventus");
    println!("    \x1b[1mchar status\x1b[0m - Display the current character");
    println!("    \x1b[1mchar help\x1b[0m - Show help for character commands");

    println!("\x1b[1mcommands\x1b[0m - Manage your commands");
    println!("    \x1b[1mcommands change [command1] [command2]\x1b[0m - Change the current commands");
    println!("    \x1b[1mcommands status\x1b[0m - Display the current commands");
    println!("    \x1b[1mcommands help\x1b[0m - Show help for commands");

    println!("\x1b[1mmeld\x1b[0m - Start the melding process with the current character and commands");
    println!("\x1b[1mhelp\x1b[0m - Display this help message");
}