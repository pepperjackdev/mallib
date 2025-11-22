use std::{ops::{Index, IndexMut}, vec};

type Shape = (usize, usize);

pub struct Matrix<T> {
    shape: Shape,
    data: Vec<Vec<T>>,
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new((rows, cols): Shape) -> Matrix<T> {
        Matrix {
            shape: (rows, cols),
            data: vec![vec![T::default(); cols]; rows],
        }
    }

    pub fn from<const ROWS: usize, const COLS: usize>(matrix: [[T; COLS]; ROWS]) -> Matrix<T> {
        Matrix {
            shape: (ROWS, COLS),
            data: matrix.iter().map(|row| row.to_vec()).collect(),
        }
    }

    pub fn get_shape(&self) -> &Shape {
        &self.shape
    }

    pub fn get_data(&self) -> &Vec<Vec<T>> {
        &self.data
    }
}
