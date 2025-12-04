use std::ops::Index;

pub const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub struct Grid<T> {
    rows: usize,
    collums: usize,
    inner: Vec<T>,
}

impl<T> Grid<T> {
    pub fn get<'a>(&'a self, x: usize, y: usize) -> Option<&'a T> {
        if y >= self.rows || x >= self.rows {
            None
        } else {
            Some(&self[(x, y)])
        }
    }

    pub fn from_lines(data_in: &str, x: fn(u8)->T, ) -> Self {
        let collums = data_in.bytes().position(|b| b==b'\n').unwrap();
        let inner = data_in.bytes().filter(|b| *b==b'\n').map(x).collect::<Vec<T>>();
        let rows = inner.len()/collums;

        Self {
            rows,
            collums,
            inner,
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.inner[y * self.collums + x]
    }
}
