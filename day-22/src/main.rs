// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::path::Path;

const DRAW: bool = false;

type Map = [[i32; 152]; 202];
const Z_SIZE: i32 = 50;

// type Map = [[i32; 22]; 18];
// const Z_SIZE: i32 = 4;

fn main() {
    let mut map: Map = [[8; 152]; 202];
    let mut commands: Vec<String> = vec![];

    if let Ok(lines) = read_lines("input.txt") {
        let input: VecDeque<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        let mut next_password = false;
        for row in input.into_iter().enumerate() {
            if row.1.is_empty() {
                next_password = true;
                continue;
            }
            if next_password {
                let path_building = row.1.replace("R", ";R;");
                let path_building = path_building.replace("L", ";L;");
                commands = path_building.split(";").map(|c| c.to_owned()).collect();
                break;
            }
            for ch in row.1.chars().into_iter().enumerate() {
                match ch.1 {
                    ' ' => map[row.0 + 1][ch.0 + 1] = 8,
                    '#' => map[row.0 + 1][ch.0 + 1] = 4,
                    '.' => map[row.0 + 1][ch.0 + 1] = 0,
                    _ => unimplemented!(),
                }
            }
        }
    }

    // draw(&map, starting_position(&map));
    println!("Starting position {:?}", starting_position(&map));
    println!("Path {:?}", commands);

    println!("Password: {}", lets_move(&map, &commands));
}

fn lets_move(map: &Map, commands: &Vec<String>) -> i32 {
    let mut position = starting_position(map);
    let mut direction: (i32, i32) = (0, 1);

    for command in commands.iter() {
        if let Ok(steps) = command.parse::<usize>() {
            for _ in 0..steps {
                let next_position = (position.0 + direction.0, position.1 + direction.1);
                match map[next_position.0 as usize][next_position.1 as usize] {
                    8 => {
                        // PART 1
                        // let mut tmp_position = position.clone();
                        // loop {
                        //     next_position =
                        //         (tmp_position.0 - direction.0, tmp_position.1 - direction.1);
                        //     if map[next_position.0 as usize][next_position.1 as usize] == 8 {
                        //         break;
                        //     }
                        //     tmp_position = next_position;
                        //     if next_position.0 == 0
                        //         || next_position.1 == 0
                        //         || next_position.0 as usize == map.len() - 1
                        //         || next_position.1 as usize == map[0].len() - 1
                        //     {
                        //         break;
                        //     }
                        // }
                        // if map[tmp_position.0 as usize][tmp_position.1 as usize] == 0 {
                        //     position = tmp_position;
                        // } else {
                        //     break;
                        // }

                        // PART 2 teleports
                        let source_plane = position_to_plane(&position);
                        let mut tmp_direction = direction.clone();
                        let base_position = ((position.0 - 1) % Z_SIZE, (position.1 - 1) % Z_SIZE);
                        let mut tmp_position;

                        match (source_plane, direction) {
                            (1, (-1, 0)) => {
                                // 6
                                tmp_position = (base_position.1 + (3 * Z_SIZE), 0);
                                tmp_direction = (0, 1);
                            }
                            (1, (0, -1)) => {
                                // 5
                                tmp_position = (flip(base_position.0) + (2 * Z_SIZE), 0);
                                tmp_direction = (0, 1);
                            }
                            (2, (1, 0)) => {
                                // 3
                                tmp_position = (base_position.1 + Z_SIZE, 2 * Z_SIZE - 1);
                                tmp_direction = (0, -1);
                            }
                            (2, (0, 1)) => {
                                // 4
                                tmp_position =
                                    (flip(base_position.0) + (2 * Z_SIZE), 2 * Z_SIZE - 1);
                                tmp_direction = (0, -1);
                            }
                            (2, (-1, 0)) => {
                                // 6
                                tmp_position = (4 * Z_SIZE - 1, base_position.1);
                            }
                            (3, (0, -1)) => {
                                // 5
                                tmp_position = (2 * Z_SIZE, base_position.0);
                                tmp_direction = (1, 0);
                            }
                            (3, (0, 1)) => {
                                // 2
                                tmp_position = (Z_SIZE - 1, base_position.0 + (2 * Z_SIZE));
                                tmp_direction = (-1, 0);
                            }
                            (4, (0, 1)) => {
                                // 2
                                tmp_position = (flip(base_position.0), (3 * Z_SIZE) - 1);
                                tmp_direction = (0, -1);
                            }
                            (4, (1, 0)) => {
                                // 6
                                tmp_position = (base_position.1 + (3 * Z_SIZE), Z_SIZE - 1);
                                tmp_direction = (0, -1);
                            }
                            (5, (-1, 0)) => {
                                // 3
                                tmp_position = (base_position.1 + Z_SIZE, Z_SIZE);
                                tmp_direction = (0, 1);
                            }
                            (5, (0, -1)) => {
                                // 1
                                tmp_position = (flip(base_position.0), Z_SIZE);
                                tmp_direction = (0, 1);
                            }
                            (6, (1, 0)) => {
                                // 2
                                tmp_position = (0, base_position.1 + (2 * Z_SIZE));
                            }

                            (6, (0, 1)) => {
                                // 4
                                tmp_position = (3 * Z_SIZE - 1, base_position.0 + Z_SIZE);
                                tmp_direction = (-1, 0);
                            }
                            (6, (0, -1)) => {
                                // 1
                                tmp_position = (0, base_position.0 + Z_SIZE);
                                tmp_direction = (1, 0);
                            }
                            other => unimplemented!("Unimplemented {:?}", other),
                        }
                        tmp_position = (tmp_position.0 + 1, tmp_position.1 + 1);

                        if map[tmp_position.0 as usize][tmp_position.1 as usize] == 0 {
                            position = tmp_position;
                            direction = tmp_direction;
                        } else {
                            break;
                        }
                    }
                    4 => {
                        break;
                    }
                    0 => {
                        position = next_position;
                    }
                    _ => unreachable!(),
                }
            }
        } else {
            match command.as_str() {
                "L" => direction = rotate_l(&direction),
                "R" => direction = rotate_r(&direction),
                other => println!("WTF {}", other),
            }
        }
    }
    let direction_to_points = match direction {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => unreachable!(),
    };
    1000 * (position.0) + 4 * (position.1) + direction_to_points
}

fn flip(p: i32) -> i32 {
    Z_SIZE - (p + 1)
}

fn position_to_plane(position: &(i32, i32)) -> i32 {
    if 2 * Z_SIZE <= position.1 - 1 {
        return 2;
    }

    if position.0 <= Z_SIZE {
        return 1;
    }
    if position.0 <= 2 * Z_SIZE {
        return 3;
    }
    if position.0 >= 3 * Z_SIZE + 1 {
        return 6;
    }
    if position.1 <= Z_SIZE {
        return 5;
    }

    return 4;
}

fn rotate_l(direction: &(i32, i32)) -> (i32, i32) {
    match direction {
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        _ => unreachable!(),
    }
}
fn rotate_r(direction: &(i32, i32)) -> (i32, i32) {
    match direction {
        (-1, 0) => (0, 1),
        (0, -1) => (-1, 0),
        (1, 0) => (0, -1),
        (0, 1) => (1, 0),
        _ => unreachable!(),
    }
}
fn starting_position(&map: &Map) -> (i32, i32) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 0 {
                return (row as i32, col as i32);
            }
        }
    }
    panic!()
}

fn draw(&map: &Map, position: &(i32, i32), direction: &(i32, i32)) {
    if !DRAW {
        return;
    }
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if row == position.0 as usize && col == position.1 as usize {
                match direction {
                    (-1, 0) => print!("^"),
                    (0, -1) => print!("<"),
                    (1, 0) => print!("v"),
                    (0, 1) => print!(">"),
                    _ => unreachable!(),
                }

                continue;
            }
            match map[row as usize][col as usize] {
                8 => print!(" "),
                4 => print!("#"),
                0 => print!("{}", position_to_plane(&(row as i32, col as i32))),
                _ => (),
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
