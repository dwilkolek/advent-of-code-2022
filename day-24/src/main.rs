// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::io::{self, BufRead};
use std::path::Path;

type Position = (usize, usize);

fn dist(a: &Position, b: &Position) -> usize {
    (a.0.abs_diff(b.0)).pow(2) + (a.1.abs_diff(b.1)).pow(2)
}

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
    }
}

#[derive(Clone, Debug)]
struct State {
    group: Position,
    exit: Position,
    time: usize,
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
        exit,
        group,
        time: 0,
    };

    let mut blizzards_at_time: HashMap<usize, HashSet<Position>> = HashMap::new();
    let mut i = 0;
    loop {
        let mut occupied: HashSet<Position> = HashSet::new();
        for b in blizzards.iter_mut() {
            b.update_position(&exit);
            occupied.insert(b.position);
        }
        i += 1;
        blizzards_at_time.insert(i, occupied);
        if i > 10000 {
            break;
        }
    }

    let mut at_exit = eval(state, &disabled, &blizzards_at_time);
    at_exit.exit = group;
    let mut at_start = eval(at_exit, &disabled, &blizzards_at_time);
    at_start.exit = exit;
    eval(at_start, &disabled, &blizzards_at_time);
}

fn eval(
    start: State,
    disabled: &HashSet<Position>,
    blizzards: &HashMap<usize, HashSet<Position>>,
) -> State {
    let mut deq = VecDeque::new();
    let mut best_time = usize::MAX;
    let mut best = start.clone();
    let mut unseen: HashSet<_> = HashSet::new();
    let time_limit = start.time + 500;
    deq.push_back(start);
    while let Some(mut state) = deq.pop_front() {
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
        if state.time > time_limit {
            continue;
        }
        state.time += 1;

        let mut new_positions = vec![];

        new_positions.push((state.group.0 + 1, state.group.1));
        new_positions.push((state.group.0, state.group.1 + 1));

        if state.group.0 > 0 {
            new_positions.push((state.group.0 - 1, state.group.1));
        }
        if state.group.1 > 0 {
            new_positions.push((state.group.0, state.group.1 - 1));
        }

        new_positions.push((state.group.0, state.group.1));

        new_positions.sort_by(|a, b| dist(b, &state.exit).cmp(&dist(a, &state.exit)));

        new_positions
            .iter()
            .filter(|p| !disabled.contains(p) && !blizzards.get(&state.time).unwrap().contains(p))
            .for_each(|next_group_position| {
                let mut next_state = state.clone();
                next_state.group = *next_group_position;
                deq.push_back(next_state);
            });
    }

    println!("Best : {}", best_time);
    return best;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
