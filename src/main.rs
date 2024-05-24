use std::collections::HashSet;
use std::time::Instant;
use std::io::Write;
use std::io;

mod potions;
mod aura;
mod map;

fn main() {
    
    let mut storage: HashSet<String> = HashSet::new();
    let mut current_luck: f64 = 1.0;
    let mut potion_expiry: Option<Instant> = None;
    let mut game_map = map::Map::new();

    // Developer Auth
    let name = get_input("Enter your name: ");
    let password = get_input("Enter your password: ");

    // Check if the name and password match
    if name == "FBDev" && password == "3310" {
        // Add the Sol aura to the storage
        println!("Sol aura added to storage.");
    } else {
        println!("Not a developer, huh.");
    }
    
    let input : String = get_input("> Enter Action : ");

    loop {
    
        if input == "q" {
            break;
        } else if input == "roll" {
            let rolled_aura = aura::roll_aura(current_luck);
            println!("You rolled: {}", rolled_aura.name);
            storage.insert(rolled_aura.name.clone());
        } else if input == "storage" {
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
        } else if input == "potion" {
            println!("Available potions:");
            println!("1. {}", potions::LUCKY_POTION.name);
            println!("2. {}", potions::FORTUNE_I_POTION.name);
            // Print the remaining potion types here
    
            let mut potion_choice = String::new();
            println!("Enter the number of the potion you want to use:");
            io::stdin().read_line(&mut potion_choice).unwrap();
    
            match potion_choice.trim().parse::<u32>() {
                Ok(1) => potions::apply_potion_effect(&potions::LUCKY_POTION, &mut current_luck, &mut potion_expiry),
                Ok(2) => potions::apply_potion_effect(&potions::FORTUNE_I_POTION, &mut current_luck, &mut potion_expiry),
                // Add cases for the remaining potion types here
                _ => println!("Invalid potion choice."),
            }
        } else if input == "move" {
            let mut direction = String::new();
            println!("Enter the direction to move (w/a/s/d):");
            io::stdin().read_line(&mut direction).unwrap();
    
            match direction.trim() {
                "w" => {
                    game_map.move_player(0, -1);
                    game_map.render_map();
                }
                "a" => {
                    game_map.move_player(-1, 0);
                    game_map.render_map();
                }
                "s" => {
                    game_map.move_player(0, 1);
                    game_map.render_map();
                }
                "d" => {
                    game_map.move_player(1, 0);
                    game_map.render_map();
                }
                _ => println!("Invalid direction!"),
            }
    
            if game_map.potions.is_empty() {
                println!("You collected all the potions!");
            }
        } else {
            println!("Invalid input, please try again.");
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
