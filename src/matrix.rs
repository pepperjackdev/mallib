use std::vec;

type Shape = (usize, usize);

pub struct Matrix<T> {
    shape: Shape,
    data: Vec<Vec<T>>
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new(shape: Shape) -> Matrix<T> {
        Matrix {
            shape,
            data: vec![vec![T::default(); shape.1]; shape.0],
        }
    }

    pub fn from<const ROWS: usize, const COLS: usize>(matrix: &[[T; COLS]; ROWS]) -> Matrix<T> {
        Matrix {
            shape: (ROWS, COLS),
            data: matrix.iter()
                .map(|row| row.to_vec())
                .collect()
        }
    }

    pub fn get_shape<'a>(&'a self) -> &'a Shape {
        &self.shape
    }

    pub fn get_data<'a>(&'a self) -> &'a Vec<Vec<T>> {
        &self.data
    }
}

#[cfg(test)]
pub mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn test_matrix_from() {
        let matrix = Matrix::from(&[
            [1, 2, 3],
            [4, 5, 6]
        ]);

        assert_eq!(matrix.get_shape(), &(2, 3));
        assert_eq!(matrix.get_data(), &vec![vec![1, 2, 3], vec![4, 5, 6]])
    }
}