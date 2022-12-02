use std::{
    result::Result,
    str::FromStr
};
use std::fs;
use Shape::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> { 
        match s {
            "A" | "X" => Ok(Rock),
            "B" | "Y" => Ok(Paper),
            "C" | "Z" => Ok(Scissors),
            _ => Err(()),
        }
    }
}

impl Into<i16> for Shape {
    fn into(self) -> i16 { 
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        }
    }
}

#[derive(Copy, Clone)]
struct Game {
    opponent_choice: Shape,
    player_choice: Shape,
}

impl Game {
    fn from_choices(s: &str) -> Self {
        let choices: Vec<&str> = s.split(" ").collect();
        Game {
            opponent_choice: Shape::from_str(choices[0]).expect("No valid shape encoding: Opponent"),
            player_choice: Shape::from_str(choices[1]).expect("No valid shape encoding: Player"),
        }
    }

    fn from_outcome(s: &str) -> Self {
        let choices: Vec<&str> = s.split(" ").collect();

        let opponent_choice = Shape::from_str(choices[0]).expect("No valid shape encoding: Opponent");
        let player_choice = match choices[1] {
            "X" => WinningRule::from_shape(opponent_choice).weaker_shape,
            "Y" => opponent_choice,
            "Z" => WinningRule::from_shape(opponent_choice).stronger_shape,
            _ => opponent_choice
        };

        Game {
            opponent_choice,
            player_choice,
        }
    }

    fn player_score(self: Self) -> i16 {
        let outcome_score = if self.player_choice == self.opponent_choice {
            3
        } else if WinningRule::from_shape(self.player_choice).weaker_shape == self.opponent_choice {
            6
        } else {
            0
        };

        let player_choice_score: i16 = self.player_choice.into();
        outcome_score + player_choice_score
    }
}

struct WinningRule {
    weaker_shape: Shape,
    stronger_shape: Shape,
}

impl WinningRule {
    fn from_shape(shape: Shape) -> Self {
        match shape {
            Rock => WinningRule {weaker_shape: Scissors, stronger_shape: Paper},
            Paper => WinningRule {weaker_shape: Rock, stronger_shape: Scissors},
            Scissors => WinningRule{weaker_shape: Paper, stronger_shape: Rock},
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");

    // Part1
    let sum: i16 = input.split("\n")
        .map(|s| Game::from_choices(s))
        .map(|g| g.player_score())
        .sum();

    println!("Part 1 Sum = {}", sum);

    // Part2
    let sum: i16 = input.split("\n")
        .map(|s| Game::from_outcome(s))
        .map(|g| g.player_score())
        .sum();

    println!("Part 2 Sum = {}", sum);
}