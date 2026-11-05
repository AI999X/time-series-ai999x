//! Every Time Series capability's own live figures (src/figures.rs): its worked example gives each
//! published figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text
//! outside its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use time_series_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/time-series-ai999x/figures,
/// except the generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("ar1_forecast", &["forecast", "paths"]),
    ("ar1_properties", &["acf"]),
    ("ar2_stationarity", &["modulus", "region"]),
    ("ar2_theoretical_acf", &["acf"]),
    ("ar_forecast", &["forecast"]),
    ("ar_least_squares", &["lag-plot"]),
    ("ar_yule_walker", &["orders"]),
    ("arima_p10_forecast", &["forecast"]),
    ("arma11_theoretical_acf", &["acf"]),
    ("arma_theoretical_acf", &["acf"]),
    ("arma_theoretical_pacf", &["pacf"]),
    ("augmented_dickey_fuller_test", &["series"]),
    ("bartlett_standard_error", &["acf"]),
    ("box_pierce_test", &["q-by-lag"]),
    ("centred_moving_average", &["smooth"]),
    ("change_point_mean_shift", &["segments"]),
    ("compound_average_growth_rate", &["growth"]),
    ("cross_correlation_at_lag", &["ccf"]),
    ("cross_correlation_best_lag", &["series-ccf"]),
    ("cusum_change_point", &["cusum"]),
    ("dickey_fuller_test", &["series"]),
    ("differencing", &["differences"]),
    ("drift_forecast", &["forecast"]),
    ("exponential_interpolation", &["interpolation"]),
    ("exponential_moving_average", &["smooth"]),
    ("exponential_smoothing_update", &["update"]),
    ("exponential_trend_forecast", &["forecast"]),
    ("forecast_error_variance", &["fan"]),
    ("holt_damped_trend", &["forecast"]),
    ("holt_linear_trend", &["forecast"]),
    ("holt_update", &["update"]),
    ("holt_winters_additive", &["forecast", "forecast-2"]),
    ("holt_winters_multiplicative", &["forecast"]),
    ("kalman_filter_constant_velocity", &["filter"]),
    ("kalman_filter_local_level", &["band", "filter"]),
    ("kalman_steady_state_constant_velocity", &["gains"]),
    ("kalman_steady_state_gain", &["convergence", "field"]),
    ("kalman_step", &["densities"]),
    ("kalman_update", &["densities"]),
    ("kpss_test", &["series"]),
    ("linear_interpolation", &["interpolation"]),
    ("linear_trend_forecast", &["forecast"]),
    ("linearly_weighted_moving_average", &["smooth"]),
    ("ljung_box_test", &["q-by-lag"]),
    ("ma1_theoretical_pacf", &["pacf"]),
    ("ma2_invertibility", &["modulus", "region"]),
    ("ma_theoretical_acf", &["acf"]),
    ("mann_kendall_test", &["trend"]),
    ("mean_absolute_error", &["errors"]),
    ("mean_absolute_percentage_error", &["errors"]),
    ("mean_absolute_scaled_error", &["errors"]),
    ("mean_forecast_error", &["errors"]),
    ("mean_squared_error", &["errors"]),
    ("partial_autocorrelations_from_autocorrelations", &["acf-pacf"]),
    ("penalised_change_points", &["segments"]),
    ("pi_weights", &["weights"]),
    ("psi_weights", &["weights"]),
    ("quadratic_trend_forecast", &["forecast"]),
    ("root_mean_squared_error", &["errors"]),
    ("rts_smoother_local_level", &["smoothing"]),
    ("sample_autocorrelation", &["acf", "lag-plot"]),
    ("sample_autocovariance", &["acov"]),
    ("sample_partial_autocorrelation", &["pacf"]),
    ("seasonal_indices_additive", &["bars"]),
    ("seasonal_indices_multiplicative", &["bars"]),
    ("seasonally_adjusted_additive", &["adjusted"]),
    ("seasonally_adjusted_multiplicative", &["adjusted"]),
    ("sens_slope", &["trend"]),
    ("ses_best_alpha", &["sse"]),
    ("simple_exponential_smoothing", &["forecasts"]),
    ("simple_moving_average", &["smooth"]),
    ("symmetric_mape", &["errors"]),
    ("tabular_cusum", &["cusum"]),
    ("trend_seasonal_regression", &["fit"]),
    ("two_change_points", &["comparison", "segments"]),
    ("yule_walker_from_autocorrelations", &["acf"]),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("dickey_fuller_test", "x", "3.1, 2.4, 3.9, 3.3, 4.6, 4.0, 5.2, 4.4, 5.9, 5.1"),
];

/// The arguments of a structured example: solve('key', a='1 V', b=8, c="(A·B)'").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn kind_of(f: &Figure) -> &str {
    &f.kind
}

/// Every text inside the canvas, no non-finite number (ax_engine::figkit::check_figure).
fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| kind_of(f) == *kind) {
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    assert!(!f.svg.contains("NaN") && !f.svg.contains("inf"), "{key} {kind}: a non-finite number");
                    check_texts(f);
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        // the model's own figures come first
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("12 V" -> "16.44 V"; "3, 6" -> "4.11, 8.22").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        // the other example: a listed replacement, else each number input scaled until the model solves it
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn an_impossible_input_draws_nothing_wrong() {
    // a non-stationary AR(1) forecast and mismatched error lists: when the model refuses, nothing is drawn
    let p = pack();
    for (key, inputs) in [("ar1_forecast", vec![("x", "58"), ("phi", "1.5"), ("mu", "50"), ("h", "3")]),
                          ("mean_absolute_error", vec![("y", "10, 12, 14"), ("f", "11, 12")])] {
        let given: Vec<(String, Input)> = inputs.iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
        let a = ax_engine::solve(p, key, &given, None);
        let figs = figures(p, key, &given, None, None);
        if !a.solved() {
            assert!(figs.is_empty(), "{key}: a figure for a refused input");
        }
    }
}
