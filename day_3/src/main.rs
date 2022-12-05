use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./day_3/input.txt").expect("error reading input");

    let mut sum = 0;
    let mut group: Vec<String> = Vec::new();
    for line in input.split('\n') {
        // Group the lines together, find the common item
        group.push(line.to_owned());

        if group.len() % 3 != 0 {
            // We don't have a set of 3 yet, keep iterating
            continue;
        }

        let common_item = find_common_item(&group);
        sum += char_value(&common_item);

        // Clear out the group for the next 3
        group.clear();
    }

    println!("total: {}", sum);
}

// Finds the item in all compartments
fn find_common_item(compartments: &Vec<String>) -> char {
    // For each compartment, keep a map of what was seen
    let length = compartments.len();
    let mut seen: Vec<HashMap<char, ()>> = Vec::new();
    for compartment in compartments {
        let mut m = HashMap::new();
        compartment.chars().for_each(|c| {
            m.insert(c, ());
        });

        seen.push(m);
    }

    // We want the item in each map, so we'll want something that was seen seen.length number of
    // times
    let mut seen_in_compartment_counts: HashMap<char, usize> = HashMap::new();
    for m in seen {
        for key in m.keys() {
            if let Some(count) = seen_in_compartment_counts.get_mut(key) {
                *count += 1
            } else {
                seen_in_compartment_counts.insert(*key, 1);
            }
        }
    }
    for (key, count) in seen_in_compartment_counts {
        if count != length {
            continue;
        }

        return key;
    }

    panic!("Couldn't find a common item")
}

fn char_value(c: &char) -> u32 {
    // We'll compare the lowered character, and deal with if it's uppercase in a second
    let lowered = c.to_lowercase().to_string();

    // Oh boy, here we go
    let mut val = match lowered.as_str() {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e" => 5,
        "f" => 6,
        "g" => 7,
        "h" => 8,
        "i" => 9,
        "j" => 10,
        "k" => 11,
        "l" => 12,
        "m" => 13,
        "n" => 14,
        "o" => 15,
        "p" => 16,
        "q" => 17,
        "r" => 18,
        "s" => 19,
        "t" => 20,
        "u" => 21,
        "v" => 22,
        "w" => 23,
        "x" => 24,
        "y" => 25,
        "z" => 26,
        x => panic!("couldn't match character: {}", x),
    };

    // If it's uppercase, just add 26 to the score, e.g. A is +26 to a, which is 1
    if c.is_uppercase() {
        val += 26;
    }

    val
}
