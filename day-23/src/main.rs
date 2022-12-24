// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use std::collections::{hash_map, HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn from_position(&self, p: &(i32, i32)) -> Vec<(i32, i32)> {
        match self {
            Direction::N => vec![(p.0 - 1, p.1 - 1), (p.0, p.1 - 1), (p.0 + 1, p.1 - 1)],
            Direction::S => vec![(p.0 - 1, p.1 + 1), (p.0, p.1 + 1), (p.0 + 1, p.1 + 1)],
            Direction::W => vec![(p.0 - 1, p.1 - 1), (p.0 - 1, p.1), (p.0 - 1, p.1 + 1)],
            Direction::E => vec![(p.0 + 1, p.1 - 1), (p.0 + 1, p.1), (p.0 + 1, p.1 + 1)],
        }
    }
    fn into_direction(&self, p: &(i32, i32)) -> (i32, i32) {
        match self {
            Direction::N => (p.0, p.1 - 1),
            Direction::S => (p.0, p.1 + 1),
            Direction::W => (p.0 - 1, p.1),
            Direction::E => (p.0 + 1, p.1),
        }
    }
}

#[derive(Debug)]
struct Elf {
    position: (i32, i32),
    directions: VecDeque<Direction>,
    decision: Option<(i32, i32)>,
}

impl Elf {
    fn new(x: i32, y: i32) -> Elf {
        let mut directions = VecDeque::new();
        directions.push_back(Direction::N);
        directions.push_back(Direction::S);
        directions.push_back(Direction::W);
        directions.push_back(Direction::E);

        Elf {
            position: (x, y),
            directions,
            decision: None,
        }
    }

    fn decide(
        &mut self,
        occupied_positions: &HashSet<(i32, i32)>,
        proposed_positions: &mut HashMap<(i32, i32), i32>,
    ) {
        let mut surrounding = vec![];
        let mut new_directions = self.directions.clone();
        let xpe = new_directions.pop_front().unwrap();
        new_directions.push_back(xpe.clone());

        for dir in self.directions.iter() {
            for p in dir.from_position(&self.position) {
                surrounding.push(p)
            }
        }
        let is_alone = surrounding
            .iter()
            .filter(|p| occupied_positions.contains(p))
            .count()
            == 0;
        if !is_alone {
            for (dir_i, dir) in self.directions.iter().enumerate() {
                let looked_at_positions = dir.from_position(&self.position);
                let is_free = looked_at_positions
                    .iter()
                    .filter(|p| occupied_positions.contains(p))
                    .count();

                if is_free == 0 {
                    // println!("{:?} selected {:?}", self.position, dir);
                    let next_position = dir.into_direction(&self.position);
                    self.decision = Some(next_position);
                    match proposed_positions.get(&next_position) {
                        Some(c) => {
                            proposed_positions.insert(next_position, c + 1);
                        }
                        None => {
                            proposed_positions.insert(next_position, 1);
                        }
                    }
                    break;
                }
            }
        }

        self.directions = new_directions;
    }

    fn move_if_possible(&mut self, proposed_positions: &HashMap<(i32, i32), i32>) -> bool {
        let mut moved = false;
        if let Some(next_position) = self.decision {
            match proposed_positions.get(&next_position) {
                Some(c) => {
                    if *c == 1 {
                        self.position = next_position;
                        moved = true;
                    }
                }
                None => {
                    unreachable!()
                }
            }
        }
        self.decision = None;
        moved
    }
}

fn main() {
    let mut elves = vec![];
    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();

        for row in input.into_iter().enumerate() {
            for col in row.1.chars().into_iter().enumerate() {
                if col.1 == '#' {
                    elves.push(Elf::new(col.0 as i32, row.0 as i32))
                }
            }
        }
    }
    count_free_spaces(&elves);
    let mut lc = 0;
    loop {
        let mut occupied_positions = HashSet::new();
        elves.iter().for_each(|e| {
            occupied_positions.insert(e.position);
        });

        let mut proposed_positions: HashMap<(i32, i32), i32> = HashMap::new();
        for elf in elves.iter_mut() {
            elf.decide(&occupied_positions, &mut proposed_positions);
        }
        let mut moved = false;
        for elf in elves.iter_mut() {
            if elf.move_if_possible(&proposed_positions) {
                moved = true;
            }
        }
        // count_free_spaces(&elves);
        lc += 1;
        if moved == false {
            println!("LC :{}", lc);
            break;
        }
    }
}

fn count_free_spaces(elves: &Vec<Elf>) {
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_min = i32::MAX;
    let mut y_max = i32::MIN;

    let mut known_positions = HashSet::new();

    for elf in elves.iter() {
        x_min = x_min.min(elf.position.0);
        x_max = x_max.max(elf.position.0);
        y_min = y_min.min(elf.position.1);
        y_max = y_max.max(elf.position.1);
        known_positions.insert(elf.position);
    }

    let mut area = 0;
    for y in y_min..=y_max {
        // print!("{:4} ", y);
        for x in x_min..=x_max {
            if known_positions.get(&(x, y)).is_none() {
                // print!(".");
                area += 1
            } else {
                // print!("#")
            }
        }
        // println!()
    }

    println!("Area: {}", area);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
