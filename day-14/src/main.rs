// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
use serde_json::{json, Value};
use std::cmp::Ordering;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

#[derive(PartialEq)]
enum Material {
    SAND,
    VOID,
    ROCK,
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        let map: &mut Vec<Vec<Material>> = &mut vec![];

        let mut bottom = 0;

        for _ in 0..2000 {
            let mut row = vec![];
            for _ in 0..1000 {
                row.push(Material::VOID)
            }
            map.push(row);
        }
        for line in input.into_iter() {
            let rock: Vec<(usize, usize)> = line
                .split(" -> ")
                .map(|c| c.split(",").map(|c| c.parse::<usize>().unwrap()).collect())
                .map(|cx: Vec<usize>| (cx[0], cx[1]))
                .collect();
            let mut iter = rock.into_iter();
            let mut current = iter.next().unwrap();
            loop {
                match iter.next() {
                    Some(next) => {
                        let xs = if current.0 > next.0 {
                            next.0..=current.0
                        } else {
                            current.0..=next.0
                        };
                        let ys = if current.1 > next.1 {
                            next.1..=current.1
                        } else {
                            current.1..=next.1
                        };
                        for x in xs {
                            for y in ys.clone() {
                                map[y][x] = Material::ROCK;
                                if y >= bottom {
                                    bottom = y
                                }
                            }
                        }
                        current = next;
                    }
                    None => {
                        break;
                    }
                }
            }
        }

        map[0][500] = Material::SAND;
        draw(map);
        let mut units_of_sand = 0;
        let mut current_sand: Option<(usize, usize)> = None;
        loop {
            if current_sand.is_none() {
                current_sand = Some((0, 500));
            }
            let sand = current_sand.unwrap();
            if (sand.0) >= bottom {
                break;
            }
            println!("{} {}", sand.0, sand.1);
            if map[sand.0 + 1][sand.1] == Material::VOID {
                current_sand = Some((sand.0 + 1, sand.1))
            } else if map[sand.0 + 1][sand.1 - 1] == Material::VOID {
                current_sand = Some((sand.0 + 1, sand.1 - 1))
            } else if map[sand.0 + 1][sand.1 + 1] == Material::VOID {
                current_sand = Some((sand.0 + 1, sand.1 + 1))
            } else {
                map[sand.0][sand.1] = Material::SAND;

                units_of_sand = units_of_sand + 1;
                current_sand = None;
            }
        }
        println!("Total sand cubes: {}", units_of_sand)
    }
}

fn draw(map: &Vec<Vec<Material>>) {
    for y in map.into_iter() {
        for x in y.into_iter() {
            match x {
                Material::ROCK => print!("#"),
                Material::VOID => print!("."),
                Material::SAND => print!("o"),
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
