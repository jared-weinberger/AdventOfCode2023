use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let stream = BufReader::new(File::open(".input-data/input.txt").unwrap());
    let goal: HashMap<String, u32> = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);
    let mut valid_games: u32 = 0;
    let mut power_sums: u64 = 0;
    let mut parsed_games: u32 = 0;
    let mut parse_failures: u32 = 0;
    for game_str in stream.lines().flatten() {
        if let Some(game) = process_game(game_str.as_str()) {
            power_sums += game.power();
            parsed_games += 1;
            let is_valid_count =
                |color| goal.contains_key(color) && goal[color] >= game.colors[color];
            let mut colors = game.colors.keys();
            if colors.all(is_valid_count) {
                valid_games += game.id;
            }
        } else {
            parse_failures += 1;
        }
    }
    println!("Valid Game ID Sum: {}", valid_games);
    println!("Game Power Sum: {}", power_sums);
    println!("Successfully parsed {} lines", parsed_games);
    println!("Failed to parse {} lines", parse_failures);
}

#[derive(Debug, PartialEq)]
struct GameSum {
    pub id: u32,
    pub colors: HashMap<String, u32>,
}
impl GameSum {
    fn power(&self) -> u64 {
        self.colors.values().map(|x| *x as u64).product()
    }
}

fn process_game(game_str: &str) -> Option<GameSum> {
    let (header, body) = game_str.split_once(": ")?;
    let game_id = parse_game_header(header)?;
    let colors = parse_game_body(body);
    Some(GameSum {
        id: game_id,
        colors,
    })
}

fn parse_game_header(header: &str) -> Option<u32> {
    header.split(' ').skip(1).nth(0)?.parse().ok()
}

fn parse_game_body(body: &str) -> HashMap<String, u32> {
    let mut color_sums = HashMap::new();
    let rounds = body.split("; ");
    for round in rounds {
        parse_round(round, &mut color_sums);
    }
    color_sums
}

fn parse_round(round: &str, totals: &mut HashMap<String, u32>) {
    for color in round.split(", ") {
        if let Some((count, color)) = color.split_once(' ') {
            if let Ok(count) = count.parse::<u32>() {
                totals
                    .entry(color.to_string())
                    .and_modify(|total_count| *total_count = max(total_count.to_owned(), count))
                    .or_insert(count);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_game() {
        let game_str = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let colors: HashMap<String, u32> = HashMap::from([
            ("red".to_string(), 1),
            ("green".to_string(), 6),
            ("blue".to_string(), 6),
        ]);
        let g = GameSum { id: 2, colors };
        assert_eq!(process_game(game_str), Some(g));
    }
}
