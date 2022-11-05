use std::fmt::{Display, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Dead,
    Alive,
}

pub struct Grid {
    rows: usize,
    cols: usize,
    cells: Vec<CellState>,
    cache: Vec<CellState>,
}

type Width = usize;
type Height = usize;

impl Grid {
    pub fn new(height: Height, width: Width) -> Self {
        let cells = vec![CellState::Dead; height * width];
        let cache = cells.clone();
        Self {
            rows: height,
            cols: width,
            cells,
            cache,
        }
    }

    pub fn set_alive(&mut self, row: usize, col: usize) {
        let cell_idx = self.cell_index(row, col);
        self.cells[cell_idx] = CellState::Alive;
    }

    pub fn iterate(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell_idx = self.cell_index(row, col);
                let nghb_cnt = self.count_neighbors(row, col);

                let next_state = match (self.cells[cell_idx], nghb_cnt) {
                    (CellState::Alive, c) if c < 2 => CellState::Dead,
                    (CellState::Alive, c) if c == 2 || c == 3 => CellState::Alive,
                    (CellState::Alive, c) if c > 3 => CellState::Dead,
                    (CellState::Dead, c) if c == 3 => CellState::Alive,
                    (state, _) => state,
                };

                self.cache[cell_idx] = next_state;
            }
        }

        std::mem::swap(&mut self.cells, &mut self.cache);
    }

    fn cell_index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn count_neighbors(&self, row: usize, col: usize) -> usize {
        let mut count = 0;

        for delta_row in [self.cols - 1, 0, 1] {
            for delta_col in [self.rows - 1, 0, 1] {
                if delta_col == 0 && delta_row == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.rows;
                let neighbor_col = (col + delta_col) % self.cols;

                let neighbor_idx = self.cell_index(neighbor_row, neighbor_col);

                count += self.cells[neighbor_idx] as usize;
            }
        }

        count
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.cells.chunks(self.cols) {
            for cell in row {
                let symbol = match cell {
                    CellState::Dead => ' ',
                    CellState::Alive => '◼',
                };
                f.write_char(symbol)?;
            }
            f.write_char('\r')?;
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_index() {
        let grid = Grid::new(6, 6);

        assert_eq!(grid.cell_index(0, 4), 4);
        assert_eq!(grid.cell_index(2, 2), 14);
        assert_eq!(grid.cell_index(5, 5), 35);
    }

    #[test]
    fn test_count_neighbors() {
        let mut grid = Grid::new(5, 5);
        grid.set_alive(1, 2);
        grid.set_alive(2, 3);
        grid.set_alive(3, 0);
        grid.set_alive(3, 1);
        grid.set_alive(3, 2);
        grid.set_alive(3, 3);

        assert_eq!(grid.count_neighbors(1, 1), 1);
        assert_eq!(grid.count_neighbors(2, 2), 5);
        assert_eq!(grid.count_neighbors(3, 4), 3);
    }

    #[test]
    fn test_iterate() {
        let mut grid = Grid::new(5, 5);
        grid.set_alive(1, 2);
        grid.set_alive(2, 3);
        grid.set_alive(3, 1);
        grid.set_alive(3, 2);
        grid.set_alive(3, 3);

        grid.iterate();

        let mut expected = Grid::new(5, 5);
        expected.set_alive(2, 1);
        expected.set_alive(2, 3);
        expected.set_alive(3, 2);
        expected.set_alive(3, 3);
        expected.set_alive(4, 2);

        assert_eq!(grid.cells, expected.cells);
    }
}
