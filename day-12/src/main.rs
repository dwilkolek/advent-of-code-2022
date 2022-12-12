#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::cmp::Ordering;
use std::collections::HashMap;
use std::f32::consts::E;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::vec;
use std::{thread, time};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let mut map: Vec<Vec<char>> = vec![];
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    let mut shortest_opts: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        for (row, line) in input.into_iter().enumerate() {
            let mut row_vec = Vec::new();
            for (col, char) in line.chars().into_iter().enumerate() {
                match char {
                    'S' => {
                        start = (row, col);
                        row_vec.push('a')
                    }
                    'E' => {
                        end = (row, col);
                        row_vec.push('z')
                    }
                    _ => row_vec.push(char),
                };
            }
            map.push(row_vec);
        }
    }

    println!("{:?}", map);

    match go(&map, vec![end], &start, &mut shortest_opts) {
        Some(route) => {
            println!("Steps: {}", route.len() - 1);
            draw_route(&map, &route);
        }
        None => {
            println!("Not found");
        }
    }
}

fn go(
    map: &Vec<Vec<char>>,
    mut steps: Vec<(usize, usize)>,
    end: &(usize, usize),
    shortest_opts: &mut HashMap<(usize, usize), Vec<(usize, usize)>>,
) -> Option<Vec<(usize, usize)>> {
    // draw_route(map, &steps);
    let all_directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    let last = steps.last().unwrap().clone();
    if end.0 == last.0 && end.1 == last.1 {
        return Some(steps);
    } else {
        let mut min_steps = usize::MAX;
        let mut new_steps: Option<Vec<(usize, usize)>> = None;
        let all_directions = vec![
            Direction::Right,
            Direction::Up,
            Direction::Down,
            Direction::Left,
        ];
        let mut options: Vec<_> = all_directions
            .into_iter()
            .map(|dir| next(map, &last, &dir))
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .collect();
        options.sort_by(|a, b| a.1.cmp(&b.1));
        for opt in options {
            let next_pos = opt.0;
            if !steps.contains(&next_pos) {
                let proceed = match shortest_opts.get(&next_pos) {
                    Some(step_count) => step_count.len() > (steps.len() + 1),
                    None => true,
                };
                if proceed {
                    let mut tmp_steps = steps.clone();
                    tmp_steps.push(next_pos);
                    shortest_opts.insert(next_pos, tmp_steps.clone());
                    match go(map, tmp_steps, end, shortest_opts) {
                        Some(steps) => {
                            if steps.len() < min_steps {
                                min_steps = steps.len();
                                new_steps = Some(steps);
                            }
                        }
                        None => {}
                    }
                }
            }
        }
        return new_steps;
    }
}

fn found_target(map: &Vec<Vec<char>>, position: &(usize, usize)) -> bool {
    map[position.0][position.1] == 'E'
}

fn is_ok(
    map: &Vec<Vec<char>>,
    a: &(usize, usize),
    b: (usize, usize),
) -> Option<((usize, usize), isize)> {
    let source = map[a.0][a.1] as isize;
    let dest = map[b.0][b.1] as isize;

    if dest - source >= -1 {
        Some((b, (dest - source)))
    } else {
        None
    }
}

fn draw_route(map: &Vec<Vec<char>>, steps: &Vec<(usize, usize)>) {
    io::stdout().flush().unwrap();
    thread::sleep(time::Duration::from_millis(50));
    for (row_index, row) in map.into_iter().enumerate() {
        for (col_index, char) in row.into_iter().enumerate() {
            if steps.contains(&(row_index, col_index)) {
                print!("{}", "*")
            } else {
                print!("{}", char)
            }
        }
        println!()
    }
}

fn next(
    map: &Vec<Vec<char>>,
    position: &(usize, usize),
    dir: &Direction,
) -> Option<((usize, usize), isize)> {
    let map_rows = map.len();
    let map_cols = map[0].len();
    match dir {
        Direction::Up => {
            if position.0 == 0 {
                None
            } else {
                is_ok(map, position, (position.0 - 1, position.1))
            }
        }
        Direction::Down => {
            if position.0 == map_rows - 1 {
                None
            } else {
                is_ok(map, position, (position.0 + 1, position.1))
            }
        }
        Direction::Left => {
            if position.1 == 0 {
                None
            } else {
                is_ok(map, position, (position.0, position.1 - 1))
            }
        }
        Direction::Right => {
            if position.1 == map_cols - 1 {
                None
            } else {
                is_ok(map, position, (position.0, position.1 + 1))
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
