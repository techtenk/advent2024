use std::ops::Index;

#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<Option<T>>,
}

impl<T> Grid<T> where T: Clone + Copy {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![None; width * height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.cells[y * self.width + x] = Some(value);
    }

    pub fn get(&self, row: usize, col: usize) -> Option<T> {
        self.cells[row * self.width + col]
    }

    pub fn get_cells(&self) -> &Vec<Option<T>> {
        &self.cells
    }

    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}