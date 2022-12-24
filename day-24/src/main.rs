// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use std::collections::{hash_map, HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::io::{self, BufRead};
use std::path::Path;

type Position = (usize, usize);

#[derive(Clone, Debug)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::E => '>',
                Direction::W => '<',
                Direction::N => '^',
                Direction::S => 'v',
            }
        )
    }
}

#[derive(Clone, Debug)]
struct Blizzard {
    direction: Direction,
    position: Position,
}

impl Blizzard {
    fn update_position(&mut self, exit: &Position) {
        // if self.position.0 == 1
        //     || self.position.1 == 1
        //     || self.position.0 == exit.0
        //     || self.position.1 == exit.1 - 1
        // {
        match self.direction {
            Direction::E => {
                if self.position.0 == exit.0 {
                    self.position = (1, self.position.1)
                } else {
                    self.position = (self.position.0 + 1, self.position.1)
                }
            }
            Direction::W => {
                if self.position.0 == 1 {
                    self.position = (exit.0, self.position.1)
                } else {
                    self.position = (self.position.0 - 1, self.position.1)
                }
            }
            Direction::N => {
                if self.position.1 == 1 {
                    self.position = (self.position.0, exit.1 - 1)
                } else {
                    self.position = (self.position.0, self.position.1 - 1)
                }
            }
            Direction::S => {
                if self.position.1 == exit.1 - 1 {
                    self.position = (self.position.0, 1)
                } else {
                    self.position = (self.position.0, self.position.1 + 1)
                }
            }
        }
        // } else {
        //     match self.direction {
        //         Direction::E => self.position = (self.position.0 + 1, self.position.1),
        //         Direction::W => self.position = (self.position.0 - 1, self.position.1),
        //         Direction::N => self.position = (self.position.0, self.position.1 - 1),
        //         Direction::S => self.position = (self.position.0, self.position.1 + 1),
        //     }
        // }
    }
}

#[derive(Clone, Debug)]
struct State {
    blizzards: Vec<Blizzard>,
    group: Position,
    exit: Position,
    time: usize,
    history: Vec<State>,
}

impl State {
    fn dist(&self) -> usize {
        (self.exit.0 - self.group.0).pow(2) + (self.exit.1 - self.group.1).pow(2)
    }
}

fn main() {
    let mut disabled: HashSet<Position> = HashSet::new();
    let mut blizzards: Vec<Blizzard> = vec![];
    let mut group = (0, 0);
    let mut exit = (0, 0);
    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        let mut position_set = false;
        for row in input.into_iter().enumerate() {
            for col in row.1.chars().into_iter().enumerate() {
                match col.1 {
                    '.' => {
                        if !position_set {
                            group = (col.0, row.0)
                        }
                        exit = (col.0, row.0);
                        position_set = true;
                    }
                    '>' => blizzards.push(Blizzard {
                        direction: Direction::E,
                        position: (col.0, row.0),
                    }),
                    '<' => blizzards.push(Blizzard {
                        direction: Direction::W,
                        position: (col.0, row.0),
                    }),
                    '^' => blizzards.push(Blizzard {
                        direction: Direction::N,
                        position: (col.0, row.0),
                    }),
                    'v' => blizzards.push(Blizzard {
                        direction: Direction::S,
                        position: (col.0, row.0),
                    }),
                    '#' => {
                        disabled.insert((col.0, row.0));
                    }
                    _ => unimplemented!(),
                }
            }
        }
    }

    let state = State {
        blizzards,
        exit,
        group,
        time: 0,
        history: vec![],
    };

    draw(&state, &disabled);

    eval(state, &disabled)
}

fn eval(start: State, disabled: &HashSet<Position>) {
    let mut deq = VecDeque::new();
    let mut best_time = usize::MAX;
    let mut dist_to_exit = usize::MAX;
    let mut best = start.clone();
    let mut unseen: HashSet<_> = HashSet::new();

    deq.push_back(start);
    while let Some(mut state) = deq.pop_front() {
        let mut occupied = HashSet::new();

        state.time += 1;

        let key = (state.time, state.group);
        if unseen.contains(&key) {
            continue;
        }
        unseen.insert(key);
        if state.exit == state.group {
            best_time = best_time.min(state.time);
            best = state.clone();
            println!(
                "Deq {}, Position: {:?}, Best: {}",
                deq.len(),
                state.group,
                best_time
            );
            continue;
        }
        if state.time > best_time {
            continue;
        }

        for b in state.blizzards.iter_mut() {
            b.update_position(&state.exit);
            occupied.insert(b.position);
        }

        let mut new_positions = vec![];

        // new_positions.push((state.group.0 + 1, state.group.1 + 1));
        new_positions.push((state.group.0 + 1, state.group.1));
        new_positions.push((state.group.0, state.group.1 + 1));

        if state.group.0 > 0 {
            new_positions.push((state.group.0 - 1, state.group.1));
            // new_positions.push((state.group.0 - 1, state.group.1 + 1));
        }
        if state.group.1 > 0 {
            new_positions.push((state.group.0, state.group.1 - 1));
            // new_positions.push((state.group.0 + 1, state.group.1 - 1));
        }

        new_positions.push((state.group.0, state.group.1));
        // if state.group.0 > 0 && state.group.1 > 0 {
        //     new_positions.push((state.group.0 - 1, state.group.1 - 1));
        // }

        new_positions
            .iter()
            .filter(|p| !disabled.contains(p) && !occupied.contains(p))
            .for_each(|next_group_position| {
                let mut next_state = state.clone();
                next_state.group = *next_group_position;
                // draw(&next_state, disabled);
                // next_state.history.push(state.clone());
                // if next_state.dist() <= dist_to_exit {
                //     dist_to_exit = dist_to_exit.min(next_state.dist());
                deq.push_back(next_state);
                // } else {
                //     deq.push_back(next_state);
                // }
            });
    }

    println!("Best : {}", best_time - 1);
    // println!("{:?}", best);

    for s in best.history.iter() {
        println!("Minute: {}", s.time + 1);
        draw(s, disabled)
    }
    draw(&best, disabled)
}

fn draw(state: &State, disabled: &HashSet<Position>) {
    for y in 0..=state.exit.1 {
        for x in 0..=state.exit.0 + 1 {
            if state.group == (x, y) {
                print!("G");
                continue;
            }
            if state.exit == (x, y) {
                print!("X");
                continue;
            }
            if disabled.contains(&(x, y)) {
                print!("#");
                continue;
            }
            let blizzards_at_position: Vec<&Blizzard> = state
                .blizzards
                .iter()
                .filter(|b| b.position == (x, y))
                .collect();
            match blizzards_at_position.len() {
                0 => print!("."),
                1 => print!("{}", blizzards_at_position.first().unwrap().direction),
                more => print!("{}", more),
            }
        }
        println!()
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
