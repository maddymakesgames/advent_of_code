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

    fn get_goal_move(&self, goal: Goal) -> Move {
        match (self, goal) {
            (Move::Rock, Goal::Win) | (Move::Paper, Goal::Tie) | (Move::Scissors, Goal::Lose) =>
                Move::Paper,
            (Move::Paper, Goal::Win) | (Move::Scissors, Goal::Tie) | (Move::Rock, Goal::Lose) =>
                Move::Scissors,
            (Move::Scissors, Goal::Win) | (Move::Rock, Goal::Tie) | (Move::Paper, Goal::Lose) =>
                Move::Rock,
        }
    }
}

#[derive(Clone, Copy)]
enum Goal {
    Win,
    Lose,
    Tie,
}

impl Goal {
    fn parse(c: char) -> Option<Goal> {
        match c {
            'X' => Some(Goal::Lose),
            'Y' => Some(Goal::Tie),
            'Z' => Some(Goal::Win),
            _ => None,
        }
    }

    fn points(&self) -> u32 {
        match self {
            Goal::Win => 6,
            Goal::Tie => 3,
            Goal::Lose => 0,
        }
    }
}

struct Game {
    opponent: Move,
    player: Goal,
}

impl Game {
    fn from_str(str: &str) -> Self {
        let mut chars = str.chars();
        let opponent = Move::parse(chars.next().unwrap()).unwrap();
        chars.next();
        let player = Goal::parse(chars.next().unwrap()).unwrap();

        Self { opponent, player }
    }

    fn points(&self) -> u32 {
        self.opponent.get_goal_move(self.player).points() + self.player.points()
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
