// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

#[derive(Debug, Clone)]
struct Shape {
    matrix: Vec<Vec<i32>>,
}

impl Shape {
    fn new(matrix: Vec<Vec<i32>>) -> Shape {
        Shape { matrix }
    }
}

#[derive(Debug, Clone)]
struct ArenaShape {
    on_arena: Vec<(usize, usize)>,
}

#[derive(Debug, Clone)]
struct Arena {
    dropped_rows_grid: Vec<Vec<usize>>,
    grid: Vec<Vec<usize>>,
    shapes: Vec<Shape>,
    current: Option<ArenaShape>,
    highest_points: usize,
    spawned_shapes: usize,
    dropped_rows: usize,
    current_shape: usize,
    jets: Vec<char>,
    current_jet: usize,
}
fn empty_row() -> Vec<usize> {
    vec![0, 0, 0, 0, 0, 0, 0]
}
impl Arena {
    fn new(shapes: Vec<Shape>, jets: Vec<char>) -> Arena {
        Arena {
            dropped_rows_grid: vec![],
            grid: vec![empty_row(), empty_row()],
            shapes,
            current: None,
            highest_points: 0,
            spawned_shapes: 0,
            dropped_rows: 0,
            current_shape: 0,
            jets,
            current_jet: 0,
        }
    }
    fn checkpoint(&self) -> String {
        format!("{:?}", self.grid.clone())
    }
    fn spawn(&mut self) {
        if self.current.is_none() {
            let next_shape = self.shapes[self.current_shape].clone();
            self.current_shape = (self.current_shape + 1) % self.shapes.len();

            let new_hight = self.highest_points + 3;
            let matrix = &next_shape.matrix;

            let mut on_arena = vec![];
            for (ri, row) in matrix.into_iter().enumerate() {
                while self.grid.get(new_hight + ri).is_none() {
                    self.grid.push(empty_row())
                }
                for (ci, col) in row.into_iter().enumerate() {
                    if *col == 1 {
                        on_arena.push((new_hight + ri, ci + 2));
                    }
                }
            }
            self.current = Some(ArenaShape { on_arena });
            self.spawned_shapes = self.spawned_shapes + 1;
            // self.draw();
        } else {
            self.drop()
        }
    }

    fn wind(&mut self) {
        if self.current.as_ref().is_some() {
            let olds = &self.current.as_ref().unwrap().on_arena.clone();
            match self.jets[self.current_jet] {
                '<' => {
                    let can_move_left = olds
                        .into_iter()
                        .map(|p| p.1 > 0 && self.grid[p.0][p.1 - 1] == 0)
                        .all(|p| p);
                    if can_move_left {
                        let new_points: Vec<(usize, usize)> =
                            olds.into_iter().map(|p| (p.0, p.1 - 1)).collect();
                        self.current.as_mut().unwrap().on_arena = new_points;
                    }
                }
                '>' => {
                    let can_move_right = olds
                        .into_iter()
                        .map(|p| p.1 < 6 && self.grid[p.0][p.1 + 1] == 0)
                        .all(|p| p);
                    if can_move_right {
                        let new_points: Vec<(usize, usize)> =
                            olds.into_iter().map(|p| (p.0, p.1 + 1)).collect();
                        self.current.as_mut().unwrap().on_arena = new_points;
                    }
                }
                _ => {}
            };
        }
    }

    fn drop(&mut self) {
        if self.spawned_shapes == 400 {
            print!("")
        }
        let olds = &self.current.as_ref().unwrap().on_arena.clone();
        let can_move_down = olds
            .into_iter()
            .map(|p| p.0 > 0 && self.grid[p.0 - 1][p.1] == 0)
            .all(|p| p);
        if can_move_down {
            let new_points: Vec<(usize, usize)> =
                olds.into_iter().map(|p| (p.0 - 1, p.1)).collect();
            self.current.as_mut().unwrap().on_arena = new_points;
        } else {
            let mut new_high = self.highest_points;
            for p in self.current.as_ref().unwrap().on_arena.clone() {
                self.grid[p.0][p.1] = 2;
                if p.0 + 1 > new_high {
                    new_high = p.0 + 1;
                }
            }
            self.current = None;
            self.highest_points = new_high;
            self.trim();
        }
    }

    fn trim(&mut self) {
        let keep_min_rows = 40;

        let rows_before_trim = self.grid.len();
        let mut last_row = self.grid.len() - 1;
        loop {
            let visit_row = &mut empty_row();
            let row_maybe = self.grid.get(last_row);
            match row_maybe {
                Some(row) => {
                    for (i, p) in row.clone().into_iter().enumerate() {
                        if p == 2 {
                            visit_row[i] = 1;
                        }
                    }
                    if visit_row.into_iter().all(|e| *e == 1) {
                        if last_row > 0 {
                            let mut to_be_dropped = self.grid[..last_row].to_vec().clone();
                            self.dropped_rows_grid.append(&mut to_be_dropped);
                            self.grid = self.grid[last_row..].to_vec();
                            let diff = rows_before_trim - self.grid.len();
                            self.dropped_rows += diff;
                            self.highest_points -= diff;
                        }
                        return;
                    }
                }
                None => {
                    break;
                }
            }
            if last_row == 0 {
                break;
            }
            last_row -= 1;
        }
    }

    fn score(&self) -> usize {
        return self.dropped_rows + self.highest_points;
    }

    fn draw(&self) {
        let grid = &self.grid;
        let dropped_rows_grid = &self.dropped_rows_grid;
        let mut index = self.dropped_rows;
        let mut drawing: Vec<String> = vec![];
        for (row_i, row) in grid.into_iter().enumerate() {
            let mut line: String = String::new();
            for (col_index, col) in row.into_iter().enumerate() {
                if col_index == 0 {
                    line.push_str("|");
                }

                if self.current.as_ref().is_some()
                    && self
                        .current
                        .as_ref()
                        .unwrap()
                        .on_arena
                        .contains(&(row_i, col_index))
                {
                    line.push_str("@")
                } else if *col == 2 {
                    line.push_str("#")
                } else {
                    line.push_str(".")
                }
            }

            line.push_str(&format!("| {}", index));
            drawing.push(line);
            index += 1;
        }
        // for (_, row) in dropped_rows_grid.into_iter().enumerate() {
        //     let mut line: String = String::new();
        //     for (col_index, col) in row.into_iter().enumerate() {
        //         if col_index == 0 {
        //             line.push_str("|")
        //         }
        //         if *col == 0 {
        //             line.push_str(".")
        //         }
        //         if *col == 2 {
        //             line.push_str("#")
        //         }
        //     }

        //     line.push_str(&format!("| {}", index));
        //     drawing.push(line);
        //     index += 1;
        // }
        println!(
            "_____Spawned: {}, JET: {}",
            self.spawned_shapes, self.jets[self.current_jet]
        );
        for l in drawing.into_iter().rev() {
            println!("{}", l)
        }
        println!("_____Spawned END: {}", self.spawned_shapes);
    }
}

fn main() {
    let flat = Shape::new(vec![vec![1, 1, 1, 1]]);
    let plus = Shape::new(vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]]);
    let l = Shape::new(vec![vec![1, 1, 1], vec![0, 0, 1], vec![0, 0, 1]]);
    let line = Shape::new(vec![vec![1], vec![1], vec![1], vec![1]]);
    let box_s = Shape::new(vec![vec![1, 1], vec![1, 1]]);

    let shapes = vec![flat, plus, l, line, box_s];

    if let Ok(lines) = read_lines("input.txt") {
        let input: Vec<String> = lines.into_iter().map(|l| l.unwrap()).collect();
        let jets: Vec<char> = input.first().unwrap().chars().collect();
        let mut arena = Arena::new(shapes, jets);
        let mut checkpoints: HashMap<(usize, String), (usize, usize, Arena)> = HashMap::new();
        let to_spawn: usize = 1_000_000_000_000;
        loop {
            // if arena.dropped_rows_grid.len() + arena.grid.len() > 660 {
            //     println!(
            //         "_____{}__________________________________________________",
            //         arena.spawned_shapes
            //     );
            //     arena.draw();
            //     println!("_______________________________________________________");
            // }
            arena.spawn();

            if arena.spawned_shapes > to_spawn {
                break;
            }

            arena.wind();
            // arena.drop();

            if arena.current_shape == 0 && arena.current.is_none() {
                let checkpoint_key = (arena.current_jet, arena.checkpoint());
                let checkpoint_value = (
                    arena.spawned_shapes,
                    arena.dropped_rows + arena.highest_points,
                    arena.clone(),
                );
                if let Some(prev) = checkpoints.insert(checkpoint_key, checkpoint_value.clone()) {
                    println!("FOUND CHECKPOINT! {}", arena.spawned_shapes);
                    let cycle_length = checkpoint_value.0 - prev.0;
                    let dropped_rows_per_cycle = checkpoint_value.1 - prev.1;
                    arena.draw();
                    let cycles = (to_spawn - checkpoint_value.0) / cycle_length;

                    arena.spawned_shapes += cycle_length * cycles;
                    arena.dropped_rows += dropped_rows_per_cycle * cycles;
                    println!("After eval {}", arena.score());
                    checkpoints.clear();
                }
            }
            if arena.current.as_ref().is_some() {
                arena.current_jet = (arena.current_jet + 1) % arena.jets.len();
            }
        }

        arena.draw();
        println!("Arena {}", arena.score())
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<std::fs::File>>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
