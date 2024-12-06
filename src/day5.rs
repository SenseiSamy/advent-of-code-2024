use std::collections::HashMap;
use std::cmp::Ordering::{Less, Greater};

#[aoc(day5, part1)]
fn part1(input: &str) -> i32 {
	let mut res = 0;
	let mut rules: HashMap<(i32, i32), bool> = HashMap::new();
	let mut parse_rule = true;
	for line in input.lines() {
		if line == "" {
			parse_rule = false;
		} else if parse_rule {
			let v: Vec<i32> = line.split('|').map(|e| e.parse::<i32>().unwrap()).collect();
			rules.insert((v[0], v[1]), true);
			rules.insert((v[1], v[0]), false);
		} else {
			let update: Vec<i32> = line.split(',').map(|e| e.parse::<i32>().unwrap()).collect();
			let len = update.len();
			if update.is_sorted_by(|a, b| rules[&(*a, *b)]) {
				res += update[len / 2];
			}
		}
	}
	res
}

#[aoc(day5, part2)]
fn part2(input: &str) -> i32 {
	let mut res = 0;
	let mut rules: HashMap<(i32, i32), bool> = HashMap::new();
	let mut parse_rule = true;
	for line in input.lines() {
		if line == "" {
			parse_rule = false;
		} else if parse_rule {
			let v: Vec<i32> = line.split('|').map(|e| e.parse::<i32>().unwrap()).collect();
			rules.insert((v[0], v[1]), true);
			rules.insert((v[1], v[0]), false);
		} else {
			let mut update: Vec<i32> = line.split(',').map(|e| e.parse::<i32>().unwrap()).collect();
			let len = update.len();
			if !update.is_sorted_by(|a, b| rules[&(*a, *b)]) {
				update.sort_by(|a, b| match rules[&(*a, *b)] {
					true => Less,
					false => Greater
				});
				res += update[len / 2];
			}
		}
	}
	res
}
