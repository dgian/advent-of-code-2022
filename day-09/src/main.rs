use core::panic;
use std::{fs, collections::HashSet, cmp};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");
    let mut short_rope = Rope::new(
        Node(Coordinate(0,0), HashSet::from([Coordinate(0, 0)]), "head".into()), 1
    );
    let mut long_rope = Rope::new(
        Node(Coordinate(0, 0), HashSet::from([Coordinate(0,0)]), "head".into()), 9
    );

    input.lines()
        .for_each(|line| {
            let (movement, steps) = line.split_at(line.find(" ").unwrap());
            let steps = steps.replace(" ", "").parse().unwrap();
            
            match movement {
                "R" | "r" => {
                    short_rope.move_n_to(steps, Direction::East);
                    long_rope.move_n_to(steps, Direction::East);
                },
                "L" | "l" => {
                    short_rope.move_n_to(steps, Direction::West);
                    long_rope.move_n_to(steps, Direction::West);
                },
                "U" | "u" => {
                    short_rope.move_n_to(steps, Direction::North);
                    long_rope.move_n_to(steps, Direction::North);
                },
                "D" | "d" => {
                    short_rope.move_n_to(steps, Direction::South);
                    long_rope.move_n_to(steps, Direction::South);
                },
                _ => panic!("PANIC")
            };
        });
    
    println!("\nPart 1: Number of nodes that the tail has been {}", short_rope.get_node(1).1.len());
    println!("\nPart 2: Number of nodes that the tail has been {}", long_rope.get_node(9).1.len());
}

struct Rope {
    head: Node,
    nodes: Vec<Node>,
}

impl Rope {
    fn new(head: Node, length: usize) -> Self {
        let mut nodes = Vec::with_capacity(length);

        for i in 0..length {
            let node = Node(Coordinate(0, 0), HashSet::from([Coordinate(0, 0)]), format!("node {}", i+1));
            nodes.push(node);
        }

        Rope { head, nodes }
    }

    fn move_n_to(&mut self, n: usize, direction: Direction) {
        for _ in 0..n {
            self.move_one_to(direction);
        }
    }

    fn move_one_to(&mut self, direction: Direction) {
        self.head.move_to(direction);
        
        let mut other = self.head.clone();
        for i in 0..self.nodes.len() {
            let node = self.nodes.get_mut(i).unwrap();
            node.follow(&other);
            other = node.clone(); 
        }
    }

    fn get_node(&self, n: usize) -> &Node {
        self.nodes.get(n-1).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate(i64, i64);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node(Coordinate, HashSet<Coordinate>, String);

impl Node {
    fn move_to(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.0.1 += 1,
            Direction::East => self.0.0 += 1,
            Direction::South => self.0.1 -= 1,
            Direction::West => self.0.0 -= 1,
            Direction::NorthWest => {
                self.0.0 -= 1;
                self.0.1 += 1;
            },
            Direction::NorthEast => {
                self.0.0 += 1;
                self.0.1 += 1;
            },
            Direction::SouthWest => {
                self.0.0 -= 1;
                self.0.1 -= 1;
            },
            Direction::SouthEast => {
                self.0.0 += 1;
                self.0.1 -= 1;
            },
            _ => {}
        }
        self.1.insert(self.0.clone());
        println!("{} moved {:?}. New position ({:?})", self.2, direction, self.0);
    }
    
    fn change_direction(&mut self, other: &Node) {
        let x_diff = other.0.0 - self.0.0;
        let y_diff = other.0.1 - self.0.1;
        let direction = Direction::from_coordinate(Coordinate(x_diff, y_diff));

        match direction {
            Direction::North => self.move_to(direction),
            Direction::South => self.move_to(direction),
            Direction::West => self.move_to(direction),
            Direction::East => self.move_to(direction),
            _ => (),
        }
    }

    fn follow(&mut self, other: &Node) {
        let x_diff = other.0.0 - self.0.0;
        let y_diff = other.0.1 - self.0.1;

        let diagonal_moves = if x_diff.abs() == y_diff.abs() { 
            cmp::min(x_diff.abs(), y_diff.abs()) - 1 
        } else { 
            cmp::min(x_diff.abs(), y_diff.abs()) 
        };   

        let direction = Direction::from_coordinate(Coordinate(x_diff, y_diff));

        match direction {
            Direction::North | Direction::West | Direction::South | Direction::East => self.move_to(direction),
            Direction::NorthWest => {
                for _ in 0..diagonal_moves  {
                    self.move_to(direction);
                }
                self.change_direction(other);
            },
            Direction::NorthEast => {
                for _ in 0..diagonal_moves  {
                    self.move_to(direction);
                }
                self.change_direction(other);
            },
            Direction::SouthWest => {
                for _ in 0..diagonal_moves  {
                    self.move_to(direction);
                }
                self.change_direction(other);
            },
            Direction::SouthEast => {
                for _ in 0..diagonal_moves  {
                    self.move_to(direction);
                }
                self.change_direction(other);
            },
            _ => (),
        };
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    East,
    West,
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
    None,
}

impl Direction {
    fn from_coordinate(coord: Coordinate) -> Self {
        match coord {
            Coordinate(x, y) if x > 1 && y == 0 => Self::East,
            Coordinate(x, y) if x < -1 && y == 0 => Self::West,
            Coordinate(x, y) if y > 1 && x == 0 => Self::North,
            Coordinate(x, y) if x > 0 && y > 0 => Self::NorthEast,
            Coordinate(x, y) if x < 0 && y > 0 => Self::NorthWest,
            Coordinate(x, y) if y < -1 && x == 0 => Self::South,
            Coordinate(x, y) if x > 0 && y < 0 => Self::SouthEast,
            Coordinate(x, y) if x < 0 && y < 0 => Self::SouthWest,
            Coordinate(_, _) => Self::None,
        }
    }
}
