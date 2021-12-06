use aoc_runner_derive::aoc_generator;
use aoc_runner_derive::aoc;

use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<u64> {
    input.split(',').map(|x| x.parse::<u64>().unwrap()).collect()
}

fn simulate_fish(fish: i64, n: i64, cache: &mut HashMap<(i64, i64), u64>) -> u64 {
    // termination condition for recursivity
    if n < 0 { return 0; }

    // try to retrieve a pre-cached result
    if let Some(cached_res) = cache.get(&(fish, n)) {
        return cached_res.to_owned();
    }

    // compute reproduction over n days
    let mut fish_count = 1;
    for i in (fish..n+fish).step_by(7) {
        fish_count += simulate_fish(8, n-i-1, cache);
    }

    // save the result in the cache
    cache.insert((fish, n), fish_count);
    fish_count
}

fn simulation(input: &[u64], n: u64) -> u64 {
    let mut population_count: u64 = 0;
    let mut cache = HashMap::new();

    for fish in input.to_vec().into_iter() {
        population_count += simulate_fish(fish as i64, n as i64, &mut cache);
    }

    population_count
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    simulation(input, 80)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    simulation(input, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_part1() {
        let input = vec![3,4,3,1,2];
        assert_eq!(solve_part1(&input), 5934);
    }

    #[test]
    fn test_day6_part2() {
        let input = vec![3,4,3,1,2];
        assert_eq!(solve_part2(&input), 26984457539);
    }
}