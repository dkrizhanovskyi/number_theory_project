/// This module handles predictions using machine learning algorithms.
/// We will start with simple linear regression and later integrate more complex models.

use ndarray::Array1;

/// Simple Linear Regression implementation
pub struct LinearRegression {
    pub weights: Option<f64>,
    pub bias: Option<f64>,
}

impl LinearRegression {
    /// Creates a new LinearRegression model
    pub fn new() -> Self {
        LinearRegression {
            weights: None,
            bias: None,
        }
    }

    /// Trains the linear regression model using the input features and target values
    pub fn fit(&mut self, x_train: &Array1<f64>, y_train: &Array1<f64>) {
        let x_mean = x_train.mean().unwrap();
        let y_mean = y_train.mean().unwrap();

        let numerator: f64 = x_train.iter().zip(y_train.iter()).map(|(x, y)| (x - x_mean) * (y - y_mean)).sum();
        let denominator: f64 = x_train.iter().map(|x| (x - x_mean).powi(2)).sum();

        self.weights = Some(numerator / denominator);
        self.bias = Some(y_mean - self.weights.unwrap() * x_mean);
    }

    /// Predicts the target values for the input features
    pub fn predict(&self, x_test: &Array1<f64>) -> Array1<f64> {
        x_test.iter().map(|x| self.weights.unwrap() * x + self.bias.unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr1;

    #[test]
    fn test_linear_regression_fit() {
        let x_train = arr1(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        let y_train = arr1(&[2.0, 4.0, 6.0, 8.0, 10.0]);

        let mut model = LinearRegression::new();
        model.fit(&x_train, &y_train);

        assert_eq!(model.weights.unwrap(), 2.0);
        assert_eq!(model.bias.unwrap(), 0.0);
    }

    #[test]
    fn test_linear_regression_predict() {
        let x_train = arr1(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        let y_train = arr1(&[2.0, 4.0, 6.0, 8.0, 10.0]);

        let mut model = LinearRegression::new();
        model.fit(&x_train, &y_train);

        let x_test = arr1(&[6.0, 7.0]);
        let predictions = model.predict(&x_test);

        assert_eq!(predictions, arr1(&[12.0, 14.0]));
    }
}
