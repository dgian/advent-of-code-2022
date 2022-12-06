use std::{fs, str::Chars};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");

    // Part1
    println!("Part 1 marker position: {}", first_distinct_pos(input.chars(), 4));

    // Part2
    println!("Part 2 marker position: {}", first_distinct_pos(input.chars(), 14));
}

fn first_distinct_pos(chars: Chars, chunk_size: usize) -> usize {
    chars.collect::<Vec<char>>()
        .windows(chunk_size)
        .enumerate()
        .find(|(_, chunk)| {
            let mut v = chunk.to_vec();
            v.sort();
            v.dedup();
            v.len() == chunk.len()
         })
         .expect("Not found marker!").0 + chunk_size
}
