mod recipes;
mod abilities;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use serde_json::{Result};
use crate::recipes::{Command, RecipeCollection, Recipe, Character};

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
    println!("Commands:");
    println!("chng-char [CHARACTER NAME]");
    println!("chng-cmd1 [COMMAND NAME]");
    println!("chng-cmd2 [COMMAND NAME]");
    println!("meld");
    println!("help [COMMAND]");
    println!("exit")

}

fn change_character(args: &str, character: &mut Character){
    match args {
        "Terra" | "-t" => *character = Character::Terra,
        "Aqua" | "-a" => *character = Character::Aqua,
        "Ventus" | "-v" | "Ven" => *character = Character::Ventus,
        "Reset" | "-r" => *character = Character::Unknown,
        _ => println!("Invalid character selection. Please use the help command for additional assistance.")
    }
}

fn process_user_command(cmd: &str, args: &str, character: &mut Character){
    match cmd{
        "chng-char" => change_character(args, character),
        _ => println!("Invalid command. Please use the help command if you need additional assistance."),
    }
}

fn main() {
    launch_banner();
    let mut command1 : Command = Command::Unknown;
    let mut command2 : Command = Command::Unknown;
    let mut character : Character = Character::Unknown;
    loop {
        print!(">");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failure to read command");

        let user_input = user_input.trim();
        let mut parts = user_input.split(" ");
        let command = parts.next().unwrap_or("");
        let args = parts.next().unwrap_or("");

        println!("Command: {}", command);
        println!("Args: {}", args);

        process_user_command(command, args, &mut character);

        println!("Current Character {:?}", character);
    }
}
