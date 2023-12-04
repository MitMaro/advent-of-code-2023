use std::{collections::HashMap, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Debug)]
pub struct Card {
	id: u32,
	winning: Vec<u32>,
	yours: Vec<u32>,
}

impl Card {
	fn new(input: &str) -> Self {
		let (card, numbers) = input.split_once(": ").unwrap();
		let id = u32::from_str(card.replace("Card", "").trim()).unwrap();
		let (winning_input, yours_input) = numbers.split_once(" | ").unwrap();

		let winning = winning_input
			.split_ascii_whitespace()
			.map(|n| u32::from_str(n).unwrap())
			.collect::<Vec<u32>>();
		let yours = yours_input
			.split_ascii_whitespace()
			.map(|n| u32::from_str(n).unwrap())
			.collect::<Vec<u32>>();

		Self { id, winning, yours }
	}

	fn matching(&self) -> u32 {
		let mut matching = 0;
		for num in &self.yours {
			if self.winning.contains(num) {
				matching += 1;
			}
		}
		matching
	}
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
	let mut score = 0;

	for card in input.lines().map(Card::new) {
		let matching = card.matching();
		score += 1 << (matching) >> 1;
	}

	score
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
	let mut removed_cards = 0;
	let data = input
		.lines()
		.map(|input| {
			let card = Card::new(input);
			(card.id, card)
		})
		.collect::<Vec<(u32, Card)>>();

	let mut process_stack = data.iter().map(|(id, _)| *id).collect::<Vec<u32>>();

	let cards: HashMap<u32, Card> = HashMap::from_iter(data);

	// this is very inefficient, since it keeps pushing to the stack, instead of doing math
	while !process_stack.is_empty() {
		removed_cards += 1;
		let next = process_stack.pop().unwrap();

		let card = cards.get(&next).unwrap();

		let matches = card.matching();

		for i in 0..matches {
			process_stack.push(next + i + 1);
		}
	}

	removed_cards
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE: &'static str = indoc! {"
		Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
		Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
		Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
		Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
		Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
		Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
	"};

	#[test]
	fn example_part1() {
		assert_eq!(part1(EXAMPLE), 13);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(EXAMPLE), 30);
	}
}
