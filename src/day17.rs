use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use lazy_static::lazy_static;
use regex::Regex;

use std::cmp::Ordering;

type Point = (i64, i64);
type Area = (Point, Point);

#[aoc_generator(day17)]
pub fn parse_input(input: &str) -> Area {
    // example input -> `target area: x=20..30, y=-10..-5`
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
    }

    let numbers: Vec<i64> = RE
        .find_iter(input)
        .map(|x| x.as_str().parse::<i64>().unwrap())
        .collect();
    ((numbers[0], numbers[1]), (numbers[2], numbers[3]))
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Probe {
    velocity: Point,
    position: Point,
}

impl Iterator for Probe {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        self.velocity.0 += match self.velocity.0.cmp(&0) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        };
        self.velocity.1 += -1;

        Some(self.position)
    }
}

fn is_inside_bbox(point: Point, area: Area) -> bool {
    let (range_x, range_y) = area;
    let (x, y) = point;
    x >= range_x.0 && x <= range_x.1 && y >= range_y.0 && y <= range_y.1
}

fn shoot(velocity: Point, area: Area) -> Option<i64> {
    let mut probe = Probe {
        velocity,
        position: (0, 0),
    };
    let mut max_y = probe.position.1;
    loop {
        let position = probe.next().unwrap();
        max_y = std::cmp::max(max_y, position.1);
        if is_inside_bbox(position, area) {
            return Some(max_y);
        }
        if probe.velocity.1 < 0 && position.1 < area.1 .0 {
            return None;
        }
        if probe.velocity.0 == 0 && (probe.position.0 < area.0 .0 || probe.position.0 > area.0 .1) {
            return None;
        }
    }
}

#[aoc(day17, part1)]
pub fn solve_part1(area: &Area) -> i64 {
    let mut shots: Vec<i64> = vec![];
    for x in 0..=area.0 .1 {
        for y in area.1 .0..500 {
            if let Some(y) = shoot((x, y), *area) {
                shots.push(y);
            }
        }
    }

    *shots.iter().max().unwrap()
}

#[aoc(day17, part2)]
pub fn solve_part2(area: &Area) -> usize {
    let mut shots: Vec<Point> = vec![];

    for x in 0..=area.0 .1 {
        for y in area.1 .0..500 {
            if shoot((x, y), *area).is_some() {
                shots.push((x, y));
            }
        }
    }

    shots.len() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day17_parse_input() {
        assert_eq!(
            parse_input("target area: x=20..30, y=-10..-5"),
            ((20, 30), (-10, -5))
        );
    }

    #[test]
    fn test_day17_solve_part1() {
        let area = ((20, 30), (-10, -5));
        assert_eq!(solve_part1(&area), 45);
    }

    #[test]
    fn test_day17_solve_part2() {
        let area = ((20, 30), (-10, -5));
        assert_eq!(solve_part2(&area), 112);
    }
}
