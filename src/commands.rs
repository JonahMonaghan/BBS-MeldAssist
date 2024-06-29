use std::str::FromStr;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub enum Command{
    QuickBlitz,
    Blitz,
    MeteorCrash,
    MagicHour,
    SlidingDash,
    FireDash,
    DarkHaze,
    SonicBlade,
    ChaosBlade,
    Zantetsuken,
    StrikeRaid,
    FreezeRaid,
    TreasureRaid,
    SparkRaid,
    WindRaid,
    FireSurge,
    BarrierSurge,
    ThunderSurge,
    AerialSlam,
    ArsSolum,
    ArsArcanum,
    TimeSplicer,
    PoisonEdge,
    WishingEdge,
    BlizzardEdge,
    StunEdge,
    SlotEdge,
    FireStrike,
    ConfusionStrike,
    BindingStrike,
    TornadoStrike,
    BrutalBlast,
    MagnetSpiral,
    Salvation,
    Windcutter,
    LimitStorm,
    CollisionMagnet,
    GeoImpact,
    Sacrifice,
    BreakTime,
    Fire,
    Fira,
    Firaga,
    DarkFiraga,
    FissionFiraga,
    TripleFiraga,
    CrawlingFire,
    Blizzard,
    Blizzara,
    Blizzaga,
    TripleBlizzaga,
    Thunder,
    Thundara,
    Thundaga,
    ThundagaShot,
    Cure,
    Cura,
    Curaga,
    Esuna,
    MineShield,
    MineSquare,
    SeekerMine,
    ZeroGravity,
    ZeroGravira,
    ZeroGraviga,
    Magnet,
    Magnera,
    Magnega,
    MunnyMagnet,
    EnergyMagnet,
    DLinkMagnet,
    Aero,
    Aerora,
    Aeroga,
    Warp,
    Faith,
    DeepFreeze,
    Glacier,
    IceBarrage,
    FiragaBurst,
    RagingStorm,
    MegaFlare,
    Quake,
    Meteor,
    Tornado,
    Transcendence,
    Mini,
    Ignite,
    Confuse,
    Bind,
    Poison,
    Slow,
    Stop,
    Stopra,
    Stopga,
    Sleep,
    Blackout,
    Block,
    RenewalBlock,
    FocusBlock,
    StunBlock,
    PoisonBlock,
    Barrier,
    RenewalBarrier,
    FocusBarrier,
    ConfuseBarrier,
    StopBarrier,
    CounterHammer,
    PaybackFang,
    CounterRush,
    ReversalSlash,
    PaybackRaid,
    CounterBlast,
    PaybackSurge,
    AerialRecovery,
    MeteorShower,
    FlameSalvo,
    ChaosSnake,
    DarkVolley,
    BubbleBlaster,
    Ragnarok,
    Thunderstorm,
    BioBarrage,
    PulseBomb,
    PrismRain,
    PhotonCharge,
    AbsoluteZero,
    LightningRay,
    SonicShadow,
    UltimaCannon,
    Multivortex,
    Lightbloom,
    DarkLink,
    Jump,
    HighJump,
    DodgeRoll,
    ThunderRoll,
    Cartwheel,
    Firewheel,
    AirSlide,
    IceSlide,
    SonicImpact,
    Slide,
    Reversal,
    Glide,
    Superglide,
    FireGlide,
    HomingSlide,
    Teleport,
    Doubleflight,
    Unknown,
}
impl FromStr for Command{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "quick blitz" | "quickblitz" | "qb" => Ok(Command::QuickBlitz),
            "blitz" => Ok(Command::Blitz),
            "meteor crash" | "meteorcrash" | "mc" => Ok(Command::MeteorCrash),
            "magic hour" | "magichour" | "mh" => Ok(Command::MagicHour),
            "sliding dash" | "slidingdash" | "sd" => Ok(Command::SlidingDash),
            "fire dash" | "firedash" | "fd" => Ok(Command::FireDash),
            "dark haze" | "darkhaze" | "dh" => Ok(Command::DarkHaze),
            "sonic blade" | "sonicblade" | "s-bld" => Ok(Command::SonicBlade),
            "chaos blade" | "chaosblade" | "c-bld" => Ok(Command::ChaosBlade),
            "zantetsuken" | "ztt" => Ok(Command::Zantetsuken),
            "strike raid" | "strikeraid" | "st-raid" => Ok(Command::StrikeRaid),
            "freeze raid" | "freezeraid" | "f-raid" => Ok(Command::FreezeRaid),
            "treasure raid" | "treasureraid" | "t-raid" => Ok(Command::TreasureRaid),
            "spark raid" | "sparkraid" | "sp-raid" => Ok(Command::SparkRaid),
            "wind raid" | "windraid" | "w-raid" => Ok(Command::WindRaid),
            "fire surge" | "firesurge" | "f-surge" => Ok(Command::FireSurge),
            "barrier surge" | "barriersurge" | "b-surge" => Ok(Command::BarrierSurge),
            "aerial slam" | "aerialslam" | "as" => Ok(Command::AerialSlam),
            "ars solum" | "arssolum" | "ars-s" => Ok(Command::ArsSolum),
            "ars arcanum" | "arsarcanum" | "ars-a" => Ok(Command::ArsArcanum),
            "time splicer" | "timesplicer" | "ts" => Ok(Command::TimeSplicer),
            "poison edge" | "poisionedge" | "p-edge" => Ok(Command::PoisonEdge),
            "wishing edge" | "wishingedge" | "w-edge" => Ok(Command::WishingEdge),
            "blizzard edge" | "blizzardedge" | "b-edge" => Ok(Command::BlizzardEdge),
            "stun edge" | "stunedge" | "st-edge" => Ok(Command::StunEdge),
            "slot edge" | "slotedge" | "sl-edge" => Ok(Command::SlotEdge),
            "fire strike" | "firestrike" | "f-strike" => Ok(Command::FireStrike),
            "confusion strike" | "confusionstrike" | "c-strike" => Ok(Command::ConfusionStrike),
            "binding strike" | "bindingstrike" | "b-strike" => Ok(Command::BindingStrike),
            "tornado strike" | "tornadostrike" | "t-strike" => Ok(Command::TornadoStrike),
            "brutal blast" | "brutalblast" | "bb" => Ok(Command::BrutalBlast),
            "magnet spiral" | "magnetspiral" | "ms" => Ok(Command::MagnetSpiral),
            "salvation" => Ok(Command::Salvation),
            "windcutter" => Ok(Command::Windcutter),
            "limit storm" | "limitstorm" | "ls" => Ok(Command::LimitStorm),
            "collision magnet" | "collisionmagnet" | "cm" => Ok(Command::CollisionMagnet),
            "geo impact" | "geoimpact" | "gi" => Ok(Command::GeoImpact),
            "sacrifice" => Ok(Command::Sacrifice),
            "break time" | "breaktime" | "bt" => Ok(Command::BreakTime),
            "fire" | "f1" => Ok(Command::Fire),
            "fira" | "f2" => Ok(Command::Fira),
            "firaga" | "f3" => Ok(Command::Firaga),
            "dark firaga" | "darkfiraga" | "f-dark" => Ok(Command::DarkFiraga),
            "fission firaga" | "fissionfiraga" | "ff" => Ok(Command::FissionFiraga),
            "triple firaga" | "triplefiraga" | "trip-f" => Ok(Command::TripleFiraga),
            "crawling fire" | "crawlingfire" | "cf" => Ok(Command::CrawlingFire),
            "blizzard" | "b1" => Ok(Command::Blizzard),
            "blizzara" | "b2" => Ok(Command::Blizzara),
            "blizzaga" | "b3" => Ok(Command::Blizzaga),
            "triple blizzaga" | "tripleblizzaga" | "trip-b" => Ok(Command::TripleBlizzaga),
            "thunder" | "t1" => Ok(Command::Thunder),
            "thundara" | "t2" => Ok(Command::Thundara),
            "thundaga" | "t3" => Ok(Command::Thundaga),
            "thundaga shot" | "thundagashot" | "t-shot" => Ok(Command::ThundagaShot),
            "cure" | "c1" => Ok(Command::Cure),
            "cura" | "c2" => Ok(Command::Cura),
            "curaga" | "c3" => Ok(Command::Curaga),
            "esuna" => Ok(Command::Esuna),
            "mine shield" | "mineshield" => Ok(Command::MineShield),
            "mine square" | "minesquare" => Ok(Command::MineSquare),
            "seeker mine" | "seekermine" => Ok(Command::SeekerMine),
            "zero gravity" | "zerogravity" | "zg1" => Ok(Command::ZeroGravity),
            "zero gravira" | "zerogravira" | "zg2" => Ok(Command::ZeroGravira),
            "zero graviga" | "zerograviga" | "zg3" => Ok(Command::ZeroGraviga),
            "magnet" | "m1" => Ok(Command::Magnet),
            "magnera" | "m2" => Ok(Command::Magnera),
            "magnega" | "m3" => Ok(Command::Magnega),
            "munny magnet" | "munnymagnet" | "m-mag" => Ok(Command::MunnyMagnet),
            "energy magnet" | "energymagnet" | "e-mag" => Ok(Command::EnergyMagnet),
            "dlink magnet" | "dlinkmagnet" | "d-mag" => Ok(Command::DLinkMagnet),
            "aero" | "a1" => Ok(Command::Aero),
            "aerora" | "a2" => Ok(Command::Aerora),
            "aeroga" | "a3" => Ok(Command::Aeroga),
            "warp" => Ok(Command::Warp),
            "faith" => Ok(Command::Faith),
            "deep freeze" | "deepfreeze" | "df" => Ok(Command::DeepFreeze),
            "glacier" => Ok(Command::Glacier),
            "ice barrage" | "icebarrage" | "ib" => Ok(Command::IceBarrage),
            "firaga burst" | "firagaburst" | "fb" => Ok(Command::FiragaBurst),
            "raging storm" | "ragingstorm" | "rs" => Ok(Command::RagingStorm),
            "mega flare" | "megaflare" | "mf" => Ok(Command::MegaFlare),
            "quake" => Ok(Command::Quake),
            "meteor" => Ok(Command::Meteor),
            "tornado" => Ok(Command::Tornado),
            "transcendence" => Ok(Command::Transcendence),
            "mini" => Ok(Command::Mini),
            "ignite" => Ok(Command::Ignite),
            "confuse" => Ok(Command::Confuse),
            "bind" => Ok(Command::Bind),
            "poison" => Ok(Command::Poison),
            "slow" => Ok(Command::Slow),
            "stop" | "s1" => Ok(Command::Stop),
            "stopra" | "s2" => Ok(Command::Stopra),
            "stopga" | "s3" => Ok(Command::Stopga),
            "sleep" => Ok(Command::Sleep),
            "blackout" => Ok(Command::Blackout),
            "block" => Ok(Command::Block),
            "renewal block" | "renewalblock" | "r-blk" => Ok(Command::RenewalBlock),
            "focus block" | "focusblock" | "f-blk" => Ok(Command::FocusBlock),
            "stun block" | "stunblock" | "s-blk" => Ok(Command::StunBlock),
            "poison block" | "poisonblock" | "p-blk" => Ok(Command::PoisonBlock),
            "barrier" => Ok(Command::Barrier),
            "renewal barrier" | "renewalbarrier" | "r-brr" => Ok(Command::RenewalBarrier),
            "focus barrier" | "focusbarrier" | "f-brr" => Ok(Command::FocusBarrier),
            "confuse barrier" | "confusebarrier" | "c-brr" => Ok(Command::ConfuseBarrier),
            "stop barrier" | "stopbarrier" | "s-brr" => Ok(Command::StopBarrier),
            "counter hammer" | "counterhammer" | "ch" => Ok(Command::CounterHammer),
            "payback fang" | "paybackfang" | "pf" => Ok(Command::PaybackFang),
            "counter rush" | "counterrush" | "cr" => Ok(Command::CounterRush),
            "reversal slash" | "reversalslash" => Ok(Command::ReversalSlash),
            "payback raid" | "paybackraid" | "p-raid" => Ok(Command::PaybackRaid),
            "counter blast" | "counterblast" | "cb" => Ok(Command::CounterBlast),
            "payback surge" | "paybacksurge" | "ps" => Ok(Command::PaybackSurge),
            "aerial recovery" | "aerialrecovery" | "ar" => Ok(Command::AerialRecovery),
            "meteor shower" | "meteorshower" | "metshr" => Ok(Command::MeteorShower),
            "flame salvo" | "flamesalvo" | "fs" => Ok(Command::FlameSalvo),
            "chaos snake" | "chaossnake" | "cs" => Ok(Command::ChaosSnake),
            "dark volley" | "darkvolley" | "dv" => Ok(Command::DarkVolley),
            "bubble blaster" | "bubbleblaster" | "bubble" => Ok(Command::BubbleBlaster),
            "ragnarok" => Ok(Command::Ragnarok),
            "thunderstorm" => Ok(Command::Thunderstorm),
            "bio barrage" | "biobarrage" | "bio" => Ok(Command::BioBarrage),
            "pulse bomb" | "pulsebomb" | "pb" => Ok(Command::PulseBomb),
            "prism rain" | "prismrain" | "pr" => Ok(Command::PrismRain),
            "photon charge" | "photoncharge" | "pc" => Ok(Command::PhotonCharge),
            "absolute zero" | "absolutezero" | "az" => Ok(Command::AbsoluteZero),
            "lightning ray" | "lightningray" | "lr" => Ok(Command::LightningRay),
            "sonic shadow" | "sonicshadow" | "ss" => Ok(Command::SonicShadow),
            "ultima cannon" | "ultimacannon" | "uc" => Ok(Command::UltimaCannon),
            "multivortex" => Ok(Command::Multivortex),
            "lightbloom" => Ok(Command::Lightbloom),
            "dark link" | "darklink" | "dl" => Ok(Command::DarkLink),
            "jump" => Ok(Command::Jump),
            "high jump" | "highjump" | "h-jump" => Ok(Command::HighJump),
            "dodge roll" | "dodgeroll" | "d-roll" => Ok(Command::DodgeRoll),
            "thunder roll" | "thunderroll" | "t-roll" => Ok(Command::ThunderRoll),
            "cartwheel" => Ok(Command::Cartwheel),
            "firewheel" => Ok(Command::Firewheel),
            "air slide" | "airslide" | "a-slide" => Ok(Command::AirSlide),
            "ice slide" | "iceslide" | "i-slide" => Ok(Command::IceSlide),
            "sonic impact" | "sonicimpact" | "si" => Ok(Command::SonicImpact),
            "slide" => Ok(Command::Slide),
            "reversal" => Ok(Command::Reversal),
            "glide" => Ok(Command::Glide),
            "superglide" => Ok(Command::Superglide),
            "fire glide" | "fireglide" | "f-glide" => Ok(Command::FireGlide),
            "homing slide" | "homingslide" | "h-slide" => Ok(Command::HomingSlide),
            "teleport" => Ok(Command::Teleport),
            "doubleflight" => Ok(Command::Doubleflight),
            "unknown" => Ok(Command::Unknown),
            _ => Err(()),
        }
    }
}

pub fn normalize_commands(command1: &Command, command2: &Command) -> (Command, Command){
    if command1 < command2 {
        (command1.clone(), command2.clone())
    }else{
        (command2.clone(), command1.clone())
    }
}

pub fn display_commands_help() {
        println!("Command management:");
        println!("\x1b[1mcommands change [options]\x1b[0m - Change the current commands.");
        println!("    Options:");
        println!("        \x1b[1m-c1 [command]\x1b[0m | \x1b[1m--command1 [command]\x1b[0m - Set the first command slot");
        println!("        \x1b[1m-c2 [command]\x1b[0m | \x1b[1m--command2 [command]\x1b[0m - Set the second command slot");
        println!("        \x1b[1m-s\x1b[0m | \x1b[1m--status\x1b[0m                    - Display the current commands after changing");
        println!("    Note: At least one of \x1b[1m-c1\x1b[0m or \x1b[1m-c2\x1b[0m options must be provided.");
        println!("          Example: \x1b[1mcommands change -c1 Fire -c2 Blizzard\x1b[0m");

        println!("\x1b[1mcommands status\x1b[0m - Display the current commands.");
        println!("\x1b[1mcommands help\x1b[0m - Show this help message.");
}

pub fn display_commands_status(command1: Command, command2: Command){
    println!("Command Slot 1: {:?}", command1);
    println!("Command Slot 2: {:?}", command2);
}

pub fn change_commands(args: &[String], command1: &mut Command, command2: &mut Command){
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
                set_command(args[index].clone(), command1);
            }
            "-c2" | "--command2" =>{
                index += 1;
                set_command(args[index].clone(), command2);
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

fn set_command(args: String, command: &mut Command){
    match args.parse::<Command>(){
        Ok(cmd) => *command = cmd,
        Err(_) => println!("Invalid Command"),
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_set_command(){
        let mut command = Command::Unknown;
        set_command("f1".to_string(), &mut command);
        assert_eq!(command, Command::Fire);
        set_command("ztt".to_string(), &mut command);
        assert_eq!(command, Command::Zantetsuken);

    }

    #[test]
    fn test_change_commands(){
        let mut command1 = Command::Unknown;
        let mut command2 = Command::Unknown;

        change_commands(&vec!["-c1".to_string(), "b1".to_string()], &mut command1, &mut command2);
        assert_eq!(command1, Command::Blizzard);
        assert_eq!(command2, Command::Unknown);

        change_commands(&vec!["-c2".to_string(), "zg1".to_string()], &mut command1, &mut command2);
        assert_eq!(command1, Command::Blizzard);
        assert_eq!(command2, Command::ZeroGravity);

        change_commands(&vec!["-c1".to_string(), "ztt".to_string(), "-c2".to_string(), "s-bld".to_string()], &mut command1, &mut command2);
        assert_eq!(command1, Command::Zantetsuken);
        assert_eq!(command2, Command::SonicBlade);
    }
}