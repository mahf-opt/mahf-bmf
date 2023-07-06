//! Utility methods.

/// Scales a value from within \[-1, 1\] to the input domain \[min, max\].
pub fn input_domain(value: &f64, min: f64, max: f64) -> f64 {
    (value + 1.0) / 2.0 * (max - min) + min
}

/// Scales a value from within the input domain \[min, max\] to \[-1, 1\].
pub fn normalized_domain(value: &f64, min: f64, max: f64) -> f64 {
    2.0 * (value - min) / (max - min) - 1.0
}
