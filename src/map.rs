use rand::Rng;
use std::collections::HashSet;
use std::io::{stdout, Write};

const MAP_WIDTH: usize = 16;
const MAP_HEIGHT: usize = 16;

pub struct Map {
    pub tiles: Vec<Vec<char>>,
    pub player_x: isize,
    pub player_y: isize,
    pub potions: HashSet<(isize, isize)>,
}

impl Map {
    pub fn new() -> Map {
        let mut tiles = vec![vec!['.'; MAP_WIDTH]; MAP_HEIGHT];
        let mut rng = rand::thread_rng();

        let player_x = MAP_WIDTH as isize / 2;
        let player_y = MAP_HEIGHT as isize / 2;
        tiles[player_y as usize][player_x as usize] = '@';

        let jake_x = (MAP_WIDTH / 4) as isize;
        let jake_y = (MAP_HEIGHT / 4) as isize;
        tiles[jake_y as usize][jake_x as usize] = 'J';

        let stella_x = (MAP_WIDTH / 4 * 3) as isize;
        let stella_y = (MAP_HEIGHT / 4 * 3) as isize;
        tiles[stella_y as usize][stella_x as usize] = 'S';

        let mut potions = HashSet::new();
        for _ in 0..3 {
            let x = rng.gen_range(0..MAP_WIDTH as isize);
            let y = rng.gen_range(0..MAP_HEIGHT as isize);
            if (x, y) != (player_x, player_y) && (x, y) != (jake_x, jake_y) && (x, y) != (stella_x, stella_y) {
                tiles[y as usize][x as usize] = 'P';
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
        stdout().flush().unwrap();
    }

    pub fn move_player(&mut self, dx: isize, dy: isize) {
        let new_x = self.player_x + dx;
        let new_y = self.player_y + dy;

        // Update the player's position
        self.tiles[self.player_y as usize][self.player_x as usize] = '.';
        self.player_x = new_x;
        self.player_y = new_y;

        // Expand the map if the new position is outside the current bounds
        if new_x < 0 || new_x >= MAP_WIDTH as isize || new_y < 0 || new_y >= MAP_HEIGHT as isize {
            self.expand_map(new_x, new_y);
        }

        // Update the player's position on the map
        self.tiles[self.player_y as usize][self.player_x as usize] = '@';

        if self.potions.contains(&(self.player_x, self.player_y)) {
            self.potions.remove(&(self.player_x, self.player_y));
            println!("You found a potion!");
            // Add code to apply the potion effect here
        }
    }

    fn expand_map(&mut self, new_x: isize, new_y: isize) {
        let mut new_tiles = vec![vec!['.'; MAP_WIDTH * 2]; MAP_HEIGHT * 2];

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                new_tiles[y + MAP_HEIGHT / 2][x + MAP_WIDTH / 2] = self.tiles[y][x];
            }
        }

        self.tiles = new_tiles;
        self.player_x = new_x + MAP_WIDTH as isize;
        self.player_y = new_y + MAP_HEIGHT as isize;
    }

    pub fn clear_screen(&self) {
        print!("{}[2J", 27 as char);
        self.render_map();
    }
}
