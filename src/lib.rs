use ndarray::{Array1, Array2};

pub struct LinearRegression {
    weights: Array1<f64>
}

impl LinearRegression {
    pub fn fit(features: &Array2<f64>, targets: &Array1<f64>) -> LinearRegression {
        todo!["To implement"];
    }

    pub fn predict(&self, features: &Array2<f64>) -> Array1<f64> {
        features.dot(&self.weights)
    }
}