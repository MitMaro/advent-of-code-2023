use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[derive(Debug)]
pub struct Board {
	grid: Vec<Vec<char>>,
	numbers: Vec<Number>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Number {
	value: u32,
	row: usize,
	col_start: usize,
	col_end: usize,
	used: bool,
}

fn has_symbol_neighbour(grid: &Vec<Vec<char>>, number: &Number) -> bool {
	for r in (number.row - 1)..=(number.row + 1) {
		for c in (number.col_start - 1)..=(number.col_end + 1) {
			let s = grid[r][c];
			if !s.is_digit(10) && s != '.' {
				return true;
			}
		}
	}
	false
}

fn get_gear_neighbour(grid: &Vec<Vec<char>>, number: &Number) -> Vec<(usize, usize, u32)> {
	let mut neighbours = vec![];
	for r in (number.row - 1)..=(number.row + 1) {
		for c in (number.col_start - 1)..=(number.col_end + 1) {
			let s = grid[r][c];
			if s == '*' {
				neighbours.push((r, c, number.value));
			}
		}
	}
	return neighbours;
}

fn break_line(row: usize, line: &Vec<char>) -> Vec<Number> {
	let mut numbers = vec![];

	let mut start = 0;
	let mut pointer = 0;
	let mut number = 0;

	loop {
		if line[pointer].is_digit(10) {
			number = number * 10 + (line[pointer] as u32 - '0' as u32);
		}
		else {
			if number > 0 {
				numbers.push(Number {
					value: number,
					row,
					col_start: start,
					col_end: pointer - 1,
					used: false,
				});
			}
			start = pointer + 1;
			number = 0;
		}
		pointer += 1;

		if pointer >= line.len() {
			break;
		}
	}
	numbers
}

impl Board {
	fn new(input: &str) -> Self {
		let mut lines = vec![vec![]];
		let mut numbers = vec![];

		for line in input.lines() {
			let mut chars = line.chars().collect::<Vec<char>>();
			chars.insert(0, '.');
			chars.push('.');
			lines.push(chars);
		}

		let rows = lines.len() - 1;
		let cols = lines[1].len() - 2;

		lines[0] = vec!['.'; cols + 2];
		lines.push(vec!['.'; cols + 2]);

		for row in 1..rows + 1 {
			numbers.append(&mut break_line(row, &lines[row]));
		}

		Board { grid: lines, numbers }
	}
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
	let board = Board::new(input);

	let mut sum = 0;
	for number in &board.numbers {
		if has_symbol_neighbour(&board.grid, number) {
			sum += number.value;
		}
	}

	sum
}
#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
	let board = Board::new(input);

	let mut neighbours = HashMap::new();
	let mut sum = 0;
	for number in &board.numbers {
		for (row, col, value) in get_gear_neighbour(&board.grid, number) {
			let neighbour = neighbours.entry((row, col)).or_insert_with(|| vec![]);
			neighbour.push(value);
		}
	}

	for neighbour in neighbours.values() {
		if neighbour.len() == 2 {
			sum += neighbour[0] * neighbour[1];
		}
	}

	sum
}

#[cfg(test)]
mod tests {
	use indoc::indoc;

	use super::*;

	static EXAMPLE: &'static str = indoc! {"
		467..114..
		...*......
		..35..633.
		......#...
		617*......
		.....+.58.
		..592.....
		......755.
		...$.*....
		.664.598..
	"};

	fn chars(line: &str) -> Vec<char> {
		line.chars().collect()
	}

	#[test]
	fn break_line_tests() {
		assert_eq!(break_line(1, &chars(".467..114...")), vec![
			Number {
				row: 1,
				value: 467,
				col_start: 1,
				col_end: 3,
				used: false
			},
			Number {
				row: 1,
				value: 114,
				col_start: 6,
				col_end: 8,
				used: false
			},
		]);
		assert_eq!(break_line(1, &chars(".4.1.")), vec![
			Number {
				row: 1,
				value: 4,
				col_start: 1,
				col_end: 1,
				used: false
			},
			Number {
				row: 1,
				value: 1,
				col_start: 3,
				col_end: 3,
				used: false
			},
		]);
	}

	#[test]
	fn example_part1() {
		assert_eq!(part1(EXAMPLE), 4361);
	}

	#[test]
	fn example_part2() {
		assert_eq!(part2(EXAMPLE), 467835);
	}
}
