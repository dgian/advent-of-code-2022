use std::{fs, ops::Range};

use array_tool::vec::Intersect;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");
    
    let range_pairs: Vec<_> = 
        input.lines()
            .map(|s| parse_input(s, ","))
            .map(|(left, right)| parse_range_pair((left, right), "-"))
            .collect();

    // Part1
    let p1: Vec<_> = range_pairs.iter()
        .filter(|(left, right)| contains_fully(&left, &right) || contains_fully(&right, &left))
        .collect();
    
    println!("Part1 count: {}", p1.len());

    // Part2
    let p2: Vec<_> = range_pairs.iter()
        .filter(|(left, right)| !left.intersect(right.clone()).is_empty())
        .collect();
    
    println!("Part2 count: {}", p2.len());
}

fn parse_input(input: &str, sep: &str) -> (String, String) {
    let (left, right) = input.split_at(input.find(sep).expect("Could not parse input"));

    (left.to_string(), right.replace(sep, ""))
}

fn parse_range_pair(pair: (String, String), separator: &str) -> (Vec<u32>, Vec<u32>) {
    (
        parse_range(
            pair.0.split_at(
                pair.0.find(separator).expect("Could not parse left range")
            ), 
            separator
        ).collect(),

        parse_range(
            pair.1.split_at(
                pair.1.find(separator).expect("Could not parse right range")
            ),
            separator
        ).collect()
    )
}

fn parse_range(range: (&str, &str), separator: &str) -> Range<u32> {
    let start = range.0.parse::<u32>()
        .expect("Could not parse range: start index");

    let end = range.1.replace(separator, "").parse::<u32>()
        .expect("Could not parse range: end index");

    Range { start, end: end + 1 }
}

fn contains_fully<T: PartialEq>(container: &Vec<T>, part: &Vec<T>) -> bool {
    part.is_empty() || container.windows(part.len()).any(|x| x == part)
}
