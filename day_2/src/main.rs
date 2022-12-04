use std::fs;
use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    // Gets the points earned for playing this shape
    fn points(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn beats(&self, other: &Shape) -> bool {
        matches!(
            (self, other),
            (Self::Paper, Self::Rock)
                | (Self::Rock, Self::Scissors)
                | (Self::Scissors, Self::Paper)
        )
    }
}

impl FromStr for Shape {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shape = match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            &_ => return Err(ParseError(s.to_owned())),
        };

        Ok(shape)
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl FromStr for Outcome {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shape = match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            &_ => return Err(ParseError(s.to_owned())),
        };

        Ok(shape)
    }
}

#[derive(Debug)]
struct ParseError(String);

fn main() {
    let input = fs::read_to_string("./day_2/input.txt").expect("error reading input");

    // The cumulative total for the entire input
    let mut total = 0;
    for (n, line) in input.split('\n').enumerate() {
        // Each line is a play:
        // The left side is the other hand, the right side is our play
        let parts: Vec<&str> = line.split(' ').collect();
        if parts.len() != 2 {
            continue;
        }

        let other_hand: Shape = parts[0]
            .parse()
            .unwrap_or_else(|_| panic!("error parsing their hand on line: {}", n));
        let outcome: Outcome = parts[1]
            .parse()
            .unwrap_or_else(|_| panic!("error parsing outcome on line: {}", n));

        // We need to determine what our hand should be based on the outcome
        let our_hand = determine_our_hand(&other_hand, &outcome);

        total += score_round(&other_hand, &our_hand);
    }

    // The output should be the total
    println!("Total: {}", total);
}

// Determines what to play based on their hand and our desired outcome
fn determine_our_hand(their_hand: &Shape, desired: &Outcome) -> Shape {
    match desired {
        Outcome::Draw => {
            // If they want a draw, we need to return what they did
            *their_hand
        }
        Outcome::Win => match their_hand {
            Shape::Rock => Shape::Paper,
            Shape::Scissors => Shape::Rock,
            Shape::Paper => Shape::Scissors,
        },
        Outcome::Loss => match their_hand {
            Shape::Rock => Shape::Scissors,
            Shape::Scissors => Shape::Paper,
            Shape::Paper => Shape::Rock,
        },
    }
}

// Takes two plays, the other persons and us, returning the score from the hand
fn score_round(other_hand: &Shape, our_hand: &Shape) -> u32 {
    // First score the outcome of the round
    let outcome_score: u32 = if other_hand == our_hand {
        3
    } else if our_hand.beats(other_hand) {
        6
    } else {
        0
    };

    outcome_score + our_hand.points()
}
