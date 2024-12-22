

#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<Option<T>>,
}

impl From<&str> for Grid<char> {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let first_line = lines.next().unwrap();
        let width = first_line.len();
        let height = input.lines().count();
        let mut grid = Grid::new(width, height);
        let mut cell_values = vec![None; width * height];
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                cell_values[i * width + j] = Some(c);
            }
        }
        grid.cells = cell_values;
        grid
    }
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

    pub fn get_cells(&self) -> Vec<Option<T>> {
        self.cells.clone()
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    /**
    * Returns the values of the cells adjacent to the cell at (x, y) in the order:
    * left, right, up, down
    */
    pub fn get_adjacent(&self, row: usize, col: usize) -> (Option<T>, Option<T>, Option<T>, Option<T>) {
        let mut adjacent = (None, None, None, None);
        if col > 0 {
            adjacent.0 = self.get(row, col - 1);
        }
        if col < self.width - 1 {
            adjacent.1 = self.get(row, col + 1);
        }
        if row > 0 {
            adjacent.2 = self.get(row - 1, col);
        }
        if row < self.height - 1 {
            adjacent.3 = self.get(row + 1, col);
        }
        adjacent
    }

    pub fn get_left(&self, row: usize, col: usize) -> Option<T> {
        if col > 0 {
            self.get(row, col - 1)
        } else {
            None
        }
    }

    pub fn get_right(&self, row: usize, col: usize) -> Option<T> {
        if col < self.width - 1 {
            self.get(row, col + 1)
        } else {
            None
        }
    }

    pub fn get_top(&self, row: usize, col: usize) -> Option<T> {
        if row > 0 {
            self.get(row - 1, col)
        } else {
            None
        }
    }

    pub fn get_bottom(&self, row: usize, col: usize) -> Option<T> {
        if row < self.height - 1 {
            self.get(row + 1, col)
        } else {
            None
        }
    }

    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}