fn part1_recursive(val: u128, pos: usize, vec: &Vec<u128>) -> bool {
	if pos == vec.len() && val == vec[0] {
		true
	} else if pos == vec.len() || val > vec[0] {
		false
	} else {
		part1_recursive(val + vec[pos], pos + 1, vec) | part1_recursive(val * vec[pos], pos + 1, vec)
	}
}

#[aoc(day7, part1)]
fn part1(input: &str) -> u128 {
	let mut res = 0;
	for line in input.lines() {
		// println!("{:?}", line.split(&[':', ' ']).collect::<Vec<&str>>());
		let line: Vec<u128> = line.split(&[':', ' ']).filter(|s| !s.is_empty()).map(|e| e.parse::<u128>().unwrap()).collect();
		if part1_recursive(line[1], 2, &line) {
			res += line[0];
		}
	}
	res
}

fn part2_recursive(val: u64, pos: usize, vec: &Vec<u64>) -> bool {
	if pos == vec.len() && val == vec[0] {
		true
	} else if pos == vec.len() || val > vec[0] {
		false
	} else {
		part2_recursive(val + vec[pos], pos + 1, vec) |
		part2_recursive(val * vec[pos], pos + 1, vec) |
		part2_recursive(val * 10u64.pow(vec[pos].ilog10() + 1) + vec[pos], pos + 1, vec)
	}
}

#[aoc(day7, part2)]
fn part2(input: &str) -> u64 {
	let mut res = 0;
	for line in input.lines() {
		let line: Vec<u64> = line.split(&[':', ' ']).filter(|s| !s.is_empty()).map(|e| e.parse::<u64>().unwrap()).collect();
		if part2_recursive(line[1], 2, &line) {
			res += line[0];
		}
	}
	res
}