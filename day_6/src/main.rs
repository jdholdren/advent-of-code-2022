use std::{collections::HashMap, fs};

struct Window<T: Copy, const N: usize> {
    contents: [T; N],
    len: usize,
}

impl<T: Copy, const N: usize> Window<T, N> {
    fn new(zero_val: T) -> Self {
        Window {
            contents: [zero_val; N],
            len: 0,
        }
    }

    // Pushes it into the moving window, then returns a copy of the window
    fn push(&mut self, t: T) -> [T; N] {
        let mut pos = self.len;

        // If we are at the cap, then we gotta shift everything
        if self.len == N {
            for i in 0..N {
                // Swap the i position with the one after it.
                // Obvs can't do that with the last one
                if i + 1 >= N {
                    continue;
                }
                self.contents[i] = self.contents[i + 1]
            }

            pos = N - 1 // We're swapping the last element
        }
        self.contents[pos] = t;
        self.len = pos + 1;

        self.contents.clone()
    }
}

fn main() {
    let input = fs::read_to_string("./day_6/input.txt").expect("error reading input");

    let ans = find_start(&input);
    println!("Answer: {}", ans);
}

fn find_start(input: &str) -> usize {
    let mut window: Window<char, 14> = Window::new(' ');
    for (i, c) in input.chars().enumerate() {
        // Get the last 4 as a slice
        let w = window.push(c);
        if check_for_duplicate(w) || has_whitespace(w) {
            continue;
        }

        return i + 1;
    }

    0
}

fn check_for_duplicate<const N: usize>(arr: [char; N]) -> bool {
    let mut hm: HashMap<char, ()> = HashMap::new();
    for c in arr {
        // Check for it being in the map, if it is, we know there's a dupe => true
        match hm.get(&c) {
            Some(_) => return true,
            None => hm.insert(c, ()),
        };
    }

    false
}

fn has_whitespace<const N: usize>(arr: [char; N]) -> bool {
    for c in arr {
        if c == ' ' {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::find_start;

    #[test]
    fn find_start_test() {
        assert_eq!(
            19,
            find_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
        );
        assert_eq!(
            23,
            find_start("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
        );
        assert_eq!(
            23,
            find_start("nppdvjthqldpwncqszvftbrmjlhg"),
            "nppdvjthqldpwncqszvftbrmjlhg"
        );
        assert_eq!(
            29,
            find_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
        );
        assert_eq!(
            26,
            find_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
        );
    }
}
