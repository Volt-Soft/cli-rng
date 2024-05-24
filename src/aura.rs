#[allow(unused_mut)]

use once_cell::sync::Lazy;
use rand::Rng;

pub struct Aura {
    pub name: String,
    pub probability: f64,
}

pub static AURAS: Lazy<Vec<Aura>> = Lazy::new(|| {
    vec![
        Aura {
            name: "Common".to_string(),
            probability: 0.5,
        },
        Aura {
            name: "Uncommon".to_string(),
            probability: 0.25,
        },
        Aura {
            name: "Good".to_string(),
            probability: 0.2,
        },
        Aura {
            name: "Natural".to_string(),
            probability: 0.125,
        },
        Aura {
            name: "Rare".to_string(),
            probability: 0.0625,
        },
        Aura {
            name: "Divinus".to_string(),
            probability: 0.03125,
        },
        Aura {
            name: "Crystallized".to_string(),
            probability: 0.015625,
        },
        Aura {
            name: "Rage".to_string(),
            probability: 0.0078125,
        },
        Aura {
            name: "Topaz".to_string(),
            probability: 0.0066666667,
        },
        Aura {
            name: "Ruby".to_string(),
            probability: 0.0028571429,
        },
        Aura {
            name: "Forbidden".to_string(),
            probability: 0.0024691358,
        },
        Aura {
            name: "Emerald".to_string(),
            probability: 0.002,
        },
        Aura {
            name: "Gilded".to_string(),
            probability: 0.0019531250,
        },
        Aura {
            name: "Ink".to_string(),
            probability: 0.0014285714,
        },
        Aura {
            name: "Jackpot".to_string(),
            probability: 0.0012820513,
        },
        Aura {
            name: "Sapphire".to_string(),
            probability: 0.0011111111,
        },
        Aura {
            name: "Aquamarine".to_string(),
            probability: 0.0011111111,
        },
        Aura {
            name: "Wind".to_string(),
            probability: 0.0011111111,
        },
        Aura {
            name: "Diaboli".to_string(),
            probability: 0.0009950249,
        },
        Aura {
            name: "Precious".to_string(),
            probability: 0.0009765625,
        },
        Aura {
            name: "Glock".to_string(),
            probability: 0.0005882353,
        },
        Aura {
            name: "Magnetic".to_string(),
            probability: 0.0004882813,
        },
        Aura {
            name: "Glacier".to_string(),
            probability: 0.0004329004,
        },
        Aura {
            name: "Siderum".to_string(),
            probability: 0.0002441406,
        },
        Aura {
            name: "Bleeding".to_string(),
            probability: 0.0002252252,
        },
        Aura {
            name: "Solar".to_string(),
            probability: 0.0000200000,
        },
        Aura {
            name: "Lunar".to_string(),
            probability: 0.0000200000,
        },
        Aura {
            name: "STARLIGHT".to_string(),
            probability: 0.0000200000,
        },
        Aura {
            name: ":Flushed:".to_string(),
            probability: 0.0001449275,
        },
        Aura {
            name: "HAZARD".to_string(),
            probability: 0.0001428571,
        },
        Aura {
            name: "Quartz".to_string(),
            probability: 0.0001220703,
        },
        Aura {
            name: "Undead".to_string(),
            probability: 0.0000833333,
        },
        Aura {
            name: "Corrosive".to_string(),
            probability: 0.0000833333,
        },
        Aura {
            name: "Rage : Heated".to_string(),
            probability: 0.0000781250,
        },
        Aura {
            name: "L E A K".to_string(),
            probability: 0.0000714286,
        },
        Aura {
            name: "Powered".to_string(),
            probability: 0.0000610352,
        },
        Aura {
            name: "Aquatic".to_string(),
            probability: 0.0000250000,
        },
        Aura {
            name: "Flushed : Lobotomy".to_string(),
            probability: 0.0000144928,
        },
        Aura {
            name: "HZARD : RAYS".to_string(),
            probability: 0.0000142857,
        },
        Aura {
            name: "Nautilus".to_string(),
            probability: 0.0000142857,
        },
        Aura {
            name: "Permaforst".to_string(),
            probability: 0.0000136054,
        },
        Aura {
            name: "Starmal".to_string(),
            probability: 0.0000111111,
        },
        Aura {
            name: "Exotic".to_string(),
            probability: 0.0000100010,
        },
        Aura {
            name: "Diaboli : Void".to_string(),
            probability: 0.0000099502,
        },
        Aura {
            name: "Undead : Devil".to_string(),
            probability: 0.0000083333,
        },
        Aura {
            name: "Comet".to_string(),
            probability: 0.0000083333,
        },
        Aura {
            name: "Jade".to_string(),
            probability: 0.0000080000,
        },
        Aura {
            name: "BOUNDED".to_string(),
            probability: 0.0000050000,
        },
        Aura {
            name: "Celestial".to_string(),
            probability: 0.0000028571,
        },
        Aura {
            name: "Kyawthuite".to_string(),
            probability: 0.0000011765,
        },
        Aura {
            name: "Arcane".to_string(),
            probability: 0.0000010000,
        },
        Aura {
            name: "Magnetic : Reverse Polarity".to_string(),
            probability: 0.0000009766,
        },
        Aura {
            name: "Undefined".to_string(),
            probability: 0.0000009009,
        },
        Aura {
            name: "Astral".to_string(),
            probability: 0.0000007500,
        },
        Aura {
            name: "Gravitational".to_string(),
            probability: 0.0000005000,
        },
        Aura {
            name: "UNBOUND".to_string(),
            probability: 0.0000005000,
        },
        Aura {
            name: "Virtual".to_string(),
            probability: 0.0000040000,
        },
        Aura {
            name: "AQUATIC . FLAME".to_string(),
            probability: 0.0000010000,
        },
        Aura {
            name: "Poseidon".to_string(),
            probability: 0.0000002500,
        },
        Aura {
            name: "Zeus".to_string(),
            probability: 0.0000002222,
        },
        Aura {
            name: "Solar : Solstice".to_string(),
            probability: 0.0000200000,
        },
        Aura {
            name: "Galaxy".to_string(),
            probability: 0.0000002000,
        },
        Aura {
            name: "Lunar : Full Moon".to_string(),
            probability: 0.0000002000,
        },
        Aura {
            name: "Twilight".to_string(),
            probability: 0.0000001667,
        },
        Aura {
            name: "Haded".to_string(),
            probability: 0.0000001500,
        },
        Aura {
            name: "Hyper-Volt".to_string(),
            probability: 0.0000001333,
        },
        Aura {
            name: "STARSCOURGE".to_string(),
            probability: 0.0000001000,
        },
        Aura {
            name: "Sailor".to_string(),
            probability: 0.0000000833,
        },
        Aura {
            name: "Glitch".to_string(),
            probability: 0.0000000820,
        },
        Aura {
            name: "Arcane : Leagacy".to_string(),
            probability: 0.0000000667,
        },
        Aura {
            name: "CHROMATIC".to_string(),
            probability: 0.0000000500,
        },
        Aura {
            name: "Arcane : Dark".to_string(),
            probability: 0.0000000333,
        },
        Aura {
            name: "Etheral".to_string(),
            probability: 0.0000000286,
        },
        Aura {
            name: "Exotic . APEX".to_string(),
            probability: 0.0000000200,
        },
        Aura {
            name: "Matrix".to_string(),
            probability: 0.0000002000,
        },
    ]
});

pub fn roll_aura(current_luck: f64) -> &'static Aura {
    let mut random_value: f64 = rand::thread_rng().gen_range(0.0..1.0) / current_luck;
    let mut cumulative_probability: f64 = 0.0;

    for aura_info in AURAS.iter() {
        cumulative_probability += aura_info.probability;
        if random_value <= cumulative_probability {
            return aura_info;
        }
    }

    unreachable!()
}
