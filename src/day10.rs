use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_string()).collect()
}

fn is_open_token(token: char) -> bool {
    ['(', '[', '{', '<'].contains(&token)
}

fn get_expected_token_for(token: char) -> char {
    match token {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => ' ',
    }
}

fn parse_line(input: &str) -> Result<Vec<char>, char> {
    let mut buffer: Vec<char> = vec![];
    for c in input.chars() {
        if is_open_token(c) {
            buffer.push(get_expected_token_for(c));
            continue;
        }

        let expected = buffer.pop().unwrap();
        if c != expected {
            return Err(c);
        }
    }
    Ok(buffer)
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[String]) -> u64 {
    input
        .iter()
        .map(|x| match parse_line(x) {
            Ok(_) => 0,
            Err(')') => 3,
            Err(']') => 57,
            Err('}') => 1197,
            Err('>') => 25137,
            _ => 0,
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[String]) -> u64 {
    let buffers = input.iter().filter_map(|x| {
        let result = parse_line(x);
        match result {
            Ok(buffer) => Some(buffer.iter().rev().collect::<String>()),
            _ => None,
        }
    });

    let mut scores: Vec<u64> = buffers
        .into_iter()
        .map(|x| {
            x.chars()
                .map(|y| match y {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                })
                .fold(0, |total, c| total * 5 + c)
        })
        .collect();

    scores.sort_unstable();
    *scores.get(scores.len() / 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    #[test]
    pub fn test_day10_solve_part1() {
        assert_eq!(solve_part1(&parse_input(RAW_INPUT)), 26397);
    }

    #[test]
    pub fn test_day10_solve_part2() {
        assert_eq!(solve_part2(&parse_input(RAW_INPUT)), 288957);
    }
}
