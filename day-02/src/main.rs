#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut plays: Vec<Play> = vec![];
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(play_str) = line {
                if play_str.len() > 2 {
                    let split: Vec<&str> = play_str.trim().split(" ").collect();
                    let opponents_choice = match split[0] {
                        "A" => Choice::Rock,
                        "B" => Choice::Paper,
                        "C" => Choice::Scissors,
                        _ => panic!("unknown"),
                    };
                    let my_choice = Choice::what_to_choose(
                        opponents_choice.clone(),
                        match split[1] {
                            "X" => PlayResult::Lose,
                            "Y" => PlayResult::Draw,
                            "Z" => PlayResult::Win,
                            _ => panic!("unknown"),
                        },
                    );
                    plays.push(Play {
                        opponent: opponents_choice,
                        me: my_choice,
                    })
                }
            }
        }
    }

    println!("{}", plays.iter().map(|play| play.points()).sum::<usize>())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn value(&self) -> usize {
        match *self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn beats(&self) -> Choice {
        match *self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    fn loses(&self) -> Choice {
        match *self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    fn what_to_choose(given: Choice, result: PlayResult) -> Choice {
        match result {
            PlayResult::Win => given.loses(),
            PlayResult::Lose => given.beats(),
            PlayResult::Draw => given,
        }
    }

    fn play_score(&self, other: &Choice) -> usize {
        if *self == *other {
            return PlayResult::Draw.value() + self.value();
        } else if self.beats() == *other {
            return PlayResult::Win.value() + self.value();
        } else {
            return PlayResult::Lose.value() + self.value();
        }
    }
}

enum PlayResult {
    Win,
    Draw,
    Lose,
}

impl PlayResult {
    fn value(&self) -> usize {
        match *self {
            PlayResult::Win => 6,
            PlayResult::Lose => 0,
            PlayResult::Draw => 3,
        }
    }
}

struct Play {
    opponent: Choice,
    me: Choice,
}

impl Play {
    fn points(&self) -> usize {
        return self.me.play_score(&self.opponent);
    }
}
