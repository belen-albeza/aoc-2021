use crate::utils::Grid;
use aoc_runner_derive::aoc;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::convert::From;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Tile {
    cost: usize,
    position: (usize, usize),
}

impl From<(usize, Point)> for Tile {
    fn from(item: (usize, Point)) -> Self {
        Self {
            cost: item.0,
            position: item.1,
        }
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Point = (usize, usize);

fn heuristic(a: Point, b: Point) -> usize {
    ((a.0 as i64 - b.0 as i64).abs() + (a.0 as i64 - b.0 as i64).abs()) as usize
}

fn a_star(grid: &Grid<usize>, start: Point, end: Point) -> Vec<Tile> {
    let mut frontier = BinaryHeap::new(); // priority queue
    let goal = Tile::from(grid.full_cell(end.0, end.1));
    frontier.push(Tile::from(grid.full_cell(start.0, start.1)));

    let mut came_from: HashMap<Point, Option<Point>> = HashMap::new();
    let mut cost_so_far: HashMap<Point, usize> = HashMap::new();

    came_from.insert(start, None);
    cost_so_far.insert(start, 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        if current == goal {
            break;
        }

        for (neighbor_cost, neighbor_position) in
            grid.neighbors_at(current.position.0 as i32, current.position.1 as i32)
        {
            let new_cost = cost_so_far[&current.position] + neighbor_cost;
            if cost_so_far.get(&neighbor_position).is_none()
                || new_cost < cost_so_far[&neighbor_position]
            {
                cost_so_far.insert(neighbor_position, new_cost);
                let priority = new_cost + heuristic(end, neighbor_position);
                frontier.push(Tile::from((priority, neighbor_position)));
                came_from.insert(neighbor_position, Some(current.position));
            }
        }
    }

    let mut path: Vec<Tile> = vec![Tile::from(grid.full_cell(end.0, end.1))];
    let mut current_cell = end;
    while let Some(cell) = came_from.get(&current_cell).unwrap() {
        path.push(Tile::from(grid.full_cell(cell.0, cell.1)));
        current_cell = *cell;
    }

    path.into_iter().rev().collect()
}

fn parse_input(input: &str) -> Grid<usize> {
    let tiles: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let width = tiles.get(0).unwrap().len();
    let cells: Vec<usize> = tiles.into_iter().flatten().collect();

    Grid::<usize>::new(&cells, width)
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let map = parse_input(input);

    let path = a_star(&map, (0, 0), (map.size().0 - 1, map.size().1 - 1));

    path.into_iter()
        .filter(|tile| tile.position != (0, 0)) // exclude starting point
        .map(|tile| tile.cost)
        .sum::<usize>() as u64
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &str) -> u64 {
    const MULTI: usize = 5;
    let mini_map = parse_input(input);
    let mini_width = mini_map.size().0;
    let mini_height = mini_map.size().1;
    let full_width = mini_width * MULTI;
    let full_height = mini_height * MULTI;

    let mut map = Grid::<usize>::new(&vec![0; full_width * full_height], full_width);
    for y in 0..mini_height {
        for x in 0..mini_width {
            for i in 0..MULTI {
                for j in 0..MULTI {
                    let offset = i + j;
                    let mut risk = mini_map.cell_at(x as i32, y as i32).unwrap() + offset;
                    if risk > 9 {
                        risk -= 9
                    }
                    map.set_at(x + mini_width * j, y + i * mini_height, risk);
                }
            }
        }
    }

    let path = a_star(&map, (0, 0), (map.size().0 - 1, map.size().1 - 1));

    path.into_iter()
        .filter(|tile| tile.position != (0, 0)) // exclude starting point
        .map(|tile| tile.cost)
        .sum::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"#;

    #[test]
    fn test_day15_solve_part1() {
        assert_eq!(solve_part1(INPUT), 40);
    }

    #[test]
    fn test_day15_solve_part2() {
        assert_eq!(solve_part2(INPUT), 315);
    }
}
