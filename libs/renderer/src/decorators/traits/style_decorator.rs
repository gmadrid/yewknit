use grids::Color;

pub trait StyleDecorator {
    fn cell_style(&self, _row: usize, _col: usize, _contents: Color) -> Option<Vec<String>> {
        None
    }
}
