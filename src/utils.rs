#[derive(Debug, PartialEq, Clone)]
pub struct Grid<T> {
    pub cells: Vec<T>,
    width: usize,
    height: usize,
}

impl<'a, T: Clone> Grid<T> {
    pub fn new(cells: &[T], width: usize) -> Self {
        let len = cells.len();
        let cells = cells.to_owned();
        Self {
            cells,
            width,
            height: len / width,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn cell_at(&self, x: i32, y: i32) -> Option<T> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }

        let i = self.index_for(x as usize, y as usize);
        Some(self.cells[i as usize].clone())
    }

    pub fn set_at(&mut self, x: usize, y: usize, value: T) {
        if x >= self.width || y >= self.height {
            panic!("Trying to set unexisting coordinates: ({}, {})", x, y);
        }

        let i = self.index_for(x, y);
        self.cells[i] = value;
    }

    pub fn neighbors_at(&self, x: i32, y: i32) -> Vec<(T, (usize, usize))> {
        [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
            .iter()
            .filter_map(|point| {
                self.cell_at(point.0, point.1)
                    .map(|cell| (cell, (point.0 as usize, point.1 as usize)))
            })
            .collect()
    }

    pub fn neighbors8_at(&self, x: i32, y: i32) -> Vec<(T, (usize, usize))> {
        [
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y),
            (x - 1, y - 1),
        ]
        .iter()
        .filter_map(|point| {
            self.cell_at(point.0, point.1)
                .map(|cell| (cell, (point.0 as usize, point.1 as usize)))
        })
        .collect()
    }

    fn index_for(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_constructor() {
        let input = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(
            Grid::<u32>::new(&input, 2),
            Grid {
                cells: vec![1, 2, 3, 4, 5, 6],
                width: 2,
                height: 3
            }
        );
    }

    #[test]
    fn test_grid_size() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let grid = Grid::<u32>::new(&input, 2);

        assert_eq!(grid.size(), (2, 3))
    }

    #[test]
    fn test_grid_cell_at() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let grid = Grid::<u32>::new(&input, 2);

        assert_eq!(grid.cell_at(-1, 0), None);
        assert_eq!(grid.cell_at(1, -1), None);
        assert_eq!(grid.cell_at(0, 3), None);
        assert_eq!(grid.cell_at(2, 0), None);
        assert_eq!(grid.cell_at(0, 0), Some(1));
        assert_eq!(grid.cell_at(1, 1), Some(4));
        assert_eq!(grid.cell_at(1, 2), Some(6));
    }

    #[test]
    fn test_grid_neighbors_at() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let grid = Grid::<u32>::new(&input, 3);
        assert_eq!(grid.neighbors_at(0, 0), vec![(2, (1, 0)), (4, (0, 1))]);
        assert_eq!(
            grid.neighbors_at(1, 0),
            vec![(3, (2, 0)), (5, (1, 1)), (1, (0, 0))]
        );
        assert_eq!(
            grid.neighbors_at(1, 1),
            vec![(2, (1, 0)), (6, (2, 1)), (8, (1, 2)), (4, (0, 1))]
        );
    }
}
