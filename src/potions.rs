use std::time::{Duration, Instant};

pub struct Potion {
    pub name: &'static str,
    pub luck_boost: f64,
    pub duration: Duration,
}

pub const LUCKY_POTION: Potion = Potion {
    name: "Lucky Potion",
    luck_boost: 0.1,
    duration: Duration::from_secs(60),
};

pub const FORTUNE_I_POTION: Potion = Potion {
    name: "Fortune I Potion",
    luck_boost: 0.2,
    duration: Duration::from_secs(60),
};

// Add the remaining potion types here

pub fn apply_potion_effect(potion: &Potion, current_luck: &mut f64, potion_expiry: &mut Option<Instant>) {
    *current_luck += potion.luck_boost;
    *potion_expiry = Some(Instant::now() + potion.duration);
    println!("Applied {} effect. Luck increased by {}%.", potion.name, potion.luck_boost * 100.0);
}
