use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;
use std::convert::From;

pub type Point = (i64, i64);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Segment {
    pub start: Point,
    pub end: Point,
}

impl From<&str> for Segment {
    fn from(raw: &str) -> Self {
        let points: Vec<i64> = raw
            .split(" -> ")
            .map(|x| x.split(','))
            .flatten()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        Self {
            start: (points[0], points[1]),
            end: (points[2], points[3]),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Dir {
    Horizontal,
    Vertical,
    Diagonal,
}

impl Segment {
    pub fn direction(&self) -> Dir {
        if self.start.0 == self.end.0 {
            Dir::Vertical
        } else if self.start.1 == self.end.1 {
            Dir::Horizontal
        } else {
            Dir::Diagonal
        }
    }

    pub fn points(&self) -> Vec<Point> {
        // Rust ranges do not support a negative step, so h_range and v_range
        // are a dirty way to bypass this.
        let h_range: Vec<i64> = if self.start.0 < self.end.0 {
            (self.start.0..=self.end.0).collect()
        } else {
            (self.end.0..=self.start.0).rev().collect()
        };

        let v_range: Vec<i64> = if self.start.1 < self.end.1 {
            (self.start.1..=self.end.1).collect()
        } else {
            (self.end.1..=self.start.1).rev().collect()
        };

        match self.direction() {
            Dir::Vertical => v_range.into_iter().map(|y| (self.start.0, y)).collect(),
            Dir::Horizontal => h_range.into_iter().map(|x| (x, self.start.1)).collect(),
            Dir::Diagonal => h_range.into_iter().zip(v_range.into_iter()).collect(),
        }
    }
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<Segment> {
    input.lines().map(Segment::from).collect()
}

fn solve(vents: &[Segment]) -> u64 {
    let mut ocean_map = HashMap::new();

    for p in vents.iter().map(|x| x.points()).flatten() {
        *ocean_map.entry(p).or_insert(0) += 1;
    }

    ocean_map
        .values()
        .fold(0, |total, x| total + if *x > 1 { 1 } else { 0 })
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Segment]) -> u64 {
    let vents = input
        .to_vec()
        .into_iter()
        .filter(|x| x.direction() != Dir::Diagonal)
        .collect::<Vec<Segment>>();
    solve(&vents)
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Segment]) -> u64 {
    solve(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_parse_segment() {
        assert_eq!(
            Segment::from("1,2 -> 3,14"),
            Segment {
                start: (1, 2),
                end: (3, 14)
            }
        );
    }

    #[test]
    fn test_day5_segment_points() {
        assert_eq!(
            Segment::from("0,0 -> 2,2").points(),
            vec![(0, 0), (1, 1), (2, 2)]
        );
        assert_eq!(
            Segment::from("2,2 -> 0,0").points(),
            vec![(2, 2), (1, 1), (0, 0)]
        );
        assert_eq!(
            Segment::from("0,0 -> 2,0").points(),
            vec![(0, 0), (1, 0), (2, 0)]
        );
        assert_eq!(
            Segment::from("2,0 -> 0,0").points(),
            vec![(2, 0), (1, 0), (0, 0)]
        );
        assert_eq!(
            Segment::from("0,0 -> 0,2").points(),
            vec![(0, 0), (0, 1), (0, 2)]
        );
    }

    const INPUT: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    #[test]
    fn test_day5_part1() {
        let input = parse_input(INPUT);
        assert_eq!(solve_part1(&input), 5);
    }

    #[test]
    fn test_day5_part2() {
        let input = parse_input(INPUT);
        assert_eq!(solve_part2(&input), 12);
    }
}
