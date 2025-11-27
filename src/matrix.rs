use std::ops::{Index, IndexMut};

pub struct Matrix<T> {
    shape: (usize, usize),
    data: Vec<Vec<T>>,
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new((rows, cols): (usize, usize)) -> Matrix<T> {
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

    pub fn get_shape(&self) -> &(usize, usize) {
        &self.shape
    }
}

impl<T: Default + Clone> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row][col]
    }
}

impl<T: Default + Clone> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row][col]
    }
}


#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;


    #[test]
    fn test_matrix_new() {
        let under_test: Matrix<f32> = Matrix::new((3, 3));
        assert_eq!(under_test.get_shape(), &(3, 3));
    }

    #[test]
    fn test_matrix_from() {
        let under_test: Matrix<f32> = Matrix::from([
            [0.0, 1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0, 9.0]
        ]);
        
        assert_eq!(under_test.get_shape(), &(2, 5))
    }

    #[test]
    fn test_matrix_index() {
        let under_test: Matrix<f32> = Matrix::from([
            [0.0, 1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0, 9.0]
        ]);

        assert_eq!(under_test[(0, 0)], 0.0);
        assert_eq!(under_test[(1, 4)], 9.0);
    }

    #[test]
    fn test_matrix_index_mut() {
        let mut under_test: Matrix<f32> = Matrix::from([
            [0.0, 1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0, 9.0]
        ]);

        under_test[(0, 0)] = 5.0;
        under_test[(1, 4)] = 8.0;

        assert_eq!(under_test[(0, 0)], 5.0);
        assert_eq!(under_test[(1, 4)], 8.0);
    }
}