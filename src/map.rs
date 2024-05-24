use rand::Rng;
use std::collections::HashSet;

const MAP_WIDTH: usize = 5;
const MAP_HEIGHT: usize = 5;

pub struct Map {
    pub tiles: Vec<Vec<char>>,
    pub player_x: usize,
    pub player_y: usize,
    pub potions: HashSet<(usize, usize)>,
}

impl Map {
    pub fn new() -> Map {
        let mut tiles = vec![vec!['.'; MAP_WIDTH]; MAP_HEIGHT];
        let mut rng = rand::thread_rng();

        let player_x = rng.gen_range(0..MAP_WIDTH);
        let player_y = rng.gen_range(0..MAP_HEIGHT);
        tiles[player_y][player_x] = '#';

        let mut potions = HashSet::new();
        for _ in 0..3 {
            let x = rng.gen_range(0..MAP_WIDTH);
            let y = rng.gen_range(0..MAP_HEIGHT);
            if (x, y) != (player_x, player_y) {
                tiles[y][x] = 'P';
                potions.insert((x, y));
            }
        }

        Map {
            tiles,
            player_x,
            player_y,
            potions,
        }
    }

    pub fn render_map(&self) {
        for row in &self.tiles {
            for tile in row {
                print!("{}", tile);
            }
            println!();
        }
    }

    pub fn move_player(&mut self, dx: isize, dy: isize) {
        let new_x = self.player_x as isize + dx;
        let new_y = self.player_y as isize + dy;

        if new_x >= 0
            && new_x < MAP_WIDTH as isize
            && new_y >= 0
            && new_y < MAP_HEIGHT as isize
        {
            self.tiles[self.player_y][self.player_x] = '.';
            self.player_x = new_x as usize;
            self.player_y = new_y as usize;
            self.tiles[self.player_y][self.player_x] = '#';

            if self.potions.contains(&(self.player_x, self.player_y)) {
                self.potions.remove(&(self.player_x, self.player_y));
                println!("You found a potion!");
                // Add code to apply the potion effect here
            }
        } else {
            println!("Can't move out of bounds!");
        }
    }
}
