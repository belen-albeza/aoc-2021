use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(measurements: &[u64]) -> u64 {
    measurements.windows(2).fold(0, |total, window| {
        total + if window[1] > window[0] { 1 } else { 0 }
    })
}

#[aoc(day1, part2)]
pub fn solve_part2(measurements: &[u64]) -> u64 {
    let triplets: Vec<&[u64]> = measurements.windows(3).collect();
    triplets.windows(2).fold(0, |total, window| {
        let prev_sum: u64 = window[0].iter().sum();
        let curr_sum: u64 = window[1].iter().sum();
        total + if curr_sum > prev_sum { 1 } else { 0 }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        let input: Vec<u64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(solve_part1(&input), 7);
    }

    #[test]
    fn test_day1_part2() {
        let input: Vec<u64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(solve_part2(&input), 5);
    }
}
