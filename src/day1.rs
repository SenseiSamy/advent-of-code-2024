use std::collections::HashMap;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
	let mut list1 = Vec::new();
	let mut list2 = Vec::new();
	
	for line in input.lines() {
		let mut iter = line.split_whitespace();
		list1.push(iter.next().unwrap().parse().unwrap());
		list2.push(iter.next().unwrap().parse().unwrap());
	}
	(list1, list2)
}

#[aoc(day1, part1)]
pub fn part1(lists: &(Vec<i32>, Vec<i32>)) -> i32 {
	let (mut list1, mut list2) = lists.clone();
	list1.sort();
	list2.sort();
	list1.iter()
		.zip(list2.iter())
		.map(|(a, b)| (a - b).abs())
		.reduce(|acc, e| acc + e)
		.unwrap()
}

#[aoc(day1, part2)]
pub fn part2(lists: &(Vec<i32>, Vec<i32>)) -> i32 {
	let (list1, list2) = lists.clone();
	let mut res = 0;
	let mut map: HashMap<i32, i32> = HashMap::new();
	for n in list2 {
		map.entry(n)
			.and_modify(|e| *e += 1)
			.or_insert(1);
	}
	for n in list1 {
		res += n * map.get(&n).unwrap_or(&0);
	}
	res
}