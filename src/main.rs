#[allow(unused_imports)]
use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use simple_user_input::get_input;
use std::collections::HashSet;

fn main() {
    let mut storage: HashSet<String> = HashSet::new();

    loop {
        let input: String = get_input(">>> Enter Action: ");
        if input == "roll" {
            let rolled_aura = get_random_aura();
            println!("{}", rolled_aura);
            storage.insert(rolled_aura);
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

fn get_random_aura() -> String {
    let auras = vec![
        ("Common", 100000),
        ("Uncommon", 25000),
        ("Good", 20000),
        ("Natural", 12500),
        ("Rare", 6250),
        ("Divinus", 3125),
        ("Crystallized", 1563),
        ("Rage", 781),
        ("Topaz", 667),
        ("Ruby", 286),
        ("Forbidden", 248),
        ("Emerald", 200),
        ("Gilded", 195),
        ("Ink", 143),
        ("Jackpot", 129),
        ("Sapphire", 125),
        ("Aquamarine", 111),
        ("Flushed : Lobotomy", 15),
        ("Hazard : Rays", 14),
        ("Nautilus", 14),
        ("Permafrost", 14),
        ("Stormal", 11),
        ("Exotic", 10),
        ("Diaboli : Void", 10),
        ("Undead : Devil", 8),
        ("Comet", 8),
        ("Jade", 8),
        ("Bounded", 5),
        ("Celestial", 3),
        ("Kaywhite", 1),
        ("Wind", 900),
        ("Diaboli", 1004),
        ("Precious", 1024),
        ("Glock", 1700),
        ("Magnetic", 2048),
        ("Glacier", 2304),
        ("Siderum", 4096),
        ("Bleeding", 4444),
        ("Lunar", 5000),
        ("Solar", 5000),
        ("Starlight", 5000),
        ("Flushed", 6900),
        ("Hazard", 7000),
        ("Quartz", 8192),
        ("Undead", 12000),
        ("Corrosive", 12000),
        ("Rage : Heated", 12800),
        ("Leak", 14000),
        ("Powered", 16384),
        ("Aquatic", 40000),
    ];

    let weights: Vec<_> = auras.iter().map(|&(_, weight)| weight).collect();
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::thread_rng();
    let index = dist.sample(&mut rng);

    let (aura_name, rarity) = auras[index];
    format!("{} (1 in {})", aura_name, rarity)
}

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {}
            Err(_no_updates_is_fine) => {}
        }
        input.trim().to_string()
    }
}
