use aoc_runner_derive::aoc;

use crate::utils::Grid;

type Cell = (u64, (usize, usize));

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

fn build_map(raw_input: &str) -> Grid<u64> {
    let input = parse_input(raw_input);
    let width = input.first().unwrap().len();
    let cells: Vec<u64> = input.into_iter().flatten().collect();
    Grid::new(&cells, width)
}

fn get_low_points(map: &Grid<u64>) -> Vec<Cell> {
    let mut low_points = vec![];

    for x in 0..(map.size().0 as i32) {
        for y in 0..(map.size().1 as i32) {
            let cell = map.cell_at(x, y).unwrap();
            if map
                .neighbors_at(x, y)
                .iter()
                .all(|(neighbor, _)| cell < *neighbor)
            {
                low_points.push((cell, (x as usize, y as usize)));
            }
        }
    }

    low_points
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let map = build_map(input);
    let low_points = get_low_points(&map);

    low_points.into_iter().map(|(x, _)| x + 1).sum()
}

fn basin_at(map: &Grid<u64>, coords: (usize, usize), marked: &mut Grid<bool>) -> Vec<Cell> {
    let (x, y) = (coords.0 as i32, coords.1 as i32);
    let height = map.cell_at(x, y).unwrap();
    let is_marked = marked.cell_at(x, y).unwrap();

    if height >= 9 || is_marked {
        return vec![];
    }

    marked.set_at(coords.0, coords.1, true);

    let neighbors: Vec<(u64, (usize, usize))> = map
        .neighbors_at(x, y)
        .into_iter()
        .filter(|(n, _)| *n > height)
        .map(|(_, point)| basin_at(map, point, marked))
        .flatten()
        .collect();

    vec![vec![(height, coords)], neighbors].concat()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let map = build_map(input);
    let low_points = get_low_points(&map);

    // There's a basin at each low point. Collect their size.
    let mut marked = Grid::<bool>::new(&vec![false; map.size().0 * map.size().1], map.size().0);
    let mut basins: Vec<usize> = low_points
        .into_iter()
        .map(|(_, coords)| basin_at(&map, coords, &mut marked))
        .map(|b| b.len())
        .collect();

    basins.sort();

    basins[basins.len() - 3..].into_iter().product::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"2199943210
3987894921
9856789892
8767896789
9899965678
"#;

    #[test]
    fn test_day9_solve_part1() {
        assert_eq!(solve_part1(INPUT), 15);
    }

    #[test]
    fn test_day9_solve_part2() {
        assert_eq!(solve_part2(INPUT), 1134);
    }
}
