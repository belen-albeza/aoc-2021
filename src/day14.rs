use aoc_runner_derive::aoc;

use std::collections::HashMap;

pub fn parse_input(input: &str) -> (String, HashMap<String, String>) {
    let mut sections = input.split("\n\n");
    let starter = sections.next().unwrap().to_owned();
    let rules: HashMap<String, String> = sections
        .next()
        .unwrap()
        .lines()
        .map(|raw| {
            let mut chunks = raw.split(" -> ");
            let key = chunks.next().unwrap().to_owned();
            let value = chunks.next().unwrap().to_owned();
            (key, value)
        })
        .collect();

    (starter, rules)
}

fn polymerize(polymer: &str, rules: &HashMap<String, String>, steps: usize) -> String {
    let mut polymer = polymer.to_string();

    for _ in 0..steps {
        let mut buffer = polymer.chars().nth(0).unwrap().to_string();

        for window in polymer.as_bytes().windows(2) {
            let pair = String::from_utf8(window.to_vec()).unwrap();
            if let Some(insert) = rules.get(&pair) {
                buffer.push_str(insert);
            }
            buffer.push(pair.chars().nth(1).unwrap());
        }

        polymer = buffer.clone();
    }

    polymer.to_string()
}

fn count_chars(input: &str) -> HashMap<char, u64> {
    let mut freqs = HashMap::new();
    for c in input.chars() {
        *freqs.entry(c).or_insert(0) += 1;
    }
    freqs
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let (template, rules) = parse_input(input);
    let polymer = polymerize(&template, &rules, 10);

    let freqs = count_chars(&polymer);
    let (_, max_count) = freqs.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
    let (_, min_count) = freqs.iter().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap();

    max_count - min_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14_parse_input() {
        let input = "NNCB\n\nCH -> B\nHH -> N";
        assert_eq!(
            parse_input(input),
            (
                "NNCB".to_string(),
                HashMap::<String, String>::from([
                    ("CH".to_string(), "B".to_string()),
                    ("HH".to_string(), "N".to_string())
                ])
            )
        );
    }

    #[test]
    fn test_day14_solve_part1() {
        let input = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;
        assert_eq!(solve_part1(input), 1588);
    }
}
