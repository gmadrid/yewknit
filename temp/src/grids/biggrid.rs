/*
 Biggrid implements the Grid API backed by a giant Vec.
 The Vec is as large as the largest supported Grid.
 This is so we can change the size of the grid without losing
 information if someone accidentally makes the Grid too small.
*/

use super::gridtrait::GridTrait;
use super::{MAX_GRID_HEIGHT, MAX_GRID_WIDTH};

pub struct BigGrid<T> {
    cells: [T; MAX_GRID_WIDTH * MAX_GRID_HEIGHT],
    height: usize,
    width: usize,
}

impl<T> BigGrid<T>
where
    T: Copy + Default,
{
    pub fn new(height: usize, width: usize) -> BigGrid<T> {
        BigGrid {
            cells: [T::default(); MAX_GRID_WIDTH * MAX_GRID_HEIGHT],
            height,
            width,
        }
    }

    fn coords_to_index(&self, row: usize, col: usize) -> usize {
        row * MAX_GRID_WIDTH + col
    }
}

impl<T> GridTrait<T> for BigGrid<T>
where
    T: Copy + Default,
{
    fn num_rows(&self) -> usize {
        self.height
    }

    fn num_cols(&self) -> usize {
        self.width
    }

    fn cell(&self, row: usize, col: usize) -> T {
        let index = self.coords_to_index(row, col);
        self.cells[index]
    }

    fn set_cell(&mut self, row: usize, col: usize, value: T) {
        let index = self.coords_to_index(row, col);
        self.cells[index] = value
    }

    fn clear(&mut self) {
        for index in 0..self.cells.len() {
            self.cells[index] = T::default();
        }
    }
}
