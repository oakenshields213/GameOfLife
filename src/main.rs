use rand::Rng;
use std::{thread, time};
use std::env;
use std::process::exit;

fn main() {
    // Parse Parameters from Command Line
    let args: Vec<_> = env::args().collect();
    if args.len() != 7 {
        println!("Parameters <Width> <Height> <Initial-Creatures> <1=wrap,0=nowrap> <iterations> <ms-pause>");
        println!("Example: 10 10 20 1 100 1000");
        exit(0)
    }
    let width = args[1].parse::<usize>().unwrap();
    let height = args[2].parse::<usize>().unwrap();
    let creatures = args[3].parse::<usize>().unwrap();
    let wrap = match args[4].as_str() {
        "0" => { false }
        "1" => { true }
        _ => false
    };
    let iterations = args[5].parse::<usize>().unwrap();
    let pause = args[6].parse::<u64>().unwrap();

    // Init Game Map
    let mut game_map = GameMap::new(width, height, wrap);

    // Insert Creatures
    game_map.random_insert(creatures);

    // Compute Iterations
    for i in 0..iterations {
        game_map.compute_next_iteration();
        game_map.print_console(true);
        thread::sleep(time::Duration::from_millis(pause));
    }
}

struct GameMap {
    width: usize,
    height: usize,
    matrix: Vec<Vec<bool>>,
    wrap_border: bool
}

impl GameMap {

    fn new(width: usize, height: usize, wrap_border: bool) -> GameMap {
        // Initialize empty matrix
        let mut matrix = Vec::new();
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                row.push(false);
            }
            matrix.push(row);
        }
        GameMap { width, height , matrix, wrap_border}
    }

    /**
     * Clears the GameMap by setting all positions to false
     */
    fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.matrix[y][x] = false;
            }
        }
    }

    /**
     * Randomly insert count creatures
     */
    fn random_insert(&mut self, count: usize ) {
        let effective_count = if count > self.width * self.height {
            count
        }
        else {
            self.width * self.height
        };
        let mut rng = rand::thread_rng();
        for i in 0..count {
            loop {
                let y = rng.gen_range(0..self.height);
                let x = rng.gen_range(0..self.height);
                if !self.matrix[y][x] {
                    self.matrix[y][x] = true;
                    break;
                }
            }
        }
    }

    /**
     * Print Game Map to console
     */
    fn print_console(&self, clear_console: bool) {
        if clear_console {
            print!("{}[2J", 27 as char);
        }
        for y in 0..self.height {
            for x in 0..self.width {
                if (self.matrix[y][x]) {
                    print!("#");
                }
                else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    /**
     * Compute next iteration
     */
    fn compute_next_iteration(& mut self) {
        let mut next_matrix = Vec::new();
        for y in 0..self.height {
            let mut row = Vec::new();
            for x in 0..self.width {
                row.push(false);
            }
            next_matrix.push(row);
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let count = self.get_neighbour_count(x, y);
                if self.matrix[y][x] {
                    if count < 2 || count > 3 {
                        next_matrix[y][x] = false
                    }
                    else if count == 2 || count == 3 {
                        next_matrix[y][x] = true
                    }
                }
                else if count == 3 {
                    next_matrix[y][x] = true
                }
            }
        }
        self.matrix = next_matrix
    }

    /**
     * Get number of active neighbours at the given position
     */
    fn get_neighbour_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for direction in 0..8 {
            if self.get_neighbour_active(x, y, direction) {
                count = count + 1
            }
        }
        count
    }

    /**
     * Returns true if the specified neighbour is active
     * direction: 0 = top, 1 = top-right, 2 = right, 3 = bottom-right,
     *            4 = bottom, 5 = bottom-left, 6 = left, 7 = top-left
     */
    fn get_neighbour_active(&self, x: usize, y: usize, direction: usize) -> bool {
        let mut eff_y = y;
        if direction == 0 || direction == 1 || direction == 7 {
            if y == 0 {
                eff_y = self.height - 1
            }
            else {
                eff_y = eff_y - 1
            }
        }
        if direction == 3 || direction == 4 || direction == 5 {
            if y == self.height - 1 {
                eff_y = 0
            }
            else {
                eff_y = eff_y + 1
            }
        }
        let mut eff_x = x;
        if direction == 5 || direction == 6 || direction == 7 {
            if x == 0 {
                eff_x = self.width - 1
            }
            else {
                eff_x = eff_x - 1
            }
        }
        if direction == 1 || direction == 2 || direction == 3 {
            if x == self.width - 1 {
                eff_x = 0
            }
            else {
                eff_x = eff_x + 1
            }
        }
        if self.wrap_border {
            self.matrix[eff_y][eff_x]
        }
        else {
            match direction {
                0 => {
                    if y == 0 {
                        false
                    }
                    else {
                        self.matrix[eff_y][eff_x]
                    }
                }
                1 => {
                    if y == 0 || x == self.width - 1 {
                        false
                    }
                    else {
                        self.matrix[eff_y][eff_x]
                    }
                }
                2 => {
                    if x == self.width - 1 {
                        false
                    }
                    else {
                        self.matrix[eff_y][eff_x]
                    }
                }
                3 => {
                    if y == self.height - 1 || x == self.width - 1 {
                        false
                    }
                    else {
                        self.matrix[eff_y][eff_x]
                    }
                }
                4 => {
                    if y == self.height - 1 {
                        false
                    } else {
                        self.matrix[eff_y][eff_x]
                    }
                }
                5 => {
                    if y == self.height - 1 || x == 0 {
                        false
                    } else {
                        self.matrix[eff_y][eff_x]
                    }
                }
                6 => {
                    if x == 0 {
                        false
                    } else {
                        self.matrix[eff_y][eff_x]
                    }
                }
                7 => {
                    if y == 0 || x == 0 {
                        false
                    } else {
                        self.matrix[eff_y][eff_x]
                    }
                }
                _ => { false }
            }
        }
    }

}