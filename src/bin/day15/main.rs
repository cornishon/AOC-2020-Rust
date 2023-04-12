use std::{num::ParseIntError, str::FromStr};

use std::collections::HashMap;

const NUMS: &str = include_str!("input.txt");
// const NUMS: &str = "3,1,2";
// const NUMS: &str = "0,3,6";

fn main() -> Result<(), ParseIntError> {
    let mut game: Game = NUMS.parse()?;

    println!("{}: {}", NUMS, game.play(30000000));
    Ok(())
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct NumEntry {
    last_seen: usize,
    turn_diff: Option<usize>,
}

#[derive(Debug)]
struct Game {
    history: HashMap<usize, NumEntry>,
    turn: usize,
    last_num: usize,
}

impl FromStr for Game {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s.split(',').map(|s| s.parse::<usize>());
        let mut turn = 0;
        let mut history: HashMap<usize, NumEntry> = HashMap::new();
        let mut last_num = 0;
        for n in nums {
            turn += 1;
            let n = n?;
            last_num = n;
            let last_seen = history.get(&n).map(|&e| e.last_seen);
            let turn_diff = last_seen.map(|n| turn - n);
            history.insert(
                n,
                NumEntry {
                    last_seen: turn,
                    turn_diff,
                },
            );
        }

        Ok(Game {
            history,
            turn,
            last_num,
        })
    }
}

impl Game {
    fn next_turn(&mut self) {
        let entry = self
            .history
            .get_mut(&self.last_num)
            .expect("the game can't be created empty");

        self.turn += 1;
        if entry.turn_diff.is_none() {
            self.update_entry(0);
        } else {
            let n = entry.turn_diff.unwrap();
            self.update_entry(n);
        }
    }

    fn update_entry(&mut self, n: usize) {
        self.history
            .entry(n)
            .and_modify(|e| {
                e.turn_diff = Some(self.turn - e.last_seen);
                e.last_seen = self.turn;
            })
            .or_insert({
                NumEntry {
                    last_seen: self.turn,
                    turn_diff: None,
                }
            });
        self.last_num = n;
    }

    fn play(&mut self, num_turns: usize) -> usize {
        while self.turn < num_turns {
            self.next_turn();
        }
        self.last_num
    }
}
