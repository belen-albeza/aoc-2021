use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_string()).collect()
}

fn build_report(input: &[String]) -> (u32, Vec<u32>) {
    let n_bits = input.get(0).unwrap().len() as u32;
    let report: Vec<u32> = input.iter().map(|x| u32::from_str_radix(x, 2).unwrap()).collect();
    (n_bits, report)
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> u32 {
    let (n_bits, report) = build_report(input);
    let mut gamma_rate = 0;

    for i in 0..n_bits {
        let mask = 2_u32.pow(i);
        let ones = report.iter().filter(|x| *x & mask == mask).count();
        if ones > report.len() / 2 {
            gamma_rate += mask
        }
    }

    let mask = 2_u32.pow(n_bits) - 1; // so for 5 bits we would get this mask 0b11111
    let epsilon_rate = !gamma_rate & mask ;

    gamma_rate * epsilon_rate
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum BitCriteria {
    MostCommon,
    LeastCommon,
}

fn filter_report(bit_index: u32, list: &[u32], criteria: BitCriteria) -> Vec<u32> {
    let mask = 2_u32.pow(bit_index);
    let ones: Vec<u32> = list.to_vec().into_iter().filter(|x| x & mask == mask).collect();
    let zeros: Vec<u32> = list.to_vec().into_iter().filter(|x| x & mask != mask).collect();

    match criteria {
        BitCriteria::MostCommon => if ones.len() >= zeros.len() { ones } else { zeros },
        BitCriteria::LeastCommon => if zeros.len() <= ones.len() { zeros } else { ones },
    }
}

fn get_rating(n_bits: u32, list: &[u32], criteria: BitCriteria) -> u32 {
    let mut candidates = list.to_vec();
    for i in (0..n_bits).rev() {
        candidates = filter_report(i, &candidates, criteria);
        if candidates.len() == 1 { break; }
    }

    *candidates.get(0).unwrap()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> u32 {
    let (n_bits, report) = build_report(input);

    let oxygen_rating = get_rating(n_bits, &report, BitCriteria::MostCommon);
    let co2_rating = get_rating(n_bits, &report, BitCriteria::LeastCommon);

    oxygen_rating * co2_rating
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part1() {
        let input: Vec<String> = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ].iter().map(|x| x.to_string()).collect();
        assert_eq!(solve_part1(&input), 198)
    }

    #[test]
    fn test_day3_part2() {
        let input: Vec<String> = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ].iter().map(|x| x.to_string()).collect();
        assert_eq!(solve_part2(&input), 230);
    }
}