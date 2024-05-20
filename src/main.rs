use rand::Rng;
use std::collections::HashSet;
use std::io::{self, Write};

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
            range: 0..=5999,
            probability: 0.6,
        },
        AuraInfo {
            name: "Uncommon".to_string(),
            range: 6000..=8999,
            probability: 0.3,
        },
        AuraInfo {
            name: "Rare".to_string(),
            range: 9000..=9999,
            probability: use rand::Rng;
            use std::collections::HashSet;
            use std::io::{self, Write};
            
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
                        range: 0..=5999,
                        probability: 0.6,
                    },
                    AuraInfo {
                        name: "Uncommon".to_string(),
                        range: 6000..=8999,
                        probability: 0.3,
                    },
                    AuraInfo {
                        name: "Rare".to_string(),
                        range: 9000..=9999,
                        probability: 0.1,
                    },
                ];
            
                let mut storage: HashSet<String> = HashSet::new();
            
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
                    } else if input == "help" {
                        println!("Available commands:");
                        println!("roll - Roll a random aura");
                        println!("storage - Display all rolled auras");
                        println!("exit - Exit the program");
                    } else if input == "exit" {
                        break;
                    } else {
                        println!("Action Invalid or unimplemented yet.");
                    }
                }
            }
            ,
        },
    ];

    let mut storage: HashSet<String> = HashSet::new();

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
        } else if input == "help" {
            println!("Available commands:");
            println!("roll - Roll a random aura");
            println!("storage - Display all rolled auras");
            println!("exit - Exit the program");
        } else if input == "exit" {
            break;
        } else {
            println!("Action Invalid or unimplemented yet.");
        }
    }
}
