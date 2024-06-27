
struct Ability{
    ability_name: String,
    ability_type: AbilityType,
    max_stack: i8,
}

pub enum AbilityType{
    Prize,
    Stat,
    Support,
}