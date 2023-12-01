use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
	input.lines().map(String::from).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<String>) -> i32 {
	let mut sum = 0;
	for line in input {
		let mut num = String::new();
		for c in line.chars() {
			if c >= '0' && c <= '9' {
				num.push(c);
				break;
			}
		}
		for c in line.chars().rev() {
			if c >= '0' && c <= '9' {
				num.push(c);
				break;
			}
		}
		sum += i32::from_str(num.as_str()).unwrap();
	}

	return sum;
}

fn find_words(input: &str, offset: usize) -> Option<char> {
	let words = [
		("one", '1'),
		("two", '2'),
		("three", '3'),
		("four", '4'),
		("five", '5'),
		("six", '6'),
		("seven", '7'),
		("eight", '8'),
		("nine", '9'),
	];

	for (word, digit) in words {
		if word.len() > input.len() - offset {
			continue;
		}
		if &input[offset..(word.len() + offset)] == word {
			return Some(digit);
		}
	}

	return None;
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<String>) -> i32 {
	let mut sum = 0;
	for line in input {
		let mut num = String::new();
		for (offset, c) in line.chars().enumerate() {
			if c >= '0' && c <= '9' {
				num.push(c);
				break;
			}
			if let Some(digit) = find_words(line.as_str(), offset) {
				num.push(digit);
				break;
			}
		}
		for (offset, c) in line.chars().rev().enumerate() {
			if c >= '0' && c <= '9' {
				num.push(c);
				break;
			}
			if let Some(digit) = find_words(line.as_str(), line.len() - offset - 1) {
				num.push(digit);
				break;
			}
		}
		sum += i32::from_str(num.as_str()).unwrap();
	}

	return sum;
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE: &'static str = indoc! {"
		1abc2
		pqr3stu8vwx
		a1b2c3d4e5f
		treb7uchet
	"};

	static EXAMPLE2: &'static str = indoc! {"
		two1nine
		eightwothree
		abcone2threexyz
		xtwone3four
		4nineeightseven2
		zoneight234
		7pqrstsixteen
	"};

	#[test]
	fn example_part1() {
		assert_eq!(part1(&input_generator(EXAMPLE)), 142);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(&input_generator(EXAMPLE2)), 281);
	}
}
