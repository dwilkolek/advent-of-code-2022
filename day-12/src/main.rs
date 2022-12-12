// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    height: usize,
}

impl Point {
    fn new(x: usize, y: usize, height: usize) -> Point {
        Point { x, y, height }
    }
}

#[derive(Debug)]
struct Map<'a> {
    points: Vec<Vec<Point>>,
    deq: VecDeque<(&'a Point, usize)>,
}

impl<'a> Map<'a> {
    fn neighbours(points: &Vec<Vec<Point>>, row: usize, col: usize) -> Vec<&Point> {
        let current_height = points[row][col].height;
        let mut options: Vec<&Point> = Vec::new();
        if row > 0 {
            Map::add_if_ok(&points[row - 1][col], current_height, &mut options);
        }
        if row < points.len() - 1 {
            Map::add_if_ok(&points[row + 1][col], current_height, &mut options);
        }
        if col > 0 {
            Map::add_if_ok(&points[row][col - 1], current_height, &mut options);
        }
        if col < points[0].len() - 1 {
            Map::add_if_ok(&points[row][col + 1], current_height, &mut options);
        }
        options
    }

    fn add_if_ok(point: &'a Point, max_height: usize, options: &mut Vec<&'a Point>) {
        if point.height >= max_height - 1 {
            options.push(point);
        }
    }

    fn visit(&'a mut self, goals: &Vec<Point>) -> usize {
        let d = &mut self.deq;
        let mut visited: HashSet<Point> = HashSet::new();
        while let Some(next) = d.pop_front() {
            // println!("Processing: {:?}", next);
            if goals.contains(&next.0) {
                return next.1;
            } else {
                let neighbours = Map::neighbours(&self.points, next.0.x, next.0.y);
                for n in neighbours.into_iter() {
                    if !visited.contains(&n) {
                        visited.insert(n.clone());
                        // println!("Adding neighbour: {:?}", n);
                        d.push_back((n, next.1 + 1));
                    } else {
                        // println!("Discarding neighbour: {:?}", n);
                    }
                }
            }
        }
        return usize::MAX;
    }
}

fn main() {
    let mut starts: Vec<Point> = vec![];
    let mut end: Point = Point::new(usize::MAX, usize::MAX, 'a' as usize);
    let mut map = Map {
        points: vec![],
        deq: VecDeque::new(),
    };
    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        for (row, line) in input.into_iter().enumerate() {
            let mut row_vec = Vec::new();
            for (col, char) in line.chars().into_iter().enumerate() {
                match char {
                    'S' => {
                        starts.push(Point::new(row, col, 'a' as usize));
                        row_vec.push(Point::new(row, col, 'a' as usize));
                    }
                    'E' => {
                        end = Point::new(row, col, 'z' as usize);
                        row_vec.push(Point::new(row, col, 'z' as usize))
                    }
                    c => {
                        if c == 'a' {
                            //part-2
                            starts.push(Point::new(row, col, 'a' as usize));
                        }
                        row_vec.push(Point::new(row, col, char as usize))
                    }
                };
            }
            map.points.push(row_vec);
        }
    }
    map.deq.push_back((&end, 0));
    println!("Minimum required steps: {}", map.visit(&starts));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
