// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]

use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        for line in input.into_iter() {
            sum += to_decimal(line);
        }
    }

    println!("Sum: {}, snafu: {}", sum, to_snafu(sum));
}

fn to_snafu(mut dec: i64) -> String {
    let mut snafu = Vec::new();
    while dec > 0 {
        let remainder = dec % 5;
        snafu.push(match remainder {
            3 => '=',
            4 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            e => unimplemented!("WTF {}", e),
        });
        if remainder >= 3 {
            dec += 5 - remainder
        }
        dec /= 5;
    }
    snafu.iter().rev().collect::<String>()
}

fn to_decimal(snafu: String) -> i64 {
    let mut exp = 0;
    let mut value = 0;
    for c in snafu.chars().into_iter().rev() {
        value += match c {
            '3' => 3 * 5_i64.pow(exp),
            '2' => 2 * 5_i64.pow(exp),
            '1' => 1 * 5_i64.pow(exp),
            '0' => 0 * 5_i64.pow(exp),
            '-' => -1 * 5_i64.pow(exp),
            '=' => -2 * 5_i64.pow(exp),
            e => unimplemented!("WTF {}", e),
        };
        exp += 1;
    }
    value
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
