use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");
    let top_elf = find_most_prolific_elves(&input, 1);

    println!("Most prolific elf!");
    println!("{} : Calories: {}", top_elf[0].0, top_elf[0].1);

    let three_top_elves = find_most_prolific_elves(&input, 3);
    println!("\nThree most prolific elves!");
    println!("Total calories is {}", calculate_total_calories(&three_top_elves));
}

fn find_most_prolific_elves(input: &String, top: usize) -> Vec<(usize, u64)> {
    let mut vec: Vec<(usize, u64)> = input.split("\n\n")
        .enumerate()
        .map(|(elf, cal_list)| cal_list.split("\n")
            .map(|cal| (elf, cal.parse::<u64>().expect("Only numbers are supported")))
            .fold((elf, 0), |(index, total_cal), (_, cal)| (index, total_cal + cal))
        ).collect();

    vec.sort_by(|a, b| a.1.cmp(&b.1));
    vec.reverse();
    vec.into_iter().take(top).collect()
}

fn calculate_total_calories(elves: &Vec<(usize, u64)>) -> u64 {
    elves.into_iter()
        .fold(0, |total, (elf, cal)| {
            println!("{} : Calories: {}", elf, cal); 
            total + cal
        })
}