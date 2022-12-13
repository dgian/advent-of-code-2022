use std::vec;

fn main() {
    // Part 1
    let mut monkey_business = MonkeyBusiness::new(init_monkeys());
    for _ in 0..20 {
        monkey_business.play_round();
    }
    println!("Part 1. Monkey business level: {}", monkey_business.get_level());

    // Part 2
    monkey_business = MonkeyBusiness::new_with_relief(init_monkeys());
    for _ in 0..10_000 {
        monkey_business.play_round();
    }
    println!("Part 2. Monkey business level: {}", monkey_business.get_level());
}

/// ================= Monkey data.
/// No external file parsing, add monkeys inline in code.
fn init_monkeys() -> Vec<Monkey> {
    let monkey_0 = Monkey {
        items: vec![93, 54, 69, 66, 71],
        operation: |x| x * 3,
        predicate: |x| x % 7 == 0,
        success_monkey: 7,
        failure_monkey: 1,
        inspections: 0,
        relief_level: 7,
        
    };
    let monkey_1 = Monkey {
        items: vec![89, 51, 80, 66],
        operation: |x| x * 17,
        predicate: |x| x % 19 == 0,
        success_monkey: 5,
        failure_monkey: 7,
        inspections: 0,
        relief_level: 19,
    };
    let monkey_2 = Monkey {
        items: vec![90, 92, 63, 91, 96, 63, 64],
        operation: |x| x + 1,
        predicate: |x| x % 13 == 0,
        success_monkey: 4,
        failure_monkey: 3,
        inspections: 0,
        relief_level: 13,
    };
    let monkey_3 = Monkey {
        items: vec![65, 77],
        operation: |x| x + 2,
        predicate: |x| x % 3 == 0,
        success_monkey: 4,
        failure_monkey: 6,
        inspections: 0,
        relief_level: 3,
    };
    let monkey_4 = Monkey {
        items: vec![76, 68, 94],
        operation: |x| x * x,
        predicate: |x| x % 2 == 0,
        success_monkey: 0,
        failure_monkey: 6,
        inspections: 0,
        relief_level: 2,
    };
    let monkey_5 = Monkey {
        items: vec![86, 65, 66, 97, 73, 83],
        operation: |x| x + 8,
        predicate: |x| x % 11 == 0,
        success_monkey: 2,
        failure_monkey: 3,
        inspections: 0,
        relief_level: 11,
    };
    let monkey_6 = Monkey {
        items: vec![78],
        operation: |x| x + 6,
        predicate: |x| x % 17 == 0,
        success_monkey: 0,
        failure_monkey: 1,
        inspections: 0,
        relief_level: 17,
    };
    let monkey_7 = Monkey {
        items: vec![89, 57, 59, 61, 87, 55, 55, 88],
        operation: |x| x + 7,
        predicate: |x| x % 5 == 0,
        success_monkey: 2,
        failure_monkey: 5,
        inspections: 0,
        relief_level: 5,
    };

    vec![monkey_0, monkey_1, monkey_2, monkey_3, monkey_4, monkey_5, monkey_6, monkey_7]
}
// ================== Monkey Data

struct MonkeyBusiness {
    monkeys: Vec<Monkey>,
    round: u16,
    relief: Option<u64>,
}

impl MonkeyBusiness {
    fn new(monkeys: Vec<Monkey>) -> Self {
        MonkeyBusiness { monkeys, round: Default::default(), relief: None }
    }

    fn new_with_relief(monkeys: Vec<Monkey>) -> Self {
        let relief = monkeys.iter().map(|m| m.relief_level).product();
        MonkeyBusiness { monkeys, round: Default::default(), relief: Some(relief) }
    }

    fn play_round(&mut self) {
        self.round += 1;
        println!("Playing round {}", self.round);

        let mut monkeys_clone = self.monkeys.clone();

        self.monkeys.iter()
            .enumerate()
            .for_each(|(i, monkey)| {
                let (mut one, mut two) = monkeys_clone
                    .get_mut(i).unwrap()
                    .inspect_items(self.relief);

                monkeys_clone
                    .get_mut(monkey.success_monkey).unwrap()
                    .move_items(&mut one);
                
                monkeys_clone
                    .get_mut(monkey.failure_monkey).unwrap()
                    .move_items(&mut two);
            });

        self.monkeys = monkeys_clone;
    }

    fn get_level(&self) -> u64 {
        let mut v: Vec<u64> = self.monkeys.iter()
            .map(|monkey| monkey.inspections).collect();

        v.sort();
        v.reverse();
        
        v.iter().take(2).product()
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: fn(u64) -> u64,
    predicate: fn(u64) -> bool,
    success_monkey: usize,
    failure_monkey: usize,
    inspections: u64,
    relief_level: u64,
}

impl Monkey {
    fn inspect_items(&mut self, relief: Option<u64>) -> (Vec<u64>, Vec<u64>) {
        if self.items.is_empty() {
            return (vec![], vec![])
        }

        let mut success = vec![];
        let mut failure = vec![];

        self.items.iter()
            .for_each(|item| {
                let new = match relief {
                    Some(r) => (self.operation)(*item) % r,
                    None => (self.operation)(*item) / 3,
                };
                if (self.predicate)(new) {
                    success.push(new);
                } else {
                    failure.push(new);
                }
            });

        self.inspections += self.items.len() as u64;
        self.items.clear();
        
        (success, failure)
    }

    fn move_items(&mut self, one: &mut Vec<u64>) {
        self.items.append(one);
    }
}
