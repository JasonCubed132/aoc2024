use anyhow::{anyhow, Result};

#[derive(Clone)]
pub struct Cell<T> {
    inner: T,
}

impl<T> Cell<T> {
    pub fn new(item: T) -> Self {
        Self { inner: item }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Coord {
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize, max_x: usize, max_y: usize) -> Self {
        Self { x, y, max_x, max_y }
    }
    pub fn add_delta(self, x_delta: i32, y_delta: i32) -> Result<Self> {
        let new_x = self.x as i32 + x_delta;
        let new_y = self.y as i32 + y_delta;

        if new_x > self.max_x as i32 || new_y > self.max_y as i32 {
            Err(anyhow!("New coords out of bounds"))
        } else {
            Ok(Self {
                x: new_x as usize,
                y: new_y as usize,
                max_x: self.max_x,
                max_y: self.max_y,
            })
        }
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    grid: Vec<Vec<Cell<T>>>,
}

impl<T: Clone + PartialEq> Grid<T> {
    pub fn new(grid: Vec<Vec<Cell<T>>>) -> Self {
        Self { grid: grid }
    }

    pub fn set_cell_contents(&mut self, pos: &Coord, inner: T) -> Result<()> {
        if pos.y >= self.grid.len() {
            return Err(anyhow!("{:?} exceeds number of rows", pos.y));
        }

        let row = &self.grid[pos.y];

        if pos.x >= row.len() {
            return Err(anyhow!("{:?} exceeds the number of columns", pos.x));
        }

        self.grid[pos.y][pos.x] = Cell::new(inner);

        Ok(())
    }

    pub fn get_cell_contents(&self, pos: &Coord) -> Result<T> {
        if pos.y >= self.grid.len() {
            return Err(anyhow!("{:?} exceeds number of rows", pos.y));
        }

        let row = &self.grid[pos.y];

        if pos.x >= row.len() {
            return Err(anyhow!("{:?} exceeds the number of columns", pos.x));
        }

        Ok(row[pos.x].inner.clone())
    }

    pub fn get_projection_iter(
        &self,
        start: Coord,
        x_delta: i32,
        y_delta: i32,
    ) -> impl Iterator<Item = (Coord, T)> {
        let mut projection = Vec::new();
        let mut current = start;

        loop {
            match self.get_cell_contents(&current) {
                Ok(contents) => projection.push((current.clone(), contents.clone())),
                Err(_) => break,
            }

            match current.add_delta(x_delta, y_delta) {
                Ok(res) => {
                    current = res;
                }
                Err(_) => {
                    break;
                }
            }
        }

        projection.into_iter()
    }

    pub fn find_first(&self, cell_contents: T) -> Result<Option<Coord>> {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                let coord = Coord::new(x, y, self.grid[y].len(), self.grid.len());
                if self.get_cell_contents(&coord)? == cell_contents {
                    return Ok(Some(coord));
                }
            }
        }
        Ok(None)
    }
}
