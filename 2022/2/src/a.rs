use std::{fs::OpenOptions, io::Read};

use anyhow::Result;

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn parse(c: char) -> Option<Move> {
        match c {
            'A' | 'X' => Some(Move::Rock),
            'B' | 'Y' => Some(Move::Paper),
            'C' | 'Z' => Some(Move::Scissors),
            _ => None,
        }
    }

    fn points(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn match_points(&self, other: Move) -> u32 {
        match (self, other) {
            (Move::Rock, Move::Scissors)
            | (Move::Paper, Move::Rock)
            | (Move::Scissors, Move::Paper) => 6,

            (Move::Rock, Move::Rock)
            | (Move::Paper, Move::Paper)
            | (Move::Scissors, Move::Scissors) => 3,

            (Move::Rock, Move::Paper)
            | (Move::Paper, Move::Scissors)
            | (Move::Scissors, Move::Rock) => 0,
        }
    }
}

struct Game {
    opponent: Move,
    player: Move,
}

impl Game {
    fn from_str(str: &str) -> Self {
        let mut chars = str.chars();
        let opponent = Move::parse(chars.next().unwrap()).unwrap();
        chars.next();
        let player = Move::parse(chars.next().unwrap()).unwrap();

        Self { opponent, player }
    }

    fn points(&self) -> u32 {
        self.player.match_points(self.opponent) + self.player.points()
    }
}

fn main() -> Result<()> {
    let mut input_file = OpenOptions::new().read(true).open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let mut total_points = 0;

    for line in input.lines() {
        let game = Game::from_str(line);
        total_points += game.points();
    }

    println!("{total_points}");

    Ok(())
}
