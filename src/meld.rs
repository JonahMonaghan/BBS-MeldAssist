use std::collections::HashMap;
use serde_json::to_string;
use crate::character::Character;
use crate::commands::{Command, normalize_commands};
use crate::get_user_input;
use crate::recipes::{Recipe, RecipeType};
use crate::utilities::generate_text_box;

pub fn meld(character: &Character, command1: &Command, command2: &Command, recipes: &serde_json::Result<HashMap<(Command, Command), Recipe>>){
    let (cmd1, cmd2) = normalize_commands(command1, command2);
    let mut meld_result;
    match recipes{
        Ok(recipes_map) => {
            if let Some(recipe) = recipes_map.get(&(cmd1, cmd2)){
                meld_result = recipe;
            }else{
                println!("No recipe found for the provided meld structure");
                return;
            }
        }
        Err(e) => {
            println!("Error loading recipes: {}", e);
            return;
        }
    }

    println!("{}", generate_meld_results(character, meld_result));
}

fn generate_meld_results(character: &Character, recipe: &Recipe) -> String {
    let mut lines = vec![
        String::from("          Meld Results           "),
        "&br".to_string(),
        format!("Current Character: {:?}", character),
        "&br".to_string(),
        format!("Command #1: {:?}", recipe.command1),
        format!("Command #2: {:?}", recipe.command2),
    ];

    if *character != Character::Unknown {
        for (index, result) in recipe.results.iter().enumerate() {
            let mut character_has_result = false;
            for chance in &result.chances {
                if *character == chance.character {
                    if !character_has_result {
                        lines.push("&br".to_string());
                        lines.push(format!("Result #{}: {:?}", index + 1, result.result));
                        character_has_result = true;
                    }
                    lines.push(format!("{:?}: {:.0}%", chance.character, chance.activation_chance * 100.0));
                }
            }
        }
    } else {
        for (index, result) in recipe.results.iter().enumerate() {
            lines.push("&br".to_string());
            lines.push(format!("Result #{}: {:?}", index + 1, result.result));
            for chance in &result.chances {
                lines.push(format!("{:?}: {:.0}%", chance.character, chance.activation_chance * 100.0));
            }
        }
    }

    let mut output = generate_text_box(&lines);
    output += "\n\n\n";
    output += fetch_ability_chart(&recipe.command_type).as_str();

    output
}

fn fetch_ability_chart(rt: &RecipeType) -> String{
    let mut lines = vec![
        String::from("          Ability Crystals          "),
    ];
    let mut abilities : Vec<&str> = vec![];

    let crystal_names = vec!["Shimmering Crystal", "Fleeting Crystal", "Pulsing Crystal", "Wellspring Crystal", "Soothing Crystal", "Hungry Crystal", "Abounding Crystal"];

    match rt{
        RecipeType::A => {
            abilities = vec!["Fire Boost", "Magic Haste", "Leaf Bracer", "Air Combo Plus", "HP Boost", "HP Prize Plus", "Link Prize Plus"];
        }
        RecipeType::B => {
            abilities = vec!["Blizzard Boost", "Magic Haste", "Leaf Bracer", "Combo Plus", "Item Boost", "HP Prize Plus", "Lucky Strike"];
        }
        RecipeType::C => {
            abilities = vec!["Thunder Boost", "Magic Haste", "Combo F Boost", "Air Combo Plus", "HP Boost", "Treasure Magnet", "Link Prize Plus"];
        }
        RecipeType::D => {
            abilities = vec!["Cure Boost", "Magic Haste", "Combo F Boost", "Combo Plus", "Item Boost", "Treasure Magnet", "Lucky Strike"];
        }
        RecipeType::E => {
            abilities = vec!["Fire Screen", "Attack Haste", "Leaf Bracer", "Combo Plus", "HP Boost", "HP Prize Plus", "Link Prize Plus"];
        }
        RecipeType::F => {
            abilities = vec!["Blizzard Screen", "Attack Haste", "Leaf Bracer", "Air Combo Plus", "Item Boost", "HP Prize Plus", "Link Prize Plus"];
        }
        RecipeType::G => {
            abilities = vec!["Thunder Screen", "Attack Haste", "Finish Boost", "Combo Plus", "HP Boost", "Treasure Magnet", "Link Prize Plus"];
        }
        RecipeType::H => {
            abilities = vec!["Dark Screen", "Attack Haste", "Finish Boost", "Air Combo Plus", "Item Boost", "Treasure Magnet", "Lucky Strike"];
        }
        RecipeType::I => {
            abilities = vec!["Fire Screen", "Attack Haste", "Finish Boost", "Combo Plus", "HP Boost", "HP Prize Plus", "Link Prize Plus"];
        }
        RecipeType::J => {
            abilities = vec!["Blizzard Screen", "Magic Haste", "Combo F Boost", "Air Combo Plus", "Item Boost", "HP Prize Plus", "EXP Walker"];
        }
        RecipeType::K => {
            abilities = vec!["Thunder Screen", "Attack Haste", "Finish Boost", "Combo Plus", "HP Boost", "Treasure Magnet", "Lucky Strike"];
        }
        RecipeType::L => {
            abilities = vec!["Dark Screen", "Magic Haste", "Combo F Boost", "Air Combo Plus", "Item Boost", "Treasure Magnet", "EXP Walker"];
        }
        RecipeType::M => {
            abilities = vec!["Fire Boost", "Reload Boost", "Finish Boost", "Once More", "Damage Syphon", "HP Prize Plus", "EXP Chance"];
        }
        RecipeType::N => {
            abilities = vec!["Blizzard Boost", "Reload Boost", "Second Chance", "Air Combo Plus", "Damage Syphon", "HP Prize Plus", "Lucky Strike"];
        }
        RecipeType::O => {
            abilities = vec!["Thunder Boost", "Reload Boost", "Combo F Boost", "Once More", "Defender", "Treasure Magnet", "EXP Chance"];
        }
        RecipeType::P => {
            abilities = vec!["Cure Boost", "Reload Boost", "Second Chance", "Combo Plus", "Defender", "Treasure Magnet", "Lucky Strike"];
        }
        _ => println!("Unknown Recipe Type.")
    }

    let mut index = 0;
    while index < crystal_names.len(){
        lines.push("&br".to_string());
        lines.push(crystal_names[index].to_string());
        lines.push(abilities[index].to_string());
        index += 1;
    }

    generate_text_box(&lines)
}

pub fn pre_meld_menu(character: &Character, command1: &Command, command2: &Command,recipes: &serde_json::Result<HashMap<(Command, Command), Recipe>>){
    let character_str = format!("Current Character: {:?}", character);
    let command1_str = format!("Command Slot 1: {:?}", command1);
    let command2_str = format!("Command Slot 2: {:?}", command2);

    let lines = vec![character_str, command1_str, command2_str];
    let text_box = generate_text_box(&lines);

    println!("You are about to view the meld data for the following meld:");
    println!("{}", text_box);

    loop {
        println!("\nIs this correct? [Y/n]");
        let user_input = get_user_input();
        if (user_input == "n") {
            println!("Exiting meld menu...");
            return;
        } else if ((user_input == "y") | (user_input == "")) {
            meld(&character, &command1, &command2, recipes);
            return;
        }
    }
}