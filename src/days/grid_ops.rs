use std::ops;

use anyhow::{anyhow, Result};

#[derive(Clone)]
pub struct Cell<T> {
    inner: T,
}

impl<T> Cell<T> {
    pub fn new(item: T) -> Self {
        Self { inner: item }
    }

    pub fn get(&self) -> &T {
        &self.inner
    }
}

#[derive(Clone, Debug)]
pub struct Delta {
    x: i32,
    y: i32,
}

impl Delta {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}

impl ops::Neg for Delta {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.get_x(),
            y: -self.get_y(),
        }
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

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn add_delta(self, delta: &Delta) -> Result<Self> {
        let new_x = self.x as i32 + delta.get_x();
        let new_y = self.y as i32 + delta.get_y();

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

    pub fn get_delta(&self, other: &Self) -> Delta {
        Delta::new(
            other.get_x() as i32 - self.get_x() as i32,
            other.get_y() as i32 - self.get_y() as i32,
        )
    }
}

impl ops::Add<Delta> for Coord {
    type Output = Self;

    fn add(self, rhs: Delta) -> Self::Output {
        self.add_delta(&rhs).unwrap()
    }
}

impl ops::Sub<Delta> for Coord {
    type Output = Self;

    fn sub(self, rhs: Delta) -> Self::Output {
        self + (-rhs)
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    grid: Vec<Vec<Cell<T>>>,
    num_rows: usize,
    num_cols: usize,
}

impl<T: Clone + PartialEq> Grid<T> {
    pub fn new(grid: Vec<Vec<Cell<T>>>) -> Result<Self> {
        let num_rows = grid.len();

        if num_rows == 0 {
            return Err(anyhow!("Cannot make grid with no rows"));
        }

        let num_cols = grid[0].len();

        if num_cols == 0 {
            return Err(anyhow!("Cannot make grid with no columns"));
        }

        for row in &grid {
            if row.len() != num_cols {
                return Err(anyhow!("Cannot make grid with different length rows"));
            }
        }

        Ok(Self {
            grid: grid,
            num_rows: num_rows,
            num_cols: num_cols,
        })
    }

    pub fn set_cell_contents(&mut self, pos: &Coord, inner: T) -> Result<()> {
        if pos.max_x != self.num_cols || pos.max_y != self.num_rows {
            return Err(anyhow!("Max x or y of pos does not match grid size"));
        }

        self.grid[pos.y][pos.x] = Cell::new(inner);

        Ok(())
    }

    pub fn get_cell_contents(&self, pos: &Coord) -> Result<T> {
        if pos.max_x != self.num_cols || pos.max_y != self.num_rows {
            return Err(anyhow!("Max x or y of pos does not match grid size"));
        }

        let row = &self.grid[pos.y];

        Ok(row[pos.x].inner.clone())
    }

    pub fn get_projection_iter(
        &self,
        start: &Coord,
        delta: &Delta,
    ) -> impl Iterator<Item = (Coord, T)> {
        let mut projection = Vec::new();
        let mut current = start.clone();

        loop {
            match self.get_cell_contents(&current) {
                Ok(contents) => projection.push((current.clone(), contents.clone())),
                Err(_) => break,
            }

            match current.add_delta(delta) {
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
        for y in 0..self.num_rows {
            for x in 0..self.num_cols {
                let coord = Coord::new(x, y, self.num_cols, self.num_rows);
                if self.get_cell_contents(&coord)? == cell_contents {
                    return Ok(Some(coord));
                }
            }
        }
        Ok(None)
    }

    pub fn get_grid(&self) -> Vec<Vec<&T>> {
        self.grid
            .iter()
            .map(|row| row.iter().map(|cell| cell.get()).collect())
            .collect()
    }

    pub fn get_num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn get_num_cols(&self) -> usize {
        self.num_cols
    }
}
