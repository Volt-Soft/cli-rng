#[allow(unused_imports)]

use rand::Rng;
use chrono;
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
// TODO : Implement a system to roll a random aura but witha certain chance percentage of aura being a certain aura.            
fn get_random_aura() -> String {
    let aura = vec![
        "Common : 1 in 2",
        "Uncommon : 1 in 4",
        "Good : 1 in 5",
        "Natural : 1 in 8",
        "Rare : 1 in 16",
        "Divinus : 1 in 32",
        "Crystallized : 1 in 64",
        "Rage : 1 in 128",
        "Topaz : 1 in 150",
        "Ruby : 1 in 350",
        "Forbidden : 1 in 404",
        "Emerald : 1 in 500",
        "Gilded : 1 in 500",
        "Ink",
        "Jackpot",
        "Sapphire",
        "Aquamarine",
        "Wind",
        "Diaboli",
        "Precious",
        "Glock",
        "Magnetic",
        "Glacier",
        "Siderum",
        "Bleeding",
        "Lunar",
        "Solar",
        "Starlight",
        "Flushed",
        "Hazard",
        "Quartz",
        "Undead",
        "Corrosive",
        "Rage : Heated",
        "Leak",
        "Powered",
        "Aquatic",
        "Flushed : Lobotomy",
        "Hazard : Rays",
        "Nautilus",
        "Permafrost",
        "Stormal",
        "Exotic",
        "Diaboli : Void",
        "Undead : Devil",
        "Comet",
        "Jade",
        "Bounded",
        "Celestial",
        "Kaywhite"

    ];
    let random_index = rand::thread_rng().gen_range(0..aura.len());    
    return aura[random_index].to_string();
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
