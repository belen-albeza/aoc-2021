use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<u64> {
    input
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn distance(a: u64, b: u64) -> u64 {
    (a as i64 - b as i64).abs() as u64
}

fn compounded_distance(a: u64, b: u64) -> u64 {
    (0..=distance(a, b)).sum()
}

fn solver(crabs: &[u64], distance_fn: fn(u64, u64) -> u64) -> u64 {
    let min_x = crabs.iter().min().copied().unwrap() as u64;
    let max_x = crabs.iter().max().copied().unwrap() as u64;

    (min_x..=max_x)
        .map(|x| {
            crabs
                .iter()
                .fold(0, |total, crab| total + distance_fn(*crab, x))
        })
        .min()
        .unwrap()
}

#[aoc(day7, part1)]
pub fn solve_part1(crabs: &[u64]) -> u64 {
    solver(crabs, distance)
}

#[aoc(day7, part2)]
pub fn solve_part2(crabs: &[u64]) -> u64 {
    solver(crabs, compounded_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_part1() {
        let input: Vec<u64> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(solve_part1(&input), 37);
    }

    #[test]
    fn test_day7_compunded_distance() {
        assert_eq!(compounded_distance(5, 5), 0);
        assert_eq!(compounded_distance(16, 5), 66);
        assert_eq!(compounded_distance(1, 5), 10);
        assert_eq!(compounded_distance(5, 0), 15);
    }

    #[test]
    fn test_day7_part2() {
        let input: Vec<u64> = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(solve_part2(&input), 168);
    }
}
