#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
	let mut res = 0;
	for line in input.lines() {
		let report: Vec<i32> = line.split_ascii_whitespace()
			.map(|e| e.parse::<i32>().unwrap())
			.collect();
		if report[0] < report[1] {
			if report.is_sorted_by(|a, b| b - a > 0 && b - a <= 3) {
				res += 1;
			}
		} else {
			if report.is_sorted_by(|a, b| b - a >= -3 && b - a < 0) {
				res += 1;
			}
		}
	}
	res
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
	let mut res = 0;
	for line in input.lines() {
		let report: Vec<i32> = line.split_ascii_whitespace()
			.map(|e| e.parse::<i32>().unwrap())
			.collect();
		if report[0] < report[1] {
			let mut n_error = 0;
			for i in 0..report.len() - 1 {
				if !(report[i + 1] - report[i] > 0 && report[i + 1] - report[i] <= 3) {
					n_error += 1;
					if n_error == 3 {
						break;
					}
				}
			}
			if n_error < 3 {
				res += 1;
			}
		} else {
			let mut n_error = 0;
			for i in 0..report.len() - 1 {
				if !(report[i + 1] - report[i] >= -3 && report[i + 1] - report[i] < 0) {
					n_error += 1;
					if n_error == 3 {
						break;
					}
				}
			}
			if n_error < 3 {
				res += 1;
			}
		}
	}
	res
}