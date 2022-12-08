use std::{fs, vec};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");

    let mut coords: Vec<Coordinate> = vec![];
    let mut columns: Vec<Vec<char>> = vec![vec![]; 99];

    // Part 1
    input.lines().enumerate()
        .for_each(|(row_idx, line)| {
            let tree_heights: Vec<u32> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect();
        
            line.chars()
                .enumerate()
                .for_each(|(idx, height)| {
                    columns.get_mut(idx).unwrap().push(height);
                });
        
        if row_idx > 0 && row_idx < input.lines().count() - 1 {
            let mut coordinates: Vec<Coordinate> = line.char_indices().into_iter()
                .map(|(col_idx, c)| (col_idx, c.to_digit(10).unwrap()))
                .filter(|(col_idx, height)|
                    (*col_idx == 0 || *col_idx == tree_heights.len() - 1)
                        || height > tree_heights[..*col_idx].into_iter().max().unwrap()
                        || height > tree_heights[*col_idx+1..].into_iter().max().unwrap()
                )
                .map(|(col_idx, _)| Coordinate(row_idx, col_idx))
                .collect();

            coords.append(&mut coordinates);
        }
        else {
            line.char_indices()
                .for_each(|(col_idx, _)| 
                    coords.push(Coordinate(row_idx, col_idx)
                )
            );
        }
    });

    columns.iter().enumerate()
        .for_each(|(col_idx, col)| {
            let tree_heights: Vec<u32> = col.iter()
                .map(|c| c.to_digit(10).unwrap())
                .collect();

            if col_idx > 0 && col_idx < columns.len() - 1 { 
                let mut coordinates: Vec<Coordinate> = col.iter()
                    .enumerate()
                    .skip(1)
                    .take(col.len() - 2)
                    .map(|(row_idx, c)| (row_idx, c.to_digit(10).unwrap()))
                    .filter(|(row_idx, _)| !coords.contains(&Coordinate(*row_idx, col_idx)))
                    .filter(|(row_idx, height)|
                        height > tree_heights[..*row_idx].into_iter().max().unwrap()
                            || height > tree_heights[*row_idx+1..].into_iter().max().unwrap()
                    )
                    .map(|(row_idx, _)| Coordinate(row_idx, col_idx))
                    .collect();

                    coords.append(&mut coordinates);
            }
    });

    println!("Part 1 visible trees {}", coords.len());

    // Part 2
    let x = input.lines().enumerate().map(|(i, line)| {
        let tree_heights: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        
        line.char_indices().into_iter()
                .map(|(idx, height)| (idx, height.to_digit(10).expect("Could not parse tree height")))
                .map(|(idx, height)| {
                    if idx == 0 || idx == tree_heights.len() - 1 || i == 0 || i == columns.len() - 1 {
                        0
                    } else {
                        let mut left = tree_heights[..idx]
                            .into_iter().rev()
                            .take_while(|x| **x < height)
                            .count();
                        let mut right = tree_heights[idx+1..]
                            .into_iter()
                            .take_while(|x| **x < height)
                            .count();
                        let mut top = columns[idx][..i]
                            .into_iter().rev()
                            .take_while(|x| x.to_digit(10).unwrap() < height)
                            .count();
                        let mut bottom = columns[idx][i+1..]
                            .into_iter()
                            .take_while(|x| x.to_digit(10).unwrap() < height)
                            .count();

                        if right < tree_heights[idx+1..].len() {
                            right += 1;
                        }
                        if left < tree_heights[..idx].len() {
                            left += 1;
                        }
                        if top < columns[idx][..i].len() {
                            top += 1;
                        }
                        if bottom < columns[idx][i+1..].len() {
                            bottom += 1;
                        }
                        
                        top * bottom * right * left
                    }
                }).max().unwrap()
    }).max().unwrap();

    println!("Part 2 max scenic score: {}", x);
}

#[derive(PartialEq, Eq, Debug)]
struct Coordinate(usize, usize);
