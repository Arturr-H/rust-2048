/*- Global allowings -*/
#![allow(
    dead_code,
    unused_variables
)]

/*- Imports -*/
use rand::Rng;
use std::fmt::Debug;
use std::io::{ stdin, stdout, Write };

/*- Constants -*/
const GRID_SIZE:(usize, usize) = (5, 5);

/*- Structs, enums & unions -*/
struct TwoDimBoard { cells: Vec<Vec<u16>> }
#[derive(Debug)]
struct Game { board: TwoDimBoard }
#[derive(Debug)]
enum Direction { Right, Left, Up, Down }
#[derive(Debug)]
enum MergeStatus { Blocked, Move((usize, usize)), Merge((usize, usize)) }

/*- Initialize -*/
fn main() -> () {
    /*- Create board -*/
    let mut game:Game = Game { board: TwoDimBoard::new(GRID_SIZE) };
    game.add_tile();
    game.add_tile();
    println!("{game:#?}");

    /*- Game loop -*/
    loop {
        let mut input:String = String::new();

        /*- Read input -*/
        stdin().read_line(&mut input).unwrap();

        /*- Print decoration -*/
        print!("==> ");
        stdout().flush().unwrap();

        /*- Get key -*/
        match input.as_str().trim() {
            "w" => game.board.merge(Direction::Up),
            "a" => game.board.merge(Direction::Left),
            "s" => game.board.merge(Direction::Down),
            "d" => game.board.merge(Direction::Right),
            _ => continue
        };
        game.add_tile();

        println!("{game:#?}");
        /*- Print decoration -*/
        print!("==> ");
        stdout().flush().unwrap();
    };
}

/*- Method implementations -*/
impl TwoDimBoard {
    /*- Constructor -*/
    pub fn new(size:(usize, usize)) -> Self {
        TwoDimBoard {
            cells: vec![
                vec![0u16; size.0]; size.1
            ]
        }
    }

    /*- Get mutable ref to tile -*/
    pub fn get_mut(&mut self, coord:(usize, usize)) -> Option<&mut u16> {
        match self.cells.get_mut(coord.1) {
            Some(row) => match row.get_mut(coord.0) {
                Some(tile) => Some(tile),
                None => None
            },
            None => None
        }
    }

    /*- Get regular reference to value -*/
    pub fn get(&self, coord:(usize, usize)) -> Option<u16> {
        match self.cells.get(coord.1) {
            Some(row) => match row.get(coord.0) {
                Some(tile) => Some(*tile),
                None => None
            },
            None => None
        }
    }

    /*- Check status between two tiles -*/
    fn cmp_status(&self, coord1:(usize, usize), coord2:(usize, usize)) -> MergeStatus {
        let curr = match self.get(coord1) { Some(e) => e, None => return MergeStatus::Blocked };
        let next = match self.get(coord2) { Some(e) => e, None => return MergeStatus::Blocked };

        if next == 0         { MergeStatus::Move(coord2) }
        else if curr == next { MergeStatus::Merge(coord2) }
        else                 { MergeStatus::Blocked }
    }
    /*- Compare tile relative to its' direction -*/
    fn directional_cmp_status(&self, direction:&Direction, coord:(usize, usize)) -> MergeStatus {
        match direction {
            Direction::Down => {
                let next_coord:(usize, usize) = (
                    coord.0, // X
                    coord.1 + 1 // y
                );

                /*- Check status -*/
                self.cmp_status(coord, next_coord)
            },
            Direction::Right => {
                let next_coord:(usize, usize) = (
                    coord.0 + 1, // X
                    coord.1 // Y
                );

                /*- Check status -*/
                self.cmp_status(coord, next_coord)
            },
            Direction::Up => {
                let next_coord:(usize, usize) = (
                    coord.0, // X
                    match coord.1.checked_sub(1) { // Y
                        Some(e) => e,
                        None => return MergeStatus::Blocked
                    }
                );

                /*- Check status -*/
                self.cmp_status(coord, next_coord)
            },
            Direction::Left => {
                let next_coord:(usize, usize) = (
                    match coord.0.checked_sub(1) { // X
                        Some(e) => e,
                        None => return MergeStatus::Blocked
                    },
                    coord.1 // Y
                );

                /*- Check status -*/
                self.cmp_status(coord, next_coord)
            },
        }
    }

    /*- Merge -*/
    fn merge(&mut self, direction:Direction) -> () {
        /*- Iterate -*/
        match direction {
            Direction::Down =>  { for x in (0..GRID_SIZE.1).rev() { for y in 0..GRID_SIZE.0 { self.merge_cell(&direction, (x, y)); }; }; },
            Direction::Left =>  { for x in (0..GRID_SIZE.1).rev() { for y in 0..GRID_SIZE.0 { self.merge_cell(&direction, (x, y)); }; }; },
            Direction::Right => { for x in 0..GRID_SIZE.1 { for y in 0..GRID_SIZE.0 { self.merge_cell(&direction, (x, y)); }; }; },
            Direction::Up =>    { for x in 0..GRID_SIZE.1 { for y in 0..GRID_SIZE.0 { self.merge_cell(&direction, (x, y)); }; }; },
        };

        self.compress(&direction);
    }
    fn merge_cell(&mut self, direction:&Direction, coord:(usize, usize)) -> () {
        let merge_status:MergeStatus = self.directional_cmp_status(&direction, coord);
        match merge_status {
            MergeStatus::Merge(to) => self.handle_merge(coord, to),
            MergeStatus::Move(to) => self.handle_move(coord, to),
            MergeStatus::Blocked => ()
        };
    }
    fn compress(&mut self, direction:&Direction) -> () {
        /*- Iterate -*/
        for _ in 0..5 {
            println!("iter {direction:?}");
            match direction {
                Direction::Down =>  { for x in (0..GRID_SIZE.1).rev() { for y in 0..GRID_SIZE.0 { self.compress_inner(x, y, direction); }; }; },
                Direction::Left =>  { for x in (0..GRID_SIZE.1).rev() { for y in 0..GRID_SIZE.0 { self.compress_inner(x, y, direction); }; }; },
                Direction::Right => { for x in 0..GRID_SIZE.1 { for y in 0..GRID_SIZE.0 { self.compress_inner(x, y, direction); }; }; },
                Direction::Up =>    { for y in 0..GRID_SIZE.1 { for x in 0..GRID_SIZE.0 { self.compress_inner(x, y, direction); }; }; },
            }
        }
    }
    fn compress_inner(&mut self, x:usize, y:usize, direction:&Direction) -> () {
        let status:MergeStatus = self.directional_cmp_status(&direction, (x, y));
        match status {
            MergeStatus::Merge(_) => (),
            MergeStatus::Move(to) => self.handle_move((x, y), to),
            MergeStatus::Blocked => (),
        };
    }

    /*- Handle moving tile from coordinate to other coordinate -*/
    fn handle_move(&mut self, curr:(usize, usize), to:(usize, usize)) -> () {
        match self.get_mut(curr) {
            Some(first_cell) => {
                let a = first_cell.clone();
                *first_cell = 0;

                /*- Get next tile and update -*/
                match self.get_mut(to) {
                    Some(second_cell) => {
                        *second_cell = a;
                    },
                    None => ()
                };
            },
            None => ()
        } 
    }
    /*- Handle merging tiles -*/
    fn handle_merge(&mut self, curr:(usize, usize), to:(usize, usize)) -> () {
        match self.get_mut(curr) {
            Some(first_cell) => {
                let a = first_cell.clone();
                *first_cell = 0;

                /*- Get next tile and update -*/
                match self.get_mut(to) {
                    Some(second_cell) => {
                        *second_cell = a + *second_cell;
                    },
                    None => ()
                };
            },
            None => ()
        } 
    }
}
impl Game {
    /*- Place tile at random spot -*/
    fn add_tile(&mut self) -> () {
        let mut random = rand::thread_rng();
        let mut empty_coords:Vec<(usize, usize)> = Vec::new();

        /*- Dims -*/
        let dimensions:(usize, usize) = (
            self.board.cells[1].len(),
            self.board.cells[0].len(),
        );

        /*- Iterate over tiles -*/
        for y in 0..dimensions.1 {
            for x in 0..dimensions.0 {
                let cell = self.board.get((x, y));
                
                /*- See if cell exists (it should 100% do) -*/
                match cell {
                    Some(value) => {
                        if value == 0 { empty_coords.push((x, y)) };
                    },
                    None => continue,
                };
            };
        };
        if empty_coords.len() == 0 { return; };

        /*- Grab random from all coords -*/
        let coord:(usize, usize) = empty_coords[random.gen_range(0..empty_coords.len())];

        /*- Change cell at place -*/
        match self.board.get_mut(coord) {
            Some(e) => {
                /*- If this is true, we'll spawn a 4 instead of a 2 -*/
                let spawn_4:bool = random.gen_bool(0.8);
                if spawn_4 { *e = 4 }
                else       { *e = 2 }
            },
            None => ()
        };
    }
}

/*- Debugging implementations -*/
const REPLACE:&'static str = "0123456789";
impl Debug for TwoDimBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string:String = String::new();
        for row in &self.cells {
            string.push_str(&format!("\n[ | ͞ ͞ ͞ ͞ ͞ ͞ ͞| | ͞ ͞ ͞ ͞ ͞ ͞ ͞| | ͞ ͞ ͞ ͞ ͞ ͞ ͞| | ͞ ͞ ͞ ͞ ͞ ͞ ͞| | ͞ ͞ ͞ ͞ ͞ ͞ ͞|  ]\n["));
            for tile in row {
                string.push_str(
                    &format!(
                        " |{:^7}|",
                        if tile == &0 { " ".to_string() } else { tile.to_string().chars().map(|e| REPLACE.chars().collect::<Vec<char>>()[e.to_string().parse::<usize>().unwrap()]).collect::<Vec<char>>().iter().collect::<String>() }
                    )
                );
            };
            string.push_str(&format!("  ]\n[ |       | |       | |       | |       | |       |  ]\n[   ͞ ͞ ͞ ͞ ͞ ͞ ͞     ͞ ͞ ͞ ͞ ͞ ͞     ͞ ͞ ͞ ͞ ͞ ͞     ͞ ͞ ͞ ͞ ͞ ͞     ͞ ͞ ͞ ͞ ͞ ͞   ]"));
        };
        write!(f, "{}", string)
    }
}