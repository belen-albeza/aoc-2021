use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

use std::collections::HashMap;

type SegmentsDisplay = (Vec<String>, Vec<String>);

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Vec<SegmentsDisplay> {
    input
        .lines()
        .map(|x| {
            let mut chunks = x.split(" | ");
            let signals = chunks
                .next()
                .unwrap()
                .split_whitespace()
                .map(String::from)
                .collect();
            let output = chunks
                .next()
                .unwrap()
                .split_whitespace()
                .map(String::from)
                .collect();
            (signals, output)
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[SegmentsDisplay]) -> u64 {
    let targets = [2, 4, 3, 7];
    input
        .into_iter()
        .map(|(_, output)| output.clone())
        .flatten()
        .fold(0, |total, x| {
            total + if targets.contains(&x.len()) { 1 } else { 0 }
        })
}

fn named_segments_frequency(signals: &[String]) -> HashMap<char, u64> {
    let mut res = HashMap::new();

    for name in signals.join("").chars() {
        *res.entry(name).or_insert(0) += 1;
    }

    res
}

fn named_segment_with_freq(freqs: &HashMap<char, u64>, target: u64) -> char {
    freqs
        .iter()
        .find_map(|(name, &freq)| if freq == target { Some(name) } else { None })
        .unwrap()
        .to_owned()
}

fn decode_signals(signals: &[String]) -> HashMap<String, u64> {
    let mut digits: [&str; 10] = [""; 10];
    let mut segs: [char; 7] = ['*'; 7]; // segments: 0 -> unmangled(a), 1 -> unmangled(b)...

    // 1, 4, 7, 8 are unique, so fill those first
    digits[1] = signals.iter().find(|x| x.len() == 2).unwrap();
    digits[8] = signals.iter().find(|x| x.len() == 7).unwrap();
    digits[4] = signals.iter().find(|x| x.len() == 4).unwrap();
    digits[7] = signals.iter().find(|x| x.len() == 3).unwrap();

    // Find segments with unique freq count
    let freqs = named_segments_frequency(signals);
    segs[1] = named_segment_with_freq(&freqs, 6); // B
    segs[4] = named_segment_with_freq(&freqs, 4); // E
    segs[5] = named_segment_with_freq(&freqs, 9); // F
    segs[2] = digits[1].chars().find(|x| *x != segs[5]).unwrap(); // C
    segs[3] = digits[4]
        .chars()
        .find(|x| ![segs[1], segs[2], segs[5]].contains(x))
        .unwrap(); // D
    segs[0] = digits[7]
        .chars()
        .find(|x| ![segs[2], segs[5]].contains(x))
        .unwrap(); // A
    segs[6] = digits[8]
        .chars()
        .find(|x| !segs.contains(x)) // only remaining unknown segment
        .unwrap(); // G

    // Digit 2 is the only one without segment 'f'
    digits[2] = signals.iter().find(|x| !x.contains(segs[5])).unwrap();
    // Digit 3 is the only one with len(5) and segments 'cf'
    digits[3] = signals
        .iter()
        .filter(|x| x.len() == 5)
        .find(|x| x.contains(segs[2]) && x.contains(segs[5]))
        .unwrap();
    // Digit 9 is the only one with len(6) and missing segment e
    digits[9] = signals
        .iter()
        .filter(|x| x.len() == 6)
        .find(|x| !x.contains(segs[4]))
        .unwrap();
    // Digit 5 is the only one with len(5) and missing segments c & e
    digits[5] = signals
        .iter()
        .filter(|x| x.len() == 5)
        .find(|x| !(x.contains(segs[4]) || x.contains(segs[2])))
        .unwrap();
    // Digit 0 is the only one with len(6) and missing segment d
    digits[0] = signals
        .iter()
        .filter(|x| x.len() == 6)
        .find(|x| !x.contains(segs[3]))
        .unwrap();
    // Digit 6 is the only one with len(6) and missing segment c
    digits[6] = signals
        .iter()
        .filter(|x| x.len() == 6)
        .find(|x| !x.contains(segs[2]))
        .unwrap();

    digits
        .iter()
        .enumerate()
        .map(|(index, digit)| (digit.chars().sorted().collect(), index as u64))
        .collect()
}

fn output_value(display: &SegmentsDisplay) -> u64 {
    let (signals, output) = display;
    let mapping = decode_signals(signals);

    output
        .into_iter()
        .map(|chunk| chunk.chars().sorted().collect::<String>())
        .map(|chunk| mapping[&chunk])
        .join("")
        .parse::<u64>()
        .unwrap()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[SegmentsDisplay]) -> u64 {
    input.iter().map(output_value).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn test_day8_parse_input() {
        let raw = "abc def | ab\nghi jkl | mn";
        assert_eq!(
            parse_input(raw),
            vec![
                (
                    vec!["abc".to_string(), "def".to_string()],
                    vec!["ab".to_string()]
                ),
                (
                    vec!["ghi".to_string(), "jkl".to_string()],
                    vec!["mn".to_string()]
                )
            ]
        );
    }

    #[test]
    fn test_day8_part1() {
        let input = parse_input(INPUT);
        assert_eq!(solve_part1(&input), 26);
    }

    #[test]
    fn test_day8_decode_signals() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let display = parse_input(input).clone();
        let (signals, _) = display.first().unwrap();

        assert_eq!(
            decode_signals(signals),
            HashMap::from([
                ("abcdeg".to_string(), 0),
                ("ab".to_string(), 1),
                ("acdfg".to_string(), 2),
                ("abcdf".to_string(), 3),
                ("abef".to_string(), 4),
                ("bcdef".to_string(), 5),
                ("bcdefg".to_string(), 6),
                ("abd".to_string(), 7),
                ("abcdefg".to_string(), 8),
                ("abcdef".to_string(), 9),
            ])
        );
    }

    #[test]
    fn test_day8_output_value() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let display = parse_input(input).clone();
        assert_eq!(output_value(&display.first().unwrap()), 5353);
    }

    #[test]
    fn test_day8_part2() {
        let input = parse_input(INPUT);
        assert_eq!(solve_part2(&input), 61229);
    }
}
