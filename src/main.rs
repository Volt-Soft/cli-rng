#[allow(unused_imports)]

use rand::Rng;
use rand::distributions::WeightedIndex;
use rand::distributions::Distribution;
use simple_user_input::get_input;


fn main() {
    loop {
        let input: String = get_input("Enter Command, help to display commands");
        if input == "roll" {
            println!("{}", get_random_aura())
        } 
        
        if input == "exit" {
            break;
        }

        else {
            println!("Action Invalid or unimplemented yet.");
        }
    }
}

fn get_random_aura() -> String {
    let auras = vec![
        ("Common", 2),
        ("Uncommon", 4),
        ("Good", 5),
        ("Natural", 8),
        ("Rare", 16),
        ("Divinus", 32),
        ("Crystallized", 64),
        ("Rage", 128),
        ("Topaz", 150),
        ("Ruby", 350),
        ("Forbidden", 404),
        ("Emerald", 500),
        ("Gilded", 512),
        ("Ink", 700),
        ("Jackpot", 777),
        ("Sapphire", 1),
        ("Aquamarine", 1),
        ("Flushed : Lobotomy", 1),
        ("Hazard : Rays", 1),
        ("Nautilus", 1),
        ("Permafrost", 1),
        ("Stormal", 1),
        ("Exotic", 99999),
        ("Diaboli : Void", 100400),
        ("Undead : Devil", 120000),
        ("Comet", 120000),
        ("Jade", 125000),
        ("Bounded", 200000),
        ("Celestial", 350000),
        ("Kaywhite", 850000),
        ("Wind", 1),
        ("Diaboli", 1),
        ("Precious", 1),
        ("Glock", 1),
        ("Magnetic", 1),
        ("Glacier", 1),
        ("Siderum", 1),
        ("Bleeding", 1),
        ("Lunar", 1),
        ("Solar", 1),
        ("Starlight", 1),
        ("Flushed", 1),
        ("Hazard", 1),
        ("Quartz", 1),
        ("Undead", 1),
        ("Corrosive", 1),
        ("Rage : Heated", 1),
        ("Leak", 1),
        ("Powered", 1),
        ("Aquatic", 1),
    ];

    let weights: Vec<_> = auras.iter().map(|&(_, weight)| weight).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::thread_rng();
    let index = dist.sample(&mut rng);

    auras[index].0.to_string()
}


mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}
