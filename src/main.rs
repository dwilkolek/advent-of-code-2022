// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let elves = create_elves();
    let top3 = find_top_beefy_elves(3, elves);
    let sum_of_top_n_snacks = top3.into_iter().map(|e| e.total_snacks()).sum::<u64>();
    println!("{}", sum_of_top_n_snacks)
}

fn find_top_beefy_elves(count: usize, elves: Vec<Elf>) -> Vec<Elf> {
    let mut elves = elves.clone();
    elves.sort_by(|a, b| {
        if a.total_snacks() > b.total_snacks() {
            return Ordering::Less;
        } else {
            return Ordering::Equal;
        }
    });
    elves.as_slice().split_at(count).0.to_vec()
}

fn create_elves() -> Vec<Elf> {
    let mut elves = vec![];
    let file = "input.txt";
    if let Ok(lines) = read_lines(file) {
        let mut snack_buf: Vec<u64> = vec![];
        for line in lines {
            if let Ok(kcal_str) = line {
                match kcal_str.parse::<u64>() {
                    Ok(kcal) => snack_buf.push(kcal),
                    Err(_) => {
                        elves.push(Elf {
                            snacks: snack_buf.clone(),
                        });
                        snack_buf = vec![]
                    }
                }
            }
        }
    }
    return elves;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone)]
struct Elf {
    snacks: Vec<u64>,
}

impl Elf {
    fn total_snacks(&self) -> u64 {
        self.snacks.iter().sum()
    }
}
