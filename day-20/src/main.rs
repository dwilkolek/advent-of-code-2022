// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use std::io::{self, BufRead};
use std::path::Path;
use std::vec;
#[derive(Debug, Clone, Copy)]
struct N {
    original_position: usize,
    value: i64,
}
fn main() {
    let mut numbers = vec![];
    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        for line in input.into_iter().enumerate() {
            numbers.push(N {
                original_position: line.0,
                value: line.1.parse::<i64>().unwrap(),
            })
        }
    }

    {
        let mut numbers = numbers.clone();
        let vec_len = numbers.len();
        for i in 0..vec_len {
            numbers = move_number(numbers, i);
        }

        let z_pos = numbers.iter().position(|n| n.value == 0).unwrap();
        println!(
            "Part 1: {}",
            [
                numbers[(z_pos + 1000) % vec_len].value,
                numbers[(z_pos + 2000) % vec_len].value,
                numbers[(z_pos + 3000) % vec_len].value
            ]
            .iter()
            .sum::<i64>()
        );
    }

    {
        let decription_key = 811_589_153;
        let mut numbers: Vec<N> = numbers
            .into_iter()
            .map(|n| N {
                value: n.value * decription_key,
                original_position: n.original_position,
            })
            .collect();
        let vec_len = numbers.len();
        for _ in 0..10 {
            for i in 0..vec_len {
                numbers = move_number(numbers, i);
            }
        }
        println!(
            "{:?}",
            numbers.iter().map(|n| n.value).collect::<Vec<i64>>()
        );
        let z_pos = numbers.iter().position(|n| n.value == 0).unwrap();
        println!(
            "Part 2: {:?}, {}",
            [
                numbers[(z_pos + 1000) % vec_len].value,
                numbers[(z_pos + 2000) % vec_len].value,
                numbers[(z_pos + 3000) % vec_len].value
            ],
            [
                numbers[(z_pos + 1000) % vec_len].value,
                numbers[(z_pos + 2000) % vec_len].value,
                numbers[(z_pos + 3000) % vec_len].value
            ]
            .iter()
            .sum::<i64>()
        );
    }
}

fn move_number(numbers: Vec<N>, orignal_position: usize) -> Vec<N> {
    let mut numbers = numbers.clone();

    let current_index = numbers
        .iter()
        .position(|n| n.original_position == orignal_position)
        .unwrap();

    let removed = numbers.remove(current_index);
    let new_index =
        ((current_index as i64 + removed.value).rem_euclid(numbers.len() as i64)) as usize;

    numbers.insert(new_index as usize, removed);
    numbers
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
