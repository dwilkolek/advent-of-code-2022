// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let entries: Vec<String> = lines
            .map(|line| match line {
                Ok(line) => line,
                _ => panic!("Wtf"),
            })
            .collect();

        let stream = entries.first().expect("Expected single line stream");
        println!(
            "Packet marker: {}",
            find_first_set_of_unique_characters(stream, 14)
        );
    }
}

fn find_first_set_of_unique_characters(stream: &String, size: usize) -> usize {
    if stream.len() < size {
        panic!("Stream is too short")
    }
    for end in size..stream.len() {
        let start = end - size;
        if &stream[start..].len() < &size {
            panic!("too few characters left for marker check")
        }
        let maybe_marker = &stream[start..end];
        if is_start_of_packet_marker(maybe_marker) {
            return end;
        }
    }
    panic!("Packet marker not found")
}

fn is_start_of_packet_marker(maybe_marker: &str) -> bool {
    let unique_characters: HashSet<_> = maybe_marker.chars().into_iter().collect();

    maybe_marker.chars().count() == unique_characters.len()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
