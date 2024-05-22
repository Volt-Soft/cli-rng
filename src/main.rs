#[allow(dead_code)]

use rand::Rng;

use std::collections::HashSet;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::io::Read;

struct AuraInfo {
    name: String,
    range: std::ops::RangeInclusive<u64>,
    probability: f64,
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn roll_aura(auras: &[AuraInfo]) -> &AuraInfo {
    let mut rng = rand::thread_rng();
    let random_value: f64 = rng.gen_range(0.0..1.0);
    let mut cumulative_probability = 0.0;

    for aura in auras {
        cumulative_probability += aura.probability;
        if random_value < cumulative_probability {
            return aura;
        }
    }

    &auras[auras.len() - 1]
}

fn main() {
    

    let auras: Vec<AuraInfo> = vec![
        AuraInfo {
            name: "Common".to_string(),
            range: 0..=4999,
            probability: 0.5,
        },
        AuraInfo {
            name: "Uncommon".to_string(),
            range: 5000..=7499,
            probability: 0.25,
        },
        AuraInfo {
            name: "Good".to_string(),
            range: 7500..=8749,
            probability: 0.2,
        },
        AuraInfo {
            name: "Natural".to_string(),
            range: 8750..=9999,
            probability: 0.125,
        },
        AuraInfo {
            name: "Rare".to_string(),
            range: 10000..=10062,
            probability: 0.0625,
        },
        AuraInfo {
            name: "Divinus".to_string(),
            range: 10063..=10093,
            probability: 0.03125,
        },
        AuraInfo {
            name: "Crystallized".to_string(),
            range: 10094..=10109,
            probability: 0.015625,
        },
        AuraInfo {
            name: "Rage".to_string(),
            range: 10110..=10117,
            probability: 0.0078125,
        },
        AuraInfo {
            name: "Topaz".to_string(),
            range: 10118..=10124,
            probability: 0.0066666667,
        },
        AuraInfo {
            name: "Ruby".to_string(),
            range: 10125..=10129,
            probability: 0.0028571429,
        },
        AuraInfo {
            name: "Forbidden".to_string(),
            range: 10130..=10132,
            probability: 0.0024691358,
        },
        AuraInfo {
            name: "Emerald".to_string(),
            range: 10133..=10134,
            probability: 0.002,
        },
        AuraInfo {
            name: "Gilded".to_string(),
            range: 10135..=10136,
            probability: 0.0019531250,
        },
        AuraInfo {
            name: "Ink".to_string(),
            range: 10137..=10137,
            probability: 0.0014285714,
        },
        AuraInfo {
            name: "Jackpot".to_string(),
            range: 10138..=10138,
            probability: 0.0012820513,
        },
        AuraInfo {
            name: "Sapphire".to_string(),
            range: 10139..=10139,
            probability: 0.0011111111,
        },
        AuraInfo {
            name: "Aquamarine".to_string(),
            range: 10140..=10140,
            probability: 0.0011111111,
        },
        AuraInfo {
            name: "Wind".to_string(),
            range: 10141..=10141,
            probability: 0.0011111111,
        },
        AuraInfo {
            name: "Diaboli".to_string(),
            range: 10142..=10143,
            probability: 0.0009950249,
        },
        AuraInfo {
            name: "Precious".to_string(),
            range: 10144..=10145,
            probability: 0.0009765625,
        },
        AuraInfo {
            name: "Glock".to_string(),
            range: 10146..=10146,
            probability: 0.0005882353,
        },
        AuraInfo {
            name: "Magnetic".to_string(),
            range: 10147..=10148,
            probability: 0.0004882813,
        },
        AuraInfo {
            name: "Glacier".to_string(),
            range: 10149..=10150,
            probability: 0.0004329004,
        },
        AuraInfo {
            name: "Siderum".to_string(),
            range: 10151..=10152,
            probability: 0.0002441406,
        },
        AuraInfo {
            name: "Bleeding".to_string(),
            range: 10153..=10153,
            probability: 0.0002252252,
        },
        AuraInfo {
            name: "Solar".to_string(),
            range: 10154..=10154,
            probability: 0.0000200000,
        },
        AuraInfo {
            name: "Lunar".to_string(),
            range: 10155..=10155,
            probability: 0.0000200000,
        },
        AuraInfo {
            name: "STARLIGHT".to_string(),
            range: 10156..=10156,
            probability: 0.0000200000,
        },
        AuraInfo {
            name: ":Flushed:".to_string(),
            range: 10157..=10157,
            probability: 0.0001449275,
        },
        AuraInfo {
            name: "HAZARD".to_string(),
            range: 10158..=10158,
            probability: 0.0001428571,
        },
        AuraInfo {
            name: "Quartz".to_string(),
            range: 10159..=10160,
            probability: 0.0001220703,
        },
        AuraInfo {
            name: "Undead".to_string(),
            range: 10161..=10161,
            probability: 0.0000833333,
        },
        AuraInfo {
            name: "Corrosive".to_string(),
            range: 10162..=10162,
            probability: 0.0000833333,
        },
        AuraInfo {
            name: "Rage : Heated".to_string(),
            range: 10163..=10163,
            probability: 0.0000781250,
        },
        AuraInfo {
            name: "L E A K".to_string(),
            range: 10164..=10164,
            probability: 0.0000714286,
        },
        AuraInfo {
            name: "Powered".to_string(),
            range: 10165..=10166,
            probability: 0.0000610352,
        },
        AuraInfo {
            name: "Aquatic".to_string(),
            range: 10167..=10167,
            probability: 0.0000250000,
        },
        AuraInfo {
            name: "Flushed : Lobotomy".to_string(),
            range: 10168..=10168,
            probability: 0.0000144928,
        },
        AuraInfo {
            name: "HZARD : RAYS".to_string(),
            range: 10169..=10169,
            probability: 0.0000142857,
        },
        AuraInfo {
            name: "Nautilus".to_string(),
            range: 10170..=10170,
            probability: 0.0000142857,
        },
        AuraInfo {
            name: "Permaforst".to_string(),
            range: 10171..=10171,
            probability: 0.0000136054,
        },
        AuraInfo {
            name: "Starmal".to_string(),
            range: 10172..=10172,
            probability: 0.0000111111,
        },
        AuraInfo {
            name: "Exotic".to_string(),
            range: 10173..=10173,
            probability: 0.0000100010,
        },
        AuraInfo {
            name: "Diaboli : Void".to_string(),
            range: 10174..=10174,
            probability: 0.0000099502,
        },
        AuraInfo {
            name: "Comet".to_string(),
            range: 10175..=10175,
            probability: 0.0000083333,
        },
        AuraInfo {
            name: "Jade".to_string(),
            range: 10176..=10176,
            probability: 0.0000080000,
        },
        AuraInfo {
            name: "BOUNDED".to_string(),
            range: 10177..=10177,
            probability: 0.0000500000,
        },
        AuraInfo {
            name: "Celestial".to_string(),
            range: 10178..=10178,
            probability: 0.0000028571,
        },
    ];

    let mut storage: HashSet<String> = HashSet::new();
    
    // Developer Auth
    let name: String = get_input(">>> Enter Name: ");
    let passwd: u32 = get_input(">>> Enter Password: ").parse().unwrap();
    if name == "FBDev" && passwd ==  3310 {
        storage.insert("Sol".to_string());
    }

    loop {

        let input: String = get_input(">>> Enter Action: ");
        if input == "roll" {
            let rolled_aura = roll_aura(&auras);
            println!("Rolled aura: {}", rolled_aura.name);
            storage.insert(rolled_aura.name.clone());
        } else if input == "storage" {
            if storage.is_empty() {
                println!("No auras rolled yet.");
            } else {
                println!("Rolled auras:");
                let mut auras: Vec<String> = storage.iter().map(|s| s.to_string()).collect();
                auras.sort();
                println!("{}", auras.join(", "));
            }
            
        } else if input == "reset" {
            storage.clear();
        }  else if input == "help" {
            println!("Available commands:");
            println!("roll - Roll a random aura");
            println!("autoroll - Roll automatically auras");
            println!("reset - Reset the aura storage");
            println!("storage - Display all rolled auras");
            println!("exit - Exit the program");
        } else if input == "exit" {
            break;
        } else if input == "autoroll" {
            println!("Press any key to stop autorolling...");
            autoroll(&auras);
        } else {
            println!("Action Invalid or unimplemented yet.");
        }
    }
}
