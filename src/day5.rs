use std::collections::HashMap;

#[aoc(day5, part1)]
fn part1(input: &str) -> i32 {
	let mut res = 0;
	let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
	let mut parse_rule = true;
	'main: for line in input.lines() {
		if line == "" {
			parse_rule = false;
		} else if parse_rule {
			let v: Vec<i32> = line.split('|').map(|e| e.parse::<i32>().unwrap()).collect();
			rules.entry(v[0]).and_modify(|e| e.push(v[1])).or_insert(Vec::from([v[1]]));
		} else {
			let update: Vec<i32> = line.split(',').map(|e| e.parse::<i32>().unwrap()).collect();
			let len = update.len();
			for i in (0..len).rev() {
				if let Some(values) = rules.get(&update[i]) {
					for y in (0..i).rev() {
						if values.contains(&update[y]) {
							continue 'main;
						}
					}
				}
			}
			res += update[len / 2];
		}
	}
	res
}

fn part2_sort(array: &mut Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> i32 {
	let mut sorted = array.clone();
	while !array.is_empty() {
		
	}

	1
}

#[aoc(day5, part2)]
fn part2(input: &str) -> i32 {
	let mut res = 0;
	let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
	let mut parse_rule = true;
	'main: for line in input.lines() {
		if line == "" {
			parse_rule = false;
		} else if parse_rule {
			let v: Vec<i32> = line.split('|').map(|e| e.parse::<i32>().unwrap()).collect();
			rules.entry(v[0]).and_modify(|e| e.push(v[1])).or_insert(Vec::from([v[1]]));
		} else {
			let mut update: Vec<i32> = line.split(',').map(|e| e.parse::<i32>().unwrap()).collect();
			let len = update.len();
			for i in (0..len).rev() {
				if let Some(values) = rules.get(&update[i]) {
					for y in (0..i).rev() {
						if values.contains(&update[y]) {
							res += part2_sort(&mut update, &rules);
							continue 'main;
						}
					}
				}
			}
		}
	}
	res
}
