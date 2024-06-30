use std::collections::HashMap;
use std::str::FromStr;
use crate::recipes::{Recipe, RecipeType};
use serde::Deserialize;
use crate::character::Character;
use crate::commands::Command;
use crate::utilities::generate_text_box;

#[derive(Debug, Deserialize, Clone)]
#[derive(PartialEq)]
pub enum AbilityName{
    FireBoost,
    FireScreen,
    BlizzardBoost,
    BlizzardScreen,
    ThunderBoost,
    ThunderScreen,
    CureBoost,
    DarkScreen,
    MagicHaste,
    AttackHaste,
    ReloadBoost,
    LeafBracer,
    FinishBoost,
    SecondChance,
    ComboFBoost,
    AirComboPlus,
    OnceMore,
    ComboPlus,
    HPBoost,
    DamageSyphon,
    ItemBoost,
    Defender,
    HPPrizePlus,
    TreasureMagnet,
    LinkPrizePlus,
    EXPChance,
    LuckyStrike,
    LuckBoost,
    EXPWalker,
    Unknown,
}
impl FromStr for AbilityName{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fire boost" | "fireboost" | "fire+" => Ok(AbilityName::FireBoost),
            "fire screen" | "firescreen" | "fire-" => Ok(AbilityName::FireScreen),
            "blizzard boost" | "blizzardboost" | "blizz+" => Ok(AbilityName::BlizzardBoost),
            "blizzard screen" | "blizzardscreen" | "blizz-" => Ok(AbilityName::BlizzardScreen),
            "thunder boost" | "thunderboost" | "thund+" => Ok(AbilityName::ThunderBoost),
            "thunder screen" | "thunderscreen" | "thund-" => Ok(AbilityName::ThunderScreen),
            "cure boost" | "cureboost" | "cure+" => Ok(AbilityName::CureBoost),
            "dark screen" | "darkscreen" | "dark-" => Ok(AbilityName::DarkScreen),
            "magic haste" | "magichaste" | "mh" => Ok(AbilityName::MagicHaste),
            "attack haste" | "attackhaste" | "ah" => Ok(AbilityName::AttackHaste),
            "reload boost" | "reloadboost" | "rb" => Ok(AbilityName::ReloadBoost),
            "leaf bracer" | "leafbracer" | "lb" => Ok(AbilityName::LeafBracer),
            "finish boost" | "finishboost" | "finish+" => Ok(AbilityName::FinishBoost),
            "second chance" | "secondchance" | "sc" => Ok(AbilityName::SecondChance),
            "combo f boost" | "combofboost" | "combof+" => Ok(AbilityName::ComboFBoost),
            "air combo plus" | "aircomboplus" | "ac+" => Ok(AbilityName::AirComboPlus),
            "combo plus" | "comboplus" | "c+" => Ok(AbilityName::ComboPlus),
            "once more" | "oncemore" | "om" => Ok(AbilityName::OnceMore),
            "hp boost" | "hpboost" | "hp+" => Ok(AbilityName::HPBoost),
            "item boost" | "itemboost" | "item+" => Ok(AbilityName::FireBoost),
            "damage syphon" | "damagesyphon" | "ds" => Ok(AbilityName::DamageSyphon),
            "defender" => Ok(AbilityName::Defender),
            "hp prize plus" | "hpprizeplus" | "hpprize+" => Ok(AbilityName::HPPrizePlus),
            "treasure magnet" | "treasuremagnet" | "tm" => Ok(AbilityName::TreasureMagnet),
            "link prize plus" | "linkprizeplus" | "lprize+" => Ok(AbilityName::LinkPrizePlus),
            "exp chance" | "expchance" | "exp%" => Ok(AbilityName::EXPChance),
            "lucky strike" | "luckystrike" | "ls" => Ok(AbilityName::LuckyStrike),
            "luck boost" | "luckboost" | "luck+" => Ok(AbilityName::LuckBoost),
            "exp walker" | "expwalker" | "expw" => Ok(AbilityName::EXPWalker),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum AbilityCrystal{
    Shimmering,
    Fleeting,
    Pulsing,
    Wellspring,
    Soothing,
    Hungry,
    Abounding
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ability{
    ability_name: AbilityName,
    recipe_types: Vec<RecipeType>,
    ability_crystal: AbilityCrystal
}

#[derive(Debug, Deserialize)]
pub struct Abilities {
    abilities: Vec<Ability>
}

fn find_ability_by_ability_name(ability_name: &str, abilities: &Abilities) -> Result<Ability, String> {
    let ability_name = AbilityName::from_str(ability_name).unwrap_or(AbilityName::Unknown);

    if let Some(ability) = abilities.abilities.iter().find(|&ability| ability.ability_name == ability_name) {
        Ok(ability.clone())
    } else {
        Err(format!("Ability not found: {:?}", ability_name))
    }
}

pub fn search_recipes_by_ability(ability_name: &str, abilities: &Abilities, recipes_result: &serde_json::Result<HashMap<(Command, Command), Recipe>>, active_character: Character){
    let ability_result = find_ability_by_ability_name(ability_name, abilities);

    match ability_result {
        Ok(ability) => {
            if let Ok(recipes) = recipes_result {
                let matching_recipes: Vec<Recipe> = recipes.values()
                    .filter(|recipe| ability.recipe_types.contains(&recipe.recipe_type))
                    .filter(|recipe| {
                        if active_character == Character::Unknown {
                            true
                        } else {
                            recipe.results.iter().any(|result| {
                                result.chances.iter().any(|chance| chance.character == active_character)
                            })
                        }
                    })
                    .cloned()
                    .collect();

                if !matching_recipes.is_empty() {
                    let results = generate_ability_search_results(&ability, matching_recipes, active_character);
                    println!("{}", results);
                } else {
                    println!("No matching recipes found for ability: {:?}", ability.ability_name);
                }
            } else {
                println!("Error with the recipes HashMap.");
            }
        }
        Err(e) => println!("{}", e),
    }
}
fn generate_ability_search_results(ability: &Ability, recipes: Vec<Recipe>, active_character: Character) -> String {
    let mut lines = vec![
        String::from("         Ability Search Results         "),
        "&br".to_string(),
        format!("Ability: {:?}", ability.ability_name),
        format!("Active Character: {:?}", active_character),
        format!("Required Crystal: {:?}", ability.ability_crystal)
    ];

    for recipe in recipes {
        lines.push("&br".to_string());
        lines.push(format!("Recipe Type: {:?}", recipe.recipe_type));
        lines.push(format!("Command #1: {:?}", recipe.command1));
        lines.push(format!("Command #2: {:?}", recipe.command2));

        if active_character != Character::Unknown {
            for (index, result) in recipe.results.iter().enumerate() {
                let mut character_has_result = false;
                for chance in &result.chances {
                    if active_character == chance.character {
                        if !character_has_result {
                            lines.push(format!("Result #{}: {:?}", index + 1, result.result));
                            character_has_result = true;
                        }
                        lines.push(format!("{:?}: {:.0}%", chance.character, chance.activation_chance * 100.0));
                    }
                }
            }
        } else {
            for (index, result) in recipe.results.iter().enumerate() {
                lines.push(format!("Result #{}: {:?}", index + 1, result.result));
                for chance in &result.chances {
                    lines.push(format!("{:?}: {:.0}%", chance.character, chance.activation_chance * 100.0));
                }
            }
        }
    }

    generate_text_box(&lines)
}