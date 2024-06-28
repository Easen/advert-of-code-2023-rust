use std::cmp;

fn main() {
    let input = include_str!("../../inputs/2.txt").trim_end();
    let games = parse_games(input);
    println!("part1 = {}", sum_possible_game_ids(&games));

    println!("part2 = {}", total_power(&games));
}

fn sum_possible_game_ids(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .filter(|game| game.red <= 12 && game.green <= 13 && game.blue <= 14)
        .map(|game| game.game_id)
        .sum()
}

fn total_power(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| game.red * game.green * game.blue)
        .sum()
}

struct Game {
    game_id: u32,
    red: u32,
    blue: u32,
    green: u32,
}

impl Game {
    pub fn parse_record(record: &str) -> Self {
        let (game, result) = record.split_once(":").unwrap();
        let (_, id) = game.split_once(" ").unwrap();

        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        result
            .split(";")
            .flat_map(|scores| scores.split(","))
            .for_each(|score| {
                let (score, colour) = score.trim().split_once(" ").unwrap();
                match colour {
                    "red" => red = cmp::max(red, score.parse::<u32>().unwrap()),
                    "green" => green = cmp::max(green, score.parse::<u32>().unwrap()),
                    "blue" => blue = cmp::max(blue, score.parse::<u32>().unwrap()),
                    _ => panic!("Unknown colour"),
                }
            });

        Self {
            game_id: id.parse().unwrap(),
            red,
            blue,
            green,
        }
    }
}

fn parse_games(input: &str) -> Vec<Game> {
    return input.lines().map(|l| Game::parse_record(&l)).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = parse_games(input);
        assert_eq!(sum_possible_game_ids(&games), 8);
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = parse_games(input);
        assert_eq!(total_power(&games), 2286);
    }
}
