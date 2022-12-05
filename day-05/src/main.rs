use std::{fs, vec};

use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");
    let (crates, commands) = input.split_at(input.find("\n\n").expect("Could not parse input format"));
    let commands = CommandList::from_commands(commands);

    // Part 1
    let mut handler_p1 = CraneHandler::from_crates_and_commands(crates, commands.clone());
    handler_p1.move_using_model(CraneModel::M9000);
    println!("Part 1 top crates: {}", handler_p1.top_crates().iter().join(""));

    // Part 2
    let mut handler_p2 = CraneHandler::from_crates_and_commands(crates, commands.clone());
    handler_p2.move_using_model(CraneModel::M9001);
    println!("Part 2 top crates: {}", handler_p2.top_crates().iter().join(""));
}

struct CraneHandler(Vec<Vec<char>>, CommandList);

impl CraneHandler {
    fn from_crates_and_commands(crates: &str, cmd_list: CommandList) -> CraneHandler {
        let mut crate_stacks: Vec<Vec<char>> = vec![vec![]; crates.lines().count()];

        crates.lines()
            .rev()
            .skip(1)
            .map(|line| {
                line.replace("[", "")
                    .replace("] ", "")
                    .replace("]", "")
                    .replace("    ", " ")
                    .replace("     ", "   ")
            })
            .for_each(|line| {
                line.chars().into_iter().enumerate()
                    .for_each(|(i, c)| {
                        if !c.is_whitespace() {
                            crate_stacks[i].push(c);
                        }
                    });
            });

        CraneHandler(crate_stacks, cmd_list)
    }

    fn top_crates(&self) -> Vec<char> {
        self.0.clone()
            .into_iter()
            .map(|stack| stack
                .into_iter()
                .last().expect("Crate stack is empty")
            )
            .collect()
    }

    fn move_using_model(&mut self, model: CraneModel) {
        self.1.0.iter().for_each(|cmd| {
            let len = self.0.get(cmd.1 - 1)
                .expect(format!("No crate stack found {}", cmd.1).as_str())
                .len();

            if len > 0 {
                let first = len - cmd.0;
                let mut crates: Vec<char> = self.0
                    .get_mut(cmd.1 - 1).expect(format!("No crates found in {}", cmd.1).as_str())
                    .drain(first..)
                    .collect();

                if model == CraneModel::M9000 {
                    crates.reverse();
                }

                self.0.get_mut(cmd.2 - 1)
                    .expect(format!("No crate stack found {}", cmd.2).as_str())
                    .append(&mut crates);
            }
        })
    }
}

#[derive(Debug, Clone)]
struct CommandList(Vec<MoveCommand>);

impl CommandList {
    fn from_commands(commands: &str) -> CommandList {
        let commands = commands.lines()
            .skip(2)
            .map(|line| -> MoveCommand {
                let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
                let cmd = re.captures(line).expect("Could not parse command format");
    
                MoveCommand (
                    cmd.get(1).expect("Could not parse move command. No items part")
                        .as_str().parse().expect("Could not parse items part. Not a number"),
                    cmd.get(2).expect("Could not parse move command. No source part")
                        .as_str().parse().expect("Could not parse source part. Not a number"),
                    cmd.get(3).expect("Could not parse move command. No target part")
                        .as_str().parse().expect("Could not parse targer part. Not a number"),
                )
            })
            .collect();

        CommandList(commands)
    }
}

#[derive(Debug, Clone, Copy)]
struct MoveCommand(usize, usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CraneModel {
    M9000,
    M9001,
}