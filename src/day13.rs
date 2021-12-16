use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;

use crate::utils::Grid;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Fold {
    Up(usize),
    Left(usize),
}

type Tile = bool;

#[derive(Debug, PartialEq, Clone)]
pub struct Manual {
    grid: Grid<Tile>,
    instructions: Vec<Fold>,
}

impl Manual {
    pub fn fold(&mut self, fold: Fold) {
        let old_width = self.grid.size().0;
        let old_height = self.grid.size().1;
        let new_width = match fold {
            Fold::Up(_) => old_width,
            Fold::Left(x) => old_width - x - 1,
        };
        let new_height = match fold {
            Fold::Up(y) => old_height - y - 1,
            Fold::Left(_) => old_height,
        };

        let tiles: Vec<Tile> = (0..(new_height * new_width))
            .map(|i| {
                let x = (i % new_width) as i32;
                let y = (i / new_width) as i32;

                let (folding_x, folding_y) = match fold {
                    Fold::Up(_) => (x, old_height as i32 - y - 1),
                    Fold::Left(_) => (old_width as i32 - x - 1, y),
                };

                let top = self.grid.cell_at(x, y).unwrap_or(false);
                let bottom = self.grid.cell_at(folding_x, folding_y).unwrap_or(false);

                top || bottom
            })
            .collect();

        self.grid = Grid::<Tile>::new(&tiles, new_width);
    }
}

impl fmt::Display for Manual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = "".to_string();
        for y in 0..self.grid.size().1 {
            for x in 0..self.grid.size().0 {
                let tile = self.grid.cell_at(x as i32, y as i32).unwrap();
                buffer.push(match tile {
                    true => '#',
                    false => '.',
                });
            }
            if y < self.grid.size().1 - 1 {
                buffer.push('\n')
            }
        }
        writeln!(f, "{}", buffer)
    }
}

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> Manual {
    lazy_static! {
        static ref PARSER: Regex =
            Regex::new(r"^fold along (?P<axis>\w{1})=(?P<position>\d+)$").unwrap();
    }

    let mut sections_split = input.split("\n\n");

    let dots: Vec<(usize, usize)> = sections_split
        .next()
        .unwrap()
        .lines()
        .map(|raw| {
            let mut dot_split = raw.split(',');
            (
                dot_split.next().unwrap().parse::<usize>().unwrap(),
                dot_split.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    let instructions = sections_split
        .next()
        .unwrap()
        .lines()
        .map(|raw| {
            let captured = PARSER.captures(raw).unwrap();
            let axis = captured.name("axis").unwrap().as_str();
            let position = captured
                .name("position")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            match axis {
                "x" => Fold::Left(position),
                "y" => Fold::Up(position),
                _ => panic!("Unrecognized axis"),
            }
        })
        .collect();

    let sheet_width = dots.iter().map(|(x, _)| x).max().unwrap() + 1;
    let sheet_height = dots.iter().map(|(_, y)| y).max().unwrap() + 1;
    let tiles: Vec<Tile> = vec![false; sheet_width * sheet_height];

    let mut grid = Grid::<Tile>::new(&tiles, sheet_width);
    for (x, y) in dots {
        grid.set_at(x, y, true);
    }

    Manual { grid, instructions }
}

#[aoc(day13, part1)]
pub fn solve_part1(manual: &Manual) -> u64 {
    let mut manual = manual.to_owned();
    let fold = *manual.instructions.get(0).unwrap();

    manual.fold(fold);

    manual.grid.cells.into_iter().filter(|x| *x).count() as u64
}

#[aoc(day13, part2)]
pub fn solve_part2(manual: &Manual) -> String {
    let mut manual = manual.to_owned();
    let instructions = manual.instructions.to_owned();

    for fold in instructions.into_iter() {
        manual.fold(fold);
    }

    format!("\n{}", manual) // line break for readability
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13_parse_input() {
        let input = "1,0\n0,1\n1,2\n2,2\n\nfold along y=1\nfold along x=1";

        assert_eq!(
            parse_input(input),
            Manual {
                grid: Grid::<Tile>::new(
                    &[false, true, false, true, false, false, false, true, true],
                    3
                ),
                instructions: vec![Fold::Up(1), Fold::Left(1)]
            }
        );
    }

    #[test]
    fn test_fold_up() {
        let mut manual = Manual {
            grid: Grid::<Tile>::new(
                &[false, true, false, true, false, false, false, true, true],
                3,
            ),
            instructions: vec![],
        };

        manual.fold(Fold::Up(1));

        assert_eq!(manual.grid.size(), (3, 1));
        assert_eq!(manual.grid.cells, vec![false, true, true]);
    }

    #[test]
    fn test_fold_left() {
        let mut manual = Manual {
            grid: Grid::<Tile>::new(
                &[false, true, false, true, false, false, false, true, true],
                3,
            ),
            instructions: vec![],
        };

        manual.fold(Fold::Left(1));

        assert_eq!(manual.grid.size(), (1, 3));
        assert_eq!(manual.grid.cells, vec![false, true, true]);
    }

    const RAW_INPUT: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

    #[test]
    fn test_day13_solve_part1() {
        let input = parse_input(RAW_INPUT);
        assert_eq!(solve_part1(&input), 17);
    }
}
