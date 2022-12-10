#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::path::Path;

const DEBUG: bool = false;

#[derive(Clone, Debug, PartialEq, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn from_input_string(input: String) -> Vec<Direction> {
        let cmd_parts: Vec<&str> = input.split(" ").collect();

        let steps: usize = cmd_parts[1].clone().parse().unwrap();
        let dir = match cmd_parts[0] {
            "U" => Direction::UP,
            "D" => Direction::DOWN,
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            _ => panic!("What the hell"),
        };
        return (0..steps).map(|_| dir.clone()).collect();
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn dist(&self, other: &Point) -> f64 {
        let inx: f64 = ((&self.x - other.x).pow(2) + (&self.y - other.y).pow(2)) as f64;
        return inx.sqrt();
    }
}

fn main() {
    let move_limit = usize::MAX;
    let mut moves: Vec<Direction> = vec![];
    let rope_length = 9;

    let spawn = Point::new(11, 15);

    let mut rope: Vec<Point> = (0..rope_length + 1).map(|_| spawn.clone()).collect();
    let mut unique_tail_positions: HashSet<Point> = HashSet::new();

    draw(&rope, &spawn);

    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        for cmd in input.into_iter() {
            let mut steps = Direction::from_input_string(cmd);
            moves.append(&mut steps)
        }
    }

    println!("Moves: {:?}", moves.len());

    for (move_index, d) in moves.into_iter().enumerate() {
        if move_index > move_limit {
            break;
        }

        let mut new_rope = rope.clone();
        new_rope[0] = next_head_position(&d, &rope[0].clone());

        recalculate(&mut new_rope);

        unique_tail_positions.insert(new_rope.last().unwrap().clone());

        rope = new_rope;
        draw(&rope, &spawn);
    }

    println!("Tail visited: {} spots", unique_tail_positions.len());
}

fn next_head_position(d: &Direction, p: &Point) -> Point {
    match d {
        Direction::UP => Point::new(p.x, p.y - 1),
        Direction::DOWN => Point::new(p.x, p.y + 1),
        Direction::LEFT => Point::new(p.x - 1, p.y),
        Direction::RIGHT => Point::new(p.x + 1, p.y),
    }
}

fn recalculate(rope: &mut Vec<Point>) {
    let mut prev = rope[0].clone();

    for i in 1..rope.len() {
        let dist = rope[i].clone().dist(&prev);
        if DEBUG {
            println!(
                "Calculatorated dist: {} between {:?} & {:?}",
                dist, prev, rope[i]
            );
        }
        if dist >= 2.0 {
            let knot = rope[i].clone();
            let diff_x = match prev.x.cmp(&knot.x) {
                Ordering::Less => -1,
                Ordering::Greater => 1,
                Ordering::Equal => 0,
            };
            let diff_y = match prev.y.cmp(&knot.y) {
                Ordering::Less => -1,
                Ordering::Greater => 1,
                Ordering::Equal => 0,
            };
            let recalculated = Point::new(knot.x + diff_x, knot.y + diff_y);
            if DEBUG {
                println!(
                    "FIXED Calculatorated dist: {} between {:?} & {:?}",
                    recalculated.dist(&prev),
                    prev,
                    recalculated
                );
            }
            rope[i] = recalculated;
        }
        prev = rope[i].clone()
    }
}

fn draw(rope: &Vec<Point>, spawn: &Point) {
    if DEBUG {
        for y in 0..22 {
            for x in 0..28 {
                let mut printed = false;
                for (i, knot) in rope.into_iter().enumerate() {
                    if knot.x == x && knot.y == y {
                        if i == 0 && !printed {
                            print!("H");
                            printed = true;
                        }
                        if !printed {
                            print!("{}", i);
                            printed = true;
                        }
                        if i == rope.len() - 1 && !printed {
                            print!("T");
                            printed = true;
                        }
                    }
                }
                if !printed {
                    if spawn.x == x && spawn.y == y {
                        print!("s")
                    } else {
                        print!(".")
                    }
                }
            }
            println!("");
        }

        println!("");
        println!("");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
