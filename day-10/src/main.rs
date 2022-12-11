use std::{fs, vec};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");
    let mut sprite_pos = 1;
    let mut cpu_cycles: Vec<i64> = vec![0];
    let mut crt_printer = CrtPrinter {
        position: 0, current_line: "".into(), multiline_screen: vec![]
    };
    
    input.lines()
        .for_each(|line| {
            crt_printer.print_line(sprite_pos);

            if line.starts_with("noop") {
                cpu_cycles.push(sprite_pos);
            } else if line.starts_with("addx") {
                cpu_cycles.push(sprite_pos);
                crt_printer.print_line(sprite_pos);
                
                let mut i = line.split_whitespace();
                i.next();
                let n: i64 = i.next().unwrap().parse().unwrap();
                sprite_pos += n;
                cpu_cycles.push(sprite_pos);
            }
        });
    
    let sum: i64 = cpu_cycles.iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, v)| (i+1) as i64 * v)
        .sum();

    println!("Part 1. Sum: {sum}");
    println!("Part 2. Screen: \n{}", crt_printer.multiline_screen.join("\n"));
}

struct CrtPrinter {
    position: i64,
    multiline_screen: Vec<String>,
    current_line: String,
}

impl CrtPrinter {
    const LINE_SIZE: usize = 40;

    fn print_line(&mut self, pos: i64) {
        let is_active_pos = self.position <= pos + 1 && self.position >= pos - 1;
        let pixel = if is_active_pos { "#" } else { "." };
        self.current_line.push_str(pixel);
        self.position += 1;

        let reached_line_limit = self.position > (CrtPrinter::LINE_SIZE as i64 - 1);
        if reached_line_limit {
            self.change_line();
        }
    }

    fn change_line(&mut self) {
        self.position = 0;
        self.multiline_screen.push(self.current_line.clone());
        self.current_line = "".into();
    }
}