use aoc_runner_derive::aoc;

use crate::utils::Grid;

pub fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|x| x.to_string().parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(raw_input: &str) -> u64 {
    let input = parse_input(raw_input);
    let width = input.first().unwrap().len();
    let cells: Vec<u64> = input.into_iter().flatten().collect();

    let map = Grid::new(&cells, width);
    let mut low_points = vec![];

    for x in 0..(map.size().0 as i32) {
        for y in 0..(map.size().1 as i32) {
            let cell = map.cell_at(x, y).unwrap();
            if map
                .neighbors_at(x, y)
                .iter()
                .all(|neighbor| cell < *neighbor)
            {
                low_points.push(cell);
            }
        }
    }

    low_points.into_iter().map(|x| x + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_solve_part1() {
        let input = r#"2199943210
3987894921
9856789892
8767896789
9899965678
"#;
        assert_eq!(solve_part1(input), 15);
    }
}
