use ndarray::{Array1, Array2};
use ndarray_linalg::Inverse;


pub struct LinearRegression {
    parameters: Array1<f64>
}

impl LinearRegression {
    pub fn fit(features: &Array2<f64>, targets: &Array1<f64>) -> LinearRegression {
        LinearRegression {
            parameters: (features.t().dot(features)).inv().expect("Should be invertible")
                .dot(&features.t())
                .dot(targets)
        }
    }

    pub fn predict(&self, features: &Array2<f64>) -> Array1<f64> {
        features.dot(&self.parameters)
    }
}

#[cfg(test)]
pub mod tests {
    use approx::assert_abs_diff_eq;
    use ndarray::{arr1, arr2};

    use crate::models::linear::LinearRegression;

    #[test]
    pub fn test_linear_regression() {
        let features = arr2(&[
            [1.0, 1.0],
            [1.0, 2.0],
            [1.0, 3.0]
        ]);

        let targets = arr1(&[
            1.0,
            3.0,
            5.0
        ]);

        let linear_regression = LinearRegression::fit(&features, &targets);

        assert_abs_diff_eq!(linear_regression.parameters, arr1(&[-1.0, 2.0]), epsilon = 1e-10)
    }
}