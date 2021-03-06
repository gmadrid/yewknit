use super::{Color, GridTrait};

pub struct TiledGrid<'a, G>
where
    G: GridTrait,
{
    grid: &'a G,
    repeat_x: u8,
    repeat_y: u8,
}

impl<'a, G> TiledGrid<'a, G>
where
    G: GridTrait,
{
    pub fn new(grid: &'a G) -> TiledGrid<'a, G> {
        TiledGrid {
            grid,
            repeat_x: 3,
            repeat_y: 3,
        }
    }

    fn to_base(&self, row: usize, col: usize) -> (usize, usize) {
        (row % self.grid.num_rows(), col % self.grid.num_cols())
    }
}

impl<'a, G> GridTrait for TiledGrid<'a, G>
where
    G: GridTrait,
{
    fn num_rows(&self) -> usize {
        self.repeat_x as usize * self.grid.num_rows()
    }

    fn num_cols(&self) -> usize {
        self.repeat_y as usize * self.grid.num_cols()
    }

    fn cell(&self, row: usize, col: usize) -> Color {
        let (base_row, base_col) = self.to_base(row, col);
        self.grid.cell(base_row, base_col)
    }

    fn set_cell(&mut self, _: usize, _: usize, _: Color) {
        unimplemented!("TiledGrid is not mutable")
    }

    fn clear(&mut self) {
        unimplemented!("TiledGrid is not mutable")
    }
}
