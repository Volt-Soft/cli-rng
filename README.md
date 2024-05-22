# Cli Rng
## Sol's RNG, but Command Line !
I am rewriting whole Sol's RNG roblox game in Rust. Actually, I just implemented a Roll system, with probabilities.

What will be added :
1. More Auras
2. Potions
3. Gauntlets
4. Biomes, Weather and Time
5. A system to permanently store Auras based on the computer, maybe a TXT or JSON that has the auras, potions, gauntlets and effects saved.

## Actions
Currently, there are 4 actions implemented :
- roll - Roll a random aura
- storage - Display all rolled auras
- exit - Exit the program
- reset - reset storage

## Downloading
To download, you need to have [Cargo](https://doc.rust-lang.org/cargo/) on your machine :
You can Download it :
```bash
cargo install cli-rng
cli-rng
```
Or, compile fromscrath :
```bash
git clone https://github.com/FBDev64/cli-rng.git
cd cli-rng
cargo run -r
```
## Feedback
[A Survey to collect your feedbacks and bug reports.](https://forms.gle/2RPzt97PhyoSHfKb9)
## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[BSD 3-Clause “New” or “Revised” License](https://choosealicense.com/licenses/bsd-3-clause/)
