use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Character{
    Terra,
    Aqua,
    Ventus,
    Unknown,
}

pub fn display_character_help() {
    println!("Character commands:");
    println!("\x1b[1mchar change [character]\x1b[0m - Change the current character. Available options: Terra, Aqua, Ventus");
    println!("\x1b[1mchar status\x1b[0m - Display the current character");
    println!("\x1b[1mchar help\x1b[0m - Show this help message");
}

pub fn display_character_status(character: Character){
    println!("The current selected character is: {:?}", character);
    if character == Character::Unknown{
        println!("Performing operations as an Unknown character will produce the results for \x1b[1mall three character\x1b[0m.");
        println!("This is completely acceptable for the intended use of the program but \x1b[1mmay produce extremely long outputs\x1b[0m.");
    }
}

pub fn change_character(args: &[String], character: &mut Character){
    if args[0].is_empty(){
        println!("\x1b[38;5;196mERROR\x1b[0m: No character specified. Please refer to \x1b[1mchar help\x1b[0m for more information.")
    }

    match args[0].to_lowercase().as_str() {
        "terra" | "-t" => set_character(Character::Terra, character),
        "aqua" | "-a" => set_character(Character::Aqua, character),
        "ventus" | "-v" | "ven" => set_character(Character::Ventus, character),
        "reset" | "-r" | "unknown" | "-u" => set_character(Character::Unknown, character),
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

fn set_character(desired_character: Character, character: &mut Character){
    *character = desired_character;
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_set_character(){
        let mut character = Character::Unknown;

        set_character(Character::Terra, &mut character);
        assert_eq!(character, Character::Terra);

        set_character(Character::Aqua, &mut character);
        assert_eq!(character, Character::Aqua);

        set_character(Character::Ventus, &mut character);
        assert_eq!(character, Character::Ventus);

        set_character(Character::Unknown, &mut character);
        assert_eq!(character, Character::Unknown);
    }

    #[test]
    fn test_change_character(){

        let mut character = Character::Unknown;

        change_character(&vec!["-t".to_string()], &mut character);
        assert_eq!(character, Character::Terra);

        change_character(&vec!["-a".to_string()], &mut character);
        assert_eq!(character, Character::Aqua);

        change_character(&vec!["-v".to_string()], &mut character);
        assert_eq!(character, Character::Ventus);

        change_character(&vec!["-r".to_string()], &mut character);
        assert_eq!(character, Character::Unknown);
    }
}