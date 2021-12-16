use aoc_runner_derive::aoc;

use crate::utils::Grid;

type Octopus = u64;

fn parse_input(input: &str) -> Grid<Octopus> {
    let octopuses: Vec<Vec<Octopus>> = input
        .lines()
        .map(|row| {
            row.chars()
                .map(|x| x.to_string().parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    let width = octopuses.get(0).unwrap().len();
    let cells: Vec<Octopus> = octopuses.into_iter().flatten().collect();
    Grid::new(&cells, width)
}

#[derive(Debug, Clone)]
struct OctopusSim {
    pub map: Grid<Octopus>,
}

impl OctopusSim {
    pub fn new(map: Grid<Octopus>) -> Self {
        Self { map }
    }

    pub fn run(&mut self, steps: usize) -> u64 {
        let mut flashes = 0;
        for _ in 0..steps {
            flashes += self.tick()
        }
        flashes
    }

    fn tick(&mut self) -> u64 {
        // 1. Increase the level energy of all octopuses by 1
        for octopus in self.map.cells.iter_mut() {
            *octopus += 1;
        }
        // 2. Flash octopuses with energy level > 9
        let mut flashes: Vec<(usize, usize)> = vec![];
        loop {
            let did_flash = self.tick_map(&mut flashes);
            if !did_flash {
                break;
            }
        }

        flashes.len() as u64
    }

    fn flash_at(&mut self, x: usize, y: usize, flashed: &[(usize, usize)]) {
        let neighbors = self.map.neighbors8_at(x as i32, y as i32);

        for (octopus, point) in neighbors
            .iter()
            .filter(|(_, point)| !flashed.contains(point))
        {
            self.map.set_at(point.0, point.1, octopus + 1);
        }
    }

    fn tick_map(&mut self, flashes: &mut Vec<(usize, usize)>) -> bool {
        let mut did_flash = false;
        for x in 0..self.map.size().0 {
            for y in 0..self.map.size().1 {
                if self.tick_cell(x, y, flashes) {
                    did_flash = true;
                }
            }
        }

        did_flash
    }

    fn tick_cell(&mut self, x: usize, y: usize, flashes: &mut Vec<(usize, usize)>) -> bool {
        if self.map.cell_at(x as i32, y as i32).unwrap() <= 9 {
            return false;
        }

        self.map.set_at(x, y, 0);

        if flashes.contains(&(x, y)) {
            false
        } else {
            self.flash_at(x, y, flashes);
            flashes.push((x, y));
            true
        }
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let map = parse_input(input);
    let mut sim = OctopusSim::new(map);

    sim.run(100)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let map = parse_input(input);
    let mut sim = OctopusSim::new(map);

    let mut steps = 0;
    let octopus_count = sim.map.size().0 * sim.map.size().1;

    loop {
        steps += 1;
        let flashes = sim.run(1);
        if flashes >= octopus_count as u64 {
            break;
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

    #[test]
    fn test_day11_solve_part1() {
        assert_eq!(solve_part1(RAW_INPUT), 1656);
    }

    #[test]
    fn test_day11_solve_part2() {
        assert_eq!(solve_part2(RAW_INPUT), 195);
    }
}
