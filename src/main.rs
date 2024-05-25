use std::collections::HashSet;
use std::io::{self, Write};
use std::time::Instant;

mod map;
mod aura;
mod potions;

fn main() {
    let mut storage: HashSet<String> = HashSet::new();
    let mut current_luck: f64 = 1.0;
    let mut potion_expiry: Option<Instant> = None;

    // Developer Auth
    let name = get_input("Enter your name: ");
    let password = get_input("Enter your password: ");

    // Check if the name and password match
    if name == "FBDev" && password == "3310" {
        // Add the Sol aura to the storage
        println!("Sol aura added to storage.");
        storage.insert("Sol".to_string());
    } else {
        println!("Not a developer, huh.");
    }

    let mut game_map = map::Map::new(); // Create a new instance of the Map struct

    game_map.render_map();

    loop {
        print!(">>> Enter Action by pressing : then the action\n>>> Type help for a full actions list.\n");
        io::stdout().flush().unwrap();

        let action = get_input(": ");

        if action == "q" {
            break;
        } else if action == "help" {
            println!("Available actions:");
            println!(":w - Move up");
            println!(":a - Move left");
            println!(":s - Move down");
            println!(":d - Move right");
            println!(":q - Quit");
            println!(":roll - Roll an aura");
            println!(":storage - Display rolled auras");
            println!(":potion - Use a potion");
        } else if action == "w" {
            game_map.clear_screen();
            game_map.move_player(0, -1);
        } else if action == "a" {
            game_map.clear_screen();
            game_map.move_player(-1, 0);
        } else if action == "s" {
            game_map.clear_screen();
            game_map.move_player(0, 1);
        } else if action == "d" {
            game_map.clear_screen();
            game_map.move_player(1, 0);
        } else if action == "roll" {
            game_map.clear_screen();
            let rolled_aura = aura::roll_aura(current_luck);
            println!("You rolled: {}", rolled_aura.name);
            storage.insert(rolled_aura.name.clone());
        } else if action == "storage" {
            game_map.clear_screen();
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
        } else if action == "potion" {
            game_map.clear_screen();
            println!("Available potions:");
            println!("1. {}", potions::LUCKY_POTION.name);
            println!("2. {}", potions::FORTUNE_I_POTION.name);
            // Print the remaining potion types here

            let potion_choice = get_input("Enter the number of the potion you want to use: ");

            match potion_choice.trim().parse::<u32>() {
                Ok(1) => potions::apply_potion_effect(&potions::LUCKY_POTION, &mut current_luck, &mut potion_expiry),
                Ok(2) => potions::apply_potion_effect(&potions::FORTUNE_I_POTION, &mut current_luck, &mut potion_expiry),
                // Add cases for the remaining potion types here
                _ => println!("Invalid potion choice."),
            }
        } else {
            game_map.clear_screen();
            println!("Invalid action!");
        }

        // Check if the potion effect has expired
        if let Some(expiry_time) = potion_expiry {
            if Instant::now() >= expiry_time {
                current_luck = 1.0;
                potion_expiry = None;
                println!("Potion effect has expired. Luck reset to normal.");
            }
        }
    }
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
