use std::{collections::HashMap, hash::Hash, iter::Sum};

const P1_START: usize = 9;
const P2_START: usize = 10;

#[derive(Debug, Copy, Clone)]
struct GameResult {
    p1_wins: usize,
    p2_wins: usize,
}

impl<'a> Sum<&'a Self> for GameResult {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(
            Self {
                p1_wins: 0,
                p2_wins: 0,
            },
            |x, y| Self {
                p1_wins: x.p1_wins + y.p1_wins,
                p2_wins: x.p2_wins + y.p2_wins,
            },
        )
    }
}

// I definitely don't know how many of these I actually need.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct GameState {
    p1_score: usize,
    p2_score: usize,
    p1_pos: usize,
    p2_pos: usize,
    p1_turn: bool,
    nth_roll: usize,
    roll_sum: usize,
}

impl GameState {
    fn update(&self, roll: usize) -> Self {
        let mut new = self.clone();
        new.nth_roll += 1;
        new.roll_sum += roll;

        if new.nth_roll == 3 {
            if self.p1_turn {
                new.p1_pos = (new.p1_pos + new.roll_sum) % 10;
                if new.p1_pos == 0 {
                    new.p1_pos = 10;
                }
                new.p1_score += new.p1_pos;
            } else {
                new.p2_pos = (new.p2_pos + new.roll_sum) % 10;
                if new.p2_pos == 0 {
                    new.p2_pos = 10;
                }
                new.p2_score += new.p2_pos;
            }
            new.nth_roll = 0;
            new.p1_turn = !self.p1_turn;
            new.roll_sum = 0;
        }

        new
    }
}

fn part1() {
    let mut roll_count = 0;
    let mut state = GameState {
        p1_score: 0,
        p2_score: 0,
        p1_pos: P1_START,
        p2_pos: P2_START,
        p1_turn: true,
        nth_roll: 0,
        roll_sum: 0,
    };

    while state.p1_score < 1000 && state.p2_score < 1000 {
        state = state.update((roll_count % 100) + 1);
        roll_count += 1;
    }

    println!(
        "Part 1: {}",
        state.p1_score.min(state.p2_score) * roll_count
    );
}

fn universe(state: GameState, database: &mut HashMap<GameState, GameResult>) {
    if database.contains_key(&state) {
        return;
    } else if state.p1_score >= 21 {
        database.insert(
            state,
            GameResult {
                p1_wins: 1,
                p2_wins: 0,
            },
        );
    } else if state.p2_score >= 21 {
        database.insert(
            state,
            GameResult {
                p1_wins: 0,
                p2_wins: 1,
            },
        );
    } else {
        let results = [1, 2, 3]
            .iter()
            .map(|n| {
                let new_state = state.update(*n);
                if !database.contains_key(&new_state) {
                    universe(new_state, database);
                }
                database[&new_state]
            })
            .collect::<Vec<GameResult>>()
            .iter()
            .sum(); // Not entirely sure why it won't let me sum without collecting
        database.insert(state, results);
    }
}

fn part2() {
    let base_state = GameState {
        p1_score: 0,
        p2_score: 0,
        p1_pos: P1_START,
        p2_pos: P2_START,
        p1_turn: true,
        nth_roll: 0,
        roll_sum: 0,
    };

    let mut database: HashMap<GameState, GameResult> = HashMap::new();

    universe(base_state, &mut database);

    let result = database[&base_state];
    println!("Part 2: {}", result.p1_wins.max(result.p2_wins));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part1();
    part2();
    Ok(())
}
