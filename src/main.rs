mod recipes;
mod abilities;

use std::cmp::PartialEq;
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

fn get_user_input() -> String{
    print!(">");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failure to read command");

    let user_input = user_input.trim().to_lowercase();

    user_input
}

fn split_args_with_quotes(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut in_quotes = false;
    let mut current_arg = String::new();

    for c in input.chars() {
        match c {
            ' ' if !in_quotes => {
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            '"' => {
                in_quotes = !in_quotes;
            }
            _ => {
                current_arg.push(c);
            }
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg.clone());
    }

    args
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
    println!("If this is your first time using this tool it is \x1b[38;5;196mhighly recommended\x1b[0m that you start by using the \x1b[38;5;196mhelp\x1b[0m command")

}

fn meld(character: &mut Character, command1: &mut Command, command2: &mut Command,recipes: &Result<HashMap<(Command, Command), Recipe>>){
    println!("A meld happens here");
}

fn pre_meld_menu(character: &mut Character, command1: &mut Command, command2: &mut Command,recipes: &Result<HashMap<(Command, Command), Recipe>>){
    let character_str = format!("{:?}", character);
    let command1_str = format!("{:?}", command1);
    let command2_str = format!("{:?}", command2);

    let max_length = character_str.len().max(command1_str.len()).max(command2_str.len());
    let box_width = max_length + 25; // Adjust as necessary for padding

    let border = "-".repeat(box_width);

    println!("You are about to view the meld data for the following meld:");
    println!("{}", border);
    println!("| Current Character: {:<width$} |", character_str, width = box_width - 23);
    println!("| Command #1: {:<width$} |", command1_str, width = box_width - 16);
    println!("| Command #2: {:<width$} |", command2_str, width = box_width - 16);
    println!("{}", border);
    loop {
        println!("\nIs this correct? [Y/n]");
        let user_input = get_user_input();
        if (user_input == "n") {
            println!("Exiting meld menu...");
            return;
        } else if ((user_input == "y") | (user_input == "")) {
            meld(character, command1, command2, recipes);
            return;
        }
    }
}


//COMMANDS FUNCTIONS

fn display_commands_help(){
    println!("The commands command is used to change the commands in your current meld.");
    println!("\n\x1b[1mcommands change [[-c1 [COMMAND NAME]]] [[-c2 [COMMAND NAME]]] [OPTIONS]\x1b[0m -- Changes your active commands");
    println!("\n\x1b[38;5;196mREQUIRED\x1b[0m - -c1 COMMAND NAME | -c2 COMMAND NAME - Enter the desired command name");
    println!("Enter a command name, any commands with spaces should be written with them ommited");
    println!("If you prefer, you may add the spaces but ensure that your command is surrounded by \" \"\n");
    println!("\x1b[38;5;46mEXAMPLE\x1b[0m");
    println!("\x1b[1mcommands -c1 zerogravity -c2 \"zero gravity\"\x1b[0m");
    println!("There is also many shorthand namings for the commands which can be found using the command \x1b[1mshorthand\x1b[0m");
    println!("The -c1 OR -c2 command allowing you to change one command at a time if desired.");
}

fn display_commands_status(command1: Command, command2: Command){
    println!("Command Slot 1: {:?}", command1);
    println!("Command Slot 2: {:?}", command2);
}

fn change_commands(args: &[String], command1: &mut Command, command2: &mut Command){
    if args[0].is_empty(){
        println!("\x1b[38;5;196mERROR\x1b[0m: No subcommand specified. Please refer to \x1b[1mcommands help\x1b[0m for more information.")
    }

    let mut index = 0;
    if(args[index] != "-c1") & (args[index] != "-c2"){
        println!("\x1b[38;5;196mERROR\x1b[0m: No command-slot found in the first argument. Please refer to \x1b[1mcommands help\x1b[0m for more information.");
        return;
    }
    while index < args.len(){
        match args[index].to_lowercase().as_str(){
            "-c1" | "--command1" =>{
                index += 1;
                change_command(args[index].clone(), command1);
            }
            "-c2" | "--command2" =>{
                index += 1;
                change_command(args[index].clone(), command2);
            }
            "-s" | "--status" => {
                display_commands_status(command1.clone(), command2.clone());
            }
            _ => {
                println!("\x1b[38;5;220mWARNING\x1b[0m: Unknown option, {}.", args[index].to_lowercase().as_str());
                println!("\x1b[38;5;220mNOTICE\x1b[0m: Some operations may have been successful, please refer to \x1b[1mcommands status\x1b[0m before continuing.");
                return;
            },
        }
        index += 1;
    }
}

fn change_command(args: String, command: &mut Command){
    match args.parse::<Command>(){
        Ok(cmd) => *command = cmd,
        Err(_) => println!("Invalid Command"),
    }
}


//CHAR FUNCTIONS

fn display_character_help(){
    println!("The char command is used to change/view your active character.");
    println!("\n\x1b[1mchar change [[CHARACTER]] [[OPTION]]\x1b[0m -- Changes your active character");
    println!("\n\x1b[38;5;196mREQUIRED\x1b[0m - CHARACTER - Enter your character's name");
    println!("Terra (-t): change your character to Terra");
    println!("Aqua (-a): change your character to Aqua");
    println!("Ventus (-v): change your character to Ventus");
    println!("Unknown (-u): sets your character to Unknown");
    println!("Reset (-r): an alternative command to set your character to Unknown");
    println!("\n\x1b[38;5;220mOPTIONAL\x1b[0m - OPTION - An additional modifier you can add to the command");
    println!("--status (-s): Immediately displays your active character");
    println!("\n\x1b[1mchar status\x1b[0m -- Displays your active character");
    println!("\n\x1b[1mchar help\x1b[0m -- Displays the help menu");
    println!("\n\x1b[38;5;46mUSAGE EXAMPLES\x1b[0m");
    println!("\x1b[1mchar change Terra\x1b[0m -- Changes the current character to Terra");
    println!("\n\x1b[1mchar change -t --status\x1b[0m -- Changes the current character to Terra then displays your current character");
}

fn display_character_status(character: Character){
    println!("The current selected character is: {:?}", character);
    if character == Character::Unknown{
        println!("Performing operations as an Unknown character will produce the results for \x1b[1mall three character\x1b[0m.");
        println!("This is completely acceptable for the intended use of the program but \x1b[1mmay produce extremely long outputs\x1b[0m.");
    }
}

fn change_character(args: &[String], character: &mut Character){
    if args[0].is_empty(){
        println!("\x1b[38;5;196mERROR\x1b[0m: No character specified. Please refer to \x1b[1mchar help\x1b[0m for more information.")
    }

    match args[0].to_lowercase().as_str() {
        "terra" | "-t" => *character = Character::Terra,
        "aqua" | "-a" => *character = Character::Aqua,
        "ventus" | "-v" | "ven" => *character = Character::Ventus,
        "reset" | "-r" | "unknown" | "-u" => *character = Character::Unknown,
        _ => println!("\x1b[38;5;196mERROR\x1b[0m: Invalid character selection. Please refer to \x1b[1mchar help\x1b[0m for more information.")
    }
    if args.len() > 1{
        let mut index = 1;
        while index < args.len(){
            match args[index].to_lowercase().as_str(){
                "-s" | "--status" => display_character_status(character.clone()),
                _ => {
                    println!("\x1b[38;5;220mWARNING\x1b[0m: Unknown option, {}.", args[index].to_lowercase().as_str());
                    println!("The change character operation has likely succeeded, however, it is recommended to use \x1b[1mchar status\x1b[0m to ensure you're happy with the result.");
                }
            }
            index += 1;
        }
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
        _ => println!("\x1b[38;5;196mERROR\x1b[0m: Unknown command {}. Use \x1b[1mhelp\x1b[0m for a detailed list of commands.", args[0])
    }
}

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
