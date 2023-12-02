use std::{cmp::max, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Round {
	red: i32,
	green: i32,
	blue: i32,
}

#[derive(Debug)]
pub struct Game {
	id: i32,
	rounds: Vec<Round>,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
	input
		.lines()
		.map(|line| {
			let (game, rounds_input) = line.split_once(':').unwrap();
			let game_id = i32::from_str(game.replace("Game ", "").as_str()).unwrap();

			let rounds = rounds_input
				.split(";")
				.map(|round_input| {
					let colors = round_input
						.split(",")
						.map(|color_input| {
							let (count, name) = color_input.trim().split_once(" ").unwrap();

							(String::from(name), i32::from_str(count).unwrap())
						})
						.collect::<Vec<(String, i32)>>();

					let mut red = 0;
					let mut green = 0;
					let mut blue = 0;

					for color in colors {
						if color.0 == "red" {
							red = color.1;
						}
						else if color.0 == "green" {
							green = color.1;
						}
						else if color.0 == "blue" {
							blue = color.1;
						}
					}

					Round { red, green, blue }
				})
				.collect();

			Game { id: game_id, rounds }
		})
		.collect()
}

#[aoc(day2, part1)]
pub fn part1(games: &Vec<Game>) -> i32 {
	let mut id_sum = 0;
	let allowed_red = 12;
	let allowed_green = 13;
	let allowed_blue = 14;

	'game: for game in games {
		for round in &game.rounds {
			if allowed_red < round.red || allowed_green < round.green || allowed_blue < round.blue {
				continue 'game;
			}
		}

		id_sum += game.id;
	}

	return id_sum;
}

#[aoc(day2, part2)]
pub fn part2(games: &Vec<Game>) -> i32 {
	let mut sum = 0;
	for game in games {
		let mut requited_red = 0;
		let mut required_green = 0;
		let mut required_blue = 0;

		for round in &game.rounds {
			requited_red = max(requited_red, round.red);
			required_green = max(required_green, round.green);
			required_blue = max(required_blue, round.blue);
		}

		sum += requited_red * required_green * required_blue;
	}

	return sum;
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE: &'static str = indoc! {"
		Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
		Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
		Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
		Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
		Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
	"};

	#[test]
	fn example_part1() {
		assert_eq!(part1(&input_generator(EXAMPLE)), 8);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&input_generator(EXAMPLE)), 2286);
	}
}
