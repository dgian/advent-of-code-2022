use std::{
    result::Result,
    str::FromStr
};
use std::fs;
use GameForecast::*;

enum GameForecast {
    // A = Rock
    AX, AY, AZ,

    // B = Paper
    BX, BY, BZ,

    // C = Scissors
    CX, CY, CZ,
}

impl FromStr for GameForecast {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> { 
        match s {
            "AX" => Ok(AX),
            "AY" => Ok(AY),
            "AZ" => Ok(AZ),
            "BX" => Ok(BX),
            "BY" => Ok(BY),
            "BZ" => Ok(BZ),
            "CX" => Ok(CX),
            "CY" => Ok(CY),
            "CZ" => Ok(CZ),
            _ => Err(()),
        }
    }
}

fn eval_choice_forecast(forecast: GameForecast) -> i16 {
    // X = Rock, Y = Paper, Z = Scissors
    match forecast {
        AX => 4, AY => 8, AZ => 3,
        BX => 1, BY => 5, BZ => 9, 
        CX => 7, CY => 2, CZ => 6,
    }
}

fn eval_outcome_forecast(forecast: GameForecast) -> i16 {
    // X = Lose, Y = Draw, Z = Win
    match forecast {
        AX => 3, AY => 4, AZ => 8,
        BX => 1, BY => 5, BZ => 9,
        CX => 2, CY => 6, CZ => 7,
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");
    
    // Part1
    let sum: i16 = input.split("\n")
        .map(|s| s.split_whitespace().collect::<String>())
        .map(|s| GameForecast::from_str(s.as_str()))
        .map(|f| eval_choice_forecast(f.expect("Could not parse game forecast")))
        .sum();

    println!("Part 1 Sum = {}", sum);

    // Part2
    let sum: i16 = input.split("\n")
        .map(|s| s.split_whitespace().collect::<String>())
        .map(|s| GameForecast::from_str(s.as_str()))
        .map(|f| eval_outcome_forecast(f.expect("Could not parse game forecast")))
        .sum();

    println!("Part 2 Sum = {}", sum);
}