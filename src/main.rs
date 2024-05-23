use std::collections::HashSet;
use std::io::Write;
use std::io;

mod aura;

fn main() {
    
    let mut storage: HashSet<String> = HashSet::new();

    // Developer Auth
    let mut name: String = String::new();
    println!("Enter your name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();

    let mut passwd_str = String::new();
    println!("Enter your password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut passwd_str).unwrap();

    let passwd: u32 = match passwd_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid password format. Please enter a number.");
            return;
        }
    };

    if name.trim() == "FBDev" && passwd == 3310 {
        println!("Spotted the... Cli ?");
    }
loop {
    let mut input = String::new();
    println!("Enter 'roll' to roll for an aura, 'storage' to view rolled auras, or 'q' to quit:");
    io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "q" {
        break;
    } else if input.trim() == "roll" {
        let rolled_aura = aura::roll_aura();
        println!("You rolled: {}", rolled_aura.name);
        storage.insert(rolled_aura.name.clone());
    } else if input.trim() == "storage" {
        if storage.is_empty() {
            println!("No auras rolled yet.");
        } else {
            println!("Rolled auras:");
            for aura_name in &storage {
                let aura_info = aura::AURAS
                    .iter()
                    .find(|info| info.name == *aura_name)
                    .unwrap();
                let probability = 1.0 / aura_info.probability;
                println!("[{}, 1 in {}]", aura_info.name, probability);
            }
        }
    } else {
        println!("Invalid input, please try again.");
    }
}
}
