use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::convert::From;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Command {
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl From<&str> for Command {
    fn from(raw: &str) -> Self {
        let tokens: Vec<&str> = raw.split(' ').collect();
        let cmd = tokens[0];
        let value = tokens[1].parse::<i64>().unwrap();

        match cmd {
            "forward" => Self::Forward(value),
            "up" => Self::Up(value),
            "down" => Self::Down(value),
            _ => panic!("Unrecognized command: {}", raw),
        }
    }
}

pub trait Submarine {
    fn exec(&mut self, cmd: &Command);
    fn run(&mut self, input: &[Command]) {
        for cmd in input {
            self.exec(cmd);
        }
    }
}

#[derive(Debug)]
pub struct SubmarineV1 {
    x: i64,
    y: i64,
}

impl Submarine for SubmarineV1 {
    fn exec(&mut self, cmd: &Command) {
        match cmd {
            Command::Forward(delta) => self.x += delta,
            Command::Up(delta) => self.y -= delta,
            Command::Down(delta) => self.y += delta,
        }
    }
}

#[derive(Debug)]
pub struct SubmarineV2 {
    x: i64,
    y: i64,
    aim: i64,
}

impl Submarine for SubmarineV2 {
    fn exec(&mut self, cmd: &Command) {
        match cmd {
            Command::Up(delta) => self.aim -= delta,
            Command::Down(delta) => self.aim += delta,
            Command::Forward(delta) => {
                self.x += delta;
                self.y += self.aim * delta;
            }
        }
    }
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Command> {
    input.lines().map(Command::from).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Command]) -> i64 {
    let mut submarine = SubmarineV1 { x: 0, y: 0 };
    submarine.run(input);

    submarine.x * submarine.y
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Command]) -> i64 {
    let mut submarine = SubmarineV2 { x: 0, y: 0, aim: 0 };
    submarine.run(input);

    submarine.x * submarine.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_day2_parse_input() {
        let input = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");
        let parsed = parse_input(&input);
        assert_eq!(parsed.len(), 6);
        assert_eq!(
            parsed,
            vec![
                Command::Forward(5),
                Command::Down(5),
                Command::Forward(8),
                Command::Up(3),
                Command::Down(8),
                Command::Forward(2),
            ]
        );
    }

    #[test]
    pub fn test_day2_part1() {
        let commands = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        assert_eq!(solve_part1(&commands), 150);
    }

    #[test]
    pub fn test_day2_part2() {
        let commands = vec![
            Command::Forward(5),
            Command::Down(5),
            Command::Forward(8),
            Command::Up(3),
            Command::Down(8),
            Command::Forward(2),
        ];

        assert_eq!(solve_part2(&commands), 900);
    }
}
