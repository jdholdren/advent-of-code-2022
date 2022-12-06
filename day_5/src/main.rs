use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

trait Stack<T> {
    fn pop(&mut self) -> Option<T>;
}

impl<T> Stack<T> for Vec<T> {
    fn pop(&mut self) -> Option<T> {
        let len = self.len();
        if len < 1 {
            return None;
        }

        Some(self.remove(len - 1))
    }
}

fn main() {
    // Set up the initial state
    let mut state = vec![
        vec!['Z', 'J', 'N', 'W', 'P', 'S'],
        vec!['G', 'S', 'T'],
        vec!['V', 'Q', 'R', 'L', 'H'],
        vec!['V', 'S', 'T', 'D'],
        vec!['Q', 'Z', 'T', 'D', 'B', 'M', 'J'],
        vec!['M', 'W', 'T', 'J', 'D', 'C', 'Z', 'L'],
        vec!['L', 'P', 'M', 'W', 'G', 'T', 'J'],
        vec!['N', 'G', 'M', 'T', 'B', 'F', 'Q', 'H'],
        vec!['R', 'D', 'G', 'C', 'P', 'B', 'Q', 'W'],
    ];

    lazy_static! {
        static ref RE: Regex = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    }

    // Get the lines of the input
    let input = fs::read_to_string("./day_5/input.txt").expect("error reading input");
    for line in input.split('\n') {
        if line == "" {
            continue;
        }

        // Parsing input
        let parts: Vec<&str> = line.split(' ').collect();
        let times: usize = parts[1].parse().unwrap();
        let from: usize = parts[3].parse::<usize>().unwrap() - 1;
        let to: usize = parts[5].parse::<usize>().unwrap() - 1;

        // Performing the action
        //
        // This is now (second part) equivalent to popping, then reversing it, then append it to
        // the destination stack. Like a workspace part of the warehouse to build the stack and
        // move it.
        let mut moving: Vec<char> = Vec::new();
        for _ in 0..times {
            // Take it from _there_ to the intermediate
            let val = state[from].pop().unwrap();
            moving.push(val);
        }
        moving.reverse();

        // Then append to the destination
        state[to].append(&mut moving);
    }

    // Output: Get the top of each stack
    let mut output = String::new();
    for v in state {
        output.push(*v.get(v.len() - 1).unwrap());
    }

    println!("Answer: {}", output);
}
