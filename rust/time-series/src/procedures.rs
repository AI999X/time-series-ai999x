//! time_series: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/time_series/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "ar1_forecast",
    "ar1_properties",
    "ar2_stationarity",
    "ar2_theoretical_acf",
    "ar_forecast",
    "ar_least_squares",
    "ar_yule_walker",
    "arima_p10_forecast",
    "arma11_theoretical_acf",
    "arma_theoretical_acf",
    "arma_theoretical_pacf",
    "arma_variance",
    "augmented_dickey_fuller_test",
    "bartlett_standard_error",
    "box_pierce_test",
    "centred_moving_average",
    "change_point_mean_shift",
    "compound_average_growth_rate",
    "cross_correlation_at_lag",
    "cross_correlation_best_lag",
    "cusum_change_point",
    "dickey_fuller_test",
    "differencing",
    "drift_forecast",
    "exponential_interpolation",
    "exponential_moving_average",
    "exponential_smoothing_update",
    "exponential_trend_forecast",
    "forecast_error_variance",
    "holt_damped_trend",
    "holt_linear_trend",
    "holt_update",
    "holt_winters_additive",
    "holt_winters_multiplicative",
    "kalman_filter_constant_velocity",
    "kalman_filter_local_level",
    "kalman_steady_state_constant_velocity",
    "kalman_steady_state_gain",
    "kalman_step",
    "kalman_update",
    "kpss_test",
    "linear_interpolation",
    "linear_trend_forecast",
    "linearly_weighted_moving_average",
    "ljung_box_test",
    "ma1_theoretical_pacf",
    "ma2_invertibility",
    "ma_theoretical_acf",
    "mann_kendall_test",
    "mean_absolute_error",
    "mean_absolute_percentage_error",
    "mean_absolute_scaled_error",
    "mean_forecast_error",
    "mean_squared_error",
    "partial_autocorrelations_from_autocorrelations",
    "penalised_change_points",
    "pi_weights",
    "psi_weights",
    "quadratic_trend_forecast",
    "root_mean_squared_error",
    "rts_smoother_local_level",
    "sample_autocorrelation",
    "sample_autocovariance",
    "sample_partial_autocorrelation",
    "sarima_forecast",
    "seasonal_indices_additive",
    "seasonal_indices_multiplicative",
    "seasonally_adjusted_additive",
    "seasonally_adjusted_multiplicative",
    "sens_slope",
    "ses_best_alpha",
    "simple_exponential_smoothing",
    "simple_moving_average",
    "symmetric_mape",
    "tabular_cusum",
    "trend_seasonal_regression",
    "two_change_points",
    "yule_walker_from_autocorrelations",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "ar1_forecast" => {
            let r = f_run_ar1_fc(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } }, match v.get("sigma2") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_26(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ar1_properties" => {
            let r = f_run_ar1_props(match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_72(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ar2_stationarity" => {
            let r = f_run_ar2_stat(match v.get("phi1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_105(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ar2_theoretical_acf" => {
            let r = f_run_ar2_acf(match v.get("phi1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(5i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_156(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ar_forecast" => {
            let r = f_run_ar_forecast(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_205(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ar_least_squares" => {
            let r = f_run_ar_ols(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_293(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ar_yule_walker" => {
            let r = f_run_ar_yw(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_355(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "arima_p10_forecast" => {
            let r = f_run_arima_forecast(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_411(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "arma11_theoretical_acf" => {
            let r = f_run_arma11_acf(match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(5i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_457(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "arma_theoretical_acf" => {
            let r = f_run_arma_acf(match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(5i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_546(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "arma_theoretical_pacf" => {
            let r = f_run_arma_pacf(match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(5i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_642(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "arma_variance" => {
            let r = f_run_arma_var(match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_719(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "augmented_dickey_fuller_test" => {
            let r = s_744(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("regression") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("c")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_822(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bartlett_standard_error" => {
            let r = f_run_bartlett(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_877(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "box_pierce_test" => {
            let r = s_902(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("model_df") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(0i64)) })().ok()? } }, (|| -> R { Ok(Py::Bool(false)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_951(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "centred_moving_average" => {
            let r = f_run_cma(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1005(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "change_point_mean_shift" => {
            let r = f_run_cp1(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("min_size") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(2i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1058(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "compound_average_growth_rate" => {
            let r = f_run_avg_growth(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1091(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cross_correlation_at_lag" => {
            let r = f_run_ccf(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1149(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cross_correlation_best_lag" => {
            let r = f_run_best_lag(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1213(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cusum_change_point" => {
            let r = f_run_cusum_cp(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1250(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dickey_fuller_test" => {
            let r = s_1275(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("regression") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("c")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1353(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "differencing" => {
            let r = f_run_diff(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } }, match v.get("D") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(0i64)) })().ok()? } }, match v.get("s") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1406(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "drift_forecast" => {
            let r = f_run_drift(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1451(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "exponential_interpolation" => {
            let r = f_run_exp_interp(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1500(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "exponential_moving_average" => {
            let r = f_run_ema(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1545(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "exponential_smoothing_update" => {
            let r = f_run_ses_update(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1576(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "exponential_trend_forecast" => {
            let r = f_run_exp_trend(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1667(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "forecast_error_variance" => {
            let r = f_run_fev(match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1735(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "holt_damped_trend" => {
            let r = f_run_holt_damped(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("b0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1793(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "holt_linear_trend" => {
            let r = f_run_holt(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("b0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1851(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "holt_update" => {
            let r = f_run_holt_update(match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1898(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "holt_winters_additive" => {
            let r = s_1923(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } }, (|| -> R { Ok(Py::Bool(false)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1961(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "holt_winters_multiplicative" => {
            let r = s_1986(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } }, (|| -> R { Ok(Py::Bool(true)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2024(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kalman_filter_constant_velocity" => {
            let r = f_run_kf_cv(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("var0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2068(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kalman_filter_local_level" => {
            let r = f_run_kf_ll(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2111(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kalman_steady_state_constant_velocity" => {
            let r = f_run_kf_cv_ss(match v.get("dt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2152(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kalman_steady_state_gain" => {
            let r = f_run_kf_ss(match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2182(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kalman_step" => {
            let r = f_run_kf_step(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2220(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kalman_update" => {
            let r = f_run_kf_update(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2251(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kpss_test" => {
            let r = s_2276(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("nlags") { Some(x) => py::from_val(x), None => { return None } }, match v.get("regression") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("c")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2356(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_interpolation" => {
            let r = f_run_lin_interp(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2402(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_trend_forecast" => {
            let r = f_run_linear_trend(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2493(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linearly_weighted_moving_average" => {
            let r = f_run_lwma(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2540(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ljung_box_test" => {
            let r = s_2565(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("model_df") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(0i64)) })().ok()? } }, (|| -> R { Ok(Py::Bool(true)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2614(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ma1_theoretical_pacf" => {
            let r = f_run_ma1_pacf(match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(5i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2659(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ma2_invertibility" => {
            let r = f_run_ma2_inv(match v.get("theta1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2692(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ma_theoretical_acf" => {
            let r = f_run_ma_acf(match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2783(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mann_kendall_test" => {
            let r = f_run_mk(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2827(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mean_absolute_error" => {
            let r = f_run_mae(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2862(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mean_absolute_percentage_error" => {
            let r = f_run_mape(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2901(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mean_absolute_scaled_error" => {
            let r = f_run_mase(match v.get("y_train") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sp") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2956(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mean_forecast_error" => {
            let r = f_run_me(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2990(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mean_squared_error" => {
            let r = f_run_mse(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3025(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "partial_autocorrelations_from_autocorrelations" => {
            let r = f_run_lev_pacf(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3060(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "penalised_change_points" => {
            let r = f_run_pelt(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("penalty") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3108(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pi_weights" => {
            let r = f_run_pi(match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(10i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3172(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "psi_weights" => {
            let r = f_run_psi(match v.get("phi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(10i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3235(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "quadratic_trend_forecast" => {
            let r = f_run_quad_trend(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3322(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "root_mean_squared_error" => {
            let r = s_3347(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3358(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rts_smoother_local_level" => {
            let r = f_run_rts(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3405(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sample_autocorrelation" => {
            let r = f_run_acf(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3459(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sample_autocovariance" => {
            let r = f_run_acov(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3507(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sample_partial_autocorrelation" => {
            let r = f_run_pacf(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3569(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sarima_forecast" => {
            let r = f_run_sarima_forecast(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("phi") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("theta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("residuals") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("d") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(0i64)) })().ok()? } }, match v.get("seasonal_phi") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("seasonal_theta") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("D") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(0i64)) })().ok()? } }, match v.get("m") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3671(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "seasonal_indices_additive" => {
            let r = s_3696(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, (|| -> R { Ok(Py::Bool(false)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3732(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "seasonal_indices_multiplicative" => {
            let r = s_3757(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, (|| -> R { Ok(Py::Bool(true)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3793(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "seasonally_adjusted_additive" => {
            let r = s_3818(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, (|| -> R { Ok(Py::Bool(false)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3856(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "seasonally_adjusted_multiplicative" => {
            let r = s_3881(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, (|| -> R { Ok(Py::Bool(true)) })().ok()?);
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3919(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sens_slope" => {
            let r = f_run_sen(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3951(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ses_best_alpha" => {
            let r = f_run_ses_best(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alphas") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3995(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simple_exponential_smoothing" => {
            let r = f_run_ses(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l0") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4033(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simple_moving_average" => {
            let r = f_run_sma(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4079(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "symmetric_mape" => {
            let r = f_run_smape(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4119(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "tabular_cusum" => {
            let r = f_run_cusum_tab(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4174(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "trend_seasonal_regression" => {
            let r = f_run_ts_reg(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4263(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "two_change_points" => {
            let r = f_run_cp2(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("min_size") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(2i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4319(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "yule_walker_from_autocorrelations" => {
            let r = f_run_yw_acf(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4354(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c__Z975() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.959963984540054)) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__acf_in(mut v_x: Py, mut v_k: Py, mut v_lo: Py) -> R {
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(3i64), Py::str("the series"))?;
    let _ = f__nonconstant(v_x.clone(), Py::str("the series"))?;
    v_k = f__int(v_k.clone(), Py::str("the lag k"), v_lo.clone(), py::sub(&py::len(&v_x.clone())?, &Py::Int(1i64))?)?;
    return Ok(Py::tuple(vec![v_x.clone(), v_k.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__acf_list(mut v_x: Py, mut v_kmax: Py) -> R {
    let mut v_g0: Py = Py::None;
    let mut v_m: Py = Py::None;
    v_m = f__mean(v_x.clone())?;
    v_g0 = f__acov(v_x.clone(), Py::Int(0i64), v_m.clone())?;
    if ({ let t3565 = v_g0.clone(); Py::Bool({ let t3566 = Py::Int(0i64); if py::eq(&t3565, &t3566) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the series must not be constant"))));
    }
    return Ok(py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t3567 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_kmax.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t3567;
out.push(py::truediv(&f__acov(v_x.clone(), v_k.clone(), v_m.clone())?, &v_g0.clone())?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__acov(mut v_x: Py, mut v_k: Py, mut v_m: Py) -> R {
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    v_m = (if ({ let t3562 = v_m.clone(); Py::Bool({ let t3563 = Py::None; if py::is(&t3562, &t3563) { true } else { false } }) }).truthy() { f__mean(v_x.clone())? } else { v_m.clone() });
    return Ok(py::truediv(&py::fsum(&{ let mut out = vec![]; for t3564 in py::range_iter(&[(py::sub(&v_n.clone(), &v_k.clone())?).i()?]) { let mut v_t: Py; 
v_t = t3564;
out.push(py::mul(&py::sub(&py::getitem(&v_x.clone(), &v_t.clone())?, &v_m.clone())?, &py::sub(&py::getitem(&v_x.clone(), &py::add(&v_t.clone(), &v_k.clone())?)?, &v_m.clone())?)?); } Py::list(out) })?, &v_n.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__arma_acov(mut v_phi: Py, mut v_theta: Py, mut v_sigma2: Py, mut v_K: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_psi: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_rhs: Py = Py::None;
    let mut v_th: Py = Py::None;
    let t2753 = (Py::tuple(vec![py::len(&v_phi.clone())?, py::len(&v_theta.clone())?])).items()?;
    if t2753.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2753[0].clone();
    v_q = t2753[1].clone();
    v_th = py::add(&Py::list(vec![Py::Float(1.0)]), &Py::list((v_theta.clone()).items()?))?;
    v_psi = f__psi(v_phi.clone(), v_theta.clone(), v_q.clone())?;
    v_rhs = { let mut out = vec![]; for t2758 in py::range_iter(&[(py::add(&py::add(&py::add(&py::minmax(&[v_p.clone(), v_q.clone()], true)?, &Py::Int(1i64))?, &v_K.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2758;
out.push(py::mul(&v_sigma2.clone(), &py::fsum(&{ let mut out = vec![]; for t2759 in py::range_iter(&[(v_k.clone()).i()?, (py::add(&v_q.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t2759;
out.push(py::mul(&py::getitem(&v_th.clone(), &v_j.clone())?, &py::getitem(&v_psi.clone(), &py::sub(&v_j.clone(), &v_k.clone())?)?)?); } Py::list(out) })?)?); } Py::list(out) };
    v_A = { let mut out = vec![]; for t2760 in py::range_iter(&[(py::add(&v_p.clone(), &Py::Int(1i64))?).i()?]) { let mut v__: Py; 
v__ = t2760;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_p.clone(), &Py::Int(1i64))?)?); } Py::list(out) };
    for t2761 in py::range_iter(&[(py::add(&v_p.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t2761;
        let t2762 = py::getitem(&v_A.clone(), &v_k.clone())?; let t2763 = v_k.clone();
        py::setitem(&t2762, &t2763, py::iadd(&py::getitem(&t2762, &t2763)?, &Py::Float(1.0))?)?;
        for t2764 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_p.clone(), &Py::Int(1i64))?).i()?]) {
            v_i = t2764;
            let t2765 = py::getitem(&v_A.clone(), &v_k.clone())?; let t2766 = py::abs(&py::sub(&v_k.clone(), &v_i.clone())?)?;
            py::setitem(&t2765, &t2766, py::sub(&py::getitem(&t2765, &t2766)?, &py::getitem(&v_phi.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?)?)?;
        }
    }
    v_g = (if (v_p.clone()).truthy() { f__solve(v_A.clone(), py::slice(&v_rhs.clone(), None, Some((py::add(&v_p.clone(), &Py::Int(1i64))?).i()?), None)?)? } else { Py::list(vec![py::getitem(&v_rhs.clone(), &Py::Int(0i64))?]) });
    for t2778 in py::range_iter(&[(py::len(&v_g.clone())?).i()?, (py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t2778;
        let _ = py::method(&v_g.clone(), "append", &[py::add(&py::fsum(&{ let mut out = vec![]; for t2779 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_p.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t2779;
out.push(py::mul(&py::getitem(&v_phi.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_g.clone(), &py::sub(&v_k.clone(), &v_i.clone())?)?)?); } Py::list(out) })?, &(if ({ let t2780 = v_k.clone(); Py::Bool({ let t2781 = py::len(&v_rhs.clone())?; if py::lt(&t2780, &t2781)? { true } else { false } }) }).truthy() { py::getitem(&v_rhs.clone(), &v_k.clone())? } else { Py::Float(0.0) }))?])?;
    }
    return Ok(py::slice(&v_g.clone(), None, Some((py::add(&v_K.clone(), &Py::Int(1i64))?).i()?), None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__arma_check(mut v_phi: Py, mut v_theta: Py, mut v_need_invertible: Py) -> R {
    v_phi = f__fin(v_phi.clone(), Py::str("the AR coefficients"))?;
    v_theta = f__fin(v_theta.clone(), Py::str("the MA coefficients"))?;
    if (py::not(&f__stationary(v_phi.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the AR part must be stationary (every root outside the unit circle)"))));
    }
    if ({ let t3208 = v_need_invertible.clone(); if !t3208.truthy() { t3208 } else { py::not(&f__stationary({ let mut out = vec![]; for t3207 in py::iter(&(v_theta.clone()))? { let mut v_t: Py; 
v_t = t3207;
out.push(py::neg(&v_t.clone())?); } Py::list(out) })?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the MA part must be invertible (every root outside the unit circle)"))));
    }
    while ({ let t3211 = v_phi.clone(); if !t3211.truthy() { t3211 } else { { let t3209 = py::getitem(&v_phi.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t3210 = Py::Int(0i64); if py::eq(&t3209, &t3210) { true } else { false } }) } } }).truthy() {
        v_phi = py::slice(&v_phi.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?;
    }
    while ({ let t3214 = v_theta.clone(); if !t3214.truthy() { t3214 } else { { let t3212 = py::getitem(&v_theta.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t3213 = Py::Int(0i64); if py::eq(&t3212, &t3213) { true } else { false } }) } } }).truthy() {
        v_theta = py::slice(&v_theta.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?;
    }
    return Ok(Py::tuple(vec![v_phi.clone(), v_theta.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ccf(mut v_x: Py, mut v_y: Py, mut v_k: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sx: Py = Py::None;
    let mut v_sy: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let t1201 = (Py::tuple(vec![f__mean(v_x.clone())?, f__mean(v_y.clone())?])).items()?;
    if t1201.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t1201[0].clone();
    v_my = t1201[1].clone();
    v_sx = py::math1("sqrt", &py::truediv(&py::fsum(&{ let mut out = vec![]; for t1202 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1202;
out.push(py::pow(&py::sub(&v_v.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) })?, &v_n.clone())?)?;
    v_sy = py::math1("sqrt", &py::truediv(&py::fsum(&{ let mut out = vec![]; for t1203 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t1203;
out.push(py::pow(&py::sub(&v_v.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) })?, &v_n.clone())?)?;
    if ({ let t1204 = v_k.clone(); Py::Bool({ let t1205 = Py::Int(0i64); if py::ge(&t1204, &t1205)? { true } else { false } }) }).truthy() {
        v_c = py::fsum(&{ let mut out = vec![]; for t1206 in py::range_iter(&[(py::sub(&v_n.clone(), &v_k.clone())?).i()?]) { let mut v_t: Py; 
v_t = t1206;
out.push(py::mul(&py::sub(&py::getitem(&v_x.clone(), &py::add(&v_t.clone(), &v_k.clone())?)?, &v_mx.clone())?, &py::sub(&py::getitem(&v_y.clone(), &v_t.clone())?, &v_my.clone())?)?); } Py::list(out) })?;
    } else {
        v_c = py::fsum(&{ let mut out = vec![]; for t1207 in py::range_iter(&[(py::add(&v_n.clone(), &v_k.clone())?).i()?]) { let mut v_t: Py; 
v_t = t1207;
out.push(py::mul(&py::sub(&py::getitem(&v_x.clone(), &v_t.clone())?, &v_mx.clone())?, &py::sub(&py::getitem(&v_y.clone(), &py::sub(&v_t.clone(), &v_k.clone())?)?, &v_my.clone())?)?); } Py::list(out) })?;
    }
    return Ok(py::truediv(&py::truediv(&v_c.clone(), &v_n.clone())?, &py::mul(&v_sx.clone(), &v_sy.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__chi2_sf(mut v_x: Py, mut v_k: Py) -> R {
    return Ok(f__gammq(py::truediv(&v_k.clone(), &Py::Float(2.0))?, py::truediv(&v_x.clone(), &Py::Float(2.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cv_mats(mut v_dt: Py, mut v_q: Py) -> R {
    return Ok(Py::tuple(vec![Py::list(vec![Py::list(vec![Py::Float(1.0), v_dt.clone()]), Py::list(vec![Py::Float(0.0), Py::Float(1.0)])]), Py::list(vec![Py::list(vec![py::truediv(&py::mul(&v_q.clone(), &py::pow(&v_dt.clone(), &Py::Int(4i64))?)?, &Py::Int(4i64))?, py::truediv(&py::mul(&v_q.clone(), &py::pow(&v_dt.clone(), &Py::Int(3i64))?)?, &Py::Int(2i64))?]), Py::list(vec![py::truediv(&py::mul(&v_q.clone(), &py::pow(&v_dt.clone(), &Py::Int(3i64))?)?, &Py::Int(2i64))?, py::mul(&v_q.clone(), &py::pow(&v_dt.clone(), &Py::Int(2i64))?)?])])]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cv_predict(mut v_P: Py, mut v_F: Py, mut v_Q: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dt: Py = Py::None;
    let mut v_p00: Py = Py::None;
    let mut v_p01: Py = Py::None;
    let mut v_p11: Py = Py::None;
    let t2147 = (Py::tuple(vec![py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, py::getitem(&py::getitem(&v_P.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?])).items()?;
    if t2147.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2147[0].clone();
    v_b = t2147[1].clone();
    v_c = t2147[2].clone();
    v_dt = py::getitem(&py::getitem(&v_F.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?;
    v_p00 = py::add(&py::add(&py::add(&v_a.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_dt.clone())?, &v_b.clone())?)?, &py::mul(&py::mul(&v_dt.clone(), &v_dt.clone())?, &v_c.clone())?)?, &py::getitem(&py::getitem(&v_Q.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?)?;
    v_p01 = py::add(&py::add(&v_b.clone(), &py::mul(&v_dt.clone(), &v_c.clone())?)?, &py::getitem(&py::getitem(&v_Q.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?)?;
    v_p11 = py::add(&v_c.clone(), &py::getitem(&py::getitem(&v_Q.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?;
    return Ok(Py::list(vec![Py::list(vec![v_p00.clone(), v_p01.clone()]), Py::list(vec![v_p01.clone(), v_p11.clone()])]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cv_update(mut v_P: Py, mut v_r: Py) -> R {
    let mut v_k0: Py = Py::None;
    let mut v_k1: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = py::add(&py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &v_r.clone())?;
    let t2145 = (Py::tuple(vec![py::truediv(&py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &v_s.clone())?, py::truediv(&py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &v_s.clone())?])).items()?;
    if t2145.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k0 = t2145[0].clone();
    v_k1 = t2145[1].clone();
    return Ok(Py::tuple(vec![Py::list(vec![Py::list(vec![py::mul(&py::sub(&Py::Int(1i64), &v_k0.clone())?, &py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?)?, py::mul(&py::sub(&Py::Int(1i64), &v_k0.clone())?, &py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?)?]), Py::list(vec![py::mul(&py::sub(&Py::Int(1i64), &v_k0.clone())?, &py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?)?, py::sub(&py::getitem(&py::getitem(&v_P.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, &py::mul(&v_k1.clone(), &py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?)?)?])]), Py::tuple(vec![v_k0.clone(), v_k1.clone()])]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__d(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_r: Py = Py::None;
    if ({ let t4358 = Py::Bool(matches!(v_a.clone(), Py::Str(_))); if t4358.truthy() { t4358 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_))) } }).truthy() {
        return Ok((if ({ let t4359 = v_a.clone(); Py::Bool({ let t4360 = v_b.clone(); if py::eq(&t4359, &t4360) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    if ({ let t4361 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t4361.truthy() { t4361 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t4365 = py::not(&{ let t4362 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t4362.truthy() { t4362 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t4365.truthy() { t4365 } else { { let t4363 = py::len(&v_a.clone())?; Py::Bool({ let t4364 = py::len(&v_b.clone())?; if py::ne(&t4363, &t4364) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
        return Ok({ let t = { let mut out = vec![]; for t4366 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_u: Py; let mut v_v: Py; 
let t4367 = (t4366).items()?;
if t4367.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t4367[0].clone();
v_v = t4367[1].clone();
out.push(f__d(v_u.clone(), v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    }
    if ({ let t4372 = { let t4368 = v_a.clone(); Py::Bool({ let t4369 = Py::None; if py::is(&t4368, &t4369) { true } else { false } }) }; if t4372.truthy() { t4372 } else { { let t4370 = v_b.clone(); Py::Bool({ let t4371 = Py::None; if py::is(&t4370, &t4371) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t4373 = v_a.clone(); Py::Bool({ let t4374 = v_b.clone(); if py::is(&t4373, &t4374) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    let t4375 = (Py::tuple(vec![py::complex(&[v_a.clone()])?, py::complex(&[v_b.clone()])?])).items()?;
    if t4375.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4375[0].clone();
    v_b = t4375[1].clone();
    v_r = py::truediv(&py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?;
    return Ok((if ({ let t4376 = v_r.clone(); Py::Bool({ let t4377 = v_r.clone(); if py::eq(&t4376, &t4377) { true } else { false } }) }).truthy() { v_r.clone() } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__decompose(mut v_x: Py, mut v_m: Py, mut v_mult: Py) -> R {
    let mut v_avg: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_trend: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    v_m = f__int(v_m.clone(), Py::str("the season length m"), Py::Int(2i64), Py::Int(52i64))?;
    let _ = f__need(v_x.clone(), py::mul(&Py::Int(2i64), &v_m.clone())?, Py::str("the series (two full seasons)"))?;
    if (v_mult.clone()).truthy() {
        let _ = f__positive(v_x.clone(), Py::str("the series"))?;
    }
    let t3905 = (Py::tuple(vec![py::len(&v_x.clone())?, py::floordiv(&v_m.clone(), &Py::Int(2i64))?])).items()?;
    if t3905.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3905[0].clone();
    v_h = t3905[1].clone();
    v_trend = py::mul(&Py::list(vec![Py::None]), &v_n.clone())?;
    for t3906 in py::range_iter(&[(v_h.clone()).i()?, (py::sub(&v_n.clone(), &v_h.clone())?).i()?]) {
        v_t = t3906;
        if (py::modulo(&v_m.clone(), &Py::Int(2i64))?).truthy() {
            py::setitem(&v_trend.clone(), &v_t.clone(), py::truediv(&py::fsum(&py::slice(&v_x.clone(), Some((py::sub(&v_t.clone(), &v_h.clone())?).i()?), Some((py::add(&py::add(&v_t.clone(), &v_h.clone())?, &Py::Int(1i64))?).i()?), None)?)?, &v_m.clone())?)?;
        } else {
            py::setitem(&v_trend.clone(), &v_t.clone(), py::truediv(&py::add(&py::add(&py::mul(&Py::Float(0.5), &py::getitem(&v_x.clone(), &py::sub(&v_t.clone(), &v_h.clone())?)?)?, &py::fsum(&py::slice(&v_x.clone(), Some((py::add(&py::sub(&v_t.clone(), &v_h.clone())?, &Py::Int(1i64))?).i()?), Some((py::add(&v_t.clone(), &v_h.clone())?).i()?), None)?)?)?, &py::mul(&Py::Float(0.5), &py::getitem(&v_x.clone(), &py::add(&v_t.clone(), &v_h.clone())?)?)?)?, &v_m.clone())?)?;
        }
    }
    v_det = { let mut out = vec![]; for t3907 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_t: Py; 
v_t = t3907;
out.push((if ({ let t3908 = py::getitem(&v_trend.clone(), &v_t.clone())?; Py::Bool({ let t3909 = Py::None; if py::is(&t3908, &t3909) { true } else { false } }) }).truthy() { Py::None } else { (if (v_mult.clone()).truthy() { py::truediv(&py::getitem(&v_x.clone(), &v_t.clone())?, &py::getitem(&v_trend.clone(), &v_t.clone())?)? } else { py::sub(&py::getitem(&v_x.clone(), &v_t.clone())?, &py::getitem(&v_trend.clone(), &v_t.clone())?)? }) })); } Py::list(out) };
    v_avg = Py::list(vec![]);
    for t3910 in py::range_iter(&[(v_m.clone()).i()?]) {
        v_i = t3910;
        v_vals = { let mut out = vec![]; for t3911 in py::range_iter(&[(v_i.clone()).i()?, (v_n.clone()).i()?, (v_m.clone()).i()?]) { let mut v_t: Py; 
v_t = t3911;
if ({ let t3912 = py::getitem(&v_det.clone(), &v_t.clone())?; Py::Bool({ let t3913 = Py::None; if !py::is(&t3912, &t3913) { true } else { false } }) }).truthy() { out.push(py::getitem(&v_det.clone(), &v_t.clone())?); }} Py::list(out) };
        let _ = py::method(&v_avg.clone(), "append", &[py::truediv(&py::fsum(&v_vals.clone())?, &py::len(&v_vals.clone())?)?])?;
    }
    v_c = py::truediv(&py::fsum(&v_avg.clone())?, &v_m.clone())?;
    v_idx = (if (v_mult.clone()).truthy() { { let mut out = vec![]; for t3914 in py::iter(&(v_avg.clone()))? { let mut v_a: Py; 
v_a = t3914;
out.push(py::truediv(&v_a.clone(), &v_c.clone())?); } Py::list(out) } } else { { let mut out = vec![]; for t3915 in py::iter(&(v_avg.clone()))? { let mut v_a: Py; 
v_a = t3915;
out.push(py::sub(&v_a.clone(), &v_c.clone())?); } Py::list(out) } });
    return Ok(Py::tuple(vec![v_x.clone(), v_idx.clone(), v_avg.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__df(mut v_x: Py, mut v_p: Py, mut v_regression: Py, mut v_mle: Py) -> R {
    let mut v_beta: Py = Py::None;
    let mut v_diag: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_nobs: Py = Py::None;
    let mut v_nt: Py = Py::None;
    let mut v_reg: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    v_p = f__int(v_p.clone(), Py::str("the number of lagged differences"), Py::Int(0i64), Py::Int(12i64))?;
    v_reg = { let t1294 = v_regression.clone(); if t1294.truthy() { t1294 } else { Py::str("c") } };
    if ({ let t1295 = v_reg.clone(); Py::Bool({ let t1296 = Py::tuple(vec![Py::str("n"), Py::str("c"), Py::str("ct")]); if !py::contains(&t1296, &t1295)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the regression must be n, c or ct"))));
    }
    let _ = f__nonconstant(v_x.clone(), Py::str("the series"))?;
    v_nt = py::getitem(&Py::dict(vec![(Py::str("n"), Py::Int(0i64)), (Py::str("c"), Py::Int(1i64)), (Py::str("ct"), Py::Int(2i64))]), &v_reg.clone())?;
    if ({ let t1299 = v_p.clone(); Py::Bool({ let t1300 = py::sub(&py::sub(&py::floordiv(&py::len(&v_x.clone())?, &Py::Int(2i64))?, &v_nt.clone())?, &Py::Int(1i64))?; if py::gt(&t1299, &t1300)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("too many lags for the series length (p <= n/2 - deterministic terms - 1)"))));
    }
    v_dx = { let mut out = vec![]; for t1301 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_x.clone())?).i()?]) { let mut v_t: Py; 
v_t = t1301;
out.push(py::sub(&py::getitem(&v_x.clone(), &v_t.clone())?, &py::getitem(&v_x.clone(), &py::sub(&v_t.clone(), &Py::Int(1i64))?)?)?); } Py::list(out) };
    let t1302 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t1302.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rows = t1302[0].clone();
    v_y = t1302[1].clone();
    for t1303 in py::iter(&(py::enumerate(&Py::list((py::range_iter(&[(v_p.clone()).i()?, (py::len(&v_dx.clone())?).i()?])).into_iter().collect::<Vec<Py>>()), 0)?))? {
        let t1309 = (t1303).items()?;
        if t1309.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t1309[0].clone();
        v_t = t1309[1].clone();
        v_row = py::add(&Py::list(vec![py::getitem(&v_x.clone(), &v_t.clone())?]), &{ let mut out = vec![]; for t1304 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_p.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t1304;
out.push(py::getitem(&v_dx.clone(), &py::sub(&v_t.clone(), &v_j.clone())?)?); } Py::list(out) })?;
        if ({ let t1305 = v_nt.clone(); Py::Bool({ let t1306 = Py::Int(1i64); if py::ge(&t1305, &t1306)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_row.clone(), "append", &[Py::Float(1.0)])?;
        }
        if ({ let t1307 = v_nt.clone(); Py::Bool({ let t1308 = Py::Int(2i64); if py::eq(&t1307, &t1308) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_row.clone(), "append", &[py::float(&py::add(&v_i.clone(), &Py::Int(1i64))?)?])?;
        }
        let _ = py::method(&v_rows.clone(), "append", &[v_row.clone()])?;
        let _ = py::method(&v_y.clone(), "append", &[py::getitem(&v_dx.clone(), &v_t.clone())?])?;
    }
    if ({ let t1310 = py::len(&v_rows.clone())?; Py::Bool({ let t1311 = py::len(&py::getitem(&v_rows.clone(), &Py::Int(0i64))?)?; if py::le(&t1310, &t1311)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the series is too short for this regression"))));
    }
    let t1351 = (f__lstsq(v_rows.clone(), v_y.clone())?).items()?;
    if t1351.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_beta = t1351[0].clone();
    v_res = t1351[1].clone();
    v_diag = t1351[2].clone();
    v_nobs = py::len(&v_y.clone())?;
    v_s2 = py::truediv(&py::fsum(&{ let mut out = vec![]; for t1352 in py::iter(&(v_res.clone()))? { let mut v_e: Py; 
v_e = t1352;
out.push(py::mul(&v_e.clone(), &v_e.clone())?); } Py::list(out) })?, &(if (v_mle.clone()).truthy() { v_nobs.clone() } else { py::sub(&v_nobs.clone(), &py::len(&py::getitem(&v_rows.clone(), &Py::Int(0i64))?)?)? }))?;
    v_se = py::math1("sqrt", &py::mul(&v_s2.clone(), &py::getitem(&v_diag.clone(), &Py::Int(0i64))?)?)?;
    return Ok(Py::dict(vec![(Py::str("tau"), py::truediv(&py::getitem(&v_beta.clone(), &Py::Int(0i64))?, &v_se.clone())?), (Py::str("gamma"), py::getitem(&v_beta.clone(), &Py::Int(0i64))?), (Py::str("se"), v_se.clone()), (Py::str("nobs"), v_nobs.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__diff(mut v_want: Py, mut v_got: Py) -> R {
    return Ok({ let t = { let mut out = vec![]; for t4355 in py::iter(&(v_got.clone()))? { let mut v_k: Py; 
v_k = t4355;
if ({ let t4356 = v_k.clone(); Py::Bool({ let t4357 = v_want.clone(); if py::contains(&t4357, &t4356)? { true } else { false } }) }).truthy() { out.push(f__d(py::getitem(&v_want.clone(), &v_k.clone())?, py::getitem(&v_got.clone(), &v_k.clone())?)?); }} Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__errors(mut v_y: Py, mut v_f: Py) -> R {
    let t4107 = (Py::tuple(vec![f__fin(v_y.clone(), Py::str("the actual values"))?, f__fin(v_f.clone(), Py::str("the forecasts"))?])).items()?;
    if t4107.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_y = t4107[0].clone();
    v_f = t4107[1].clone();
    let _ = f__same_len(v_y.clone(), v_f.clone(), Py::str("the actual values"), Py::str("the forecasts"))?;
    return Ok(Py::tuple(vec![v_y.clone(), v_f.clone(), { let mut out = vec![]; for t4110 in py::iter(&(py::zip(&[v_y.clone(), v_f.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4111 = (t4110).items()?;
if t4111.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4111[0].clone();
v_b = t4111[1].clone();
out.push(py::sub(&v_a.clone(), &v_b.clone())?); } Py::list(out) }]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_x: Py, mut v_name: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_out = Py::list(vec![]);
    for t4344 in py::iter(&(v_x.clone()))? {
        v_v = t4344;
        if ({ let t4346 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t4346.truthy() { t4346 } else { { let t4345 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t4345.truthy() { t4345 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must contain finite numbers only".to_string()].concat()))));
        }
        let _ = py::method(&v_out.clone(), "append", &[py::float(&v_v.clone())?])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gammq(mut v_a: Py, mut v_x: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_ap: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_de: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lf: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tiny: Py = Py::None;
    if ({ let t2598 = v_x.clone(); Py::Bool({ let t2599 = Py::Int(0i64); if py::le(&t2598, &t2599)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_lf = py::sub(&py::sub(&py::mul(&v_a.clone(), &py::math1("log", &v_x.clone())?)?, &v_x.clone())?, &py::math1("lgamma", &v_a.clone())?)?;
    if ({ let t2600 = v_x.clone(); Py::Bool({ let t2601 = py::add(&v_a.clone(), &Py::Float(1.0))?; if py::lt(&t2600, &t2601)? { true } else { false } }) }).truthy() {
        let t2602 = (Py::tuple(vec![v_a.clone(), py::truediv(&Py::Float(1.0), &v_a.clone())?])).items()?;
        if t2602.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ap = t2602[0].clone();
        v_s = t2602[1].clone();
        v_d = v_s.clone();
        for t2603 in py::range_iter(&[(Py::Int(100000i64)).i()?]) {
            v__ = t2603;
            v_ap = py::iadd(&v_ap, &Py::Float(1.0))?;
            v_d = py::mul(&v_d, &py::truediv(&v_x.clone(), &v_ap.clone())?)?;
            v_s = py::iadd(&v_s, &v_d.clone())?;
            if ({ let t2604 = py::abs(&v_d.clone())?; Py::Bool({ let t2605 = py::mul(&py::abs(&v_s.clone())?, &Py::Float(1e-17))?; if py::lt(&t2604, &t2605)? { true } else { false } }) }).truthy() {
                break;
            }
        }
        return Ok(py::sub(&Py::Float(1.0), &py::mul(&v_s.clone(), &py::math1("exp", &v_lf.clone())?)?)?);
    }
    v_tiny = Py::Float(1e-300);
    v_b = py::sub(&py::add(&v_x.clone(), &Py::Float(1.0))?, &v_a.clone())?;
    let t2606 = (Py::tuple(vec![py::truediv(&Py::Float(1.0), &v_tiny.clone())?, py::truediv(&Py::Float(1.0), &v_b.clone())?])).items()?;
    if t2606.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2606[0].clone();
    v_d = t2606[1].clone();
    v_h = v_d.clone();
    for t2607 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(100000i64)).i()?]) {
        v_i = t2607;
        v_an = py::mul(&py::neg(&v_i.clone())?, &py::sub(&v_i.clone(), &v_a.clone())?)?;
        v_b = py::iadd(&v_b, &Py::Float(2.0))?;
        v_d = py::add(&py::mul(&v_an.clone(), &v_d.clone())?, &v_b.clone())?;
        v_d = (if ({ let t2608 = py::abs(&v_d.clone())?; Py::Bool({ let t2609 = v_tiny.clone(); if py::lt(&t2608, &t2609)? { true } else { false } }) }).truthy() { v_tiny.clone() } else { v_d.clone() });
        v_c = py::add(&v_b.clone(), &py::truediv(&v_an.clone(), &v_c.clone())?)?;
        v_c = (if ({ let t2610 = py::abs(&v_c.clone())?; Py::Bool({ let t2611 = v_tiny.clone(); if py::lt(&t2610, &t2611)? { true } else { false } }) }).truthy() { v_tiny.clone() } else { v_c.clone() });
        v_d = py::truediv(&Py::Float(1.0), &v_d.clone())?;
        v_de = py::mul(&v_d.clone(), &v_c.clone())?;
        v_h = py::mul(&v_h, &v_de.clone())?;
        if ({ let t2612 = py::abs(&py::sub(&v_de.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t2613 = Py::Float(4e-16); if py::lt(&t2612, &t2613)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    return Ok(py::mul(&py::math1("exp", &v_lf.clone())?, &v_h.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__holt_core(mut v_x: Py, mut v_alpha: Py, mut v_beta: Py, mut v_l0: Py, mut v_b0: Py, mut v_h: Py, mut v_phi: Py, mut v_wrong: Py) -> R {
    let mut v_damp: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_lev: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_sse: Py = Py::None;
    let mut v_tr: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t1818 = (Py::tuple(vec![v_l0.clone(), v_b0.clone(), Py::Float(0.0)])).items()?;
    if t1818.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lev = t1818[0].clone();
    v_tr = t1818[1].clone();
    v_sse = t1818[2].clone();
    for t1819 in py::iter(&(v_x.clone()))? {
        v_v = t1819;
        v_f = py::add(&v_lev.clone(), &py::mul(&v_phi.clone(), &v_tr.clone())?)?;
        v_sse = py::iadd(&v_sse, &py::pow(&py::sub(&v_v.clone(), &v_f.clone())?, &Py::Int(2i64))?)?;
        v_new = py::add(&py::mul(&v_alpha.clone(), &v_v.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_alpha.clone())?, &v_f.clone())?)?;
        v_tr = py::add(&py::mul(&v_beta.clone(), &py::sub(&(if (v_wrong.clone()).truthy() { v_v.clone() } else { v_new.clone() }), &v_lev.clone())?)?, &py::mul(&py::mul(&py::sub(&Py::Int(1i64), &v_beta.clone())?, &v_phi.clone())?, &v_tr.clone())?)?;
        v_lev = v_new.clone();
    }
    v_damp = py::fsum(&{ let mut out = vec![]; for t1820 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_h.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t1820;
out.push(py::pow(&v_phi.clone(), &v_j.clone())?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("forecast"), py::add(&v_lev.clone(), &py::mul(&v_damp.clone(), &v_tr.clone())?)?), (Py::str("level"), v_lev.clone()), (Py::str("trend"), v_tr.clone()), (Py::str("sse"), v_sse.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__holt_inputs(mut v_x: Py, mut v_alpha: Py, mut v_beta: Py, mut v_l0: Py, mut v_b0: Py, mut v_h: Py) -> R {
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the series"))?;
    let t1831 = (Py::tuple(vec![f__unit(v_alpha.clone(), Py::str("alpha"))?, f__unit(v_beta.clone(), Py::str("beta"))?])).items()?;
    if t1831.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_alpha = t1831[0].clone();
    v_beta = t1831[1].clone();
    v_l0 = (if ({ let t1832 = v_l0.clone(); Py::Bool({ let t1833 = Py::None; if py::is(&t1832, &t1833) { true } else { false } }) }).truthy() { py::getitem(&v_x.clone(), &Py::Int(0i64))? } else { f__num(v_l0.clone(), Py::str("the initial level"))? });
    v_b0 = (if ({ let t1834 = v_b0.clone(); Py::Bool({ let t1835 = Py::None; if py::is(&t1834, &t1835) { true } else { false } }) }).truthy() { py::sub(&py::getitem(&v_x.clone(), &Py::Int(1i64))?, &py::getitem(&v_x.clone(), &Py::Int(0i64))?)? } else { f__num(v_b0.clone(), Py::str("the initial trend"))? });
    return Ok(Py::tuple(vec![v_x.clone(), v_alpha.clone(), v_beta.clone(), v_l0.clone(), v_b0.clone(), f__int(v_h.clone(), Py::str("the horizon h"), Py::Int(1i64), Py::Int(1000i64))?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hw(mut v_x: Py, mut v_m: Py, mut v_alpha: Py, mut v_beta: Py, mut v_gamma: Py, mut v_h: Py, mut v_mult: Py, mut v_wrong: Py) -> R {
    let mut v_base: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_fc: Py = Py::None;
    let mut v_lev: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_seas: Py = Py::None;
    let mut v_sse: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tr: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    v_m = f__int(v_m.clone(), Py::str("the season length m"), Py::Int(2i64), Py::Int(52i64))?;
    let _ = f__need(v_x.clone(), py::mul(&Py::Int(2i64), &v_m.clone())?, Py::str("the series (two full seasons)"))?;
    let t2012 = (Py::tuple(vec![f__unit(v_alpha.clone(), Py::str("alpha"))?, f__unit(v_beta.clone(), Py::str("beta"))?, f__unit(v_gamma.clone(), Py::str("gamma"))?])).items()?;
    if t2012.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_alpha = t2012[0].clone();
    v_beta = t2012[1].clone();
    v_gamma = t2012[2].clone();
    v_h = f__int(v_h.clone(), Py::str("the horizon h"), Py::Int(1i64), Py::Int(1000i64))?;
    if (v_mult.clone()).truthy() {
        let _ = f__positive(v_x.clone(), Py::str("the series"))?;
    }
    let t2018 = (f__hw_init(v_x.clone(), v_m.clone(), v_mult.clone())?).items()?;
    if t2018.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_lev = t2018[0].clone();
    v_tr = t2018[1].clone();
    v_s = t2018[2].clone();
    if ({ let t2021 = v_mult.clone(); if !t2021.truthy() { t2021 } else { { let t2019 = v_lev.clone(); Py::Bool({ let t2020 = Py::Int(0i64); if py::le(&t2019, &t2020)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the first season's mean must be positive"))));
    }
    v_sse = Py::Float(0.0);
    for t2022 in py::iter(&(py::enumerate(&v_x.clone(), 0)?))? {
        let t2023 = (t2022).items()?;
        if t2023.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_t = t2023[0].clone();
        v_v = t2023[1].clone();
        v_base = py::add(&v_lev.clone(), &v_tr.clone())?;
        v_f = (if (v_mult.clone()).truthy() { py::mul(&v_base.clone(), &py::getitem(&v_s.clone(), &v_t.clone())?)? } else { py::add(&v_base.clone(), &py::getitem(&v_s.clone(), &v_t.clone())?)? });
        v_sse = py::iadd(&v_sse, &py::pow(&py::sub(&v_v.clone(), &v_f.clone())?, &Py::Int(2i64))?)?;
        if (v_wrong.clone()).truthy() {
            v_new = py::add(&py::mul(&v_alpha.clone(), &v_v.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_alpha.clone())?, &v_base.clone())?)?;
        } else {
            v_new = py::add(&py::mul(&v_alpha.clone(), &(if (v_mult.clone()).truthy() { py::truediv(&v_v.clone(), &py::getitem(&v_s.clone(), &v_t.clone())?)? } else { py::sub(&v_v.clone(), &py::getitem(&v_s.clone(), &v_t.clone())?)? }))?, &py::mul(&py::sub(&Py::Int(1i64), &v_alpha.clone())?, &v_base.clone())?)?;
        }
        let _ = py::method(&v_s.clone(), "append", &[py::add(&py::mul(&v_gamma.clone(), &(if (v_mult.clone()).truthy() { py::truediv(&v_v.clone(), &v_base.clone())? } else { py::sub(&v_v.clone(), &v_base.clone())? }))?, &py::mul(&py::sub(&Py::Int(1i64), &v_gamma.clone())?, &py::getitem(&v_s.clone(), &v_t.clone())?)?)?])?;
        v_tr = py::add(&py::mul(&v_beta.clone(), &py::sub(&v_new.clone(), &v_lev.clone())?)?, &py::mul(&py::sub(&Py::Int(1i64), &v_beta.clone())?, &v_tr.clone())?)?;
        v_lev = v_new.clone();
    }
    v_n = py::len(&v_x.clone())?;
    v_seas = py::getitem(&v_s.clone(), &py::add(&v_n.clone(), &py::modulo(&py::sub(&v_h.clone(), &Py::Int(1i64))?, &v_m.clone())?)?)?;
    v_fc = (if (v_mult.clone()).truthy() { py::mul(&py::add(&v_lev.clone(), &py::mul(&v_h.clone(), &v_tr.clone())?)?, &v_seas.clone())? } else { py::add(&py::add(&v_lev.clone(), &py::mul(&v_h.clone(), &v_tr.clone())?)?, &v_seas.clone())? });
    return Ok(Py::dict(vec![(Py::str("forecast"), v_fc.clone()), (Py::str("level"), v_lev.clone()), (Py::str("trend"), v_tr.clone()), (Py::str("sse"), v_sse.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hw_init(mut v_x: Py, mut v_m: Py, mut v_mult: Py) -> R {
    let mut v_b0: Py = Py::None;
    let mut v_l0: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_l0 = py::truediv(&py::fsum(&py::slice(&v_x.clone(), None, Some((v_m.clone()).i()?), None)?)?, &v_m.clone())?;
    v_b0 = py::truediv(&py::sub(&py::truediv(&py::fsum(&py::slice(&v_x.clone(), Some((v_m.clone()).i()?), Some((py::mul(&Py::Int(2i64), &v_m.clone())?).i()?), None)?)?, &v_m.clone())?, &v_l0.clone())?, &v_m.clone())?;
    v_s = (if (v_mult.clone()).truthy() { { let mut out = vec![]; for t2016 in py::iter(&(py::slice(&v_x.clone(), None, Some((v_m.clone()).i()?), None)?))? { let mut v_v: Py; 
v_v = t2016;
out.push(py::truediv(&v_v.clone(), &v_l0.clone())?); } Py::list(out) } } else { { let mut out = vec![]; for t2017 in py::iter(&(py::slice(&v_x.clone(), None, Some((v_m.clone()).i()?), None)?))? { let mut v_v: Py; 
v_v = t2017;
out.push(py::sub(&v_v.clone(), &v_l0.clone())?); } Py::list(out) } });
    return Ok(Py::tuple(vec![v_l0.clone(), v_b0.clone(), v_s.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__int(mut v_v: Py, mut v_name: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    if ({ let t4295 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t4295.truthy() { t4295 } else { { let t4294 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t4294.truthy() { t4294 } else { { let t4293 = py::not(&py::math1("isfinite", &v_v.clone())?); if t4293.truthy() { t4293 } else { { let t4291 = py::float(&v_v.clone())?; Py::Bool({ let t4292 = py::int(&v_v.clone())?; if py::ne(&t4291, &t4292) { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number".to_string()].concat()))));
    }
    v_v = py::int(&v_v.clone())?;
    if ({ let t4300 = { let t4296 = v_lo.clone(); Py::Bool({ let t4297 = Py::None; if !py::is(&t4296, &t4297) { true } else { false } }) }; if !t4300.truthy() { t4300 } else { { let t4298 = v_v.clone(); Py::Bool({ let t4299 = v_lo.clone(); if py::lt(&t4298, &t4299)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be at least ".to_string(), py::format(&v_lo.clone(), "")?].concat()))));
    }
    if ({ let t4305 = { let t4301 = v_hi.clone(); Py::Bool({ let t4302 = Py::None; if !py::is(&t4301, &t4302) { true } else { false } }) }; if !t4305.truthy() { t4305 } else { { let t4303 = v_v.clone(); Py::Bool({ let t4304 = v_hi.clone(); if py::gt(&t4303, &t4304)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be at most ".to_string(), py::format(&v_hi.clone(), "")?].concat()))));
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__interp_in(mut v_t: Py, mut v_y: Py, mut v_t0: Py) -> R {
    let mut v_i: Py = Py::None;
    let t2384 = (Py::tuple(vec![f__fin(v_t.clone(), Py::str("the times"))?, f__fin(v_y.clone(), Py::str("the values"))?])).items()?;
    if t2384.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t2384[0].clone();
    v_y = t2384[1].clone();
    let _ = f__same_len(v_t.clone(), v_y.clone(), Py::str("the times"), Py::str("the values"))?;
    let _ = f__need(v_t.clone(), Py::Int(2i64), Py::str("the times"))?;
    if (Py::Bool(({ let mut out = vec![]; for t2389 in py::iter(&(py::zip(&[v_t.clone(), py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2390 = (t2389).items()?;
if t2390.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2390[0].clone();
v_b = t2390[1].clone();
out.push({ let t2391 = v_b.clone(); Py::Bool({ let t2392 = v_a.clone(); if py::le(&t2391, &t2392)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the times must be strictly increasing"))));
    }
    v_t0 = f__num(v_t0.clone(), Py::str("the time t0"))?;
    if (py::not(&{ let t2395 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t2396 = v_t0.clone(); if py::le(&t2395, &t2396)? { { let t2397 = py::getitem(&v_t.clone(), &py::neg(&Py::Int(1i64))?)?; if py::le(&t2396, &t2397)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("t0 must lie between the first and last time (no extrapolation)"))));
    }
    v_i = py::minmax(&[{ let mut out = vec![]; for t2398 in py::range_iter(&[(py::sub(&py::len(&v_t.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t2398;
if ({ let t2399 = py::getitem(&v_t.clone(), &v_j.clone())?; Py::Bool({ let t2400 = v_t0.clone(); if py::le(&t2399, &t2400)? { true } else { false } }) }).truthy() { out.push(v_j.clone()); }} Py::list(out) }], true)?;
    return Ok(Py::tuple(vec![v_t.clone(), v_y.clone(), v_t0.clone(), v_i.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__levinson(mut v_r: Py, mut v_p: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_pacf: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t4347 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::Float(1.0)])).items()?;
    if t4347.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t4347[0].clone();
    v_pacf = t4347[1].clone();
    v_v = t4347[2].clone();
    for t4348 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_p.clone(), &Py::Int(1i64))?).i()?]) {
        v_m = t4348;
        v_a = py::truediv(&py::sub(&py::getitem(&v_r.clone(), &v_m.clone())?, &py::fsum(&{ let mut out = vec![]; for t4349 in py::range_iter(&[(py::sub(&v_m.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t4349;
out.push(py::mul(&py::getitem(&v_phi.clone(), &v_j.clone())?, &py::getitem(&v_r.clone(), &py::sub(&py::sub(&v_m.clone(), &Py::Int(1i64))?, &v_j.clone())?)?)?); } Py::list(out) })?)?, &v_v.clone())?;
        if (py::not(&{ let t4350 = py::abs(&v_a.clone())?; Py::Bool({ let t4351 = Py::Int(1i64); if py::lt(&t4350, &t4351)? { true } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the autocorrelations are not a valid (positive definite) sequence"))));
        }
        v_phi = py::add(&{ let mut out = vec![]; for t4352 in py::range_iter(&[(py::sub(&v_m.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t4352;
out.push(py::sub(&py::getitem(&v_phi.clone(), &v_j.clone())?, &py::mul(&v_a.clone(), &py::getitem(&v_phi.clone(), &py::sub(&py::sub(&v_m.clone(), &Py::Int(2i64))?, &v_j.clone())?)?)?)?); } Py::list(out) }, &Py::list(vec![v_a.clone()]))?;
        let _ = py::method(&v_pacf.clone(), "append", &[v_a.clone()])?;
        v_v = py::mul(&v_v, &py::sub(&Py::Int(1i64), &py::mul(&v_a.clone(), &v_a.clone())?)?)?;
    }
    return Ok(Py::tuple(vec![v_phi.clone(), v_pacf.clone(), v_v.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ll_filter(mut v_x: Py, mut v_q: Py, mut v_r: Py, mut v_m0: Py, mut v_p0: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_ll: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_ms: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pp: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the measurements"))?;
    let _ = f__need(v_x.clone(), Py::Int(1i64), Py::str("the measurements"))?;
    v_q = f__num(v_q.clone(), Py::str("the process variance"))?;
    v_r = f__pos(v_r.clone(), Py::str("the measurement variance"))?;
    let t3392 = (Py::tuple(vec![f__num(v_m0.clone(), Py::str("the initial estimate"))?, f__num(v_p0.clone(), Py::str("the initial variance"))?])).items()?;
    if t3392.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3392[0].clone();
    v_p = t3392[1].clone();
    if ({ let t3397 = { let t3393 = v_q.clone(); Py::Bool({ let t3394 = Py::Int(0i64); if py::lt(&t3393, &t3394)? { true } else { false } }) }; if t3397.truthy() { t3397 } else { { let t3395 = v_p.clone(); Py::Bool({ let t3396 = Py::Int(0i64); if py::lt(&t3395, &t3396)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the variances must be non-negative"))));
    }
    let t3398 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t3398.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_ms = t3398[0].clone();
    v_ps = t3398[1].clone();
    v_ll = t3398[2].clone();
    v_k = t3398[3].clone();
    for t3399 in py::iter(&(v_x.clone()))? {
        v_z = t3399;
        v_pp = py::add(&v_p.clone(), &v_q.clone())?;
        v_s = py::add(&v_pp.clone(), &v_r.clone())?;
        v_k = py::truediv(&v_pp.clone(), &v_s.clone())?;
        v_v = py::sub(&v_z.clone(), &v_m.clone())?;
        v_ll = py::iadd(&v_ll, &py::mul(&py::neg(&Py::Float(0.5))?, &py::add(&py::math1("log", &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_s.clone())?)?, &py::truediv(&py::mul(&v_v.clone(), &v_v.clone())?, &v_s.clone())?)?)?)?;
        v_m = py::add(&v_m.clone(), &py::mul(&v_k.clone(), &v_v.clone())?)?;
        v_p = py::mul(&py::sub(&Py::Int(1i64), &v_k.clone())?, &v_pp.clone())?;
        let _ = py::method(&v_ms.clone(), "append", &[v_m.clone()])?;
        let _ = py::method(&v_ps.clone(), "append", &[v_p.clone()])?;
    }
    return Ok(Py::tuple(vec![v_ms.clone(), v_ps.clone(), v_ll.clone(), v_k.clone(), v_q.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lstsq(mut v_X: Py, mut v_y: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_Rinv: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_big: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_diag: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_norm: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_resid: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vn: Py = Py::None;
    let t4222 = (Py::tuple(vec![py::len(&v_X.clone())?, py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?])).items()?;
    if t4222.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4222[0].clone();
    v_p = t4222[1].clone();
    if ({ let t4223 = v_n.clone(); Py::Bool({ let t4224 = v_p.clone(); if py::le(&t4223, &t4224)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("too few observations for the regression"))));
    }
    v_A = { let mut out = vec![]; for t4225 in py::iter(&(v_X.clone()))? { let mut v_r: Py; 
v_r = t4225;
out.push(Py::list((py::map(&py::func(|a: &[Py]| -> R { let x = a.first().cloned().ok_or_else(|| PyErr::Type("float() takes one argument".into()))?; py::float(&x) }), &v_r.clone())?).items()?)); } Py::list(out) };
    v_b = Py::list((py::map(&py::func(|a: &[Py]| -> R { let x = a.first().cloned().ok_or_else(|| PyErr::Type("float() takes one argument".into()))?; py::float(&x) }), &v_y.clone())?).items()?);
    for t4226 in py::range_iter(&[(v_p.clone()).i()?]) {
        v_j = t4226;
        v_norm = py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t4227 in py::range_iter(&[(v_j.clone()).i()?, (v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4227;
out.push(py::pow(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?;
        if ({ let t4228 = v_norm.clone(); Py::Bool({ let t4229 = Py::Int(0i64); if py::eq(&t4228, &t4229) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the regressors are collinear"))));
        }
        v_alpha = (if ({ let t4230 = py::getitem(&py::getitem(&v_A.clone(), &v_j.clone())?, &v_j.clone())?; Py::Bool({ let t4231 = Py::Int(0i64); if py::ge(&t4230, &t4231)? { true } else { false } }) }).truthy() { py::neg(&v_norm.clone())? } else { v_norm.clone() });
        v_v = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
        py::setitem(&v_v.clone(), &v_j.clone(), py::sub(&py::getitem(&py::getitem(&v_A.clone(), &v_j.clone())?, &v_j.clone())?, &v_alpha.clone())?)?;
        for t4232 in py::range_iter(&[(py::add(&v_j.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_i = t4232;
            py::setitem(&v_v.clone(), &v_i.clone(), py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?)?;
        }
        v_vn = py::fsum(&{ let mut out = vec![]; for t4233 in py::range_iter(&[(v_j.clone()).i()?, (v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4233;
out.push(py::pow(&py::getitem(&v_v.clone(), &v_i.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
        if ({ let t4234 = v_vn.clone(); Py::Bool({ let t4235 = Py::Int(0i64); if py::eq(&t4234, &t4235) { true } else { false } }) }).truthy() {
            continue;
        }
        for t4236 in py::range_iter(&[(v_j.clone()).i()?, (v_p.clone()).i()?]) {
            v_k = t4236;
            v_f = py::truediv(&py::mul(&Py::Int(2i64), &py::fsum(&{ let mut out = vec![]; for t4237 in py::range_iter(&[(v_j.clone()).i()?, (v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4237;
out.push(py::mul(&py::getitem(&v_v.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_k.clone())?)?); } Py::list(out) })?)?, &v_vn.clone())?;
            for t4238 in py::range_iter(&[(v_j.clone()).i()?, (v_n.clone()).i()?]) {
                v_i = t4238;
                let t4239 = py::getitem(&v_A.clone(), &v_i.clone())?; let t4240 = v_k.clone();
                py::setitem(&t4239, &t4240, py::sub(&py::getitem(&t4239, &t4240)?, &py::mul(&v_f.clone(), &py::getitem(&v_v.clone(), &v_i.clone())?)?)?)?;
            }
        }
        v_f = py::truediv(&py::mul(&Py::Int(2i64), &py::fsum(&{ let mut out = vec![]; for t4241 in py::range_iter(&[(v_j.clone()).i()?, (v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4241;
out.push(py::mul(&py::getitem(&v_v.clone(), &v_i.clone())?, &py::getitem(&v_b.clone(), &v_i.clone())?)?); } Py::list(out) })?)?, &v_vn.clone())?;
        for t4242 in py::range_iter(&[(v_j.clone()).i()?, (v_n.clone()).i()?]) {
            v_i = t4242;
            let t4243 = v_b.clone(); let t4244 = v_i.clone();
            py::setitem(&t4243, &t4244, py::sub(&py::getitem(&t4243, &t4244)?, &py::mul(&v_f.clone(), &py::getitem(&v_v.clone(), &v_i.clone())?)?)?)?;
        }
    }
    v_big = py::minmax(&[{ let mut out = vec![]; for t4245 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_i: Py; 
v_i = t4245;
out.push(py::abs(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_i.clone())?)?); } Py::list(out) }], true)?;
    if (Py::Bool(({ let mut out = vec![]; for t4246 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_i: Py; 
v_i = t4246;
out.push({ let t4247 = py::abs(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_i.clone())?)?; Py::Bool({ let t4248 = py::mul(&Py::Float(1e-11), &v_big.clone())?; if py::le(&t4247, &t4248)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the regressors are collinear"))));
    }
    v_beta = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_p.clone())?;
    for t4249 in py::range_iter(&[(py::sub(&v_p.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_i = t4249;
        py::setitem(&v_beta.clone(), &v_i.clone(), py::truediv(&py::sub(&py::getitem(&v_b.clone(), &v_i.clone())?, &py::fsum(&{ let mut out = vec![]; for t4250 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_p.clone()).i()?]) { let mut v_j: Py; 
v_j = t4250;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_beta.clone(), &v_j.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_i.clone())?)?)?;
    }
    v_Rinv = { let mut out = vec![]; for t4251 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v__: Py; 
v__ = t4251;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_p.clone())?); } Py::list(out) };
    for t4252 in py::range_iter(&[(v_p.clone()).i()?]) {
        v_c = t4252;
        for t4253 in py::range_iter(&[(py::sub(&v_p.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
            v_i = t4253;
            v_s = py::sub(&(if ({ let t4254 = v_i.clone(); Py::Bool({ let t4255 = v_c.clone(); if py::eq(&t4254, &t4255) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.0) }), &py::fsum(&{ let mut out = vec![]; for t4256 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_p.clone()).i()?]) { let mut v_j: Py; 
v_j = t4256;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_Rinv.clone(), &v_j.clone())?, &v_c.clone())?)?); } Py::list(out) })?)?;
            py::setitem(&py::getitem(&v_Rinv.clone(), &v_i.clone())?, &v_c.clone(), py::truediv(&v_s.clone(), &py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_i.clone())?)?)?;
        }
    }
    v_diag = { let mut out = vec![]; for t4257 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_i: Py; 
v_i = t4257;
out.push(py::fsum(&{ let mut out = vec![]; for t4258 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_c: Py; 
v_c = t4258;
out.push(py::pow(&py::getitem(&py::getitem(&v_Rinv.clone(), &v_i.clone())?, &v_c.clone())?, &Py::Int(2i64))?); } Py::list(out) })?); } Py::list(out) };
    v_resid = { let mut out = vec![]; for t4259 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4259;
out.push(py::sub(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::fsum(&{ let mut out = vec![]; for t4260 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_j: Py; 
v_j = t4260;
out.push(py::mul(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_beta.clone(), &v_j.clone())?)?); } Py::list(out) })?)?); } Py::list(out) };
    return Ok(Py::tuple(vec![v_beta.clone(), v_resid.clone(), v_diag.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mean(mut v_x: Py) -> R {
    return Ok(py::truediv(&py::fsum(&v_x.clone())?, &py::len(&v_x.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__median(mut v_x: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = py::sorted(&v_x.clone(), (Py::Bool(false)).truthy())?;
    v_n = py::len(&v_s.clone())?;
    return Ok((if (py::modulo(&v_n.clone(), &Py::Int(2i64))?).truthy() { py::getitem(&v_s.clone(), &py::floordiv(&v_n.clone(), &Py::Int(2i64))?)? } else { py::truediv(&py::add(&py::getitem(&v_s.clone(), &py::sub(&py::floordiv(&v_n.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?, &py::getitem(&v_s.clone(), &py::floordiv(&v_n.clone(), &Py::Int(2i64))?)?)?, &Py::Float(2.0))? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_x: Py, mut v_n: Py, mut v_name: Py) -> R {
    if ({ let t4147 = py::len(&v_x.clone())?; Py::Bool({ let t4148 = v_n.clone(); if py::lt(&t4147, &t4148)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " needs at least ".to_string(), py::format(&v_n.clone(), "")?, " values".to_string()].concat()))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonconstant(mut v_x: Py, mut v_name: Py) -> R {
    if ({ let t3537 = py::minmax(&[v_x.clone()], true)?; Py::Bool({ let t3538 = py::minmax(&[v_x.clone()], false)?; if py::eq(&t3537, &t3538) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must not be constant".to_string()].concat()))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__num(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t4150 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t4150.truthy() { t4150 } else { { let t4149 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t4149.truthy() { t4149 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))));
    }
    return Ok(py::float(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__polymul(mut v_p: Py, mut v_q: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_out = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&py::add(&py::len(&v_p.clone())?, &py::len(&v_q.clone())?)?, &Py::Int(1i64))?)?;
    for t3645 in py::range_iter(&[(py::len(&v_p.clone())?).i()?]) {
        v_i = t3645;
        for t3646 in py::range_iter(&[(py::len(&v_q.clone())?).i()?]) {
            v_j = t3646;
            let t3647 = v_out.clone(); let t3648 = py::add(&v_i.clone(), &v_j.clone())?;
            py::setitem(&t3647, &t3648, py::iadd(&py::getitem(&t3647, &t3648)?, &py::mul(&py::getitem(&v_p.clone(), &v_i.clone())?, &py::getitem(&v_q.clone(), &v_j.clone())?)?)?)?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__portmanteau(mut v_x: Py, mut v_h: Py, mut v_ljung: Py, mut v_model_df: Py) -> R {
    let mut v_md: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(3i64), Py::str("the series"))?;
    let _ = f__nonconstant(v_x.clone(), Py::str("the series"))?;
    v_h = f__int(v_h.clone(), Py::str("the number of lags h"), Py::Int(1i64), py::sub(&py::len(&v_x.clone())?, &Py::Int(1i64))?)?;
    v_md = f__int(v_model_df.clone(), Py::str("the fitted-model degrees of freedom"), Py::Int(0i64), Py::None)?;
    if ({ let t2588 = v_md.clone(); Py::Bool({ let t2589 = v_h.clone(); if py::ge(&t2588, &t2589)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of lags h must exceed the fitted-model degrees of freedom"))));
    }
    v_n = py::len(&v_x.clone())?;
    v_r = f__acf_list(v_x.clone(), v_h.clone())?;
    v_q = (if (v_ljung.clone()).truthy() { py::mul(&py::mul(&v_n.clone(), &py::add(&v_n.clone(), &Py::Int(2i64))?)?, &py::fsum(&{ let mut out = vec![]; for t2596 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_h.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2596;
out.push(py::truediv(&py::pow(&py::getitem(&v_r.clone(), &v_k.clone())?, &Py::Int(2i64))?, &py::sub(&v_n.clone(), &v_k.clone())?)?); } Py::list(out) })?)? } else { py::mul(&v_n.clone(), &py::fsum(&{ let mut out = vec![]; for t2597 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_h.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2597;
out.push(py::pow(&py::getitem(&v_r.clone(), &v_k.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)? });
    return Ok(Py::dict(vec![(Py::str("q"), v_q.clone()), (Py::str("p_value"), f__r12(f__chi2_sf(v_q.clone(), py::sub(&v_h.clone(), &v_md.clone())?)?)?), (Py::str("df"), py::sub(&v_h.clone(), &v_md.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_v: Py, mut v_name: Py) -> R {
    v_v = f__num(v_v.clone(), v_name.clone())?;
    if ({ let t3390 = v_v.clone(); Py::Bool({ let t3391 = Py::Int(0i64); if py::le(&t3390, &t3391)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))));
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__positive(mut v_x: Py, mut v_name: Py) -> R {
    if (Py::Bool(({ let mut out = vec![]; for t3902 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3902;
out.push({ let t3903 = v_v.clone(); Py::Bool({ let t3904 = Py::Int(0i64); if py::le(&t3903, &t3904)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must contain positive values only".to_string()].concat()))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__psi(mut v_phi: Py, mut v_theta: Py, mut v_K: Py) -> R {
    let mut v_j: Py = Py::None;
    let mut v_psi: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_psi = Py::list(vec![Py::Float(1.0)]);
    for t3231 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) {
        v_j = t3231;
        v_v = (if ({ let t3232 = v_j.clone(); Py::Bool({ let t3233 = py::len(&v_theta.clone())?; if py::le(&t3232, &t3233)? { true } else { false } }) }).truthy() { py::getitem(&v_theta.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)? } else { Py::Float(0.0) });
        v_v = py::iadd(&v_v, &py::fsum(&{ let mut out = vec![]; for t3234 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::minmax(&[v_j.clone(), py::len(&v_phi.clone())?], false)?, &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t3234;
out.push(py::mul(&py::getitem(&v_phi.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_psi.clone(), &py::sub(&v_j.clone(), &v_i.clone())?)?)?); } Py::list(out) })?)?;
        let _ = py::method(&v_psi.clone(), "append", &[v_v.clone()])?;
    }
    return Ok(v_psi.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__quad_inverse_roots(mut v_b: Py, mut v_c: Py) -> R {
    let mut v_disc: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_disc = py::sub(&py::mul(&v_b.clone(), &v_b.clone())?, &py::mul(&Py::Int(4i64), &v_c.clone())?)?;
    if ({ let t2687 = v_disc.clone(); Py::Bool({ let t2688 = Py::Int(0i64); if py::ge(&t2687, &t2688)? { true } else { false } }) }).truthy() {
        v_s = py::math1("sqrt", &v_disc.clone())?;
        return Ok(Py::tuple(vec![py::minmax(&[py::abs(&py::truediv(&py::add(&py::neg(&v_b.clone())?, &v_s.clone())?, &Py::Int(2i64))?)?, py::abs(&py::truediv(&py::sub(&py::neg(&v_b.clone())?, &v_s.clone())?, &Py::Int(2i64))?)?], true)?, Py::str("real")]));
    }
    return Ok(Py::tuple(vec![py::math1("sqrt", &v_c.clone())?, Py::str("complex")]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r12(mut v_v: Py) -> R {
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_v.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__reflection(mut v_coefs: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_ks: Py = Py::None;
    let mut v_m: Py = Py::None;
    v_a = Py::list((v_coefs.clone()).items()?);
    v_ks = Py::list(vec![]);
    for t3634 in py::range_iter(&[(py::len(&v_a.clone())?).i()?, (Py::Int(0i64)).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_m = t3634;
        v_k = py::getitem(&v_a.clone(), &py::sub(&v_m.clone(), &Py::Int(1i64))?)?;
        let _ = py::method(&v_ks.clone(), "append", &[v_k.clone()])?;
        if ({ let t3635 = py::abs(&v_k.clone())?; Py::Bool({ let t3636 = Py::Int(1i64); if py::ge(&t3635, &t3636)? { true } else { false } }) }).truthy() {
            return Ok(v_ks.clone());
        }
        v_a = { let mut out = vec![]; for t3637 in py::range_iter(&[(py::sub(&v_m.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t3637;
out.push(py::truediv(&py::add(&py::getitem(&v_a.clone(), &v_j.clone())?, &py::mul(&v_k.clone(), &py::getitem(&v_a.clone(), &py::sub(&py::sub(&v_m.clone(), &Py::Int(2i64))?, &v_j.clone())?)?)?)?, &py::sub(&Py::Int(1i64), &py::mul(&v_k.clone(), &v_k.clone())?)?)?); } Py::list(out) };
    }
    return Ok(v_ks.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__same_len(mut v_a: Py, mut v_b: Py, mut v_na: Py, mut v_nb: Py) -> R {
    if ({ let t4108 = py::len(&v_a.clone())?; Py::Bool({ let t4109 = py::len(&v_b.clone())?; if py::ne(&t4108, &t4109) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_na.clone(), "")?, " and ".to_string(), py::format(&v_nb.clone(), "")?, " must have the same length".to_string()].concat()))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sarima_polys(mut v_phi: Py, mut v_theta: Py, mut v_d: Py, mut v_sphi: Py, mut v_stheta: Py, mut v_D: Py, mut v_m: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_ar: Py = Py::None;
    let mut v_ma: Py = Py::None;
    v_ar = py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t3642 in py::iter(&(v_phi.clone()))? { let mut v_v: Py; 
v_v = t3642;
out.push(py::neg(&v_v.clone())?); } Py::list(out) })?;
    if ({ let t3643 = py::len(&v_sphi.clone())?; Py::Bool({ let t3644 = Py::Int(0i64); if py::gt(&t3643, &t3644)? { true } else { false } }) }).truthy() {
        v_ar = f__polymul(v_ar.clone(), f__seasonal_poly(v_sphi.clone(), v_m.clone(), py::neg(&Py::Float(1.0))?)?)?;
    }
    for t3650 in py::range_iter(&[(v_d.clone()).i()?]) {
        v__ = t3650;
        v_ar = f__polymul(v_ar.clone(), Py::list(vec![Py::Float(1.0), py::neg(&Py::Float(1.0))?]))?;
    }
    for t3651 in py::range_iter(&[(v_D.clone()).i()?]) {
        v__ = t3651;
        v_ar = f__polymul(v_ar.clone(), f__seasonal_poly(Py::list(vec![Py::Float(1.0)]), v_m.clone(), py::neg(&Py::Float(1.0))?)?)?;
    }
    v_ma = py::add(&Py::list(vec![Py::Float(1.0)]), &Py::list((v_theta.clone()).items()?))?;
    if ({ let t3652 = py::len(&v_stheta.clone())?; Py::Bool({ let t3653 = Py::Int(0i64); if py::gt(&t3652, &t3653)? { true } else { false } }) }).truthy() {
        v_ma = f__polymul(v_ma.clone(), f__seasonal_poly(v_stheta.clone(), v_m.clone(), Py::Float(1.0))?)?;
    }
    return Ok(Py::tuple(vec![f__trimz(v_ar.clone())?, f__trimz(v_ma.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__seasonal_poly(mut v_coefs: Py, mut v_m: Py, mut v_sign: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_out = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&py::mul(&v_m.clone(), &py::len(&v_coefs.clone())?)?, &Py::Int(1i64))?)?;
    py::setitem(&v_out.clone(), &Py::Int(0i64), Py::Float(1.0))?;
    for t3649 in py::range_iter(&[(py::len(&v_coefs.clone())?).i()?]) {
        v_k = t3649;
        py::setitem(&v_out.clone(), &py::mul(&v_m.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, py::mul(&v_sign.clone(), &py::getitem(&v_coefs.clone(), &v_k.clone())?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__seg_cost(mut v_x: Py) -> R {
    return Ok((if (v_x.clone()).truthy() { f__sse(v_x.clone())? } else { Py::Float(0.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__solve(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_piv: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_M = { let mut out = vec![]; for t2767 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2767;
out.push(py::add(&Py::list((py::map(&py::func(|a: &[Py]| -> R { let x = a.first().cloned().ok_or_else(|| PyErr::Type("float() takes one argument".into()))?; py::float(&x) }), &py::getitem(&v_A.clone(), &v_i.clone())?)?).items()?), &Py::list(vec![py::float(&py::getitem(&v_b.clone(), &v_i.clone())?)?]))?); } Py::list(out) };
    for t2768 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t2768;
        v_piv = py::minmax_key(&[Py::list((py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_A = v_A.clone(); let v_M = v_M.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        if ({ let t2769 = py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_piv.clone())?, &v_c.clone())?)?; Py::Bool({ let t2770 = Py::Float(1e-300); if py::lt(&t2769, &t2770)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the linear system is singular"))));
        }
        let t2771 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_piv.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])).items()?;
        if t2771.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_M.clone(), &v_c.clone(), t2771[0].clone())?;
        py::setitem(&v_M.clone(), &v_piv.clone(), t2771[1].clone())?;
        for t2772 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t2772;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
            if (v_f.clone()).truthy() {
                for t2773 in py::range_iter(&[(v_c.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_j = t2773;
                    let t2774 = py::getitem(&v_M.clone(), &v_r.clone())?; let t2775 = v_j.clone();
                    py::setitem(&t2774, &t2775, py::sub(&py::getitem(&t2774, &t2775)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_j.clone())?)?)?)?;
                }
            }
        }
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t2776 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_i = t2776;
        py::setitem(&v_x.clone(), &v_i.clone(), py::truediv(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_n.clone())?, &py::fsum(&{ let mut out = vec![]; for t2777 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2777;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_i.clone())?)?)?;
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sse(mut v_x: Py) -> R {
    let mut v_m: Py = Py::None;
    v_m = f__mean(v_x.clone())?;
    return Ok(py::fsum(&{ let mut out = vec![]; for t4310 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t4310;
out.push(py::pow(&py::sub(&v_v.clone(), &v_m.clone())?, &Py::Int(2i64))?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__stationary(mut v_phi: Py) -> R {
    return Ok(Py::Bool(({ let mut out = vec![]; for t3638 in py::iter(&(f__reflection(v_phi.clone())?))? { let mut v_k: Py; 
v_k = t3638;
out.push({ let t3639 = py::abs(&v_k.clone())?; Py::Bool({ let t3640 = Py::Int(1i64); if py::lt(&t3639, &t3640)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__t(mut v_n: Py) -> R {
    return Ok({ let mut out = vec![]; for t3319 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t3319;
out.push(py::float(&v_i.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__trimz(mut v_c: Py) -> R {
    v_c = Py::list((v_c.clone()).items()?);
    while ({ let t3658 = { let t3654 = py::len(&v_c.clone())?; Py::Bool({ let t3655 = Py::Int(1i64); if py::gt(&t3654, &t3655)? { true } else { false } }) }; if !t3658.truthy() { t3658 } else { { let t3656 = py::getitem(&v_c.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t3657 = Py::Int(0i64); if py::eq(&t3656, &t3657) { true } else { false } }) } } }).truthy() {
        let _ = py::method(&v_c.clone(), "pop", &[])?;
    }
    return Ok(v_c.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__unit(mut v_v: Py, mut v_name: Py) -> R {
    v_v = f__num(v_v.clone(), v_name.clone())?;
    if (py::not(&{ let t4027 = Py::Int(0i64); Py::Bool({ let t4028 = v_v.clone(); if py::lt(&t4027, &t4028)? { { let t4029 = Py::Int(1i64); if py::le(&t4028, &t4029)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must lie in (0, 1]".to_string()].concat()))));
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__xc_in(mut v_x: Py, mut v_y: Py) -> R {
    let t1177 = (Py::tuple(vec![f__fin(v_x.clone(), Py::str("x"))?, f__fin(v_y.clone(), Py::str("y"))?])).items()?;
    if t1177.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1177[0].clone();
    v_y = t1177[1].clone();
    let _ = f__same_len(v_x.clone(), v_y.clone(), Py::str("x"), Py::str("y"))?;
    let _ = f__need(v_x.clone(), Py::Int(3i64), Py::str("x"))?;
    let _ = f__nonconstant(v_x.clone(), Py::str("x"))?;
    let _ = f__nonconstant(v_y.clone(), Py::str("y"))?;
    return Ok(Py::tuple(vec![v_x.clone(), v_y.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_acf(mut v_x: Py, mut v_k: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3452 = (f__acf_in(v_x.clone(), v_k.clone(), Py::Int(1i64))?).items()?;
    if t3452.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3452[0].clone();
    v_k = t3452[1].clone();
    v_r = f__acf_list(v_x.clone(), v_k.clone())?;
    return Ok(Py::dict(vec![(Py::str("acf"), py::getitem(&v_r.clone(), &v_k.clone())?), (Py::str("acf_list"), py::slice(&v_r.clone(), Some((Py::Int(1i64)).i()?), None, None)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_acov(mut v_x: Py, mut v_k: Py) -> R {
    let mut v_m: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the series"))?;
    v_k = f__int(v_k.clone(), Py::str("the lag k"), Py::Int(0i64), py::sub(&py::len(&v_x.clone())?, &Py::Int(1i64))?)?;
    v_m = f__mean(v_x.clone())?;
    return Ok(Py::dict(vec![(Py::str("acov"), f__acov(v_x.clone(), v_k.clone(), v_m.clone())?), (Py::str("gamma0"), f__acov(v_x.clone(), Py::Int(0i64), v_m.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_adjusted(mut v_x: Py, mut v_m: Py, mut v_mult: Py) -> R {
    let mut v__avg: Py = Py::None;
    let mut v_adj: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let t3916 = (f__decompose(v_x.clone(), v_m.clone(), v_mult.clone())?).items()?;
    if t3916.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3916[0].clone();
    v_idx = t3916[1].clone();
    v__avg = t3916[2].clone();
    v_m = py::len(&v_idx.clone())?;
    v_adj = { let mut out = vec![]; for t3917 in py::iter(&(py::enumerate(&v_x.clone(), 0)?))? { let mut v_t: Py; let mut v_v: Py; 
let t3918 = (t3917).items()?;
if t3918.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t3918[0].clone();
v_v = t3918[1].clone();
out.push((if (v_mult.clone()).truthy() { py::truediv(&v_v.clone(), &py::getitem(&v_idx.clone(), &py::modulo(&v_t.clone(), &v_m.clone())?)?)? } else { py::sub(&v_v.clone(), &py::getitem(&v_idx.clone(), &py::modulo(&v_t.clone(), &v_m.clone())?)?)? })); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("adjusted"), v_adj.clone()), (Py::str("last"), py::getitem(&v_adj.clone(), &py::neg(&Py::Int(1i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ar1_fc(mut v_x: Py, mut v_phi: Py, mut v_mu: Py, mut v_h: Py, mut v_sigma2: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3 = (Py::tuple(vec![f__num(v_x.clone(), Py::str("the last value"))?, f__num(v_phi.clone(), Py::str("phi"))?, f__num(v_mu.clone(), Py::str("the mean"))?])).items()?;
    if t3.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3[0].clone();
    v_phi = t3[1].clone();
    v_mu = t3[2].clone();
    if (py::not(&{ let t4 = py::abs(&v_phi.clone())?; Py::Bool({ let t5 = Py::Int(1i64); if py::lt(&t4, &t5)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("|phi| must be less than 1 (stationarity)"))));
    }
    v_h = f__int(v_h.clone(), Py::str("the horizon h"), Py::Int(1i64), Py::Int(100000i64))?;
    v_out = Py::dict(vec![(Py::str("forecast"), f__r12(py::add(&v_mu.clone(), &py::mul(&py::pow(&v_phi.clone(), &v_h.clone())?, &py::sub(&v_x.clone(), &v_mu.clone())?)?)?)?)]);
    if ({ let t21 = v_sigma2.clone(); Py::Bool({ let t22 = Py::None; if !py::is(&t21, &t22) { true } else { false } }) }).truthy() {
        v_s2 = f__pos(v_sigma2.clone(), Py::str("the innovation variance"))?;
        v_v = py::truediv(&py::mul(&v_s2.clone(), &py::sub(&Py::Int(1i64), &py::pow(&v_phi.clone(), &py::mul(&Py::Int(2i64), &v_h.clone())?)?)?)?, &py::sub(&Py::Int(1i64), &py::mul(&v_phi.clone(), &v_phi.clone())?)?)?;
        let _ = { let t25 = v_out.clone(); py::setitem(&t25, &Py::str("variance"), f__r12(v_v.clone())?)?; py::setitem(&t25, &Py::str("lower"), f__r12(py::sub(&py::getitem(&v_out.clone(), &Py::str("forecast"))?, &py::mul(&c__Z975()?, &py::math1("sqrt", &v_v.clone())?)?)?)?)?; py::setitem(&t25, &Py::str("upper"), f__r12(py::add(&py::getitem(&v_out.clone(), &Py::str("forecast"))?, &py::mul(&c__Z975()?, &py::math1("sqrt", &v_v.clone())?)?)?)?)?; Py::None };
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ar1_props(mut v_phi: Py, mut v_sigma2: Py, mut v_k: Py, mut v_c: Py) -> R {
    let mut v_g0: Py = Py::None;
    let mut v_s2: Py = Py::None;
    v_phi = f__num(v_phi.clone(), Py::str("phi"))?;
    if (py::not(&{ let t53 = py::abs(&v_phi.clone())?; Py::Bool({ let t54 = Py::Int(1i64); if py::lt(&t53, &t54)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("|phi| must be less than 1 (stationarity)"))));
    }
    v_s2 = f__pos(v_sigma2.clone(), Py::str("the innovation variance"))?;
    v_k = f__int(v_k.clone(), Py::str("the lag k"), Py::Int(0i64), Py::Int(500i64))?;
    v_c = f__num(v_c.clone(), Py::str("the constant"))?;
    v_g0 = py::truediv(&v_s2.clone(), &py::sub(&Py::Int(1i64), &py::mul(&v_phi.clone(), &v_phi.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("mean"), py::truediv(&v_c.clone(), &py::sub(&Py::Int(1i64), &v_phi.clone())?)?), (Py::str("variance"), v_g0.clone()), (Py::str("acf_k"), py::pow(&v_phi.clone(), &v_k.clone())?), (Py::str("acov_k"), py::mul(&v_g0.clone(), &py::pow(&v_phi.clone(), &v_k.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ar2_acf(mut v_phi1: Py, mut v_phi2: Py, mut v_K: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t132 = (Py::tuple(vec![f__num(v_phi1.clone(), Py::str("phi1"))?, f__num(v_phi2.clone(), Py::str("phi2"))?])).items()?;
    if t132.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_phi1 = t132[0].clone();
    v_phi2 = t132[1].clone();
    if (py::not(&f__stationary(Py::list(vec![v_phi1.clone(), v_phi2.clone()]))?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the AR(2) process must be stationary"))));
    }
    v_K = f__int(v_K.clone(), Py::str("the number of lags K"), Py::Int(1i64), Py::Int(500i64))?;
    v_r = Py::list(vec![Py::Float(1.0), py::truediv(&v_phi1.clone(), &py::sub(&Py::Int(1i64), &v_phi2.clone())?)?]);
    for t155 in py::range_iter(&[(Py::Int(2i64)).i()?, (py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) {
        v__ = t155;
        let _ = py::method(&v_r.clone(), "append", &[py::add(&py::mul(&v_phi1.clone(), &py::getitem(&v_r.clone(), &py::neg(&Py::Int(1i64))?)?)?, &py::mul(&v_phi2.clone(), &py::getitem(&v_r.clone(), &py::neg(&Py::Int(2i64))?)?)?)?])?;
    }
    return Ok(Py::dict(vec![(Py::str("acf"), py::slice(&v_r.clone(), Some((Py::Int(1i64)).i()?), Some((py::add(&v_K.clone(), &Py::Int(1i64))?).i()?), None)?), (Py::str("rho1"), py::getitem(&v_r.clone(), &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ar2_stat(mut v_phi1: Py, mut v_phi2: Py) -> R {
    let mut v_kind: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t99 = (Py::tuple(vec![f__num(v_phi1.clone(), Py::str("phi1"))?, f__num(v_phi2.clone(), Py::str("phi2"))?])).items()?;
    if t99.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_phi1 = t99[0].clone();
    v_phi2 = t99[1].clone();
    let t102 = (f__quad_inverse_roots(py::neg(&v_phi1.clone())?, py::neg(&v_phi2.clone())?)?).items()?;
    if t102.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t102[0].clone();
    v_kind = t102[1].clone();
    return Ok(Py::dict(vec![(Py::str("stationary"), (if ({ let t103 = v_m.clone(); Py::Bool({ let t104 = Py::Int(1i64); if py::lt(&t103, &t104)? { true } else { false } }) }).truthy() { Py::str("yes") } else { Py::str("no") })), (Py::str("max_modulus"), v_m.clone()), (Py::str("roots"), v_kind.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ar_forecast(mut v_x: Py, mut v_phi: Py, mut v_c: Py, mut v_h: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_hist: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    v_phi = f__fin(v_phi.clone(), Py::str("the coefficients"))?;
    v_c = f__num(v_c.clone(), Py::str("the constant"))?;
    v_h = f__int(v_h.clone(), Py::str("the horizon h"), Py::Int(1i64), Py::Int(500i64))?;
    v_p = py::len(&v_phi.clone())?;
    if ({ let t201 = py::len(&v_x.clone())?; Py::Bool({ let t202 = v_p.clone(); if py::lt(&t201, &t202)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the series needs at least p values"))));
    }
    v_hist = Py::list((v_x.clone()).items()?);
    v_out = Py::list(vec![]);
    for t203 in py::range_iter(&[(v_h.clone()).i()?]) {
        v__ = t203;
        v_v = py::add(&v_c.clone(), &py::fsum(&{ let mut out = vec![]; for t204 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_j: Py; 
v_j = t204;
out.push(py::mul(&py::getitem(&v_phi.clone(), &v_j.clone())?, &py::getitem(&v_hist.clone(), &py::sub(&py::neg(&Py::Int(1i64))?, &v_j.clone())?)?)?); } Py::list(out) })?)?;
        let _ = py::method(&v_hist.clone(), "append", &[v_v.clone()])?;
        let _ = py::method(&v_out.clone(), "append", &[v_v.clone()])?;
    }
    return Ok(Py::dict(vec![(Py::str("forecasts"), v_out.clone()), (Py::str("forecast"), py::getitem(&v_out.clone(), &py::neg(&Py::Int(1i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ar_ols(mut v_x: Py, mut v_p: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v__dg: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    v_p = f__int(v_p.clone(), Py::str("the order p"), Py::Int(1i64), Py::Int(10i64))?;
    if ({ let t248 = py::sub(&py::len(&v_x.clone())?, &v_p.clone())?; Py::Bool({ let t249 = py::add(&v_p.clone(), &Py::Int(2i64))?; if py::le(&t248, &t249)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("too few observations: need n - p > p + 2"))));
    }
    v_X = { let mut out = vec![]; for t250 in py::range_iter(&[(v_p.clone()).i()?, (py::len(&v_x.clone())?).i()?]) { let mut v_t: Py; 
v_t = t250;
out.push(py::add(&Py::list(vec![Py::Float(1.0)]), &{ let mut out = vec![]; for t251 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_p.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t251;
out.push(py::getitem(&v_x.clone(), &py::sub(&v_t.clone(), &v_j.clone())?)?); } Py::list(out) })?); } Py::list(out) };
    v_y = py::slice(&v_x.clone(), Some((v_p.clone()).i()?), None, None)?;
    let t291 = (f__lstsq(v_X.clone(), v_y.clone())?).items()?;
    if t291.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_beta = t291[0].clone();
    v_res = t291[1].clone();
    v__dg = t291[2].clone();
    return Ok(Py::dict(vec![(Py::str("const"), py::getitem(&v_beta.clone(), &Py::Int(0i64))?), (Py::str("phi"), py::slice(&v_beta.clone(), Some((Py::Int(1i64)).i()?), None, None)?), (Py::str("phi1"), py::getitem(&v_beta.clone(), &Py::Int(1i64))?), (Py::str("sigma2"), py::truediv(&py::fsum(&{ let mut out = vec![]; for t292 in py::iter(&(v_res.clone()))? { let mut v_e: Py; 
v_e = t292;
out.push(py::mul(&v_e.clone(), &v_e.clone())?); } Py::list(out) })?, &py::len(&v_y.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ar_yw(mut v_x: Py, mut v_p: Py) -> R {
    let mut v__pacf: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(4i64), Py::str("the series"))?;
    let _ = f__nonconstant(v_x.clone(), Py::str("the series"))?;
    v_p = f__int(v_p.clone(), Py::str("the order p"), Py::Int(1i64), Py::Int(10i64))?;
    if ({ let t340 = v_p.clone(); Py::Bool({ let t341 = py::sub(&py::len(&v_x.clone())?, &Py::Int(1i64))?; if py::ge(&t340, &t341)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the order p must be less than n - 1"))));
    }
    v_r = f__acf_list(v_x.clone(), v_p.clone())?;
    let t354 = (f__levinson(v_r.clone(), v_p.clone())?).items()?;
    if t354.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t354[0].clone();
    v__pacf = t354[1].clone();
    v_v = t354[2].clone();
    return Ok(Py::dict(vec![(Py::str("phi"), v_phi.clone()), (Py::str("phi1"), py::getitem(&v_phi.clone(), &Py::Int(0i64))?), (Py::str("sigma2"), py::mul(&f__acov(v_x.clone(), Py::Int(0i64), Py::None)?, &v_v.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_arima_forecast(mut v_x: Py, mut v_phi: Py, mut v_h: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_level: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    v_phi = f__fin(v_phi.clone(), Py::str("the coefficients"))?;
    v_h = f__int(v_h.clone(), Py::str("the horizon h"), Py::Int(1i64), Py::Int(500i64))?;
    v_p = py::len(&v_phi.clone())?;
    if ({ let t398 = py::len(&v_x.clone())?; Py::Bool({ let t399 = py::add(&v_p.clone(), &Py::Int(2i64))?; if py::lt(&t398, &t399)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the series needs at least p + 2 values"))));
    }
    if (py::not(&f__stationary(v_phi.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the AR part must be stationary (every root outside the unit circle)"))));
    }
    v_dx = { let mut out = vec![]; for t407 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_x.clone())?).i()?]) { let mut v_t: Py; 
v_t = t407;
out.push(py::sub(&py::getitem(&v_x.clone(), &v_t.clone())?, &py::getitem(&v_x.clone(), &py::sub(&v_t.clone(), &Py::Int(1i64))?)?)?); } Py::list(out) };
    let t408 = (Py::tuple(vec![py::getitem(&v_x.clone(), &py::neg(&Py::Int(1i64))?)?, Py::list(vec![])])).items()?;
    if t408.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_level = t408[0].clone();
    v_out = t408[1].clone();
    for t409 in py::range_iter(&[(v_h.clone()).i()?]) {
        v__ = t409;
        v_v = py::fsum(&{ let mut out = vec![]; for t410 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_j: Py; 
v_j = t410;
out.push(py::mul(&py::getitem(&v_phi.clone(), &v_j.clone())?, &py::getitem(&v_dx.clone(), &py::sub(&py::neg(&Py::Int(1i64))?, &v_j.clone())?)?)?); } Py::list(out) })?;
        let _ = py::method(&v_dx.clone(), "append", &[v_v.clone()])?;
        v_level = py::iadd(&v_level, &v_v.clone())?;
        let _ = py::method(&v_out.clone(), "append", &[v_level.clone()])?;
    }
    return Ok(Py::dict(vec![(Py::str("forecasts"), v_out.clone()), (Py::str("forecast"), py::getitem(&v_out.clone(), &py::neg(&Py::Int(1i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_arma11_acf(mut v_phi: Py, mut v_theta: Py, mut v_K: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_acf: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let t438 = (Py::tuple(vec![f__num(v_phi.clone(), Py::str("phi"))?, f__num(v_theta.clone(), Py::str("theta"))?])).items()?;
    if t438.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t438[0].clone();
    v_theta = t438[1].clone();
    if (py::not(&{ let t439 = py::abs(&v_phi.clone())?; Py::Bool({ let t440 = Py::Int(1i64); if py::lt(&t439, &t440)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("|phi| must be less than 1 (stationarity)"))));
    }
    v_K = f__int(v_K.clone(), Py::str("the number of lags K"), Py::Int(1i64), Py::Int(200i64))?;
    v_r1 = py::truediv(&py::mul(&py::add(&Py::Int(1i64), &py::mul(&v_phi.clone(), &v_theta.clone())?)?, &py::add(&v_phi.clone(), &v_theta.clone())?)?, &py::add(&py::add(&Py::Int(1i64), &py::mul(&py::mul(&Py::Int(2i64), &v_phi.clone())?, &v_theta.clone())?)?, &py::mul(&v_theta.clone(), &v_theta.clone())?)?)?;
    v_acf = Py::list(vec![v_r1.clone()]);
    for t456 in py::range_iter(&[(py::sub(&v_K.clone(), &Py::Int(1i64))?).i()?]) {
        v__ = t456;
        let _ = py::method(&v_acf.clone(), "append", &[py::mul(&v_phi.clone(), &py::getitem(&v_acf.clone(), &py::neg(&Py::Int(1i64))?)?)?])?;
    }
    return Ok(Py::dict(vec![(Py::str("acf"), v_acf.clone()), (Py::str("rho1"), v_r1.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_arma_acf(mut v_phi: Py, mut v_theta: Py, mut v_K: Py) -> R {
    let mut v_acf: Py = Py::None;
    let mut v_g: Py = Py::None;
    let t500 = (f__arma_check(v_phi.clone(), v_theta.clone(), Py::Bool(false))?).items()?;
    if t500.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t500[0].clone();
    v_theta = t500[1].clone();
    v_K = f__int(v_K.clone(), Py::str("the number of lags K"), Py::Int(1i64), Py::Int(200i64))?;
    v_g = f__arma_acov(v_phi.clone(), v_theta.clone(), Py::Float(1.0), v_K.clone())?;
    v_acf = { let mut out = vec![]; for t545 in py::iter(&(py::slice(&v_g.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_v: Py; 
v_v = t545;
out.push(py::truediv(&v_v.clone(), &py::getitem(&v_g.clone(), &Py::Int(0i64))?)?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("acf"), v_acf.clone()), (Py::str("rho1"), py::getitem(&v_acf.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_arma_pacf(mut v_phi: Py, mut v_theta: Py, mut v_K: Py) -> R {
    let mut v__p: Py = Py::None;
    let mut v__v: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_pacf: Py = Py::None;
    let t589 = (f__arma_check(v_phi.clone(), v_theta.clone(), Py::Bool(false))?).items()?;
    if t589.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t589[0].clone();
    v_theta = t589[1].clone();
    v_K = f__int(v_K.clone(), Py::str("the number of lags K"), Py::Int(1i64), Py::Int(200i64))?;
    v_g = f__arma_acov(v_phi.clone(), v_theta.clone(), Py::Float(1.0), v_K.clone())?;
    let t641 = (f__levinson({ let mut out = vec![]; for t640 in py::iter(&(v_g.clone()))? { let mut v_v: Py; 
v_v = t640;
out.push(py::truediv(&v_v.clone(), &py::getitem(&v_g.clone(), &Py::Int(0i64))?)?); } Py::list(out) }, v_K.clone())?).items()?;
    if t641.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v__p = t641[0].clone();
    v_pacf = t641[1].clone();
    v__v = t641[2].clone();
    return Ok(Py::dict(vec![(Py::str("pacf"), v_pacf.clone()), (Py::str("pacf1"), py::getitem(&v_pacf.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_arma_var(mut v_phi: Py, mut v_theta: Py, mut v_sigma2: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let t685 = (f__arma_check(v_phi.clone(), v_theta.clone(), Py::Bool(false))?).items()?;
    if t685.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t685[0].clone();
    v_theta = t685[1].clone();
    v_s2 = f__pos(v_sigma2.clone(), Py::str("the innovation variance"))?;
    v_g = f__arma_acov(v_phi.clone(), v_theta.clone(), v_s2.clone(), Py::Int(1i64))?;
    return Ok(Py::dict(vec![(Py::str("gamma0"), py::getitem(&v_g.clone(), &Py::Int(0i64))?), (Py::str("gamma1"), py::getitem(&v_g.clone(), &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_avg_growth(mut v_x: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the series"))?;
    let _ = f__positive(v_x.clone(), Py::str("the series"))?;
    v_n = py::sub(&py::len(&v_x.clone())?, &Py::Int(1i64))?;
    v_g = py::pow(&py::truediv(&py::getitem(&v_x.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_x.clone(), &Py::Int(0i64))?)?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("rate"), f__r12(py::sub(&v_g.clone(), &Py::Int(1i64))?)?), (Py::str("rate_percent"), f__r12(py::mul(&Py::Int(100i64), &py::sub(&v_g.clone(), &Py::Int(1i64))?)?)?), (Py::str("total_growth"), f__r12(py::sub(&py::truediv(&py::getitem(&v_x.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_x.clone(), &Py::Int(0i64))?)?, &Py::Int(1i64))?)?), (Py::str("periods"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_bartlett(mut v_x: Py, mut v_k: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_se: Py = Py::None;
    let t869 = (f__acf_in(v_x.clone(), v_k.clone(), Py::Int(1i64))?).items()?;
    if t869.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t869[0].clone();
    v_k = t869[1].clone();
    v_r = f__acf_list(v_x.clone(), v_k.clone())?;
    v_se = py::math1("sqrt", &py::truediv(&py::add(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &py::fsum(&{ let mut out = vec![]; for t876 in py::iter(&(py::slice(&v_r.clone(), Some((Py::Int(1i64)).i()?), Some((v_k.clone()).i()?), None)?))? { let mut v_v: Py; 
v_v = t876;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?)?)?, &py::len(&v_x.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("se"), v_se.clone()), (Py::str("half_width_95"), py::mul(&c__Z975()?, &v_se.clone())?), (Py::str("acf"), py::getitem(&v_r.clone(), &v_k.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_best_lag(mut v_x: Py, mut v_y: Py, mut v_K: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t1184 = (f__xc_in(v_x.clone(), v_y.clone())?).items()?;
    if t1184.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1184[0].clone();
    v_y = t1184[1].clone();
    v_K = f__int(v_K.clone(), Py::str("the largest lag K"), Py::Int(0i64), py::sub(&py::len(&v_x.clone())?, &Py::Int(1i64))?)?;
    v_best = Py::None;
    for t1200 in py::iter(&(py::sorted_key(&Py::list((py::range_iter(&[(py::neg(&v_K.clone())?).i()?, (py::add(&v_K.clone(), &Py::Int(1i64))?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_K = v_K.clone(); let v_best = v_best.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_j: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::abs(&v_j.clone())?, py::neg(&v_j.clone())?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?))? {
        v_k = t1200;
        v_r = f__ccf(v_x.clone(), v_y.clone(), v_k.clone())?;
        if ({ let t1212 = { let t1208 = v_best.clone(); Py::Bool({ let t1209 = Py::None; if py::is(&t1208, &t1209) { true } else { false } }) }; if t1212.truthy() { t1212 } else { { let t1210 = py::abs(&v_r.clone())?; Py::Bool({ let t1211 = py::abs(&py::getitem(&v_best.clone(), &Py::Int(1i64))?)?; if py::gt(&t1210, &t1211)? { true } else { false } }) } } }).truthy() {
            v_best = Py::tuple(vec![v_k.clone(), v_r.clone()]);
        }
    }
    return Ok(Py::dict(vec![(Py::str("lag"), py::getitem(&v_best.clone(), &Py::Int(0i64))?), (Py::str("ccf"), py::getitem(&v_best.clone(), &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ccf(mut v_x: Py, mut v_y: Py, mut v_k: Py) -> R {
    let t1126 = (f__xc_in(v_x.clone(), v_y.clone())?).items()?;
    if t1126.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1126[0].clone();
    v_y = t1126[1].clone();
    v_k = f__int(v_k.clone(), Py::str("the lag k"), Py::Int(0i64), py::sub(&py::len(&v_x.clone())?, &Py::Int(1i64))?)?;
    return Ok(Py::dict(vec![(Py::str("ccf"), f__ccf(v_x.clone(), v_y.clone(), v_k.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_cma(mut v_x: Py, mut v_k: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    v_k = f__int(v_k.clone(), Py::str("the window k"), Py::Int(2i64), Py::None)?;
    v_h = py::floordiv(&v_k.clone(), &Py::Int(2i64))?;
    if ({ let t1002 = { let t996 = py::modulo(&v_k.clone(), &Py::Int(2i64))?; if !t996.truthy() { t996 } else { { let t994 = v_k.clone(); Py::Bool({ let t995 = py::len(&v_x.clone())?; if py::gt(&t994, &t995)? { true } else { false } }) } } }; if t1002.truthy() { t1002 } else { { let t1001 = { let t997 = py::modulo(&v_k.clone(), &Py::Int(2i64))?; Py::Bool({ let t998 = Py::Int(0i64); if py::eq(&t997, &t998) { true } else { false } }) }; if !t1001.truthy() { t1001 } else { { let t999 = py::add(&v_k.clone(), &Py::Int(1i64))?; Py::Bool({ let t1000 = py::len(&v_x.clone())?; if py::gt(&t999, &t1000)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the series is too short for a centred moving average of this length"))));
    }
    if (py::modulo(&v_k.clone(), &Py::Int(2i64))?).truthy() {
        v_vals = { let mut out = vec![]; for t1003 in py::range_iter(&[(v_h.clone()).i()?, (py::sub(&py::len(&v_x.clone())?, &v_h.clone())?).i()?]) { let mut v_i: Py; 
v_i = t1003;
out.push(py::truediv(&py::fsum(&py::slice(&v_x.clone(), Some((py::sub(&v_i.clone(), &v_h.clone())?).i()?), Some((py::add(&py::add(&v_i.clone(), &v_h.clone())?, &Py::Int(1i64))?).i()?), None)?)?, &v_k.clone())?); } Py::list(out) };
    } else {
        v_vals = { let mut out = vec![]; for t1004 in py::range_iter(&[(v_h.clone()).i()?, (py::sub(&py::len(&v_x.clone())?, &v_h.clone())?).i()?]) { let mut v_i: Py; 
v_i = t1004;
out.push(py::truediv(&py::add(&py::add(&py::mul(&Py::Float(0.5), &py::getitem(&v_x.clone(), &py::sub(&v_i.clone(), &v_h.clone())?)?)?, &py::fsum(&py::slice(&v_x.clone(), Some((py::add(&py::sub(&v_i.clone(), &v_h.clone())?, &Py::Int(1i64))?).i()?), Some((py::add(&v_i.clone(), &v_h.clone())?).i()?), None)?)?)?, &py::mul(&Py::Float(0.5), &py::getitem(&v_x.clone(), &py::add(&v_i.clone(), &v_h.clone())?)?)?)?, &v_k.clone())?); } Py::list(out) };
    }
    return Ok(Py::dict(vec![(Py::str("cma"), v_vals.clone()), (Py::str("first_index"), py::add(&v_h.clone(), &Py::Int(1i64))?), (Py::str("first"), py::getitem(&v_vals.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_cp1(mut v_x: Py, mut v_min_size: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_ms: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    v_ms = f__int(v_min_size.clone(), Py::str("the minimum segment length"), Py::Int(1i64), Py::None)?;
    if ({ let t1048 = py::len(&v_x.clone())?; Py::Bool({ let t1049 = py::mul(&Py::Int(2i64), &v_ms.clone())?; if py::lt(&t1048, &t1049)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the series is too short for two segments of the minimum length"))));
    }
    v_best = Py::None;
    for t1050 in py::range_iter(&[(v_ms.clone()).i()?, (py::add(&py::sub(&py::len(&v_x.clone())?, &v_ms.clone())?, &Py::Int(1i64))?).i()?]) {
        v_k = t1050;
        v_c = py::add(&f__seg_cost(py::slice(&v_x.clone(), None, Some((v_k.clone()).i()?), None)?)?, &f__seg_cost(py::slice(&v_x.clone(), Some((v_k.clone()).i()?), None, None)?)?)?;
        if ({ let t1056 = { let t1052 = v_best.clone(); Py::Bool({ let t1053 = Py::None; if py::is(&t1052, &t1053) { true } else { false } }) }; if t1056.truthy() { t1056 } else { { let t1054 = v_c.clone(); Py::Bool({ let t1055 = py::getitem(&v_best.clone(), &Py::Int(1i64))?; if py::lt(&t1054, &t1055)? { true } else { false } }) } } }).truthy() {
            v_best = Py::tuple(vec![v_k.clone(), v_c.clone()]);
        }
    }
    let t1057 = (v_best.clone()).items()?;
    if t1057.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t1057[0].clone();
    v_c = t1057[1].clone();
    return Ok(Py::dict(vec![(Py::str("change_index"), v_k.clone()), (Py::str("mean_before"), f__mean(py::slice(&v_x.clone(), None, Some((v_k.clone()).i()?), None)?)?), (Py::str("mean_after"), f__mean(py::slice(&v_x.clone(), Some((v_k.clone()).i()?), None, None)?)?), (Py::str("cost"), v_c.clone()), (Py::str("reduction"), py::sub(&f__sse(v_x.clone())?, &v_c.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_cp2(mut v_x: Py, mut v_min_size: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cost: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_ms: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    v_ms = f__int(v_min_size.clone(), Py::str("the minimum segment length"), Py::Int(1i64), Py::None)?;
    v_n = py::len(&v_x.clone())?;
    if ({ let t4306 = v_n.clone(); Py::Bool({ let t4307 = py::mul(&Py::Int(3i64), &v_ms.clone())?; if py::lt(&t4306, &t4307)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the series is too short for three segments of the minimum length"))));
    }
    v_cost = Py::dict(vec![]);
    v_c = { let v_c = v_c.clone(); let v_cost = v_cost.clone(); let v_min_size = v_min_size.clone(); let v_ms = v_ms.clone(); let v_n = v_n.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(1).cloned().unwrap_or(Py::None);
            if ({ let t4308 = Py::tuple(vec![v_a.clone(), v_b.clone()]); Py::Bool({ let t4309 = v_cost.clone(); if !py::contains(&t4309, &t4308)? { true } else { false } }) }).truthy() {
                py::setitem(&v_cost.clone(), &Py::tuple(vec![v_a.clone(), v_b.clone()]), f__seg_cost(py::slice(&v_x.clone(), Some((v_a.clone()).i()?), Some((v_b.clone()).i()?), None)?)?)?;
            }
            return Ok(py::getitem(&v_cost.clone(), &Py::tuple(vec![v_a.clone(), v_b.clone()]))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_best = Py::None;
    for t4311 in py::range_iter(&[(v_ms.clone()).i()?, (py::add(&py::sub(&v_n.clone(), &py::mul(&Py::Int(2i64), &v_ms.clone())?)?, &Py::Int(1i64))?).i()?]) {
        v_i = t4311;
        for t4312 in py::range_iter(&[(py::add(&v_i.clone(), &v_ms.clone())?).i()?, (py::add(&py::sub(&v_n.clone(), &v_ms.clone())?, &Py::Int(1i64))?).i()?]) {
            v_j = t4312;
            v_v = py::add(&py::add(&py::call(&v_c.clone(), &[Py::Int(0i64), v_i.clone()])?, &py::call(&v_c.clone(), &[v_i.clone(), v_j.clone()])?)?, &py::call(&v_c.clone(), &[v_j.clone(), v_n.clone()])?)?;
            if ({ let t4317 = { let t4313 = v_best.clone(); Py::Bool({ let t4314 = Py::None; if py::is(&t4313, &t4314) { true } else { false } }) }; if t4317.truthy() { t4317 } else { { let t4315 = v_v.clone(); Py::Bool({ let t4316 = py::getitem(&v_best.clone(), &Py::Int(2i64))?; if py::lt(&t4315, &t4316)? { true } else { false } }) } } }).truthy() {
                v_best = Py::tuple(vec![v_i.clone(), v_j.clone(), v_v.clone()]);
            }
        }
    }
    let t4318 = (v_best.clone()).items()?;
    if t4318.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_i = t4318[0].clone();
    v_j = t4318[1].clone();
    v_v = t4318[2].clone();
    return Ok(Py::dict(vec![(Py::str("change_1"), v_i.clone()), (Py::str("change_2"), v_j.clone()), (Py::str("means"), Py::list(vec![f__mean(py::slice(&v_x.clone(), None, Some((v_i.clone()).i()?), None)?)?, f__mean(py::slice(&v_x.clone(), Some((v_i.clone()).i()?), Some((v_j.clone()).i()?), None)?)?, f__mean(py::slice(&v_x.clone(), Some((v_j.clone()).i()?), None, None)?)?])), (Py::str("cost"), v_v.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_cusum_cp(mut v_x: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(3i64), Py::str("the series"))?;
    v_m = f__mean(v_x.clone())?;
    let t1243 = (Py::tuple(vec![Py::Float(0.0), Py::None])).items()?;
    if t1243.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1243[0].clone();
    v_best = t1243[1].clone();
    for t1244 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_x.clone())?).i()?]) {
        v_k = t1244;
        v_s = py::iadd(&v_s, &py::sub(&py::getitem(&v_x.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, &v_m.clone())?)?;
        if ({ let t1249 = { let t1245 = v_best.clone(); Py::Bool({ let t1246 = Py::None; if py::is(&t1245, &t1246) { true } else { false } }) }; if t1249.truthy() { t1249 } else { { let t1247 = py::abs(&v_s.clone())?; Py::Bool({ let t1248 = py::getitem(&v_best.clone(), &Py::Int(1i64))?; if py::gt(&t1247, &t1248)? { true } else { false } }) } } }).truthy() {
            v_best = Py::tuple(vec![v_k.clone(), py::abs(&v_s.clone())?]);
        }
    }
    v_k = py::getitem(&v_best.clone(), &Py::Int(0i64))?;
    return Ok(Py::dict(vec![(Py::str("change_index"), v_k.clone()), (Py::str("max_cusum"), py::getitem(&v_best.clone(), &Py::Int(1i64))?), (Py::str("mean_before"), f__mean(py::slice(&v_x.clone(), None, Some((v_k.clone()).i()?), None)?)?), (Py::str("mean_after"), f__mean(py::slice(&v_x.clone(), Some((v_k.clone()).i()?), None, None)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_cusum_tab(mut v_x: Py, mut v_mu0: Py, mut v_k: Py, mut v_h: Py) -> R {
    let mut v_alarm: Py = Py::None;
    let mut v_cm: Py = Py::None;
    let mut v_cp: Py = Py::None;
    let mut v_side: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(1i64), Py::str("the series"))?;
    let t4151 = (Py::tuple(vec![f__num(v_mu0.clone(), Py::str("the target"))?, f__num(v_k.clone(), Py::str("the allowance k"))?, f__num(v_h.clone(), Py::str("the decision interval h"))?])).items()?;
    if t4151.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_mu0 = t4151[0].clone();
    v_k = t4151[1].clone();
    v_h = t4151[2].clone();
    if ({ let t4156 = { let t4152 = v_k.clone(); Py::Bool({ let t4153 = Py::Int(0i64); if py::lt(&t4152, &t4153)? { true } else { false } }) }; if t4156.truthy() { t4156 } else { { let t4154 = v_h.clone(); Py::Bool({ let t4155 = Py::Int(0i64); if py::le(&t4154, &t4155)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the allowance k must be >= 0 and the decision interval h > 0"))));
    }
    let t4157 = Py::Float(0.0);
    v_cp = t4157.clone();
    v_cm = t4157.clone();
    let t4158 = (Py::tuple(vec![Py::Int(0i64), Py::str("none")])).items()?;
    if t4158.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_alarm = t4158[0].clone();
    v_side = t4158[1].clone();
    for t4159 in py::iter(&(py::enumerate(&v_x.clone(), (Py::Int(1i64)).i()?)?))? {
        let t4173 = (t4159).items()?;
        if t4173.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_t = t4173[0].clone();
        v_v = t4173[1].clone();
        v_cp = py::minmax(&[Py::Float(0.0), py::sub(&py::sub(&py::add(&v_cp.clone(), &v_v.clone())?, &v_mu0.clone())?, &v_k.clone())?], true)?;
        v_cm = py::minmax(&[Py::Float(0.0), py::sub(&py::sub(&py::add(&v_cm.clone(), &v_mu0.clone())?, &v_k.clone())?, &v_v.clone())?], true)?;
        if ({ let t4165 = py::not(&v_alarm.clone()); if !t4165.truthy() { t4165 } else { { let t4164 = { let t4160 = v_cp.clone(); Py::Bool({ let t4161 = v_h.clone(); if py::gt(&t4160, &t4161)? { true } else { false } }) }; if t4164.truthy() { t4164 } else { { let t4162 = v_cm.clone(); Py::Bool({ let t4163 = v_h.clone(); if py::gt(&t4162, &t4163)? { true } else { false } }) } } } } }).truthy() {
            v_alarm = v_t.clone();
            v_side = (if ({ let t4170 = { let t4166 = v_cp.clone(); Py::Bool({ let t4167 = v_h.clone(); if py::gt(&t4166, &t4167)? { true } else { false } }) }; if !t4170.truthy() { t4170 } else { { let t4168 = v_cm.clone(); Py::Bool({ let t4169 = v_h.clone(); if py::gt(&t4168, &t4169)? { true } else { false } }) } } }).truthy() { Py::str("both") } else { (if ({ let t4171 = v_cp.clone(); Py::Bool({ let t4172 = v_h.clone(); if py::gt(&t4171, &t4172)? { true } else { false } }) }).truthy() { Py::str("upper") } else { Py::str("lower") }) });
        }
    }
    return Ok(Py::dict(vec![(Py::str("c_plus"), v_cp.clone()), (Py::str("c_minus"), v_cm.clone()), (Py::str("alarm_index"), v_alarm.clone()), (Py::str("side"), v_side.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_diff(mut v_x: Py, mut v_d: Py, mut v_D: Py, mut v_s: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    v_d = f__int(v_d.clone(), Py::str("the differencing order d"), Py::Int(0i64), Py::Int(3i64))?;
    v_D = f__int(v_D.clone(), Py::str("the seasonal differencing order D"), Py::Int(0i64), Py::Int(2i64))?;
    if ({ let t1396 = py::add(&v_d.clone(), &v_D.clone())?; Py::Bool({ let t1397 = Py::Int(0i64); if py::eq(&t1396, &t1397) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at least one difference (d or D) is needed"))));
    }
    if (v_D.clone()).truthy() {
        if ({ let t1398 = v_s.clone(); Py::Bool({ let t1399 = Py::None; if py::is(&t1398, &t1399) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a seasonal difference needs the seasonal lag s"))));
        }
        v_s = f__int(v_s.clone(), Py::str("the seasonal lag s"), Py::Int(2i64), Py::Int(52i64))?;
    }
    v_y = Py::list((v_x.clone()).items()?);
    for t1400 in py::range_iter(&[(v_D.clone()).i()?]) {
        v__ = t1400;
        v_y = { let mut out = vec![]; for t1401 in py::range_iter(&[(v_s.clone()).i()?, (py::len(&v_y.clone())?).i()?]) { let mut v_t: Py; 
v_t = t1401;
out.push(py::sub(&py::getitem(&v_y.clone(), &v_t.clone())?, &py::getitem(&v_y.clone(), &py::sub(&v_t.clone(), &v_s.clone())?)?)?); } Py::list(out) };
    }
    for t1402 in py::range_iter(&[(v_d.clone()).i()?]) {
        v__ = t1402;
        v_y = { let mut out = vec![]; for t1403 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_y.clone())?).i()?]) { let mut v_t: Py; 
v_t = t1403;
out.push(py::sub(&py::getitem(&v_y.clone(), &v_t.clone())?, &py::getitem(&v_y.clone(), &py::sub(&v_t.clone(), &Py::Int(1i64))?)?)?); } Py::list(out) };
    }
    if ({ let t1404 = py::len(&v_y.clone())?; Py::Bool({ let t1405 = Py::Int(1i64); if py::lt(&t1404, &t1405)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the series is too short for these differences"))));
    }
    return Ok(Py::dict(vec![(Py::str("differenced"), v_y.clone()), (Py::str("last"), py::getitem(&v_y.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("length"), py::len(&v_y.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_drift(mut v_x: Py, mut v_h: Py) -> R {
    let mut v_d: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the series"))?;
    v_h = f__int(v_h.clone(), Py::str("the horizon h"), Py::Int(1i64), Py::Int(100000i64))?;
    v_d = py::truediv(&py::sub(&py::getitem(&v_x.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_x.clone(), &Py::Int(0i64))?)?, &py::sub(&py::len(&v_x.clone())?, &Py::Int(1i64))?)?;
    return Ok(Py::dict(vec![(Py::str("drift"), v_d.clone()), (Py::str("forecast"), py::add(&py::getitem(&v_x.clone(), &py::neg(&Py::Int(1i64))?)?, &py::mul(&v_h.clone(), &v_d.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ema(mut v_x: Py, mut v_k: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    v_k = f__int(v_k.clone(), Py::str("the span k"), Py::Int(1i64), Py::None)?;
    v_a = py::truediv(&Py::Float(2.0), &py::add(&v_k.clone(), &Py::Int(1i64))?)?;
    let t1543 = (Py::tuple(vec![py::getitem(&v_x.clone(), &Py::Int(0i64))?, Py::list(vec![py::getitem(&v_x.clone(), &Py::Int(0i64))?])])).items()?;
    if t1543.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_e = t1543[0].clone();
    v_out = t1543[1].clone();
    for t1544 in py::iter(&(py::slice(&v_x.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? {
        v_v = t1544;
        v_e = py::add(&py::mul(&v_a.clone(), &v_v.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_a.clone())?, &v_e.clone())?)?;
        let _ = py::method(&v_out.clone(), "append", &[v_e.clone()])?;
    }
    return Ok(Py::dict(vec![(Py::str("ema"), v_out.clone()), (Py::str("last"), py::getitem(&v_out.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("alpha"), v_a.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_exp_interp(mut v_t: Py, mut v_y: Py, mut v_t0: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t1496 = (f__interp_in(v_t.clone(), v_y.clone(), v_t0.clone())?).items()?;
    if t1496.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_t = t1496[0].clone();
    v_y = t1496[1].clone();
    v_t0 = t1496[2].clone();
    v_i = t1496[3].clone();
    let _ = f__positive(v_y.clone(), Py::str("the values"))?;
    v_w = py::truediv(&py::sub(&v_t0.clone(), &py::getitem(&v_t.clone(), &v_i.clone())?)?, &py::sub(&py::getitem(&v_t.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_t.clone(), &v_i.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("value"), f__r12(py::mul(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::pow(&py::truediv(&py::getitem(&v_y.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_y.clone(), &v_i.clone())?)?, &v_w.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_exp_trend(mut v_x: Py, mut v_h: Py) -> R {
    let mut v__dg: Py = Py::None;
    let mut v__res: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(3i64), Py::str("the series"))?;
    let _ = f__positive(v_x.clone(), Py::str("the series"))?;
    v_h = f__int(v_h.clone(), Py::str("the horizon h"), Py::Int(1i64), Py::Int(10000i64))?;
    v_n = py::len(&v_x.clone())?;
    let t1666 = (f__lstsq({ let mut out = vec![]; for t1664 in py::iter(&(f__t(v_n.clone())?))? { let mut v_t: Py; 
v_t = t1664;
out.push(Py::list(vec![Py::Float(1.0), v_t.clone()])); } Py::list(out) }, { let mut out = vec![]; for t1665 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1665;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) })?).items()?;
    if t1666.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_beta = t1666[0].clone();
    v__res = t1666[1].clone();
    v__dg = t1666[2].clone();
    v_a = py::math1("exp", &py::getitem(&v_beta.clone(), &Py::Int(0i64))?)?;
    return Ok(Py::dict(vec![(Py::str("a"), f__r12(v_a.clone())?), (Py::str("b"), f__r12(py::getitem(&v_beta.clone(), &Py::Int(1i64))?)?), (Py::str("growth_rate"), f__r12(py::math1("expm1", &py::getitem(&v_beta.clone(), &Py::Int(1i64))?)?)?), (Py::str("forecast"), f__r12(py::mul(&v_a.clone(), &py::math1("exp", &py::mul(&py::getitem(&v_beta.clone(), &Py::Int(1i64))?, &py::add(&v_n.clone(), &v_h.clone())?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_fev(mut v_phi: Py, mut v_theta: Py, mut v_sigma2: Py, mut v_h: Py) -> R {
    let mut v_psi: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t1710 = (f__arma_check(v_phi.clone(), v_theta.clone(), Py::Bool(false))?).items()?;
    if t1710.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t1710[0].clone();
    v_theta = t1710[1].clone();
    v_s2 = f__pos(v_sigma2.clone(), Py::str("the innovation variance"))?;
    v_h = f__int(v_h.clone(), Py::str("the horizon h"), Py::Int(1i64), Py::Int(1000i64))?;
    v_psi = f__psi(v_phi.clone(), v_theta.clone(), py::sub(&v_h.clone(), &Py::Int(1i64))?)?;
    v_v = py::mul(&v_s2.clone(), &py::fsum(&{ let mut out = vec![]; for t1734 in py::iter(&(v_psi.clone()))? { let mut v_w: Py; 
v_w = t1734;
out.push(py::mul(&v_w.clone(), &v_w.clone())?); } Py::list(out) })?)?;
    return Ok(Py::dict(vec![(Py::str("variance"), v_v.clone()), (Py::str("sd"), py::math1("sqrt", &v_v.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_holt(mut v_x: Py, mut v_alpha: Py, mut v_beta: Py, mut v_l0: Py, mut v_b0: Py, mut v_h: Py) -> R {
    return Ok({ let mut v: Vec<Py> = vec![]; v.extend((f__holt_inputs(v_x.clone(), v_alpha.clone(), v_beta.clone(), v_l0.clone(), v_b0.clone(), v_h.clone())?).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "x") { Some(x) => x, None => { return Err(PyErr::Type("_holt_core() missing argument x".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "alpha") { Some(x) => x, None => { return Err(PyErr::Type("_holt_core() missing argument alpha".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "beta") { Some(x) => x, None => { return Err(PyErr::Type("_holt_core() missing argument beta".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "l0") { Some(x) => x, None => { return Err(PyErr::Type("_holt_core() missing argument l0".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "b0") { Some(x) => x, None => { return Err(PyErr::Type("_holt_core() missing argument b0".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "h") { Some(x) => x, None => { return Err(PyErr::Type("_holt_core() missing argument h".into())) } } }; let a6 = if 6 < v.len() { v[6].clone() } else { match py::kw_take(&mut d, "phi") { Some(x) => x, None => Py::Float(1.0) } }; let a7 = if 7 < v.len() { v[7].clone() } else { match py::kw_take(&mut d, "wrong") { Some(x) => x, None => Py::Bool(false) } }; if v.len() > 8 { return Err(PyErr::Type("_holt_core() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_holt_core() got an unexpected keyword argument".into())) } f__holt_core(a0, a1, a2, a3, a4, a5, a6, a7)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_holt_damped(mut v_x: Py, mut v_alpha: Py, mut v_beta: Py, mut v_phi: Py, mut v_l0: Py, mut v_b0: Py, mut v_h: Py) -> R {
    let mut v_args: Py = Py::None;
    v_args = f__holt_inputs(v_x.clone(), v_alpha.clone(), v_beta.clone(), v_l0.clone(), v_b0.clone(), v_h.clone())?;
    return Ok({ let mut v: Vec<Py> = vec![]; v.extend((v_args.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; d.push((Py::str("phi"), f__unit(v_phi.clone(), Py::str("the damping parameter phi"))?)); let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "x") { Some(x) => x, None => { return Err(PyErr::Type("_holt_core() missing argument x".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "alpha") { Some(x) => x, None => { return Err(PyErr::Type("_holt_core() missing argument alpha".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "beta") { Some(x) => x, None => { return Err(PyErr::Type("_holt_core() missing argument beta".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "l0") { Some(x) => x, None => { return Err(PyErr::Type("_holt_core() missing argument l0".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "b0") { Some(x) => x, None => { return Err(PyErr::Type("_holt_core() missing argument b0".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "h") { Some(x) => x, None => { return Err(PyErr::Type("_holt_core() missing argument h".into())) } } }; let a6 = if 6 < v.len() { v[6].clone() } else { match py::kw_take(&mut d, "phi") { Some(x) => x, None => Py::Float(1.0) } }; let a7 = if 7 < v.len() { v[7].clone() } else { match py::kw_take(&mut d, "wrong") { Some(x) => x, None => Py::Bool(false) } }; if v.len() > 8 { return Err(PyErr::Type("_holt_core() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_holt_core() got an unexpected keyword argument".into())) } f__holt_core(a0, a1, a2, a3, a4, a5, a6, a7)? });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_holt_update(mut v_l: Py, mut v_b: Py, mut v_y: Py, mut v_alpha: Py, mut v_beta: Py, mut v_h: Py) -> R {
    let mut v_new: Py = Py::None;
    let mut v_tr: Py = Py::None;
    let t1878 = (Py::tuple(vec![f__num(v_l.clone(), Py::str("the previous level"))?, f__num(v_b.clone(), Py::str("the previous trend"))?, f__num(v_y.clone(), Py::str("the new observation"))?])).items()?;
    if t1878.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_l = t1878[0].clone();
    v_b = t1878[1].clone();
    v_y = t1878[2].clone();
    let t1882 = (Py::tuple(vec![f__unit(v_alpha.clone(), Py::str("alpha"))?, f__unit(v_beta.clone(), Py::str("beta"))?])).items()?;
    if t1882.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_alpha = t1882[0].clone();
    v_beta = t1882[1].clone();
    v_h = f__int(v_h.clone(), Py::str("the horizon h"), Py::Int(1i64), Py::Int(1000i64))?;
    v_new = py::add(&py::mul(&v_alpha.clone(), &v_y.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_alpha.clone())?, &py::add(&v_l.clone(), &v_b.clone())?)?)?;
    v_tr = py::add(&py::mul(&v_beta.clone(), &py::sub(&v_new.clone(), &v_l.clone())?)?, &py::mul(&py::sub(&Py::Int(1i64), &v_beta.clone())?, &v_b.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("level"), v_new.clone()), (Py::str("trend"), v_tr.clone()), (Py::str("forecast"), py::add(&v_new.clone(), &py::mul(&v_h.clone(), &v_tr.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_kf_cv(mut v_x: Py, mut v_dt: Py, mut v_q: Py, mut v_r: Py, mut v_p0: Py, mut v_v0: Py, mut v_var0: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Pn: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_k0: Py = Py::None;
    let mut v_k1: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the measured positions"))?;
    let t2056 = (Py::tuple(vec![f__pos(v_dt.clone(), Py::str("the time step"))?, f__num(v_q.clone(), Py::str("the acceleration variance"))?, f__pos(v_r.clone(), Py::str("the measurement variance"))?])).items()?;
    if t2056.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_dt = t2056[0].clone();
    v_q = t2056[1].clone();
    v_r = t2056[2].clone();
    v_var0 = f__num(v_var0.clone(), Py::str("the initial variance"))?;
    if ({ let t2061 = { let t2057 = v_q.clone(); Py::Bool({ let t2058 = Py::Int(0i64); if py::lt(&t2057, &t2058)? { true } else { false } }) }; if t2061.truthy() { t2061 } else { { let t2059 = v_var0.clone(); Py::Bool({ let t2060 = Py::Int(0i64); if py::lt(&t2059, &t2060)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the variances must be non-negative"))));
    }
    let t2062 = (f__cv_mats(v_dt.clone(), v_q.clone())?).items()?;
    if t2062.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t2062[0].clone();
    v_Q = t2062[1].clone();
    v_s = Py::list(vec![f__num(v_p0.clone(), Py::str("the initial position"))?, f__num(v_v0.clone(), Py::str("the initial velocity"))?]);
    v_P = Py::list(vec![Py::list(vec![v_var0.clone(), Py::Float(0.0)]), Py::list(vec![Py::Float(0.0), v_var0.clone()])]);
    for t2063 in py::iter(&(v_x.clone()))? {
        v_z = t2063;
        v_s = Py::list(vec![py::add(&py::getitem(&v_s.clone(), &Py::Int(0i64))?, &py::mul(&v_dt.clone(), &py::getitem(&v_s.clone(), &Py::Int(1i64))?)?)?, py::getitem(&v_s.clone(), &Py::Int(1i64))?]);
        v_P = f__cv_predict(v_P.clone(), v_F.clone(), v_Q.clone())?;
        let t2066 = (f__cv_update(v_P.clone(), v_r.clone())?).items()?;
        if t2066.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Pn = t2066[0].clone();
        let t2067 = (t2066[1].clone()).items()?;
        if t2067.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k0 = t2067[0].clone();
        v_k1 = t2067[1].clone();
        v_v = py::sub(&v_z.clone(), &py::getitem(&v_s.clone(), &Py::Int(0i64))?)?;
        v_s = Py::list(vec![py::add(&py::getitem(&v_s.clone(), &Py::Int(0i64))?, &py::mul(&v_k0.clone(), &v_v.clone())?)?, py::add(&py::getitem(&v_s.clone(), &Py::Int(1i64))?, &py::mul(&v_k1.clone(), &v_v.clone())?)?]);
        v_P = v_Pn.clone();
    }
    return Ok(Py::dict(vec![(Py::str("position"), py::getitem(&v_s.clone(), &Py::Int(0i64))?), (Py::str("velocity"), py::getitem(&v_s.clone(), &Py::Int(1i64))?), (Py::str("p11"), py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?), (Py::str("p12"), py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?), (Py::str("p22"), py::getitem(&py::getitem(&v_P.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_kf_cv_ss(mut v_dt: Py, mut v_q: Py, mut v_r: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Pn: Py = Py::None;
    let mut v_Pu: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v__k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t2140 = (Py::tuple(vec![f__pos(v_dt.clone(), Py::str("the time step"))?, f__pos(v_q.clone(), Py::str("the acceleration variance"))?, f__pos(v_r.clone(), Py::str("the measurement variance"))?])).items()?;
    if t2140.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_dt = t2140[0].clone();
    v_q = t2140[1].clone();
    v_r = t2140[2].clone();
    let t2141 = (f__cv_mats(v_dt.clone(), v_q.clone())?).items()?;
    if t2141.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t2141[0].clone();
    v_Q = t2141[1].clone();
    v_P = { let mut out = vec![]; for t2142 in py::iter(&(v_Q.clone()))? { let mut v_row: Py; 
v_row = t2142;
out.push(py::slice(&v_row.clone(), None, None, None)?); } Py::list(out) };
    v_P = Py::list(vec![Py::list(vec![py::add(&py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &v_r.clone())?, py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?]), Py::list(vec![py::getitem(&py::getitem(&v_P.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, py::add(&py::getitem(&py::getitem(&v_P.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, &v_r.clone())?])]);
    let mut t2143 = false;
    for t2144 in py::range_iter(&[(Py::Int(1000000i64)).i()?]) {
        v__ = t2144;
        let t2146 = (f__cv_update(v_P.clone(), v_r.clone())?).items()?;
        if t2146.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Pu = t2146[0].clone();
        v__k = t2146[1].clone();
        v_Pn = f__cv_predict(v_Pu.clone(), v_F.clone(), v_Q.clone())?;
        if ({ let t2148 = py::minmax(&[{ let mut out = vec![]; for t2150 in py::range_iter(&[(Py::Int(2i64)).i()?]) { let mut v_i: Py; 
v_i = t2150;
for t2151 in py::range_iter(&[(Py::Int(2i64)).i()?]) { let mut v_j: Py; 
v_j = t2151;
out.push(py::abs(&py::sub(&py::getitem(&py::getitem(&v_Pn.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_j.clone())?)?)?); }} Py::list(out) }], true)?; Py::Bool({ let t2149 = py::mul(&Py::Float(1e-15), &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&py::getitem(&v_Pn.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?)?], true)?)?; if py::le(&t2148, &t2149)? { true } else { false } }) }).truthy() {
            v_P = v_Pn.clone();
            { t2143 = true; break; }
        }
        v_P = v_Pn.clone();
    }
    if !t2143 {
        return Err(py::value_error(py::to_str(&Py::str("the Riccati iteration did not converge"))));
    }
    v_s = py::add(&py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &v_r.clone())?;
    return Ok(Py::dict(vec![(Py::str("k_position"), py::truediv(&py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &v_s.clone())?), (Py::str("k_velocity"), py::truediv(&py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &v_s.clone())?), (Py::str("prior_p11"), py::getitem(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_kf_ll(mut v_x: Py, mut v_q: Py, mut v_r: Py, mut v_m0: Py, mut v_p0: Py) -> R {
    let mut v__q_: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_ll: Py = Py::None;
    let mut v_ms: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let t2110 = (f__ll_filter(v_x.clone(), v_q.clone(), v_r.clone(), v_m0.clone(), v_p0.clone())?).items()?;
    if t2110.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_ms = t2110[0].clone();
    v_ps = t2110[1].clone();
    v_ll = t2110[2].clone();
    v_k = t2110[3].clone();
    v__q_ = t2110[4].clone();
    return Ok(Py::dict(vec![(Py::str("estimate"), py::getitem(&v_ms.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("variance"), py::getitem(&v_ps.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("gain"), v_k.clone()), (Py::str("estimates"), v_ms.clone()), (Py::str("log_likelihood"), f__r12(v_ll.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_kf_ss(mut v_q: Py, mut v_r: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_pp: Py = Py::None;
    let t2181 = (Py::tuple(vec![f__pos(v_q.clone(), Py::str("the process variance"))?, f__pos(v_r.clone(), Py::str("the measurement variance"))?])).items()?;
    if t2181.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_q = t2181[0].clone();
    v_r = t2181[1].clone();
    v_pp = py::truediv(&py::add(&v_q.clone(), &py::math1("sqrt", &py::add(&py::mul(&v_q.clone(), &v_q.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_q.clone())?, &v_r.clone())?)?)?)?, &Py::Int(2i64))?;
    v_k = py::truediv(&v_pp.clone(), &py::add(&v_pp.clone(), &v_r.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("gain"), v_k.clone()), (Py::str("prior_variance"), v_pp.clone()), (Py::str("posterior_variance"), py::mul(&py::sub(&Py::Int(1i64), &v_k.clone())?, &v_pp.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_kf_step(mut v_m: Py, mut v_p: Py, mut v_z: Py, mut v_q: Py, mut v_r: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_pp: Py = Py::None;
    let t2209 = (Py::tuple(vec![f__num(v_m.clone(), Py::str("the previous estimate"))?, f__num(v_z.clone(), Py::str("the measurement"))?])).items()?;
    if t2209.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2209[0].clone();
    v_z = t2209[1].clone();
    v_p = f__num(v_p.clone(), Py::str("the previous variance"))?;
    v_q = f__num(v_q.clone(), Py::str("the process variance"))?;
    v_r = f__pos(v_r.clone(), Py::str("the measurement variance"))?;
    if ({ let t2219 = { let t2212 = v_p.clone(); Py::Bool({ let t2213 = Py::Int(0i64); if py::lt(&t2212, &t2213)? { true } else { false } }) }; if t2219.truthy() { t2219 } else { { let t2218 = { let t2214 = v_q.clone(); Py::Bool({ let t2215 = Py::Int(0i64); if py::lt(&t2214, &t2215)? { true } else { false } }) }; if t2218.truthy() { t2218 } else { { let t2216 = py::add(&v_p.clone(), &v_q.clone())?; Py::Bool({ let t2217 = Py::Int(0i64); if py::le(&t2216, &t2217)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the variances must be non-negative and p + q > 0"))));
    }
    v_pp = py::add(&v_p.clone(), &v_q.clone())?;
    v_k = py::truediv(&v_pp.clone(), &py::add(&v_pp.clone(), &v_r.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("mean"), py::add(&v_m.clone(), &py::mul(&v_k.clone(), &py::sub(&v_z.clone(), &v_m.clone())?)?)?), (Py::str("variance"), py::mul(&py::sub(&Py::Int(1i64), &v_k.clone())?, &v_pp.clone())?), (Py::str("gain"), v_k.clone()), (Py::str("prior_variance"), v_pp.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_kf_update(mut v_m: Py, mut v_p: Py, mut v_z: Py, mut v_r: Py) -> R {
    let mut v_k: Py = Py::None;
    let t2247 = (Py::tuple(vec![f__num(v_m.clone(), Py::str("the prior mean"))?, f__num(v_z.clone(), Py::str("the measurement"))?])).items()?;
    if t2247.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2247[0].clone();
    v_z = t2247[1].clone();
    let t2250 = (Py::tuple(vec![f__pos(v_p.clone(), Py::str("the prior variance"))?, f__pos(v_r.clone(), Py::str("the measurement variance"))?])).items()?;
    if t2250.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2250[0].clone();
    v_r = t2250[1].clone();
    v_k = py::truediv(&v_p.clone(), &py::add(&v_p.clone(), &v_r.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("mean"), py::add(&v_m.clone(), &py::mul(&v_k.clone(), &py::sub(&v_z.clone(), &v_m.clone())?)?)?), (Py::str("variance"), py::mul(&py::sub(&Py::Int(1i64), &v_k.clone())?, &v_p.clone())?), (Py::str("gain"), v_k.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_kpss(mut v_x: Py, mut v_nlags: Py, mut v_regression: Py, mut v_flat: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v__beta: Py = Py::None;
    let mut v__dg: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_eta: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lrv: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_reg: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(4i64), Py::str("the series"))?;
    let _ = f__nonconstant(v_x.clone(), Py::str("the series"))?;
    v_L = f__int(v_nlags.clone(), Py::str("the number of lags"), Py::Int(0i64), Py::None)?;
    v_reg = { let t2299 = v_regression.clone(); if t2299.truthy() { t2299 } else { Py::str("c") } };
    if ({ let t2300 = v_reg.clone(); Py::Bool({ let t2301 = Py::tuple(vec![Py::str("c"), Py::str("ct")]); if !py::contains(&t2301, &t2300)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the KPSS regression must be c (level) or ct (trend)"))));
    }
    v_n = py::len(&v_x.clone())?;
    if ({ let t2302 = v_L.clone(); Py::Bool({ let t2303 = v_n.clone(); if py::ge(&t2302, &t2303)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of lags must be less than n"))));
    }
    if ({ let t2304 = v_reg.clone(); Py::Bool({ let t2305 = Py::str("c"); if py::eq(&t2304, &t2305) { true } else { false } }) }).truthy() {
        v_m = f__mean(v_x.clone())?;
        v_e = { let mut out = vec![]; for t2306 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t2306;
out.push(py::sub(&v_v.clone(), &v_m.clone())?); } Py::list(out) };
    } else {
        let t2348 = (f__lstsq({ let mut out = vec![]; for t2347 in py::iter(&(f__t(v_n.clone())?))? { let mut v_t: Py; 
v_t = t2347;
out.push(Py::list(vec![Py::Float(1.0), v_t.clone()])); } Py::list(out) }, v_x.clone())?).items()?;
        if t2348.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v__beta = t2348[0].clone();
        v_e = t2348[1].clone();
        v__dg = t2348[2].clone();
    }
    let t2349 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t2349.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t2349[0].clone();
    v_eta = t2349[1].clone();
    for t2350 in py::iter(&(v_e.clone()))? {
        v_v = t2350;
        v_s = py::iadd(&v_s, &v_v.clone())?;
        v_eta = py::iadd(&v_eta, &py::mul(&v_s.clone(), &v_s.clone())?)?;
    }
    v_eta = py::truediv(&v_eta, &py::mul(&v_n.clone(), &v_n.clone())?)?;
    v_lrv = py::fsum(&{ let mut out = vec![]; for t2351 in py::iter(&(v_e.clone()))? { let mut v_v: Py; 
v_v = t2351;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?;
    for t2352 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_L.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t2352;
        v_w = (if (v_flat.clone()).truthy() { Py::Float(1.0) } else { py::sub(&Py::Float(1.0), &py::truediv(&v_i.clone(), &py::add(&v_L.clone(), &Py::Float(1.0))?)?)? });
        v_lrv = py::iadd(&v_lrv, &py::mul(&py::mul(&Py::Int(2i64), &v_w.clone())?, &py::fsum(&{ let mut out = vec![]; for t2353 in py::range_iter(&[(v_i.clone()).i()?, (v_n.clone()).i()?]) { let mut v_t: Py; 
v_t = t2353;
out.push(py::mul(&py::getitem(&v_e.clone(), &v_t.clone())?, &py::getitem(&v_e.clone(), &py::sub(&v_t.clone(), &v_i.clone())?)?)?); } Py::list(out) })?)?)?;
    }
    v_lrv = py::truediv(&v_lrv, &v_n.clone())?;
    if ({ let t2354 = v_lrv.clone(); Py::Bool({ let t2355 = Py::Int(0i64); if py::le(&t2354, &t2355)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the long-run variance estimate is not positive"))));
    }
    return Ok(Py::dict(vec![(Py::str("kpss"), py::truediv(&v_eta.clone(), &v_lrv.clone())?), (Py::str("long_run_variance"), v_lrv.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_lev_pacf(mut v_rho: Py) -> R {
    let mut v__phi: Py = Py::None;
    let mut v__v: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_rho = f__fin(v_rho.clone(), Py::str("the autocorrelations"))?;
    let t3059 = (f__levinson(py::add(&Py::list(vec![Py::Float(1.0)]), &v_rho.clone())?, py::len(&v_rho.clone())?)?).items()?;
    if t3059.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v__phi = t3059[0].clone();
    v_p = t3059[1].clone();
    v__v = t3059[2].clone();
    return Ok(Py::dict(vec![(Py::str("pacf"), v_p.clone()), (Py::str("last"), py::getitem(&v_p.clone(), &py::neg(&Py::Int(1i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_lin_interp(mut v_t: Py, mut v_y: Py, mut v_t0: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t2401 = (f__interp_in(v_t.clone(), v_y.clone(), v_t0.clone())?).items()?;
    if t2401.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_t = t2401[0].clone();
    v_y = t2401[1].clone();
    v_t0 = t2401[2].clone();
    v_i = t2401[3].clone();
    v_w = py::truediv(&py::sub(&v_t0.clone(), &py::getitem(&v_t.clone(), &v_i.clone())?)?, &py::sub(&py::getitem(&v_t.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_t.clone(), &v_i.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("value"), py::add(&py::getitem(&v_y.clone(), &v_i.clone())?, &py::mul(&v_w.clone(), &py::sub(&py::getitem(&v_y.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_y.clone(), &v_i.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_linear_trend(mut v_x: Py, mut v_h: Py) -> R {
    let mut v__dg: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_res: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(3i64), Py::str("the series"))?;
    let _ = f__nonconstant(v_x.clone(), Py::str("the series"))?;
    v_h = f__int(v_h.clone(), Py::str("the horizon h"), Py::Int(1i64), Py::Int(10000i64))?;
    v_n = py::len(&v_x.clone())?;
    let t2490 = (f__lstsq({ let mut out = vec![]; for t2489 in py::iter(&(f__t(v_n.clone())?))? { let mut v_t: Py; 
v_t = t2489;
out.push(Py::list(vec![Py::Float(1.0), v_t.clone()])); } Py::list(out) }, v_x.clone())?).items()?;
    if t2490.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_beta = t2490[0].clone();
    v_res = t2490[1].clone();
    v__dg = t2490[2].clone();
    v_r2 = py::sub(&Py::Int(1i64), &py::truediv(&py::fsum(&{ let mut out = vec![]; for t2491 in py::iter(&(v_res.clone()))? { let mut v_e: Py; 
v_e = t2491;
out.push(py::mul(&v_e.clone(), &v_e.clone())?); } Py::list(out) })?, &f__sse(v_x.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("slope"), py::getitem(&v_beta.clone(), &Py::Int(1i64))?), (Py::str("intercept"), py::getitem(&v_beta.clone(), &Py::Int(0i64))?), (Py::str("forecast"), py::add(&py::getitem(&v_beta.clone(), &Py::Int(0i64))?, &py::mul(&py::getitem(&v_beta.clone(), &Py::Int(1i64))?, &py::add(&v_n.clone(), &v_h.clone())?)?)?), (Py::str("r_squared"), v_r2.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_lwma(mut v_x: Py, mut v_k: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    v_k = f__int(v_k.clone(), Py::str("the window k"), Py::Int(1i64), Py::None)?;
    if ({ let t2536 = v_k.clone(); Py::Bool({ let t2537 = py::len(&v_x.clone())?; if py::gt(&t2536, &t2537)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the window k must not exceed the number of values"))));
    }
    v_den = py::truediv(&py::mul(&v_k.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?;
    v_s = { let mut out = vec![]; for t2538 in py::range_iter(&[(py::add(&py::sub(&py::len(&v_x.clone())?, &v_k.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t2538;
out.push(py::truediv(&py::fsum(&{ let mut out = vec![]; for t2539 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t2539;
out.push(py::mul(&py::add(&v_j.clone(), &Py::Int(1i64))?, &py::getitem(&v_x.clone(), &py::add(&v_i.clone(), &v_j.clone())?)?)?); } Py::list(out) })?, &v_den.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("lwma"), v_s.clone()), (Py::str("last"), py::getitem(&v_s.clone(), &py::neg(&Py::Int(1i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ma1_pacf(mut v_theta: Py, mut v_K: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_th: Py = Py::None;
    v_th = f__num(v_theta.clone(), Py::str("theta"))?;
    if ({ let t2641 = py::abs(&v_th.clone())?; Py::Bool({ let t2642 = Py::Int(1i64); if py::eq(&t2641, &t2642) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("|theta| = 1 is not allowed (the partial autocorrelations do not decay)"))));
    }
    v_K = f__int(v_K.clone(), Py::str("the number of lags K"), Py::Int(1i64), Py::Int(500i64))?;
    v_p = { let mut out = vec![]; for t2658 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2658;
out.push(f__r12(py::truediv(&py::mul(&py::neg(&py::pow(&py::neg(&v_th.clone())?, &v_k.clone())?)?, &py::sub(&Py::Int(1i64), &py::mul(&v_th.clone(), &v_th.clone())?)?)?, &py::sub(&Py::Int(1i64), &py::pow(&v_th.clone(), &py::mul(&Py::Int(2i64), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?)?)?)?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("pacf"), v_p.clone()), (Py::str("pacf1"), py::getitem(&v_p.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ma2_inv(mut v_theta1: Py, mut v_theta2: Py) -> R {
    let mut v_kind: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let t2686 = (Py::tuple(vec![f__num(v_theta1.clone(), Py::str("theta1"))?, f__num(v_theta2.clone(), Py::str("theta2"))?])).items()?;
    if t2686.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t1 = t2686[0].clone();
    v_t2 = t2686[1].clone();
    let t2689 = (f__quad_inverse_roots(v_t1.clone(), v_t2.clone())?).items()?;
    if t2689.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2689[0].clone();
    v_kind = t2689[1].clone();
    return Ok(Py::dict(vec![(Py::str("invertible"), (if ({ let t2690 = v_m.clone(); Py::Bool({ let t2691 = Py::Int(1i64); if py::lt(&t2690, &t2691)? { true } else { false } }) }).truthy() { Py::str("yes") } else { Py::str("no") })), (Py::str("max_modulus"), v_m.clone()), (Py::str("roots"), v_kind.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ma_acf(mut v_theta: Py, mut v_K: Py) -> R {
    let mut v__phi: Py = Py::None;
    let mut v_acf: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_th: Py = Py::None;
    let t2735 = (f__arma_check(Py::list(vec![]), v_theta.clone(), Py::Bool(false))?).items()?;
    if t2735.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__phi = t2735[0].clone();
    v_th = t2735[1].clone();
    v_q = py::len(&v_theta.clone())?;
    v_K = (if ({ let t2736 = v_K.clone(); Py::Bool({ let t2737 = Py::None; if py::is(&t2736, &t2737) { true } else { false } }) }).truthy() { py::add(&v_q.clone(), &Py::Int(1i64))? } else { f__int(v_K.clone(), Py::str("the number of lags K"), Py::Int(1i64), Py::Int(200i64))? });
    v_g = f__arma_acov(Py::list(vec![]), v_th.clone(), Py::Float(1.0), v_K.clone())?;
    v_acf = { let mut out = vec![]; for t2782 in py::iter(&(py::slice(&v_g.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_v: Py; 
v_v = t2782;
out.push(py::truediv(&v_v.clone(), &py::getitem(&v_g.clone(), &Py::Int(0i64))?)?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("acf"), v_acf.clone()), (Py::str("rho1"), py::getitem(&v_acf.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_mae(mut v_y: Py, mut v_f: Py) -> R {
    let mut v_e: Py = Py::None;
    let t2860 = (f__errors(v_y.clone(), v_f.clone())?).items()?;
    if t2860.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_y = t2860[0].clone();
    v_f = t2860[1].clone();
    v_e = t2860[2].clone();
    return Ok(Py::dict(vec![(Py::str("mae"), py::truediv(&py::fsum(&{ let mut out = vec![]; for t2861 in py::iter(&(v_e.clone()))? { let mut v_v: Py; 
v_v = t2861;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?, &py::len(&v_e.clone())?)?), (Py::str("n"), py::len(&v_e.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_mape(mut v_y: Py, mut v_f: Py) -> R {
    let mut v_e: Py = Py::None;
    let t2895 = (f__errors(v_y.clone(), v_f.clone())?).items()?;
    if t2895.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_y = t2895[0].clone();
    v_f = t2895[1].clone();
    v_e = t2895[2].clone();
    if (Py::Bool(({ let mut out = vec![]; for t2896 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t2896;
out.push({ let t2897 = v_v.clone(); Py::Bool({ let t2898 = Py::Int(0i64); if py::eq(&t2897, &t2898) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("MAPE is undefined when an actual value is 0"))));
    }
    return Ok(Py::dict(vec![(Py::str("mape"), py::truediv(&py::mul(&Py::Int(100i64), &py::fsum(&{ let mut out = vec![]; for t2899 in py::iter(&(py::zip(&[v_e.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2900 = (t2899).items()?;
if t2900.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2900[0].clone();
v_b = t2900[1].clone();
out.push(py::abs(&py::truediv(&v_a.clone(), &v_b.clone())?)?); } Py::list(out) })?)?, &py::len(&v_e.clone())?)?), (Py::str("n"), py::len(&v_e.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_mase(mut v_y_train: Py, mut v_y: Py, mut v_f: Py, mut v_sp: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_mae: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_tr: Py = Py::None;
    v_tr = f__fin(v_y_train.clone(), Py::str("the training data"))?;
    let t2934 = (f__errors(v_y.clone(), v_f.clone())?).items()?;
    if t2934.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_y = t2934[0].clone();
    v_f = t2934[1].clone();
    v_e = t2934[2].clone();
    v_sp = f__int(v_sp.clone(), Py::str("the seasonal period sp"), Py::Int(1i64), Py::None)?;
    if ({ let t2950 = py::len(&v_tr.clone())?; Py::Bool({ let t2951 = v_sp.clone(); if py::le(&t2950, &t2951)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the training data must be longer than the seasonal period"))));
    }
    v_scale = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2952 in py::range_iter(&[(v_sp.clone()).i()?, (py::len(&v_tr.clone())?).i()?]) { let mut v_t: Py; 
v_t = t2952;
out.push(py::abs(&py::sub(&py::getitem(&v_tr.clone(), &v_t.clone())?, &py::getitem(&v_tr.clone(), &py::sub(&v_t.clone(), &v_sp.clone())?)?)?)?); } Py::list(out) })?, &py::sub(&py::len(&v_tr.clone())?, &v_sp.clone())?)?;
    if ({ let t2953 = v_scale.clone(); Py::Bool({ let t2954 = Py::Int(0i64); if py::eq(&t2953, &t2954) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the naive in-sample error is 0 (constant training data): MASE is undefined"))));
    }
    v_mae = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2955 in py::iter(&(v_e.clone()))? { let mut v_v: Py; 
v_v = t2955;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?, &py::len(&v_e.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("mase"), py::truediv(&v_mae.clone(), &v_scale.clone())?), (Py::str("mae"), v_mae.clone()), (Py::str("scale"), v_scale.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_me(mut v_y: Py, mut v_f: Py) -> R {
    let mut v_e: Py = Py::None;
    let t2989 = (f__errors(v_y.clone(), v_f.clone())?).items()?;
    if t2989.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_y = t2989[0].clone();
    v_f = t2989[1].clone();
    v_e = t2989[2].clone();
    return Ok(Py::dict(vec![(Py::str("mean_error"), py::truediv(&py::fsum(&v_e.clone())?, &py::len(&v_e.clone())?)?), (Py::str("n"), py::len(&v_e.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_mk(mut v_x: Py) -> R {
    let mut v_counts: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_var: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(4i64), Py::str("the series"))?;
    v_n = py::len(&v_x.clone())?;
    v_s = py::sum(&{ let mut out = vec![]; for t2813 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2813;
for t2814 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2814;
out.push(py::sub(&{ let t2815 = py::getitem(&v_x.clone(), &v_j.clone())?; Py::Bool({ let t2816 = py::getitem(&v_x.clone(), &v_i.clone())?; if py::gt(&t2815, &t2816)? { true } else { false } }) }, &{ let t2817 = py::getitem(&v_x.clone(), &v_j.clone())?; Py::Bool({ let t2818 = py::getitem(&v_x.clone(), &v_i.clone())?; if py::lt(&t2817, &t2818)? { true } else { false } }) })?); }} Py::list(out) }, None)?;
    v_counts = Py::dict(vec![]);
    for t2819 in py::iter(&(v_x.clone()))? {
        v_v = t2819;
        py::setitem(&v_counts.clone(), &v_v.clone(), py::add(&py::method(&v_counts.clone(), "get", &[v_v.clone(), Py::Int(0i64)])?, &Py::Int(1i64))?)?;
    }
    v_var = py::truediv(&py::sub(&py::mul(&py::mul(&v_n.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, &py::add(&py::mul(&Py::Int(2i64), &v_n.clone())?, &Py::Int(5i64))?)?, &py::sum(&{ let mut out = vec![]; for t2820 in py::iter(&(py::method(&v_counts.clone(), "values", &[])?))? { let mut v_c: Py; 
v_c = t2820;
out.push(py::mul(&py::mul(&v_c.clone(), &py::sub(&v_c.clone(), &Py::Int(1i64))?)?, &py::add(&py::mul(&Py::Int(2i64), &v_c.clone())?, &Py::Int(5i64))?)?); } Py::list(out) }, None)?)?, &Py::Int(18i64))?;
    if ({ let t2821 = v_var.clone(); Py::Bool({ let t2822 = Py::Int(0i64); if py::le(&t2821, &t2822)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the series must not be constant"))));
    }
    v_z = (if ({ let t2823 = v_s.clone(); Py::Bool({ let t2824 = Py::Int(0i64); if py::gt(&t2823, &t2824)? { true } else { false } }) }).truthy() { py::truediv(&py::sub(&v_s.clone(), &Py::Int(1i64))?, &py::math1("sqrt", &v_var.clone())?)? } else { (if ({ let t2825 = v_s.clone(); Py::Bool({ let t2826 = Py::Int(0i64); if py::lt(&t2825, &t2826)? { true } else { false } }) }).truthy() { py::truediv(&py::add(&v_s.clone(), &Py::Int(1i64))?, &py::math1("sqrt", &v_var.clone())?)? } else { Py::Float(0.0) }) });
    return Ok(Py::dict(vec![(Py::str("s"), v_s.clone()), (Py::str("var_s"), v_var.clone()), (Py::str("z"), v_z.clone()), (Py::str("p_value"), f__r12(py::math1("erfc", &py::truediv(&py::abs(&v_z.clone())?, &py::math1("sqrt", &Py::Int(2i64))?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_mse(mut v_y: Py, mut v_f: Py) -> R {
    let mut v_e: Py = Py::None;
    let t3356 = (f__errors(v_y.clone(), v_f.clone())?).items()?;
    if t3356.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_y = t3356[0].clone();
    v_f = t3356[1].clone();
    v_e = t3356[2].clone();
    return Ok(Py::dict(vec![(Py::str("mse"), py::truediv(&py::fsum(&{ let mut out = vec![]; for t3357 in py::iter(&(v_e.clone()))? { let mut v_v: Py; 
v_v = t3357;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?, &py::len(&v_e.clone())?)?), (Py::str("n"), py::len(&v_e.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_pacf(mut v_x: Py, mut v_k: Py) -> R {
    let mut v__phi: Py = Py::None;
    let mut v__v: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(4i64), Py::str("the series"))?;
    let _ = f__nonconstant(v_x.clone(), Py::str("the series"))?;
    v_k = f__int(v_k.clone(), Py::str("the lag k"), Py::Int(1i64), Py::None)?;
    if ({ let t3554 = v_k.clone(); Py::Bool({ let t3555 = py::floordiv(&py::len(&v_x.clone())?, &Py::Int(2i64))?; if py::ge(&t3554, &t3555)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the lag k must be less than n/2"))));
    }
    let t3568 = (f__levinson(f__acf_list(v_x.clone(), v_k.clone())?, v_k.clone())?).items()?;
    if t3568.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v__phi = t3568[0].clone();
    v_p = t3568[1].clone();
    v__v = t3568[2].clone();
    return Ok(Py::dict(vec![(Py::str("pacf"), py::getitem(&v_p.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("pacf_list"), v_p.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_pelt(mut v_x: Py, mut v_penalty: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_cost: Py = Py::None;
    let mut v_cps: Py = Py::None;
    let mut v_edges: Py = Py::None;
    let mut v_last: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pen: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the series"))?;
    v_pen = f__num(v_penalty.clone(), Py::str("the penalty"))?;
    if ({ let t3092 = v_pen.clone(); Py::Bool({ let t3093 = Py::Int(0i64); if py::le(&t3092, &t3093)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the penalty must be positive"))));
    }
    v_n = py::len(&v_x.clone())?;
    v_F = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?;
    v_last = py::mul(&Py::list(vec![Py::Int(0i64)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?;
    for t3094 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_b = t3094;
        v_best = Py::None;
        for t3095 in py::range_iter(&[(Py::Int(0i64)).i()?, (v_b.clone()).i()?]) {
            v_a = t3095;
            v_v = py::add(&py::add(&(if (v_a.clone()).truthy() { py::getitem(&v_F.clone(), &v_a.clone())? } else { Py::Float(0.0) }), &f__seg_cost(py::slice(&v_x.clone(), Some((v_a.clone()).i()?), Some((v_b.clone()).i()?), None)?)?)?, &v_pen.clone())?;
            if ({ let t3101 = { let t3097 = v_best.clone(); Py::Bool({ let t3098 = Py::None; if py::is(&t3097, &t3098) { true } else { false } }) }; if t3101.truthy() { t3101 } else { { let t3099 = v_v.clone(); Py::Bool({ let t3100 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::lt(&t3099, &t3100)? { true } else { false } }) } } }).truthy() {
                v_best = Py::tuple(vec![v_v.clone(), v_a.clone()]);
            }
        }
        let t3102 = (v_best.clone()).items()?;
        if t3102.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_F.clone(), &v_b.clone(), t3102[0].clone())?;
        py::setitem(&v_last.clone(), &v_b.clone(), t3102[1].clone())?;
    }
    let t3103 = (Py::tuple(vec![Py::list(vec![]), v_n.clone()])).items()?;
    if t3103.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cps = t3103[0].clone();
    v_b = t3103[1].clone();
    while ({ let t3104 = py::getitem(&v_last.clone(), &v_b.clone())?; Py::Bool({ let t3105 = Py::Int(0i64); if py::gt(&t3104, &t3105)? { true } else { false } }) }).truthy() {
        let _ = py::method(&v_cps.clone(), "append", &[py::getitem(&v_last.clone(), &v_b.clone())?])?;
        v_b = py::getitem(&v_last.clone(), &v_b.clone())?;
    }
    let _ = py::method(&v_cps.clone(), "reverse", &[])?;
    v_edges = py::add(&py::add(&Py::list(vec![Py::Int(0i64)]), &v_cps.clone())?, &Py::list(vec![v_n.clone()]))?;
    v_cost = py::fsum(&{ let mut out = vec![]; for t3106 in py::iter(&(py::zip(&[v_edges.clone(), py::slice(&v_edges.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3107 = (t3106).items()?;
if t3107.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3107[0].clone();
v_b = t3107[1].clone();
out.push(f__seg_cost(py::slice(&v_x.clone(), Some((v_a.clone()).i()?), Some((v_b.clone()).i()?), None)?)?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("n_changes"), py::len(&v_cps.clone())?), (Py::str("change_indices"), v_cps.clone()), (Py::str("cost"), v_cost.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_pi(mut v_phi: Py, mut v_theta: Py, mut v_K: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_pi: Py = Py::None;
    let t3151 = (f__arma_check(v_phi.clone(), v_theta.clone(), Py::Bool(true))?).items()?;
    if t3151.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t3151[0].clone();
    v_theta = t3151[1].clone();
    v_K = f__int(v_K.clone(), Py::str("the number of weights K"), Py::Int(1i64), Py::Int(500i64))?;
    v_c = Py::list(vec![Py::Float(1.0)]);
    for t3167 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) {
        v_j = t3167;
        v_a = (if ({ let t3168 = v_j.clone(); Py::Bool({ let t3169 = py::len(&v_phi.clone())?; if py::le(&t3168, &t3169)? { true } else { false } }) }).truthy() { py::neg(&py::getitem(&v_phi.clone(), &py::sub(&v_j.clone(), &Py::Int(1i64))?)?)? } else { Py::Float(0.0) });
        let _ = py::method(&v_c.clone(), "append", &[py::sub(&v_a.clone(), &py::fsum(&{ let mut out = vec![]; for t3170 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::minmax(&[v_j.clone(), py::len(&v_theta.clone())?], false)?, &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t3170;
out.push(py::mul(&py::getitem(&v_theta.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_c.clone(), &py::sub(&v_j.clone(), &v_i.clone())?)?)?); } Py::list(out) })?)?])?;
    }
    v_pi = { let mut out = vec![]; for t3171 in py::iter(&(py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_v: Py; 
v_v = t3171;
out.push(py::neg(&v_v.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("pi"), v_pi.clone()), (Py::str("pi1"), py::getitem(&v_pi.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_psi(mut v_phi: Py, mut v_theta: Py, mut v_K: Py) -> R {
    let mut v_psi: Py = Py::None;
    let t3215 = (f__arma_check(v_phi.clone(), v_theta.clone(), Py::Bool(false))?).items()?;
    if t3215.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t3215[0].clone();
    v_theta = t3215[1].clone();
    v_K = f__int(v_K.clone(), Py::str("the number of weights K"), Py::Int(1i64), Py::Int(500i64))?;
    v_psi = py::slice(&f__psi(v_phi.clone(), v_theta.clone(), v_K.clone())?, Some((Py::Int(1i64)).i()?), None, None)?;
    return Ok(Py::dict(vec![(Py::str("psi"), v_psi.clone()), (Py::str("psi1"), py::getitem(&v_psi.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_quad_trend(mut v_x: Py, mut v_h: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v__dg: Py = Py::None;
    let mut v__res: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(4i64), Py::str("the series"))?;
    v_h = f__int(v_h.clone(), Py::str("the horizon h"), Py::Int(1i64), Py::Int(10000i64))?;
    v_n = py::len(&v_x.clone())?;
    let t3321 = (f__lstsq({ let mut out = vec![]; for t3320 in py::iter(&(f__t(v_n.clone())?))? { let mut v_t: Py; 
v_t = t3320;
out.push(Py::list(vec![Py::Float(1.0), v_t.clone(), py::mul(&v_t.clone(), &v_t.clone())?])); } Py::list(out) }, v_x.clone())?).items()?;
    if t3321.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_beta = t3321[0].clone();
    v__res = t3321[1].clone();
    v__dg = t3321[2].clone();
    v_T = py::add(&v_n.clone(), &v_h.clone())?;
    return Ok(Py::dict(vec![(Py::str("b0"), py::getitem(&v_beta.clone(), &Py::Int(0i64))?), (Py::str("b1"), py::getitem(&v_beta.clone(), &Py::Int(1i64))?), (Py::str("b2"), py::getitem(&v_beta.clone(), &Py::Int(2i64))?), (Py::str("forecast"), py::add(&py::add(&py::getitem(&v_beta.clone(), &Py::Int(0i64))?, &py::mul(&py::getitem(&v_beta.clone(), &Py::Int(1i64))?, &v_T.clone())?)?, &py::mul(&py::mul(&py::getitem(&v_beta.clone(), &Py::Int(2i64))?, &v_T.clone())?, &v_T.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_rts(mut v_x: Py, mut v_q: Py, mut v_r: Py, mut v_m0: Py, mut v_p0: Py) -> R {
    let mut v__k: Py = Py::None;
    let mut v__ll: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_ms: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pp: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let mut v_sm: Py = Py::None;
    let mut v_sp: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t3400 = (f__ll_filter(v_x.clone(), v_q.clone(), v_r.clone(), v_m0.clone(), v_p0.clone())?).items()?;
    if t3400.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_ms = t3400[0].clone();
    v_ps = t3400[1].clone();
    v__ll = t3400[2].clone();
    v__k = t3400[3].clone();
    v_q = t3400[4].clone();
    v_n = py::len(&v_ms.clone())?;
    let t3401 = (Py::tuple(vec![py::slice(&v_ms.clone(), None, None, None)?, py::slice(&v_ps.clone(), None, None, None)?])).items()?;
    if t3401.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sm = t3401[0].clone();
    v_sp = t3401[1].clone();
    for t3402 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(2i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_t = t3402;
        v_pp = py::add(&py::getitem(&v_ps.clone(), &v_t.clone())?, &v_q.clone())?;
        v_c = (if ({ let t3403 = v_pp.clone(); Py::Bool({ let t3404 = Py::Int(0i64); if py::gt(&t3403, &t3404)? { true } else { false } }) }).truthy() { py::truediv(&py::getitem(&v_ps.clone(), &v_t.clone())?, &v_pp.clone())? } else { Py::Float(0.0) });
        py::setitem(&v_sm.clone(), &v_t.clone(), py::add(&py::getitem(&v_ms.clone(), &v_t.clone())?, &py::mul(&v_c.clone(), &py::sub(&py::getitem(&v_sm.clone(), &py::add(&v_t.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_ms.clone(), &v_t.clone())?)?)?)?)?;
        py::setitem(&v_sp.clone(), &v_t.clone(), py::add(&py::getitem(&v_ps.clone(), &v_t.clone())?, &py::mul(&py::mul(&v_c.clone(), &v_c.clone())?, &py::sub(&py::getitem(&v_sp.clone(), &py::add(&v_t.clone(), &Py::Int(1i64))?)?, &v_pp.clone())?)?)?)?;
    }
    return Ok(Py::dict(vec![(Py::str("smoothed"), v_sm.clone()), (Py::str("first"), py::getitem(&v_sm.clone(), &Py::Int(0i64))?), (Py::str("first_variance"), py::getitem(&v_sp.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_sarima_forecast(mut v_x: Py, mut v_phi: Py, mut v_theta: Py, mut v_residuals: Py, mut v_d: Py, mut v_seasonal_phi: Py, mut v_seasonal_theta: Py, mut v_D: Py, mut v_m: Py, mut v_c: Py, mut v_h: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_ar: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_hist: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_ma: Py = Py::None;
    let mut v_need: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_seasonal: Py = Py::None;
    let mut v_sphi: Py = Py::None;
    let mut v_stheta: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    v_phi = (if ({ let t3597 = v_phi.clone(); Py::Bool({ let t3598 = Py::None; if py::is(&t3597, &t3598) { true } else { false } }) }).truthy() { Py::list(vec![]) } else { f__fin(v_phi.clone(), Py::str("the AR coefficients"))? });
    v_theta = (if ({ let t3599 = v_theta.clone(); Py::Bool({ let t3600 = Py::None; if py::is(&t3599, &t3600) { true } else { false } }) }).truthy() { Py::list(vec![]) } else { f__fin(v_theta.clone(), Py::str("the MA coefficients"))? });
    v_sphi = (if ({ let t3601 = v_seasonal_phi.clone(); Py::Bool({ let t3602 = Py::None; if py::is(&t3601, &t3602) { true } else { false } }) }).truthy() { Py::list(vec![]) } else { f__fin(v_seasonal_phi.clone(), Py::str("the seasonal AR coefficients"))? });
    v_stheta = (if ({ let t3603 = v_seasonal_theta.clone(); Py::Bool({ let t3604 = Py::None; if py::is(&t3603, &t3604) { true } else { false } }) }).truthy() { Py::list(vec![]) } else { f__fin(v_seasonal_theta.clone(), Py::str("the seasonal MA coefficients"))? });
    v_d = f__int(v_d.clone(), Py::str("the differencing order d"), Py::Int(0i64), Py::Int(2i64))?;
    v_D = f__int(v_D.clone(), Py::str("the seasonal differencing order D"), Py::Int(0i64), Py::Int(1i64))?;
    v_c = f__num(v_c.clone(), Py::str("the constant"))?;
    v_h = f__int(v_h.clone(), Py::str("the horizon h"), Py::Int(1i64), Py::Int(500i64))?;
    v_seasonal = { let t3629 = { let t3622 = py::len(&v_sphi.clone())?; Py::Bool({ let t3623 = Py::Int(0i64); if py::gt(&t3622, &t3623)? { true } else { false } }) }; if t3629.truthy() { t3629 } else { { let t3628 = { let t3624 = py::len(&v_stheta.clone())?; Py::Bool({ let t3625 = Py::Int(0i64); if py::gt(&t3624, &t3625)? { true } else { false } }) }; if t3628.truthy() { t3628 } else { { let t3626 = v_D.clone(); Py::Bool({ let t3627 = Py::Int(0i64); if py::gt(&t3626, &t3627)? { true } else { false } }) } } } } };
    if (v_seasonal.clone()).truthy() {
        if ({ let t3630 = v_m.clone(); Py::Bool({ let t3631 = Py::None; if py::is(&t3630, &t3631) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a seasonal part needs the season length m"))));
        }
        v_m = f__int(v_m.clone(), Py::str("the season length m"), Py::Int(2i64), Py::Int(52i64))?;
    } else {
        if ({ let t3632 = v_m.clone(); Py::Bool({ let t3633 = Py::None; if !py::is(&t3632, &t3633) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a season length was given without a seasonal part"))));
        }
    }
    if ({ let t3641 = py::not(&f__stationary(v_phi.clone())?); if t3641.truthy() { t3641 } else { py::not(&f__stationary(v_sphi.clone())?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the AR parts must be stationary (every root outside the unit circle)"))));
    }
    let t3659 = (f__sarima_polys(v_phi.clone(), v_theta.clone(), v_d.clone(), v_sphi.clone(), v_stheta.clone(), v_D.clone(), v_m.clone())?).items()?;
    if t3659.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ar = t3659[0].clone();
    v_ma = t3659[1].clone();
    v_L = py::sub(&py::len(&v_ma.clone())?, &Py::Int(1i64))?;
    v_e = (if ({ let t3660 = v_residuals.clone(); Py::Bool({ let t3661 = Py::None; if py::is(&t3660, &t3661) { true } else { false } }) }).truthy() { Py::list(vec![]) } else { f__fin(v_residuals.clone(), Py::str("the residuals"))? });
    if ({ let t3662 = py::len(&v_e.clone())?; Py::Bool({ let t3663 = v_L.clone(); if py::ne(&t3662, &t3663) { true } else { false } }) }).truthy() {
        if ({ let t3664 = v_L.clone(); Py::Bool({ let t3665 = Py::Int(0i64); if py::eq(&t3664, &t3665) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the model has no moving-average part: residuals are not used"))));
        }
        return Err(py::value_error(py::to_str(&Py::str(&["the moving-average part reaches back ".to_string(), py::format(&v_L.clone(), "")?, " residuals: give exactly ".to_string(), py::format(&v_L.clone(), "")?, ", oldest first".to_string()].concat()))));
    }
    v_need = py::minmax(&[py::sub(&py::len(&v_ar.clone())?, &Py::Int(1i64))?, Py::Int(1i64)], true)?;
    if ({ let t3666 = py::len(&v_x.clone())?; Py::Bool({ let t3667 = v_need.clone(); if py::lt(&t3666, &t3667)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the series needs at least ".to_string(), py::format(&v_need.clone(), "")?, " values for this model".to_string()].concat()))));
    }
    v_hist = Py::list((v_x.clone()).items()?);
    v_out = Py::list(vec![]);
    for t3668 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_h.clone(), &Py::Int(1i64))?).i()?]) {
        v_j = t3668;
        v_terms = Py::list(vec![v_c.clone()]);
        for t3669 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_ar.clone())?).i()?]) {
            v_k = t3669;
            let _ = py::method(&v_terms.clone(), "append", &[py::mul(&py::neg(&py::getitem(&v_ar.clone(), &v_k.clone())?)?, &py::getitem(&v_hist.clone(), &py::neg(&v_k.clone())?)?)?])?;
        }
        for t3670 in py::range_iter(&[(v_j.clone()).i()?, (py::add(&v_L.clone(), &Py::Int(1i64))?).i()?]) {
            v_i = t3670;
            let _ = py::method(&v_terms.clone(), "append", &[py::mul(&py::getitem(&v_ma.clone(), &v_i.clone())?, &py::getitem(&v_e.clone(), &py::sub(&py::sub(&v_L.clone(), &Py::Int(1i64))?, &py::sub(&v_i.clone(), &v_j.clone())?)?)?)?])?;
        }
        v_v = py::fsum(&v_terms.clone())?;
        let _ = py::method(&v_hist.clone(), "append", &[v_v.clone()])?;
        let _ = py::method(&v_out.clone(), "append", &[v_v.clone()])?;
    }
    return Ok(Py::dict(vec![(Py::str("forecasts"), v_out.clone()), (Py::str("forecast"), py::getitem(&v_out.clone(), &py::neg(&Py::Int(1i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_seasonal_indices(mut v_x: Py, mut v_m: Py, mut v_mult: Py) -> R {
    let mut v__avg: Py = Py::None;
    let mut v__x: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let t3792 = (f__decompose(v_x.clone(), v_m.clone(), v_mult.clone())?).items()?;
    if t3792.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v__x = t3792[0].clone();
    v_idx = t3792[1].clone();
    v__avg = t3792[2].clone();
    return Ok(Py::dict(vec![(Py::str("indices"), v_idx.clone()), (Py::str("first"), py::getitem(&v_idx.clone(), &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_sen(mut v_x: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(3i64), Py::str("the series"))?;
    v_n = py::len(&v_x.clone())?;
    v_s = { let mut out = vec![]; for t3949 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3949;
for t3950 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3950;
out.push(py::truediv(&py::sub(&py::getitem(&v_x.clone(), &v_j.clone())?, &py::getitem(&v_x.clone(), &v_i.clone())?)?, &py::sub(&v_j.clone(), &v_i.clone())?)?); }} Py::list(out) };
    v_b = f__median(v_s.clone())?;
    return Ok(Py::dict(vec![(Py::str("slope"), v_b.clone()), (Py::str("intercept"), py::sub(&f__median(v_x.clone())?, &py::truediv(&py::mul(&v_b.clone(), &py::add(&v_n.clone(), &Py::Int(1i64))?)?, &Py::Int(2i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ses(mut v_x: Py, mut v_alpha: Py, mut v_l0: Py) -> R {
    let mut v_lev: Py = Py::None;
    let mut v_sse: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(1i64), Py::str("the series"))?;
    v_alpha = f__unit(v_alpha.clone(), Py::str("alpha"))?;
    v_lev = (if ({ let t4030 = v_l0.clone(); Py::Bool({ let t4031 = Py::None; if py::is(&t4030, &t4031) { true } else { false } }) }).truthy() { py::getitem(&v_x.clone(), &Py::Int(0i64))? } else { f__num(v_l0.clone(), Py::str("the initial level"))? });
    v_sse = Py::Float(0.0);
    for t4032 in py::iter(&(v_x.clone()))? {
        v_v = t4032;
        v_sse = py::iadd(&v_sse, &py::pow(&py::sub(&v_v.clone(), &v_lev.clone())?, &Py::Int(2i64))?)?;
        v_lev = py::add(&v_lev.clone(), &py::mul(&v_alpha.clone(), &py::sub(&v_v.clone(), &v_lev.clone())?)?)?;
    }
    return Ok(Py::dict(vec![(Py::str("forecast"), v_lev.clone()), (Py::str("sse"), v_sse.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ses_best(mut v_x: Py, mut v_alphas: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    let _ = f__need(v_x.clone(), Py::Int(2i64), Py::str("the series"))?;
    v_alphas = f__fin(v_alphas.clone(), Py::str("the candidate alphas"))?;
    v_best = Py::None;
    for t3981 in py::iter(&(v_alphas.clone()))? {
        v_a = t3981;
        v_r = f_run_ses(v_x.clone(), v_a.clone(), Py::None)?;
        if ({ let t3994 = { let t3990 = v_best.clone(); Py::Bool({ let t3991 = Py::None; if py::is(&t3990, &t3991) { true } else { false } }) }; if t3994.truthy() { t3994 } else { { let t3992 = py::getitem(&v_r.clone(), &Py::str("sse"))?; Py::Bool({ let t3993 = py::getitem(&v_best.clone(), &Py::Int(1i64))?; if py::lt(&t3992, &t3993)? { true } else { false } }) } } }).truthy() {
            v_best = Py::tuple(vec![v_a.clone(), py::getitem(&v_r.clone(), &Py::str("sse"))?, py::getitem(&v_r.clone(), &Py::str("forecast"))?]);
        }
    }
    return Ok(Py::dict(vec![(Py::str("alpha"), py::getitem(&v_best.clone(), &Py::Int(0i64))?), (Py::str("sse"), py::getitem(&v_best.clone(), &Py::Int(1i64))?), (Py::str("forecast"), py::getitem(&v_best.clone(), &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ses_update(mut v_F: Py, mut v_A: Py, mut v_alpha: Py) -> R {
    let t1575 = (Py::tuple(vec![f__num(v_F.clone(), Py::str("the previous forecast"))?, f__num(v_A.clone(), Py::str("the actual value"))?, f__unit(v_alpha.clone(), Py::str("alpha"))?])).items()?;
    if t1575.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_F = t1575[0].clone();
    v_A = t1575[1].clone();
    v_alpha = t1575[2].clone();
    return Ok(Py::dict(vec![(Py::str("forecast"), py::add(&v_F.clone(), &py::mul(&v_alpha.clone(), &py::sub(&v_A.clone(), &v_F.clone())?)?)?), (Py::str("error"), py::sub(&v_A.clone(), &v_F.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_sma(mut v_x: Py, mut v_k: Py) -> R {
    let mut v_s: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    v_k = f__int(v_k.clone(), Py::str("the window k"), Py::Int(1i64), Py::None)?;
    if ({ let t4076 = v_k.clone(); Py::Bool({ let t4077 = py::len(&v_x.clone())?; if py::gt(&t4076, &t4077)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the window k must not exceed the number of values"))));
    }
    v_s = { let mut out = vec![]; for t4078 in py::range_iter(&[(py::add(&py::sub(&py::len(&v_x.clone())?, &v_k.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t4078;
out.push(py::truediv(&py::fsum(&py::slice(&v_x.clone(), Some((v_i.clone()).i()?), Some((py::add(&v_i.clone(), &v_k.clone())?).i()?), None)?)?, &v_k.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("sma"), v_s.clone()), (Py::str("last"), py::getitem(&v_s.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("count"), py::len(&v_s.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_smape(mut v_y: Py, mut v_f: Py) -> R {
    let mut v_e: Py = Py::None;
    let t4112 = (f__errors(v_y.clone(), v_f.clone())?).items()?;
    if t4112.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_y = t4112[0].clone();
    v_f = t4112[1].clone();
    v_e = t4112[2].clone();
    if (Py::Bool(({ let mut out = vec![]; for t4113 in py::iter(&(py::zip(&[v_y.clone(), v_f.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4114 = (t4113).items()?;
if t4114.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4114[0].clone();
v_b = t4114[1].clone();
out.push({ let t4115 = py::add(&py::abs(&v_a.clone())?, &py::abs(&v_b.clone())?)?; Py::Bool({ let t4116 = Py::Int(0i64); if py::eq(&t4115, &t4116) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("sMAPE is undefined when an actual value and its forecast are both 0"))));
    }
    return Ok(Py::dict(vec![(Py::str("smape"), py::truediv(&py::mul(&Py::Int(100i64), &py::fsum(&{ let mut out = vec![]; for t4117 in py::iter(&(py::zip(&[v_y.clone(), v_f.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4118 = (t4117).items()?;
if t4118.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4118[0].clone();
v_b = t4118[1].clone();
out.push(py::truediv(&py::mul(&Py::Int(2i64), &py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?)?, &py::add(&py::abs(&v_a.clone())?, &py::abs(&v_b.clone())?)?)?); } Py::list(out) })?)?, &py::len(&v_y.clone())?)?), (Py::str("n"), py::len(&v_y.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ts_reg(mut v_x: Py, mut v_m: Py, mut v_h: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v__dg: Py = Py::None;
    let mut v__res: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_eff: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_row: Py = Py::None;
    v_x = f__fin(v_x.clone(), Py::str("the series"))?;
    v_m = f__int(v_m.clone(), Py::str("the season length m"), Py::Int(2i64), Py::Int(52i64))?;
    v_h = f__int(v_h.clone(), Py::str("the horizon h"), Py::Int(1i64), Py::Int(10000i64))?;
    v_n = py::len(&v_x.clone())?;
    if ({ let t4217 = v_n.clone(); Py::Bool({ let t4218 = py::add(&v_m.clone(), &Py::Int(3i64))?; if py::lt(&t4217, &t4218)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the series needs at least m + 3 values"))));
    }
    v_row = { let v_h = v_h.clone(); let v_m = v_m.clone(); let v_n = v_n.clone(); let v_row = v_row.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_s: Py = Py::None;
            v_s = py::modulo(&py::sub(&v_t.clone(), &Py::Int(1i64))?, &v_m.clone())?;
            return Ok(py::add(&Py::list(vec![Py::Float(1.0), py::float(&v_t.clone())?]), &{ let mut out = vec![]; for t4219 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_m.clone()).i()?]) { let mut v_j: Py; 
v_j = t4219;
out.push((if ({ let t4220 = v_s.clone(); Py::Bool({ let t4221 = v_j.clone(); if py::eq(&t4220, &t4221) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.0) })); } Py::list(out) })?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t4262 = (f__lstsq({ let mut out = vec![]; for t4261 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_t: Py; 
v_t = t4261;
out.push(py::call(&v_row.clone(), &[v_t.clone()])?); } Py::list(out) }, v_x.clone())?).items()?;
    if t4262.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_beta = t4262[0].clone();
    v__res = t4262[1].clone();
    v__dg = t4262[2].clone();
    v_eff = py::add(&Py::list(vec![Py::Float(0.0)]), &py::slice(&v_beta.clone(), Some((Py::Int(2i64)).i()?), None, None)?)?;
    v_T = py::add(&v_n.clone(), &v_h.clone())?;
    return Ok(Py::dict(vec![(Py::str("forecast"), py::add(&py::add(&py::getitem(&v_beta.clone(), &Py::Int(0i64))?, &py::mul(&py::getitem(&v_beta.clone(), &Py::Int(1i64))?, &v_T.clone())?)?, &py::getitem(&v_eff.clone(), &py::modulo(&py::sub(&v_T.clone(), &Py::Int(1i64))?, &v_m.clone())?)?)?), (Py::str("intercept"), py::getitem(&v_beta.clone(), &Py::Int(0i64))?), (Py::str("slope"), py::getitem(&v_beta.clone(), &Py::Int(1i64))?), (Py::str("season_effects"), v_eff.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_yw_acf(mut v_rho: Py) -> R {
    let mut v__pacf: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_rho = f__fin(v_rho.clone(), Py::str("the autocorrelations"))?;
    v_p = py::len(&v_rho.clone())?;
    let t4353 = (f__levinson(py::add(&Py::list(vec![Py::Float(1.0)]), &v_rho.clone())?, v_p.clone())?).items()?;
    if t4353.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t4353[0].clone();
    v__pacf = t4353[1].clone();
    v_v = t4353[2].clone();
    return Ok(Py::dict(vec![(Py::str("phi"), v_phi.clone()), (Py::str("phi1"), py::getitem(&v_phi.clone(), &Py::Int(0i64))?), (Py::str("variance_ratio"), v_v.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1005(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1029 = v_i.clone(); f_run_cma(py::getitem(&t1029, &Py::str("x"))?, py::getitem(&t1029, &Py::str("k"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_105(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t129 = v_i.clone(); f_run_ar2_stat(py::getitem(&t129, &Py::str("phi1"))?, py::getitem(&t129, &Py::str("phi2"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1058(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1082 = v_i.clone(); f_run_cp1(py::getitem(&t1082, &Py::str("x"))?, py::method(&t1082, "get", &[Py::str("min_size"), Py::Int(2i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1091(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1115 = v_i.clone(); f_run_avg_growth(py::getitem(&t1115, &Py::str("x"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1149(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1173 = v_i.clone(); f_run_ccf(py::getitem(&t1173, &Py::str("x"))?, py::getitem(&t1173, &Py::str("y"))?, py::getitem(&t1173, &Py::str("k"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1213(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1237 = v_i.clone(); f_run_best_lag(py::getitem(&t1237, &Py::str("x"))?, py::getitem(&t1237, &Py::str("y"))?, py::getitem(&t1237, &Py::str("K"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1250(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1274 = v_i.clone(); f_run_cusum_cp(py::getitem(&t1274, &Py::str("x"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1275(mut v_x: Py, mut v_regression: Py) -> R {
    return Ok(f__df(v_x.clone(), Py::Int(0i64), v_regression.clone(), Py::Bool(false))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1353(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1377 = v_i.clone(); s_1275(py::getitem(&t1377, &Py::str("x"))?, py::method(&t1377, "get", &[Py::str("regression"), Py::str("c")])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1406(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1430 = v_i.clone(); f_run_diff(py::getitem(&t1430, &Py::str("x"))?, py::method(&t1430, "get", &[Py::str("d"), Py::Int(1i64)])?, py::method(&t1430, "get", &[Py::str("D"), Py::Int(0i64)])?, py::method(&t1430, "get", &[Py::str("s"), Py::None])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1451(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1475 = v_i.clone(); f_run_drift(py::getitem(&t1475, &Py::str("x"))?, py::method(&t1475, "get", &[Py::str("h"), Py::Int(1i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1500(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1524 = v_i.clone(); f_run_exp_interp(py::getitem(&t1524, &Py::str("t"))?, py::getitem(&t1524, &Py::str("y"))?, py::getitem(&t1524, &Py::str("t0"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1545(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1569 = v_i.clone(); f_run_ema(py::getitem(&t1569, &Py::str("x"))?, py::getitem(&t1569, &Py::str("k"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_156(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t180 = v_i.clone(); f_run_ar2_acf(py::getitem(&t180, &Py::str("phi1"))?, py::getitem(&t180, &Py::str("phi2"))?, py::method(&t180, "get", &[Py::str("K"), Py::Int(5i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1576(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1600 = v_i.clone(); f_run_ses_update(py::getitem(&t1600, &Py::str("F"))?, py::getitem(&t1600, &Py::str("A"))?, py::getitem(&t1600, &Py::str("alpha"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1667(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1691 = v_i.clone(); f_run_exp_trend(py::getitem(&t1691, &Py::str("x"))?, py::method(&t1691, "get", &[Py::str("h"), Py::Int(1i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1735(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1759 = v_i.clone(); f_run_fev(py::getitem(&t1759, &Py::str("phi"))?, py::getitem(&t1759, &Py::str("theta"))?, py::getitem(&t1759, &Py::str("sigma2"))?, py::getitem(&t1759, &Py::str("h"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1793(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1817 = v_i.clone(); f_run_holt_damped(py::getitem(&t1817, &Py::str("x"))?, py::getitem(&t1817, &Py::str("alpha"))?, py::getitem(&t1817, &Py::str("beta"))?, py::getitem(&t1817, &Py::str("phi"))?, py::method(&t1817, "get", &[Py::str("l0"), Py::None])?, py::method(&t1817, "get", &[Py::str("b0"), Py::None])?, py::method(&t1817, "get", &[Py::str("h"), Py::Int(1i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1851(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1875 = v_i.clone(); f_run_holt(py::getitem(&t1875, &Py::str("x"))?, py::getitem(&t1875, &Py::str("alpha"))?, py::getitem(&t1875, &Py::str("beta"))?, py::method(&t1875, "get", &[Py::str("l0"), Py::None])?, py::method(&t1875, "get", &[Py::str("b0"), Py::None])?, py::method(&t1875, "get", &[Py::str("h"), Py::Int(1i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1898(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1922 = v_i.clone(); f_run_holt_update(py::getitem(&t1922, &Py::str("l"))?, py::getitem(&t1922, &Py::str("b"))?, py::getitem(&t1922, &Py::str("y"))?, py::getitem(&t1922, &Py::str("alpha"))?, py::getitem(&t1922, &Py::str("beta"))?, py::method(&t1922, "get", &[Py::str("h"), Py::Int(1i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1923(mut v_x: Py, mut v_m: Py, mut v_alpha: Py, mut v_beta: Py, mut v_gamma: Py, mut v_h: Py, mut v__m: Py) -> R {
    return Ok(f__hw(v_x.clone(), v_m.clone(), v_alpha.clone(), v_beta.clone(), v_gamma.clone(), v_h.clone(), v__m.clone(), Py::Bool(false))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1961(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t1985 = v_i.clone(); s_1923(py::getitem(&t1985, &Py::str("x"))?, py::getitem(&t1985, &Py::str("m"))?, py::getitem(&t1985, &Py::str("alpha"))?, py::getitem(&t1985, &Py::str("beta"))?, py::getitem(&t1985, &Py::str("gamma"))?, py::method(&t1985, "get", &[Py::str("h"), Py::Int(1i64)])?, py::method(&t1985, "get", &[Py::str("_m"), Py::Bool(false)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1986(mut v_x: Py, mut v_m: Py, mut v_alpha: Py, mut v_beta: Py, mut v_gamma: Py, mut v_h: Py, mut v__m: Py) -> R {
    return Ok(f__hw(v_x.clone(), v_m.clone(), v_alpha.clone(), v_beta.clone(), v_gamma.clone(), v_h.clone(), v__m.clone(), Py::Bool(false))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2024(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2048 = v_i.clone(); s_1986(py::getitem(&t2048, &Py::str("x"))?, py::getitem(&t2048, &Py::str("m"))?, py::getitem(&t2048, &Py::str("alpha"))?, py::getitem(&t2048, &Py::str("beta"))?, py::getitem(&t2048, &Py::str("gamma"))?, py::method(&t2048, "get", &[Py::str("h"), Py::Int(1i64)])?, py::method(&t2048, "get", &[Py::str("_m"), Py::Bool(true)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_205(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t229 = v_i.clone(); f_run_ar_forecast(py::getitem(&t229, &Py::str("x"))?, py::getitem(&t229, &Py::str("phi"))?, py::method(&t229, "get", &[Py::str("c"), Py::Float(0.0)])?, py::method(&t229, "get", &[Py::str("h"), Py::Int(1i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2068(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2092 = v_i.clone(); f_run_kf_cv(py::getitem(&t2092, &Py::str("x"))?, py::getitem(&t2092, &Py::str("dt"))?, py::getitem(&t2092, &Py::str("q"))?, py::getitem(&t2092, &Py::str("r"))?, py::getitem(&t2092, &Py::str("p0"))?, py::getitem(&t2092, &Py::str("v0"))?, py::getitem(&t2092, &Py::str("var0"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2111(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2135 = v_i.clone(); f_run_kf_ll(py::getitem(&t2135, &Py::str("x"))?, py::getitem(&t2135, &Py::str("q"))?, py::getitem(&t2135, &Py::str("r"))?, py::getitem(&t2135, &Py::str("m0"))?, py::getitem(&t2135, &Py::str("p0"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2152(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2176 = v_i.clone(); f_run_kf_cv_ss(py::getitem(&t2176, &Py::str("dt"))?, py::getitem(&t2176, &Py::str("q"))?, py::getitem(&t2176, &Py::str("r"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2182(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2206 = v_i.clone(); f_run_kf_ss(py::getitem(&t2206, &Py::str("q"))?, py::getitem(&t2206, &Py::str("r"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2220(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2244 = v_i.clone(); f_run_kf_step(py::getitem(&t2244, &Py::str("m"))?, py::getitem(&t2244, &Py::str("p"))?, py::getitem(&t2244, &Py::str("z"))?, py::getitem(&t2244, &Py::str("q"))?, py::getitem(&t2244, &Py::str("r"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2251(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2275 = v_i.clone(); f_run_kf_update(py::getitem(&t2275, &Py::str("m"))?, py::getitem(&t2275, &Py::str("p"))?, py::getitem(&t2275, &Py::str("z"))?, py::getitem(&t2275, &Py::str("r"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2276(mut v_x: Py, mut v_nlags: Py, mut v_regression: Py) -> R {
    return Ok(f_run_kpss(v_x.clone(), v_nlags.clone(), v_regression.clone(), Py::Bool(false))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2356(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2380 = v_i.clone(); s_2276(py::getitem(&t2380, &Py::str("x"))?, py::getitem(&t2380, &Py::str("nlags"))?, py::method(&t2380, "get", &[Py::str("regression"), Py::str("c")])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2402(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2426 = v_i.clone(); f_run_lin_interp(py::getitem(&t2426, &Py::str("t"))?, py::getitem(&t2426, &Py::str("y"))?, py::getitem(&t2426, &Py::str("t0"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2493(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2517 = v_i.clone(); f_run_linear_trend(py::getitem(&t2517, &Py::str("x"))?, py::method(&t2517, "get", &[Py::str("h"), Py::Int(1i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2540(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2564 = v_i.clone(); f_run_lwma(py::getitem(&t2564, &Py::str("x"))?, py::getitem(&t2564, &Py::str("k"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2565(mut v_x: Py, mut v_h: Py, mut v_model_df: Py, mut v__l: Py) -> R {
    return Ok(f__portmanteau(v_x.clone(), v_h.clone(), v__l.clone(), v_model_df.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_26(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t50 = v_i.clone(); f_run_ar1_fc(py::getitem(&t50, &Py::str("x"))?, py::getitem(&t50, &Py::str("phi"))?, py::getitem(&t50, &Py::str("mu"))?, py::method(&t50, "get", &[Py::str("h"), Py::Int(1i64)])?, py::method(&t50, "get", &[Py::str("sigma2"), Py::None])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2614(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2638 = v_i.clone(); s_2565(py::getitem(&t2638, &Py::str("x"))?, py::getitem(&t2638, &Py::str("h"))?, py::method(&t2638, "get", &[Py::str("model_df"), Py::Int(0i64)])?, py::method(&t2638, "get", &[Py::str("_l"), Py::Bool(true)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2659(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2683 = v_i.clone(); f_run_ma1_pacf(py::getitem(&t2683, &Py::str("theta"))?, py::method(&t2683, "get", &[Py::str("K"), Py::Int(5i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2692(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2716 = v_i.clone(); f_run_ma2_inv(py::getitem(&t2716, &Py::str("theta1"))?, py::getitem(&t2716, &Py::str("theta2"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2783(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2807 = v_i.clone(); f_run_ma_acf(py::getitem(&t2807, &Py::str("theta"))?, py::method(&t2807, "get", &[Py::str("K"), Py::None])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2827(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2851 = v_i.clone(); f_run_mk(py::getitem(&t2851, &Py::str("x"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2862(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2886 = v_i.clone(); f_run_mae(py::getitem(&t2886, &Py::str("y"))?, py::getitem(&t2886, &Py::str("f"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2901(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2925 = v_i.clone(); f_run_mape(py::getitem(&t2925, &Py::str("y"))?, py::getitem(&t2925, &Py::str("f"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_293(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t317 = v_i.clone(); f_run_ar_ols(py::getitem(&t317, &Py::str("x"))?, py::getitem(&t317, &Py::str("p"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2956(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t2980 = v_i.clone(); f_run_mase(py::getitem(&t2980, &Py::str("y_train"))?, py::getitem(&t2980, &Py::str("y"))?, py::getitem(&t2980, &Py::str("f"))?, py::method(&t2980, "get", &[Py::str("sp"), Py::Int(1i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2990(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3014 = v_i.clone(); f_run_me(py::getitem(&t3014, &Py::str("y"))?, py::getitem(&t3014, &Py::str("f"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3025(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3049 = v_i.clone(); f_run_mse(py::getitem(&t3049, &Py::str("y"))?, py::getitem(&t3049, &Py::str("f"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3060(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3084 = v_i.clone(); f_run_lev_pacf(py::getitem(&t3084, &Py::str("rho"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3108(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3132 = v_i.clone(); f_run_pelt(py::getitem(&t3132, &Py::str("x"))?, py::getitem(&t3132, &Py::str("penalty"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3172(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3196 = v_i.clone(); f_run_pi(py::getitem(&t3196, &Py::str("phi"))?, py::getitem(&t3196, &Py::str("theta"))?, py::method(&t3196, "get", &[Py::str("K"), Py::Int(10i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3235(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3259 = v_i.clone(); f_run_psi(py::getitem(&t3259, &Py::str("phi"))?, py::getitem(&t3259, &Py::str("theta"))?, py::method(&t3259, "get", &[Py::str("K"), Py::Int(10i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3322(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3346 = v_i.clone(); f_run_quad_trend(py::getitem(&t3346, &Py::str("x"))?, py::method(&t3346, "get", &[Py::str("h"), Py::Int(1i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3347(mut v_y: Py, mut v_f: Py) -> R {
    return Ok(py::call(&{ let v_f = v_f.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_o: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::dict(vec![(Py::str("rmse"), py::math1("sqrt", &py::getitem(&v_o.clone(), &Py::str("mse"))?)?), (Py::str("n"), py::getitem(&v_o.clone(), &Py::str("n"))?)]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f_run_mse(v_y.clone(), v_f.clone())?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3358(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3382 = v_i.clone(); s_3347(py::getitem(&t3382, &Py::str("y"))?, py::getitem(&t3382, &Py::str("f"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3405(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3429 = v_i.clone(); f_run_rts(py::getitem(&t3429, &Py::str("x"))?, py::getitem(&t3429, &Py::str("q"))?, py::getitem(&t3429, &Py::str("r"))?, py::getitem(&t3429, &Py::str("m0"))?, py::getitem(&t3429, &Py::str("p0"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3459(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3483 = v_i.clone(); f_run_acf(py::getitem(&t3483, &Py::str("x"))?, py::getitem(&t3483, &Py::str("k"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3507(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3531 = v_i.clone(); f_run_acov(py::getitem(&t3531, &Py::str("x"))?, py::getitem(&t3531, &Py::str("k"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_355(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t379 = v_i.clone(); f_run_ar_yw(py::getitem(&t379, &Py::str("x"))?, py::getitem(&t379, &Py::str("p"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3569(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3593 = v_i.clone(); f_run_pacf(py::getitem(&t3593, &Py::str("x"))?, py::getitem(&t3593, &Py::str("k"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3671(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3695 = v_i.clone(); f_run_sarima_forecast(py::getitem(&t3695, &Py::str("x"))?, py::method(&t3695, "get", &[Py::str("phi"), Py::None])?, py::method(&t3695, "get", &[Py::str("theta"), Py::None])?, py::method(&t3695, "get", &[Py::str("residuals"), Py::None])?, py::method(&t3695, "get", &[Py::str("d"), Py::Int(0i64)])?, py::method(&t3695, "get", &[Py::str("seasonal_phi"), Py::None])?, py::method(&t3695, "get", &[Py::str("seasonal_theta"), Py::None])?, py::method(&t3695, "get", &[Py::str("D"), Py::Int(0i64)])?, py::method(&t3695, "get", &[Py::str("m"), Py::None])?, py::method(&t3695, "get", &[Py::str("c"), Py::Float(0.0)])?, py::method(&t3695, "get", &[Py::str("h"), Py::Int(1i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3696(mut v_x: Py, mut v_m: Py, mut v__m: Py) -> R {
    return Ok(f_run_seasonal_indices(v_x.clone(), v_m.clone(), v__m.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3732(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3756 = v_i.clone(); s_3696(py::getitem(&t3756, &Py::str("x"))?, py::getitem(&t3756, &Py::str("m"))?, py::method(&t3756, "get", &[Py::str("_m"), Py::Bool(false)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3757(mut v_x: Py, mut v_m: Py, mut v__m: Py) -> R {
    return Ok(f_run_seasonal_indices(v_x.clone(), v_m.clone(), v__m.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3793(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3817 = v_i.clone(); s_3757(py::getitem(&t3817, &Py::str("x"))?, py::getitem(&t3817, &Py::str("m"))?, py::method(&t3817, "get", &[Py::str("_m"), Py::Bool(true)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3818(mut v_x: Py, mut v_m: Py, mut v__m: Py) -> R {
    return Ok(f_run_adjusted(v_x.clone(), v_m.clone(), v__m.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3856(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3880 = v_i.clone(); s_3818(py::getitem(&t3880, &Py::str("x"))?, py::getitem(&t3880, &Py::str("m"))?, py::method(&t3880, "get", &[Py::str("_m"), Py::Bool(false)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3881(mut v_x: Py, mut v_m: Py, mut v__m: Py) -> R {
    return Ok(f_run_adjusted(v_x.clone(), v_m.clone(), v__m.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3919(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3943 = v_i.clone(); s_3881(py::getitem(&t3943, &Py::str("x"))?, py::getitem(&t3943, &Py::str("m"))?, py::method(&t3943, "get", &[Py::str("_m"), Py::Bool(true)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3951(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t3975 = v_i.clone(); f_run_sen(py::getitem(&t3975, &Py::str("x"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3995(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4019 = v_i.clone(); f_run_ses_best(py::getitem(&t4019, &Py::str("x"))?, py::getitem(&t4019, &Py::str("alphas"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4033(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4057 = v_i.clone(); f_run_ses(py::getitem(&t4057, &Py::str("x"))?, py::getitem(&t4057, &Py::str("alpha"))?, py::method(&t4057, "get", &[Py::str("l0"), Py::None])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4079(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4103 = v_i.clone(); f_run_sma(py::getitem(&t4103, &Py::str("x"))?, py::getitem(&t4103, &Py::str("k"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_411(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t435 = v_i.clone(); f_run_arima_forecast(py::getitem(&t435, &Py::str("x"))?, py::getitem(&t435, &Py::str("phi"))?, py::method(&t435, "get", &[Py::str("h"), Py::Int(1i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4119(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4143 = v_i.clone(); f_run_smape(py::getitem(&t4143, &Py::str("y"))?, py::getitem(&t4143, &Py::str("f"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4174(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4198 = v_i.clone(); f_run_cusum_tab(py::getitem(&t4198, &Py::str("x"))?, py::getitem(&t4198, &Py::str("mu0"))?, py::getitem(&t4198, &Py::str("k"))?, py::getitem(&t4198, &Py::str("h"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4263(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4287 = v_i.clone(); f_run_ts_reg(py::getitem(&t4287, &Py::str("x"))?, py::getitem(&t4287, &Py::str("m"))?, py::method(&t4287, "get", &[Py::str("h"), Py::Int(1i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4319(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4343 = v_i.clone(); f_run_cp2(py::getitem(&t4343, &Py::str("x"))?, py::method(&t4343, "get", &[Py::str("min_size"), Py::Int(2i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4354(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t4378 = v_i.clone(); f_run_yw_acf(py::getitem(&t4378, &Py::str("rho"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_457(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t481 = v_i.clone(); f_run_arma11_acf(py::getitem(&t481, &Py::str("phi"))?, py::getitem(&t481, &Py::str("theta"))?, py::method(&t481, "get", &[Py::str("K"), Py::Int(5i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_546(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t570 = v_i.clone(); f_run_arma_acf(py::getitem(&t570, &Py::str("phi"))?, py::getitem(&t570, &Py::str("theta"))?, py::method(&t570, "get", &[Py::str("K"), Py::Int(5i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_642(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t666 = v_i.clone(); f_run_arma_pacf(py::getitem(&t666, &Py::str("phi"))?, py::getitem(&t666, &Py::str("theta"))?, py::method(&t666, "get", &[Py::str("K"), Py::Int(5i64)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_719(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t743 = v_i.clone(); f_run_arma_var(py::getitem(&t743, &Py::str("phi"))?, py::getitem(&t743, &Py::str("theta"))?, py::getitem(&t743, &Py::str("sigma2"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_72(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t96 = v_i.clone(); f_run_ar1_props(py::getitem(&t96, &Py::str("phi"))?, py::getitem(&t96, &Py::str("sigma2"))?, py::method(&t96, "get", &[Py::str("k"), Py::Int(1i64)])?, py::method(&t96, "get", &[Py::str("c"), Py::Float(0.0)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_744(mut v_x: Py, mut v_p: Py, mut v_regression: Py) -> R {
    return Ok(f__df(v_x.clone(), v_p.clone(), v_regression.clone(), Py::Bool(false))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_822(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t846 = v_i.clone(); s_744(py::getitem(&t846, &Py::str("x"))?, py::getitem(&t846, &Py::str("p"))?, py::method(&t846, "get", &[Py::str("regression"), Py::str("c")])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_877(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t901 = v_i.clone(); f_run_bartlett(py::getitem(&t901, &Py::str("x"))?, py::getitem(&t901, &Py::str("k"))?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_902(mut v_x: Py, mut v_h: Py, mut v_model_df: Py, mut v__l: Py) -> R {
    return Ok(f__portmanteau(v_x.clone(), v_h.clone(), v__l.clone(), v_model_df.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_951(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff({ let t975 = v_i.clone(); s_902(py::getitem(&t975, &Py::str("x"))?, py::getitem(&t975, &Py::str("h"))?, py::method(&t975, "get", &[Py::str("model_df"), Py::Int(0i64)])?, py::method(&t975, "get", &[Py::str("_l"), Py::Bool(false)])?)? }, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
