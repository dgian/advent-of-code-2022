use std::fs;
use itertools::Itertools;
use array_tool::vec::Intersect;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");
    
    // Part1
    let sum: u32 = input.lines()
        .map(|line| eval_priority_of_single(&line))
        .sum();
    println!("Part1 sum is {}", sum);

    // Part2
    let sum: u32 = eval_priority_of_group(&input, 3);
    println!("Part2 sum is {}", sum);
}


fn eval_priority_of_single(items_code: &str) -> u32 {
    let (left, right) = items_code.split_at(items_code.chars().count()/2);

    let left = left.chars().collect_vec();
    let right = right.chars().collect_vec();

    left.intersect(right).iter()
        .map(|code| parse_priority(code))
        .sum()
}

fn eval_priority_of_group(items_code: &str, size: usize) -> u32 {
    items_code.lines().chunks(size).into_iter()
        .map(|mut rucksack| -> u32 {
            let (first, second, third) = rucksack
                .next_tuple().expect("Expected group of 3 rucksacks");
            
            first.chars().collect_vec()      
                .intersect(second.chars().collect_vec())
                .intersect(third.chars().collect_vec())
                .iter()
                .map(|code| parse_priority(code))
                .sum()
        }).sum()
}

fn parse_priority(c: &char) -> u32 {  
    if c.is_ascii_uppercase() {
        c.to_ascii_lowercase() as u32 - 70
    } else {
        c.to_ascii_uppercase() as u32 - 64
    }
}
