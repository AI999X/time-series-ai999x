//! Time Series: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p time-series-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&time_series_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "ar1_forecast", "inputs": {"x": "58", "phi": "0.7", "mu": "50", "h": "3"}}));
    println!();
    show(json!({"key": "forecast_error_variance", "inputs": {"phi": [0.5], "theta": [0.3], "sigma2": "2", "h": "3"}}));
    println!();
    show(json!({"key": "bartlett_standard_error", "inputs": {"x": [4, 6, 5, 8, 7, 9, 8, 10, 9, 12], "k": "3"}}));
    println!();
    show(json!({"key": "cusum_change_point", "inputs": {"x": [5, 6, 5, 7, 6, 12, 13, 12, 14]}}));
    println!();
    show(json!({"key": "seasonal_indices_additive", "inputs": {"x": [30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51], "m": "4"}}));
    println!();
    show(json!({"key": "cross_correlation_best_lag", "inputs": {"x": [1, 3, 2, 5, 4, 6, 5, 8], "y": [3, 2, 5, 4, 6, 5, 8, 7], "K": "3"}}));
    println!();
    show(json!({"key": "ses_best_alpha", "inputs": {"alphas": [0.1, 0.3, 0.5], "x": [20, 22, 21, 24, 23, 26]}}));
    println!();
    show(json!({"key": "mean_absolute_error", "inputs": {"y": [10, 12, 14, 13], "f": [11, 12, 13, 15]}}));
    println!();
    show(json!({"key": "exponential_interpolation", "inputs": {"t": [1, 2, 3, 4], "y": [100, 120, 150, 170], "t0": "2.5"}}));
    println!();
    show(json!({"key": "kalman_filter_constant_velocity", "inputs": {"x": [1.1, 1.9, 3.2, 4, 4.8], "dt": "1", "q": "0.1", "r": "1", "p0": "0", "v0": "1", "var0": "10"}}));
    println!();
    show(json!({"key": "centred_moving_average", "inputs": {"x": [12, 15, 14, 18, 20, 19, 23, 25], "k": "4"}}));
    println!();
    show(json!({"key": "augmented_dickey_fuller_test", "inputs": {"x": [10.2, 10.8, 10.5, 11.3, 11, 11.9, 11.6, 12.4, 12, 12.9, 12.5, 13.4], "p": "2", "regression": "c"}}));
    println!();
    show(json!({"key": "compound_average_growth_rate", "inputs": {"x": [100, 110, 125, 133, 150]}}));
    println!();
    show(json!({"key": "exponential_trend_forecast", "inputs": {"x": [100, 112, 124, 140, 155], "h": "1"}}));
    println!();
    show(json!({"key": "ar_least_squares", "inputs": {"x": [4, 6, 5, 8, 7, 9, 8, 10, 9, 12], "p": "1"}}));
}
