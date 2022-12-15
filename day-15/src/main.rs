use regex::Regex;
// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
use serde_json::{json, Value};
use std::char::MAX;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::path::Path;
use std::{thread, vec};

#[derive(Clone)]
struct Ap {
    sensor: (i32, i32),
    beacon: (i32, i32),
}

impl Ap {
    fn tuning_freq(beacon: (i32, i32)) -> i32 {
        return beacon.0 * 4000000 + beacon.1;
    }
    fn contains(&self, p: (i32, i32)) -> bool {
        return Ap::dist(self.sensor, self.beacon) >= Ap::dist(self.sensor, p);
    }
    fn get_coverage(&self, y: i32) -> Vec<(i32, i32)> {
        let mut cov: Vec<_> = vec![];
        let dist = Ap::dist(self.sensor, self.beacon);
        for x in self.sensor.0 - dist..=self.sensor.0 + dist {
            if Ap::dist(self.sensor, (x, y)) <= dist {
                cov.push((x, y))
            }
        }

        cov
    }

    fn dist(a: (i32, i32), b: (i32, i32)) -> i32 {
        return (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as i32;
    }
}

fn main() {
    let reg = Regex::new("Sensor at x=([-]{0,1}[0-9]+), y=([-]{0,1}[0-9]+): closest beacon is at x=([-]{0,1}[0-9]+), y=([-]{0,1}[0-9]+)").unwrap();

    let mut aps: Vec<Ap> = vec![];

    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();

        for line in input.into_iter() {
            let cap = reg.captures(&line).unwrap();

            aps.push(Ap {
                sensor: (
                    cap.get(1).unwrap().as_str().parse().unwrap(),
                    cap.get(2).unwrap().as_str().parse().unwrap(),
                ),
                beacon: (
                    cap.get(3).unwrap().as_str().parse().unwrap(),
                    cap.get(4).unwrap().as_str().parse().unwrap(),
                ),
            })
        }

        let y = 2000000;
        let mut coverage_on_y: HashSet<(i32, i32)> = HashSet::new();
        for (i, ap) in aps.into_iter().enumerate() {
            println!("Sensor {}", i);
            ap.get_coverage(y).into_iter().for_each(|p| {
                if p.1 == y {
                    coverage_on_y.insert(p);
                }
            })
        }
        println!("Coverage on Y({}) is: {}", y, coverage_on_y.len() - 1);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
