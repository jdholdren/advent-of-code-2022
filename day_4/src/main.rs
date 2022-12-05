use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct ParseError(String);

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        ParseError(format!("{}", err))
    }
}

#[derive(Debug)]
struct Range {
    from: usize,
    to: usize,
}

impl Range {
    fn width(&self) -> usize {
        self.to - self.from
    }
}

impl FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // There should be two integers separated by a dash
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(ParseError(format!("Bad input: {}", s)));
        }

        // Parse both sides
        let from: usize = parts[0].parse()?;
        let to: usize = parts[1].parse()?;

        Ok(Range { from, to })
    }
}

fn main() {
    let input = fs::read_to_string("./day_4/input.txt").expect("error reading input");

    let mut count = 0;
    for line in input.split('\n') {
        // There will be a pair here, two ranges
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            continue;
        }

        let range_1: Range = parts[0].parse().expect("error parsing first range");
        let range_2: Range = parts[1].parse().expect("error parsing second range");

        if any_overlap(&range_1, &range_2) {
            count += 1;
        }
    }

    println!("Answer: {}", count);
}

fn fully_overlapped(r1: &Range, r2: &Range) -> bool {
    // Sort these so it's easier to compare, i.e. we can check the wider bounds at least cover the
    // smaller bounds
    let wider: &Range;
    let smaller: &Range;

    if r1.width() > r2.width() {
        wider = r1;
        smaller = r2;
    } else {
        wider = r2;
        smaller = r1;
    }

    // If the wider's from is lower or equal AND the wider's high is greater or equal
    wider.from <= smaller.from && wider.to >= smaller.to
}

fn any_overlap(r1: &Range, r2: &Range) -> bool {
    // They need to be ordered
    let first: &Range;
    let second: &Range;
    if r1.from < r2.from {
        first = r1;
        second = r2;
    } else {
        first = r2;
        second = r1;
    }

    first.to >= second.from
}

#[cfg(test)]
mod tests {
    use super::Range;

    #[test]
    fn any_overlap() {
        let table = vec![
            (
                Range { from: 68, to: 68 },
                Range { from: 69, to: 90 },
                false,
            ),
            (Range { from: 68, to: 70 }, Range { from: 69, to: 90 }, true),
            (Range { from: 68, to: 70 }, Range { from: 69, to: 90 }, true),
        ];

        for tc in table {
            let got = super::any_overlap(&tc.0, &tc.1);
            assert_eq!(got, tc.2);
        }
    }
}
