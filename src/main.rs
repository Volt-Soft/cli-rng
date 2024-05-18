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
fn get_random_aura() -> String {
    let aura = vec![
        "Common",
        "Uncommon",
        "Good",
        "Natural",
        "Rare",
        "Divinus",
        "Crystallized",
        "Rage",
        "Topaz",
        "Ruby",
        "Forbidden",
        "Emerald",
        "Gilded",
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
    let random_index = rand::thread_rng().gen_range(0.jokes.len());    
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
