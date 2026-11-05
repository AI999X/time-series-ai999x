//! Time Series' own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model computes:
//! correlograms, forecasts with their history, smoothers, filters, tests over their settings, change points. Every
//! value drawn is an output of this request or of a further run of the model; a figure the model cannot support for
//! these inputs is left out (docs/FIGURE_PARITY.md).

use crate::figures_plot::{xs_ys, Plot};
use ax_engine::figkit::{self as k, num, Solved};
use ax_engine::figures::Figure;
use ax_engine::{Answer, Input, Val};

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    use crate::figures_fc as fc;
    use crate::figures_more as mo;
    let v: Vec<Option<Figure>> = match s.key {
        "ar1_forecast" => vec![fc::ar1_forecast(s), fc::ar1_paths(s)],
        "ar1_properties" => vec![ar1_acf(s)],
        "ar2_stationarity" => vec![roots_modulus(s, true), roots_region(s, true)],
        "ma2_invertibility" => vec![roots_modulus(s, false), roots_region(s, false)],
        "ar2_theoretical_acf" => vec![ar2_acf(s)],
        "arma11_theoretical_acf" => vec![arma11_acf(s)],
        "arma_theoretical_acf" => vec![list_stems(s, "acf", "acf", "K", 10, "ARMA autocorrelation function",
                                                  "ρ(k), autocorrelation", "autocorrelations")],
        "ma_theoretical_acf" => vec![ma_acf(s)],
        "arma_theoretical_pacf" => vec![list_stems(s, "pacf", "pacf", "K", 10, "ARMA partial autocorrelation function",
                                                   "φ(k,k), partial autocorr.", "partial autocorrelations")],
        "ma1_theoretical_pacf" => vec![list_stems(s, "pacf", "pacf", "K", 10,
                                                  "MA(1) partial autocorrelations alternate and decay",
                                                  "φ(k,k), partial autocorr.", "partial autocorrelations")],
        "pi_weights" => vec![list_stems(s, "weights", "pi", "K", 10, "π weights: the model as an autoregression on all past values",
                                        "πⱼ, weight", "π weights")],
        "psi_weights" => vec![list_stems(s, "weights", "psi", "K", 10,
                                         "ψ weights: the model as a moving average of all past shocks", "ψⱼ, weight", "ψ weights")],
        "partial_autocorrelations_from_autocorrelations" => vec![acf_pacf(s)],
        "yule_walker_from_autocorrelations" => vec![yw_acf(s)],
        "sample_autocorrelation" => vec![sample_acf(s), lag_plot(s, false)],
        "ar_least_squares" => vec![lag_plot(s, true)],
        "sample_autocovariance" => vec![sample_acov(s)],
        "sample_partial_autocorrelation" => vec![sample_pacf(s)],
        "bartlett_standard_error" => vec![bartlett(s)],
        "cross_correlation_at_lag" => vec![ccf_at(s)],
        "cross_correlation_best_lag" => vec![ccf_best(s)],
        "ar_yule_walker" => vec![yw_orders(s)],
        "ar_forecast" | "arima_p10_forecast" | "drift_forecast" | "linear_trend_forecast" | "exponential_trend_forecast"
        | "quadratic_trend_forecast" | "holt_winters_multiplicative" => vec![fc::history_forecast(s)],
        "holt_linear_trend" | "holt_damped_trend" => vec![fc::holt_pair(s)],
        "holt_winters_additive" => vec![fc::hw_three(s), fc::hw_alpha(s)],
        "holt_update" => vec![fc::holt_update(s)],
        "exponential_smoothing_update" => vec![fc::ses_update(s)],
        "simple_exponential_smoothing" => vec![fc::ses_forecasts(s)],
        "ses_best_alpha" => vec![fc::ses_sse(s)],
        "trend_seasonal_regression" => vec![fc::trend_season_fit(s)],
        "centred_moving_average" | "simple_moving_average" | "linearly_weighted_moving_average"
        | "exponential_moving_average" => vec![fc::smooth(s)],
        "seasonal_indices_additive" | "seasonal_indices_multiplicative" => vec![fc::season_bars(s)],
        "seasonally_adjusted_additive" | "seasonally_adjusted_multiplicative" => vec![fc::adjusted(s)],
        "compound_average_growth_rate" => vec![fc::growth(s)],
        "differencing" => vec![fc::differences(s)],
        "exponential_interpolation" | "linear_interpolation" => vec![fc::interpolation(s)],
        "forecast_error_variance" => vec![fc::error_fan(s)],
        "mean_absolute_error" | "mean_absolute_percentage_error" | "mean_forecast_error" | "mean_squared_error"
        | "root_mean_squared_error" | "symmetric_mape" => vec![mo::errors(s)],
        "mean_absolute_scaled_error" => vec![mo::mase(s)],
        "augmented_dickey_fuller_test" | "dickey_fuller_test" | "kpss_test" => vec![mo::stat_by_setting(s)],
        "box_pierce_test" | "ljung_box_test" => vec![mo::q_by_lag(s)],
        "change_point_mean_shift" => vec![mo::one_change(s)],
        "two_change_points" => vec![mo::two_changes(s), mo::change_comparison(s)],
        "penalised_change_points" => vec![mo::penalised(s)],
        "cusum_change_point" => vec![mo::cusum_path(s)],
        "tabular_cusum" => vec![mo::tabular_cusum(s)],
        "mann_kendall_test" | "sens_slope" => vec![mo::trend(s)],
        "kalman_filter_constant_velocity" => vec![mo::kf_cv(s)],
        "kalman_filter_local_level" => vec![mo::kf_band(s), mo::kf_filter(s)],
        "rts_smoother_local_level" => vec![mo::rts_family(s)],
        "kalman_steady_state_constant_velocity" => vec![mo::cv_gains(s)],
        "kalman_steady_state_gain" => vec![mo::gain_convergence(s), mo::gain_field(s)],
        "kalman_step" | "kalman_update" => vec![mo::densities(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ shared helpers
/// A number input as read (a whole number too).
pub fn n(s: &Solved, sym: &str) -> Option<f64> {
    if let Some(x) = s.inp(sym) {
        return Some(x);
    }
    let v = match s.val(sym) {
        Some(Val::Num(x)) => Some(x),
        Some(Val::Int(i)) => Some(i as f64),
        _ => match &s.given.iter().find(|g| g.0 == sym)?.1 {
            // a plain number input as given (the model's numbers here carry no unit)
            Input::Float(x) => Some(*x),
            Input::Int(i) => Some(*i as f64),
            Input::Json(j) => j.as_f64(),
            Input::Text(t) => t.trim().replace('−', "-").parse::<f64>().ok(),
        },
    };
    v.filter(|x| x.is_finite())
}

/// A list as an input.
pub fn vj(v: &[f64]) -> Input {
    Input::Json(serde_json::Value::Array(v.iter().map(|x| serde_json::json!(x)).collect()))
}

pub fn ii(i: i64) -> Input {
    Input::Int(i)
}

pub fn fl(x: f64) -> Input {
    Input::Float(x)
}

/// A list written out ("1, 2, 3"), at most `max` values.
pub fn lst(v: &[f64]) -> String {
    let mut t: Vec<String> = v.iter().take(10).map(|x| num(*x)).collect();
    if v.len() > 10 {
        t.push("...".into());
    }
    t.join(", ")
}

/// A series against its period t = 1 ... n.
pub fn idx(v: &[f64]) -> Vec<(f64, f64)> {
    v.iter().enumerate().map(|(i, x)| (i as f64 + 1.0, *x)).collect()
}

/// A number output of another answer.
pub fn o(a: &Answer, sym: &str) -> Option<f64> {
    k::num_of(a, sym)
}

/// A list output of another answer.
pub fn ol(a: &Answer, sym: &str) -> Option<Vec<f64>> {
    k::list_of(a, sym)
}

/// Another capability of the model, given this request's inputs that it also takes, with some replaced.
pub fn other(s: &Solved, key: &str, set: &[(&str, Input)]) -> Option<Answer> {
    let spec = s.p.procedure(key)?;
    let has = |sym: &str| spec.inputs.iter().any(|i| i.symbol == sym);
    let mut g: Vec<(&str, Input)> = vec![];
    for (kk, v) in s.given.iter() {
        if has(kk) && !set.iter().any(|x| x.0 == kk.as_str()) {
            g.push((kk.as_str(), v.clone()));
        }
    }
    for (kk, v) in set {
        if has(kk) {
            g.push((kk, v.clone()));
        }
    }
    s.solve_other(key, &g)
}

/// Two panels stacked into one figure.
pub fn two(s: &Solved, kind: &str, title: &str, caption: &str, a: String, b: String) -> Figure {
    let (body, h) = k::stack(&[a, b]);
    s.figure(kind, title, caption, &body, h)
}

/// Points as stems shifted sideways by `dx` (two correlograms side by side).
fn shift(pts: &[(f64, f64)], dx: f64) -> Vec<(f64, f64)> {
    pts.iter().map(|p| (p.0 + dx, p.1)).collect()
}

/// A list output as (lag, value) for lags 1, 2, ...
fn lags(v: &[f64]) -> Vec<(f64, f64)> {
    idx(v)
}

// ------------------------------------------------------------------------------------------------ correlograms
/// A list output of this capability over K lags (the run with K set), drawn as stems with lag 1 marked.
#[allow(clippy::too_many_arguments)]
fn list_stems(s: &Solved, kind: &str, out: &str, ksym: &str, kk: i64, title: &str, ylab: &str, what: &str)
              -> Option<Figure> {
    let a = s.with_input(&[(ksym, ii(kk))])?;
    let v = ol(&a, out)?;
    let pts = lags(&v);
    let (xs, ys) = xs_ys(&[&pts]);
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.hline(0.0, "");
    p.stems(&pts, 0, "");
    p.mark(1.0, v[0], &num(v[0]));
    let body = p.finish("k, lag", ylab, &model_short(s));
    let model = model_words(s);
    Some(s.chart(kind, title, &format!("The {what} of {model} at lags 1 to {} as the model returns them: {}; marked: lag 1, \
                                        {}.", v.len(), lst(&v), num(v[0])), &body))
}

/// The process the request names ("the ARMA model φ = (0.5), θ = (0.4)").
fn model_parts(s: &Solved) -> Vec<String> {
    let mut parts = vec![];
    for (sym, name) in [("phi", "φ"), ("theta", "θ")] {
        if let Some(v) = s.vec(sym).or_else(|| n(s, sym).map(|x| vec![x])) {
            parts.push(format!("{name} = {}", lst(&v)));
        }
    }
    parts
}

fn model_words(s: &Solved) -> String {
    let parts = model_parts(s);
    if parts.is_empty() { "the process".into() } else { format!("the process with {}", parts.join("; ")) }
}

/// The process's coefficients as a short heading ("φ = 0.5, 0.2; θ = 0.4").
fn model_short(s: &Solved) -> String {
    model_parts(s).join("; ")
}

fn ar1_acf(s: &Solved) -> Option<Figure> {
    let phi = n(s, "phi")?;
    let kk = n(s, "k").unwrap_or(1.0);
    let top = (kk as i64).max(10).min(40);
    let mut pts = vec![];
    for lag in 0..=top {
        if let Some(a) = s.with_input(&[("k", ii(lag))]) {
            if let Some(r) = o(&a, "acf_k") {
                pts.push((lag as f64, r));
            }
        }
    }
    if pts.len() < 3 {
        return None;
    }
    let rk = pts.iter().find(|p| p.0 == kk).map(|p| p.1)?;
    let (xs, ys) = xs_ys(&[&pts]);
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.hline(0.0, "");
    p.stems(&pts, 0, "");
    p.mark(kk, rk, &format!("ρ({}) = {}", kk, num(rk)));
    let mean = s.out("mean").map(|m| format!(" The process mean is {} and its variance {}.", num(m),
                                             num(s.out("variance").unwrap_or(f64::NAN)))).unwrap_or_default();
    let body = p.finish("k, lag", "ρ(k), autocorrelation", &format!("AR(1), φ = {}", num(phi)));
    Some(s.chart("acf", "AR(1): geometric decay of the autocorrelation",
                 &format!("The autocorrelation function of the AR(1) process with φ = {} at lags 0 to {top}, each a run \
                           of the model: it decays geometrically, as φ^k.{mean} Marked: ρ({kk}) = {}.", num(phi),
                          num(rk)), &body))
}

fn ar2_acf(s: &Solved) -> Option<Figure> {
    let (p1, p2) = (n(s, "phi1")?, n(s, "phi2")?);
    let a = s.with_input(&[("K", ii(12))])?;
    let v = ol(&a, "acf")?;
    // a contrasting model: damped oscillation (complex roots), or smooth decay when the example oscillates
    let (c1, c2) = if (p1 - 1.0).abs() < 1e-9 && (p2 + 0.6).abs() < 1e-9 { (0.5, 0.3) } else { (1.0, -0.6) };
    let b = s.with_input(&[("phi1", fl(c1)), ("phi2", fl(c2)), ("K", ii(12))]);
    let w = b.as_ref().and_then(|b| ol(b, "acf"));
    let pa = shift(&lags(&v), -0.12);
    let pb = w.as_ref().map(|w| shift(&lags(w), 0.12)).unwrap_or_default();
    let (xs, ys) = xs_ys(&[&pa, &pb]);
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.hline(0.0, "");
    p.stems(&pa, 0, &format!("φ = {}, {} (example)", num(p1), num(p2)));
    if !pb.is_empty() {
        p.stems(&pb, 1, &format!("φ = {}, {}", num(c1), num(c2)));
    }
    let body = p.finish("k, lag", "ρ(k), autocorrelation", "");
    let other_t = w.as_ref().map(|w| format!(", beside the model φ₁ = {}, φ₂ = {} for contrast (ρ(1) = {})", num(c1),
                                             num(c2), num(w[0]))).unwrap_or_default();
    Some(s.chart("acf", "AR(2) autocorrelation: smooth decay or damped oscillation",
                 &format!("Theoretical autocorrelations of the AR(2) process φ₁ = {}, φ₂ = {} at lags 1 to {} from the \
                           model (ρ(1) = {}){other_t}. Stems are offset slightly to keep both visible.", num(p1), num(p2),
                          v.len(), num(v[0])), &body))
}

fn arma11_acf(s: &Solved) -> Option<Figure> {
    let (phi, th) = (n(s, "phi")?, n(s, "theta")?);
    let v = ol(&s.with_input(&[("K", ii(10))])?, "acf")?;
    let w = s.with_input(&[("theta", fl(0.0)), ("K", ii(10))]).and_then(|b| ol(&b, "acf"));
    let pa = shift(&lags(&v), -0.12);
    let pb = w.as_ref().map(|w| shift(&lags(w), 0.12)).unwrap_or_default();
    let (xs, ys) = xs_ys(&[&pa, &pb]);
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.hline(0.0, "");
    p.stems(&pa, 0, &format!("ARMA(1,1), θ = {}", num(th)));
    if !pb.is_empty() {
        p.stems(&pb, 1, "θ = 0: AR(1)");
    }
    let body = p.finish("k, lag", "ρ(k), autocorrelation", &format!("φ = {}", num(phi)));
    Some(s.chart("acf", "ARMA(1,1) autocorrelation against AR(1)",
                 &format!("Theoretical autocorrelations of the ARMA(1,1) process φ = {}, θ = {} at lags 1 to {} (ρ(1) = \
                           {}), beside the pure AR(1) with the same φ (θ = 0{}), both from the model: the MA term \
                           changes the first lag, after which both decay by the factor φ.", num(phi), num(th), v.len(),
                          num(v[0]), w.map(|w| format!(", ρ(1) = {}", num(w[0]))).unwrap_or_default()), &body))
}

fn ma_acf(s: &Solved) -> Option<Figure> {
    let th = s.vec("theta")?;
    let kk = (th.len() as i64 + 4).max(6);
    let v = ol(&s.with_input(&[("K", ii(kk))])?, "acf")?;
    let pts = lags(&v);
    let (xs, ys) = xs_ys(&[&pts]);
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.hline(0.0, "");
    p.stems(&pts, 0, "");
    p.mark(1.0, v[0], &num(v[0]));
    let q = th.len();
    let body = p.finish("k, lag", "ρ(k), autocorrelation", &format!("MA({q}), θ = {}", lst(&th)));
    Some(s.chart("acf", &format!("MA({q}) autocorrelation cuts off after lag {q}"),
                 &format!("Theoretical autocorrelations of the MA({q}) process with θ = ({}): zero beyond lag {q}. Lags 1 \
                           to {} as returned by the model: {}; marked: lag 1, {}.", lst(&th), v.len(), lst(&v), num(v[0])),
                 &body))
}

fn acf_pacf(s: &Solved) -> Option<Figure> {
    let rho = s.vec("rho")?;
    let pacf = s.out_list("pacf")?;
    let pa = shift(&lags(&rho), -0.08);
    let pb = shift(&lags(&pacf), 0.08);
    let (xs, ys) = xs_ys(&[&pa, &pb]);
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.hline(0.0, "");
    p.stems(&pa, 3, "ρ(k) (input)");
    p.stems(&pb, 0, "φ(k,k) (returned)");
    let body = p.finish("k, lag", "correlation", "");
    Some(s.chart("acf-pacf", "From autocorrelations to partial autocorrelations",
                 &format!("The stated autocorrelations ρ = {} and the partial autocorrelations the model derives from \
                           them: {}; the first equals ρ(1), later ones remove the part explained by shorter lags.",
                          lst(&rho), lst(&pacf)), &body))
}

fn yw_acf(s: &Solved) -> Option<Figure> {
    let rho = s.vec("rho")?;
    let phi = s.out_list("phi")?;
    let a = other(s, "arma_theoretical_acf", &[("phi", vj(&phi)), ("theta", vj(&[0.0])), ("K", ii(8))])?;
    let v = ol(&a, "acf")?;
    let pa = lags(&v);
    let pb = lags(&rho);
    let (xs, ys) = xs_ys(&[&pa, &pb]);
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.hline(0.0, "");
    p.stems(&pa, 0, &format!("AR({}) with the returned φ", phi.len()));
    p.dots(&pb, 1, "given ρ (input)");
    let body = p.finish("k, lag", "ρ(k), autocorrelation", &format!("φ = {}", lst(&phi)));
    Some(s.chart("acf", &format!("Yule-Walker: an AR({}) that reproduces the autocorrelations", phi.len()),
                 &format!("The Yule-Walker equations turn the stated autocorrelations ρ = {} into AR({}) coefficients φ = \
                           {}; the stems are the theoretical autocorrelations of that AR model, lags 1 to {}, from the \
                           model: they pass through the given values and continue the pattern.", lst(&rho), phi.len(),
                          lst(&phi), v.len()), &body))
}

/// The 95 % band half-width from Bartlett's standard error at a lag (the model's bartlett_standard_error).
fn bartlett_hw(s: &Solved, x: &[f64], lag: i64) -> Option<(f64, f64)> {
    let a = other(s, "bartlett_standard_error", &[("x", vj(x)), ("k", ii(lag))])?;
    Some((o(&a, "half_width_95")?, o(&a, "acf")?))
}

fn sample_acf(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let kk = n(s, "k")? as i64;
    let top = kk.max(((x.len() as i64) - 2).min(8));
    let mut pts = vec![];
    let mut hw = vec![];
    for lag in 1..=top {
        if let Some((h, r)) = bartlett_hw(s, &x, lag) {
            pts.push((lag as f64, r));
            hw.push((lag as f64, h));
        }
    }
    // the autocorrelations are the model's own list (the run with k = top), when it gives them
    if let Some(a) = s.with_input(&[("k", ii(top))]) {
        if let Some(v) = ol(&a, "acf_list") {
            pts = lags(&v);
        }
    }
    if pts.is_empty() {
        return None;
    }
    let lo: Vec<(f64, f64)> = hw.iter().map(|p| (p.0, -p.1)).collect();
    let (xs, mut ys) = xs_ys(&[&pts, &hw, &lo]);
    ys.push(0.0);
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.band(&lo, &hw, 3, "95 % band (Bartlett)");
    p.hline(0.0, "");
    p.stems(&pts, 0, "sample autocorrelation");
    if let Some(r) = pts.iter().find(|q| q.0 == kk as f64) {
        p.mark(r.0, r.1, &num(r.1));
    }
    let body = p.finish("k, lag", "autocorrelation", "");
    let vals: Vec<f64> = pts.iter().map(|q| q.1).collect();
    let hws: Vec<f64> = hw.iter().map(|q| q.1).collect();
    Some(s.chart("acf", "Autocorrelation function with its significance band",
                 &format!("Sample autocorrelations of x = {} at lags 1 to {} ({}) and the 95 % band from Bartlett's \
                           standard errors ({}), all returned by the model.", lst(&x), pts.len(), lst(&vals),
                          if hws.is_empty() { "not available".into() } else {
                              format!("±{} to ±{}", num(hws[0]), num(hws[hws.len() - 1]))
                          }), &body))
}

fn lag_plot(s: &Solved, fit: bool) -> Option<Figure> {
    let x = s.vec("x")?;
    if x.len() < 3 {
        return None;
    }
    let pairs: Vec<(f64, f64)> = x.windows(2).map(|w| (w[0], w[1])).collect();
    let (xs, ys) = xs_ys(&[&pairs]);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    let caption;
    let title;
    let header;
    if fit {
        // the line of the AR(1) least-squares fit (the run with p = 1), drawn from its returned values
        let a = s.with_input(&[("p", ii(1))])?;
        let (c, phi) = (o(&a, "const")?, o(&a, "phi1")?);
        let (lo, hi) = (xs.iter().cloned().fold(f64::INFINITY, f64::min), xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
        p.line(&[(lo, c + phi * lo), (hi, c + phi * hi)], 1, "least-squares AR(1)");
        p.dots(&pairs, 0, "pairs (x(t-1), x(t))");
        header = format!("x(t) = {} + {} x(t-1)", num(c), num(phi));
        title = "AR(1) least squares: the line through the lag plot".to_string();
        caption = format!("Lag plot of the series x = {}: each point pairs a value with the one before it. The model's \
                           least-squares AR(1) fit is x(t) = {} + {}·x(t-1) (innovation variance {}), drawn from those \
                           returned values.", lst(&x), num(c), num(phi), num(o(&a, "sigma2").unwrap_or(f64::NAN)));
    } else {
        let r1 = s.with_input(&[("k", ii(1))]).and_then(|a| o(&a, "acf"))?;
        p.dots(&pairs, 0, "pairs (x(t-1), x(t))");
        header = format!("lag-1 autocorrelation {}", num(r1));
        title = "Lag-1 scatter behind the sample autocorrelation".to_string();
        caption = format!("Lag plot of the series x = {}: each point pairs a value with the one before it. The model's \
                           lag-1 sample autocorrelation of these values is {}.", lst(&x), num(r1));
    }
    let body = p.finish("x(t-1)", "x(t)", &header);
    Some(s.chart("lag-plot", &title, &caption, &body))
}

fn sample_acov(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let kk = n(s, "k")?;
    let top = (kk as i64).max(((x.len() as i64) - 2).min(6));
    let mut pts = vec![];
    for lag in 0..=top {
        if let Some(v) = s.with_input(&[("k", ii(lag))]).and_then(|a| o(&a, "acov")) {
            pts.push((lag as f64, v));
        }
    }
    if pts.len() < 2 {
        return None;
    }
    let (xs, ys) = xs_ys(&[&pts]);
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.hline(0.0, "");
    p.stems(&pts, 0, "sample autocovariance");
    let g0 = s.out("gamma0")?;
    p.mark(0.0, g0, &format!("γ(0) = {}", num(g0)));
    let body = p.finish("k, lag", "γ(k), autocovariance", "");
    let vals: Vec<f64> = pts.iter().map(|q| q.1).collect();
    Some(s.chart("acov", "Sample autocovariance by lag",
                 &format!("Sample autocovariances of x = {} at lags 0 to {top}, each a run of the model: γ(0) = {} is the \
                           variance (divisor n), then {}.", lst(&x), num(g0), lst(&vals[1..])), &body))
}

fn sample_pacf(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let kk = n(s, "k")? as i64;
    let mut v = None;
    let mut top = kk.max(((x.len() as i64) / 2 - 1).min(6));
    while top >= kk && v.is_none() {
        v = s.with_input(&[("k", ii(top))]).and_then(|a| ol(&a, "pacf_list"));
        top -= 1;
    }
    let v = v?;
    let pts = lags(&v);
    let band = 1.96 / (x.len() as f64).sqrt();
    let (xs, mut ys) = xs_ys(&[&pts]);
    ys.extend([band, -band]);
    let mut p = Plot::new(&xs, &ys, true, false)?;
    let ends = [(xs[0] - 0.5, 0.0), (xs[xs.len() - 1] + 0.5, 0.0)];
    let up: Vec<(f64, f64)> = ends.iter().map(|e| (e.0, band)).collect();
    let dn: Vec<(f64, f64)> = ends.iter().map(|e| (e.0, -band)).collect();
    p.band(&dn, &up, 3, &format!("±1.96/√n = ±{}", num(band)));
    p.hline(0.0, "");
    p.stems(&pts, 0, "sample partial autocorrelation");
    let body = p.finish("k, lag", "φ(k,k), partial autocorr.", "");
    Some(s.chart("pacf", "Sample partial autocorrelations with their band",
                 &format!("Sample partial autocorrelations of x = {} at lags 1 to {}, as returned by the model ({}), with \
                           the approximate 95 % band ±1.96/√n for n = {} (computed from n).", lst(&x), v.len(), lst(&v),
                          x.len()), &body))
}

fn bartlett(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let kk = n(s, "k")? as i64;
    let top = kk.max(((x.len() as i64) - 2).min(6));
    let mut pts = vec![];
    let mut hw = vec![];
    for lag in 1..=top {
        if let Some((h, r)) = bartlett_hw(s, &x, lag) {
            pts.push((lag as f64, r));
            hw.push((lag as f64, h));
        }
    }
    if pts.len() < 2 {
        return None;
    }
    let lo: Vec<(f64, f64)> = hw.iter().map(|p| (p.0, -p.1)).collect();
    let (xs, ys) = xs_ys(&[&pts, &hw, &lo]);
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.band(&lo, &hw, 3, "95 % band from Bartlett's SE");
    p.hline(0.0, "");
    p.stems(&pts, 0, "sample autocorrelation");
    let se = s.out("se")?;
    let body = p.finish("k, lag", "autocorrelation", &format!("k = {kk}: SE = {}", num(se)));
    Some(s.chart("acf", "Autocorrelations against Bartlett's widening band",
                 &format!("Sample autocorrelations of x = {} at lags 1 to {} and the 95 % band ±1.96·SE whose half-width \
                           the model returns at each lag (Bartlett's standard error grows with the lag: ±{} to ±{}); at \
                           k = {kk}: SE = {}.", lst(&x), pts.len(), num(hw[0].1), num(hw[hw.len() - 1].1), num(se)),
                 &body))
}

/// The cross-correlation at a lag: lag >= 0 a run with k = lag, a negative lag the run with x and y swapped.
fn ccf(s: &Solved, x: &[f64], y: &[f64], lag: i64) -> Option<f64> {
    let (a, b) = if lag >= 0 { (x, y) } else { (y, x) };
    let r = other(s, "cross_correlation_at_lag", &[("x", vj(a)), ("y", vj(b)), ("k", ii(lag.abs()))])?;
    o(&r, "ccf")
}

fn ccf_at(s: &Solved) -> Option<Figure> {
    let (x, y) = (s.vec("x")?, s.vec("y")?);
    let kk = n(s, "k")? as i64;
    let top = kk.abs().max(3);
    let pts: Vec<(f64, f64)> = (-top..=top).filter_map(|l| ccf(s, &x, &y, l).map(|r| (l as f64, r))).collect();
    if pts.len() < 3 {
        return None;
    }
    let (xs, ys) = xs_ys(&[&pts]);
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.hline(0.0, "");
    p.stems(&pts, 0, "cross-correlation of x and y");
    let r = s.out("ccf")?;
    p.mark(kk as f64, r, &format!("k = {kk}: {}", num(r)));
    let body = p.finish("k, lag", "cross-correlation", "");
    let vals: Vec<String> = pts.iter().map(|q| format!("{}: {}", q.0, num(q.1))).collect();
    Some(s.chart("ccf", "Cross-correlation across lags",
                 &format!("Cross-correlations of x = {} and y = {} at lags -{top} to {top} (non-negative lags are runs of \
                           the model; a negative lag -k is the run with x and y swapped at lag k): {}; marked: k = {kk}.",
                          lst(&x), lst(&y), vals.join(", ")), &body))
}

fn ccf_best(s: &Solved) -> Option<Figure> {
    let (x, y) = (s.vec("x")?, s.vec("y")?);
    let kmax = n(s, "K")? as i64;
    let (best, r) = (s.out("lag")?, s.out("ccf")?);
    let pts: Vec<(f64, f64)> = (-kmax..=kmax).filter_map(|l| ccf(s, &x, &y, l).map(|v| (l as f64, v))).collect();
    if pts.is_empty() {
        return None;
    }
    let (sx, sy) = (idx(&x), idx(&y));
    let (xs, ys) = xs_ys(&[&sx, &sy]);
    let mut top = Plot::new(&xs, &ys, false, false)?;
    top.line_dots(&sx, 0, "x");
    top.line_dots(&sy, 1, "y");
    let a = top.finish("t, period", "value", "");
    let (xs, ys) = xs_ys(&[&pts]);
    let mut bot = Plot::new(&xs, &ys, true, false)?;
    bot.hline(0.0, "");
    bot.stems(&pts, 0, "");
    bot.mark(best, r, &format!("best lag {}: {}", best, num(r)));
    let b = bot.finish("k, lag", "cross-correlation", "");
    Some(two(s, "series-ccf", "The lag at which two series line up best",
             &format!("Two series x = {} and y = {} (top) and their cross-correlations at lags -{kmax} to {kmax} \
                       (bottom; non-negative lags run directly, negative ones with x and y swapped): the model returns the \
                       best lag {} with correlation {}.", lst(&x), lst(&y), best, num(r)), a, b))
}

fn yw_orders(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let pp = n(s, "p")? as i64;
    let top = pp.max(4).min(8);
    let mut fits: Vec<(i64, Vec<f64>, f64)> = vec![];
    for order in 1..=top {
        if let Some(a) = s.with_input(&[("p", ii(order))]) {
            if let (Some(phi), Some(v)) = (ol(&a, "phi"), o(&a, "sigma2")) {
                fits.push((order, phi, v));
            }
        }
    }
    if fits.len() < 2 {
        return None;
    }
    let mut series: Vec<Vec<(f64, f64)>> = vec![];
    for j in 0..top as usize {
        let pts: Vec<(f64, f64)> = fits.iter().filter(|f| f.1.len() > j).map(|f| (f.0 as f64, f.1[j])).collect();
        if !pts.is_empty() {
            series.push(pts);
        }
    }
    let refs: Vec<&[(f64, f64)]> = series.iter().map(|v| v.as_slice()).collect();
    let (xs, ys) = xs_ys(&refs);
    let mut a = Plot::new(&xs, &ys, true, false)?;
    a.hline(0.0, "");
    for (j, pts) in series.iter().enumerate().take(4) {
        a.line_dots(pts, j, &format!("φ{}", j + 1));
    }
    let a = a.finish("p, AR order", "coefficient φⱼ", "");
    let sig: Vec<(f64, f64)> = fits.iter().map(|f| (f.0 as f64, f.2)).collect();
    let (xs, ys) = xs_ys(&[&sig]);
    let mut b = Plot::new(&xs, &ys, false, false)?;
    b.line_dots(&sig, 0, "");
    if let Some(m) = sig.iter().find(|q| q.0 == pp as f64) {
        b.mark(m.0, m.1, &format!("p = {pp}: σ² = {}", num(m.1)));
    }
    let b = b.finish("p, AR order", "σ², innovation variance", "");
    let each: Vec<String> = fits.iter().map(|f| format!("p = {}: φ = {}, σ² = {}", f.0, lst(&f.1), num(f.2))).collect();
    Some(two(s, "orders", &format!("Yule-Walker coefficients for AR orders 1 to {}", fits.last()?.0),
             &format!("Yule-Walker AR fits of x = {} for orders p = 1 to {}, each a run of the model: the coefficients \
                       (top) and the innovation variance (bottom). {}.", lst(&x), fits.last()?.0, each.join("; ")), a, b))
}

// ------------------------------------------------------------------------------------------------ AR(2) / MA(2) roots
fn roots_syms(ar: bool) -> (&'static str, &'static str, &'static str, &'static str, &'static str) {
    if ar { ("phi1", "phi2", "stationary", "φ", "stationar") } else { ("theta1", "theta2", "invertible", "θ", "invertib") }
}

fn roots_modulus(s: &Solved, ar: bool) -> Option<Figure> {
    let (s1, s2, verdict, g, _) = roots_syms(ar);
    let (a1, a2) = (n(s, s1)?, n(s, s2)?);
    let m = s.out("max_modulus")?;
    let lo = (-2.0f64).min(a1 - 0.2);
    let hi = 2.0f64.max(a1 + 0.2);
    let xs = k::span(lo, hi, 31, false);
    let mut vals = vec![-0.5, a2, 0.6];
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    vals.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
    let mut series = vec![];
    for v in vals {
        let pts = s.curve(s1, &xs, "max_modulus", &[(s2, v)]);
        if pts.len() >= 2 {
            let ex = (v - a2).abs() < 1e-9;
            series.push((format!("{g}₂ = {}{}", num(v), if ex { " (example)" } else { "" }), pts, ex));
        }
    }
    if series.is_empty() {
        return None;
    }
    let refs: Vec<&[(f64, f64)]> = series.iter().map(|v| v.1.as_slice()).collect();
    let (xs2, mut ys) = xs_ys(&refs);
    ys.push(0.0);
    ys.push(1.0);
    let ys: Vec<f64> = ys.into_iter().map(|y| y.min(3.0)).collect();
    let mut p = Plot::new(&xs2, &ys, true, false)?;
    let mut c = 1;
    for (name, pts, ex) in &series {
        let cc = if *ex { 0 } else { c += 1; c - 1 };
        p.line(pts, cc, name);
    }
    p.hline_at(1.0, "modulus 1", true);
    p.mark(a1, m, &num(m));
    let body = p.finish(&format!("{g}₁"), "largest root modulus", "");
    let name = if ar { "AR(2)" } else { "MA(2)" };
    Some(s.chart("modulus", &format!("{name}: root modulus across {g}₁"),
                 &format!("The largest root modulus the model returns for {name} models as {g}₁ runs from {} to {} (31 \
                           runs per curve), for {} values of {g}₂; the model is {verdict} where the curve is below 1. \
                           Marked: {g}₁ = {}, {g}₂ = {}, modulus {}.", num(lo), num(hi), series.len(), num(a1), num(a2),
                          num(m)), &body))
}

fn roots_region(s: &Solved, ar: bool) -> Option<Figure> {
    let (s1, s2, verdict, g, _) = roots_syms(ar);
    let (a1, a2) = (n(s, s1)?, n(s, s2)?);
    let xs = k::span((-2.15f64).min(a1 - 0.1), 2.15f64.max(a1 + 0.1), 23, false);
    let ys = k::span((-1.15f64).min(a2 - 0.1), 1.15f64.max(a2 + 0.1), 13, false);
    let no = format!("not {verdict}");
    let names = vec![no.clone(), "real roots".to_string(), "complex roots".to_string()];
    let mut cells = vec![];
    let mut count = [0usize; 3];
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            let Some(a) = s.with(&[(s1, *x), (s2, *y)]) else { continue };
            let (Some(v), Some(r)) = (text(&a, verdict), text(&a, "roots")) else { continue };
            let c = if v != "yes" { 0 } else if r == "real" { 1 } else { 2 };
            count[c] += 1;
            cells.push((i, j, names[c].clone()));
        }
    }
    if cells.len() < 20 {
        return None;
    }
    let m = s.out("max_modulus")?;
    let body = k::decision(&xs, &ys, &cells, &names, (a1, a2), &format!("{g}₁"), &format!("{g}₂"),
                           &format!("max |root| {}", num(m)));
    let name = if ar { "AR(2)" } else { "MA(2)" };
    Some(s.chart("region", &format!("Where an {name} model is {verdict}"),
                 &format!("Which {name} models are {verdict}: the model's verdict at {} points of the ({g}₁, {g}₂) plane, \
                           {verdict} with real roots ({}), with complex roots ({}), or not {verdict} ({}); together they \
                           trace a triangle. Marked: {g}₁ = {}, {g}₂ = {}: {}, {} roots, largest root modulus {}.",
                          cells.len(), count[1], count[2], count[0], num(a1), num(a2),
                          s.out_text(verdict).unwrap_or_default(), s.out_text("roots").unwrap_or_default(), num(m)),
                 &body))
}

/// A text or true/false output of another answer as words.
pub fn text(a: &Answer, sym: &str) -> Option<String> {
    match &a.values.iter().find(|x| x.0 == sym)?.1 {
        serde_json::Value::String(t) => Some(t.clone()),
        serde_json::Value::Bool(b) => Some(if *b { "yes".into() } else { "no".into() }),
        _ => None,
    }
}
