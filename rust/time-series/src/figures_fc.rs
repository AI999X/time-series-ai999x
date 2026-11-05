//! Time Series' own figures: forecasts with their history, exponential smoothing, smoothers, seasonal indices,
//! growth, differencing, interpolation (see figures.rs).

use crate::figures::{fl, idx, ii, lst, n, o, ol, other, two, vj};
use crate::figures_plot::{xs_ys, Plot};
use ax_engine::figkit::{self as k, num, Solved};
use ax_engine::figures::Figure;
use ax_engine::Answer;

fn dedup(mut v: Vec<f64>) -> Vec<f64> {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    v.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    v
}

/// The forecasts h = 1 ... hmax of a run (self or another capability) as (t0 + h, forecast).
fn forecasts<F: Fn(i64) -> Option<Answer>>(hmax: i64, t0: f64, run: F) -> Vec<(f64, f64)> {
    (1..=hmax).filter_map(|h| run(h).and_then(|a| o(&a, "forecast")).map(|f| (t0 + h as f64, f))).collect()
}

// ------------------------------------------------------------------------------------------------ AR(1)
pub fn ar1_forecast(s: &Solved) -> Option<Figure> {
    let (x, phi, mu) = (n(s, "x")?, n(s, "phi")?, n(s, "mu")?);
    let h = n(s, "h").unwrap_or(1.0);
    let given_var = n(s, "sigma2");
    let var = given_var.unwrap_or(1.0);
    let hmax = (h as i64).max(12).min(40);
    let mut f = vec![(0.0, x)];
    let mut lo = vec![];
    let mut hi = vec![];
    for hh in 1..=hmax {
        if let Some(a) = s.with_input(&[("h", ii(hh)), ("sigma2", fl(var))]) {
            if let Some(v) = o(&a, "forecast") {
                f.push((hh as f64, v));
            }
            if let (Some(l), Some(u)) = (o(&a, "lower"), o(&a, "upper")) {
                lo.push((hh as f64, l));
                hi.push((hh as f64, u));
            }
        }
    }
    if f.len() < 3 {
        return None;
    }
    let fh = s.out("forecast")?;
    let (xs, mut ys) = xs_ys(&[&f, &lo, &hi]);
    ys.push(mu);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    let band_name = if given_var.is_some() { "95 % interval".to_string() } else { "95 % interval, σ² = 1".to_string() };
    p.band(&lo, &hi, 3, &band_name);
    p.hline_at(mu, &format!("mean μ = {}", num(mu)), true);
    p.line_dots(&f, 0, "forecast");
    p.mark(h, fh, &num(fh));
    let body = p.finish("h, steps ahead", "forecast", "");
    let last = lo.last().zip(hi.last());
    let var_t = if given_var.is_some() { format!("innovation variance σ² = {}", num(var)) } else {
        "innovation variance σ² = 1 (not stated: the band shows its shape for a unit variance)".to_string()
    };
    Some(s.chart("forecast", "AR(1) forecast fan reverting to the mean",
                 &format!("AR(1) forecasts from the last value x = {} with φ = {}, μ = {} and {var_t}, 1 to {hmax} steps \
                           ahead, each a run of the model, with the 95 % interval it returns at each step{}; marked: h = \
                           {h}, forecast = {}.", num(x), num(phi), num(mu),
                          last.map(|(l, u)| format!(" (at h = {hmax}: [{}, {}])", num(l.1), num(u.1))).unwrap_or_default(),
                          num(fh)), &body))
}

pub fn ar1_paths(s: &Solved) -> Option<Figure> {
    let (x, phi, mu) = (n(s, "x")?, n(s, "phi")?, n(s, "mu")?);
    let phis = dedup(vec![0.3, phi, 0.95]);
    let mut series = vec![];
    for ph in &phis {
        let mut pts = vec![(0.0, x)];
        for hh in 1..=12 {
            if let Some(v) = s.with_input(&[("phi", fl(*ph)), ("h", ii(hh))]).and_then(|a| o(&a, "forecast")) {
                pts.push((hh as f64, v));
            }
        }
        if pts.len() >= 3 {
            series.push((*ph, pts));
        }
    }
    if series.is_empty() {
        return None;
    }
    let refs: Vec<&[(f64, f64)]> = series.iter().map(|v| v.1.as_slice()).collect();
    let (xs, mut ys) = xs_ys(&refs);
    ys.push(mu);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.hline_at(mu, &format!("mean μ = {}", num(mu)), true);
    let mut c = 1;
    let mut parts = vec![];
    for (ph, pts) in &series {
        let ex = (ph - phi).abs() < 1e-12;
        let cc = if ex { 0 } else { c += 1; c - 1 };
        p.line_dots(pts, cc, &format!("φ = {}{}", num(*ph), if ex { " (example)" } else { "" }));
        let at3 = pts.iter().find(|q| q.0 == 3.0).map(|q| num(q.1)).unwrap_or_else(|| "-".into());
        parts.push(format!("φ = {}: {} at h = 3, {} at h = 12", num(*ph), at3, num(pts[pts.len() - 1].1)));
    }
    let body = p.finish("h, steps ahead", "forecast", "");
    Some(s.chart("paths", "AR(1) forecasts return to the mean at a rate set by φ",
                 &format!("AR(1) forecasts from the last value x = {} towards the mean μ = {}, h = 1 to 12 steps ahead, \
                           for {} AR coefficients (each point a run of the model): {}. The larger φ, the slower the return \
                           to the mean.", num(x), num(mu), series.len(), parts.join("; ")), &body))
}

// ------------------------------------------------------------------------------------------------ history + forecast
pub fn history_forecast(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let nn = x.len() as f64;
    let h = n(s, "h").unwrap_or(1.0);
    let (hdef, title) = match s.key {
        "ar_forecast" => (8, "AR(p) forecasts continuing the series"),
        "arima_p10_forecast" => (8, "ARIMA(1,1,0): the differences follow an AR(1)"),
        "drift_forecast" => (6, "Drift forecast: the average change carried forward"),
        "linear_trend_forecast" => (4, "Least-squares trend line and its forecasts"),
        "exponential_trend_forecast" => (4, "Exponential trend: constant growth rate"),
        "quadratic_trend_forecast" => (3, "Quadratic trend fitted to the series"),
        _ => (8, "Holt-Winters multiplicative forecast"),
    };
    let hmax = (h as i64).max(hdef).min(40);
    let mut f: Vec<(f64, f64)> = vec![];
    if matches!(s.key, "ar_forecast" | "arima_p10_forecast") {
        let a = s.with_input(&[("h", ii(hmax))])?;
        f = ol(&a, "forecasts")?.iter().enumerate().map(|(i, v)| (nn + 1.0 + i as f64, *v)).collect();
    }
    if f.is_empty() {
        f = forecasts(hmax, nn, |hh| s.with_input(&[("h", ii(hh))]));
    }
    if f.is_empty() {
        return None;
    }
    let fh = s.out("forecast")?;
    let hist = idx(&x);
    let mut path = vec![(nn, x[x.len() - 1])];
    path.extend(f.iter().copied());
    // a fitted curve drawn from the returned coefficients (t = 1 ... n + hmax)
    let ts = k::span(1.0, nn + hmax as f64, 61, false);
    let (fit, fit_name, fit_t): (Vec<(f64, f64)>, String, String) = match s.key {
        "drift_forecast" => {
            let d = s.out("drift")?;
            (vec![(1.0, x[0]), (nn, x[x.len() - 1])], format!("first to last, drift {}", num(d)),
             format!(" The returned drift {} is the slope of the line from the first to the last value (dashed).", num(d)))
        }
        "linear_trend_forecast" => {
            let (b0, b1) = (s.out("intercept")?, s.out("slope")?);
            (ts.iter().map(|t| (*t, b0 + b1 * t)).collect(), format!("fitted {} + {}·t", num(b0), num(b1)),
             format!(" The returned trend: intercept {}, slope {} per period (t = 1 ... n, R² = {}), drawn dashed.",
                     num(b0), num(b1), num(s.out("r_squared").unwrap_or(f64::NAN))))
        }
        "exponential_trend_forecast" => {
            let (a, b) = (s.out("a")?, s.out("b")?);
            (ts.iter().map(|t| (*t, a * (b * t).exp())).collect(), format!("fitted {}·e^({} t)", num(a), num(b)),
             format!(" The returned exponential trend a = {}, b = {} (growth {} % per period, t = 1 ... n), drawn \
                      dashed from those values.", num(a), num(b), num(100.0 * s.out("growth_rate").unwrap_or(f64::NAN))))
        }
        "quadratic_trend_forecast" => {
            let (b0, b1, b2) = (s.out("b0")?, s.out("b1")?, s.out("b2")?);
            (ts.iter().map(|t| (*t, b0 + b1 * t + b2 * t * t)).collect(),
             "fitted quadratic".to_string(),
             format!(" The returned quadratic trend b₀ = {}, b₁ = {}, b₂ = {} (t = 1 ... n), drawn dashed from those \
                      coefficients.", num(b0), num(b1), num(b2)))
        }
        _ => (vec![], String::new(), String::new()),
    };
    let (xs, ys) = xs_ys(&[&hist, &path, &fit]);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.vline(nn + 0.5, "forecast");
    if !fit.is_empty() {
        p.dashed(&fit, 2, &fit_name);
    }
    p.line_dots(&hist, 1, "history");
    p.line_dots(&path, 0, "forecast");
    p.mark(nn + h, fh, &num(fh));
    let what = match s.key {
        "ar_forecast" => format!("An AR model with coefficients φ = {}{}.", lst(&s.vec("phi")?),
                                 n(s, "c").map(|c| format!(" and constant c = {}", num(c))).unwrap_or_default()),
        "arima_p10_forecast" => format!("ARIMA(1,1,0) with AR coefficient φ = {} on the first differences.",
                                        lst(&s.vec("phi")?)),
        "holt_winters_multiplicative" => format!("Holt-Winters multiplicative forecasts (m = {}, α = {}, β = {}, γ = {}): \
                                                  the seasonal swing grows with the level.", num(n(s, "m")?),
                                                 num(n(s, "alpha")?), num(n(s, "beta")?), num(n(s, "gamma")?)),
        _ => String::new(),
    };
    let body = p.finish("t, period", "x", "");
    Some(s.chart("forecast", title,
                 &format!("{what}{fit_t} History x = {} and the forecasts the model returns for h = 1 to {hmax} periods \
                           ahead, each a run of the model (other inputs as given); marked: h = {h}, forecast = {}.",
                          lst(&x), num(fh)).trim_start().to_string(), &body))
}

pub fn holt_pair(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let nn = x.len() as f64;
    let (alpha, beta) = (n(s, "alpha")?, n(s, "beta")?);
    let damped = s.key == "holt_damped_trend";
    let own = forecasts(8, nn, |hh| s.with_input(&[("h", ii(hh))]));
    let oth = if damped {
        forecasts(8, nn, |hh| other(s, "holt_linear_trend", &[("h", ii(hh))]))
    } else {
        forecasts(8, nn, |hh| other(s, "holt_damped_trend", &[("phi", fl(0.9)), ("h", ii(hh))]))
    };
    if own.is_empty() {
        return None;
    }
    let hist = idx(&x);
    let start = (nn, x[x.len() - 1]);
    let mut a = vec![start];
    a.extend(own.iter().copied());
    let mut b = vec![start];
    b.extend(oth.iter().copied());
    let (xs, ys) = xs_ys(&[&hist, &a, &b]);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.vline(nn + 0.5, "forecast");
    p.line_dots(&hist, 1, "history");
    let (own_n, oth_n) = if damped { ("damped trend", "undamped linear trend") } else { ("linear trend", "damped trend, φ = 0.9") };
    if oth.len() >= 2 {
        p.dashed(&b, 3, oth_n);
    }
    p.line_dots(&a, 0, own_n);
    let last = own[own.len() - 1];
    p.mark(last.0, last.1, &num(last.1));
    let body = p.finish("t, period", "x", "");
    let phi_t = if damped { format!(", damping φ = {}", num(n(s, "phi")?)) } else { String::new() };
    let oth_t = oth.last().map(|q| format!(" (the other method gives {})", num(q.1))).unwrap_or_default();
    Some(s.chart("forecast", if damped { "A damped trend flattens the forecast" } else {
                     "Holt's linear trend against a damped trend"
                 },
                 &format!("History x = {} and forecasts h = 1 to 8 from Holt's method with α = {}, β = {}{phi_t} ({own_n}), \
                           beside the {oth_n} with the same α and β, every point a run of the model; marked: h = 8, \
                           forecast = {}{oth_t}.", lst(&x), num(alpha), num(beta), num(last.1)), &body))
}

pub fn hw_three(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let nn = x.len() as f64;
    let hist = idx(&x);
    let start = (nn, x[x.len() - 1]);
    let runs: Vec<(&str, Vec<(f64, f64)>)> = vec![
        ("HW additive", forecasts(8, nn, |hh| s.with_input(&[("h", ii(hh))]))),
        ("HW multiplicative", forecasts(8, nn, |hh| other(s, "holt_winters_multiplicative", &[("h", ii(hh))]))),
        ("trend + season", forecasts(8, nn, |hh| other(s, "trend_seasonal_regression", &[("h", ii(hh))]))),
    ];
    if runs[0].1.is_empty() {
        return None;
    }
    let paths: Vec<(&str, Vec<(f64, f64)>)> = runs.iter().filter(|r| !r.1.is_empty())
        .map(|(nm, f)| (*nm, std::iter::once(start).chain(f.iter().copied()).collect())).collect();
    let mut refs: Vec<&[(f64, f64)]> = paths.iter().map(|v| v.1.as_slice()).collect();
    refs.push(&hist);
    let (xs, ys) = xs_ys(&refs);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.vline(nn + 0.5, "forecast");
    p.line_dots(&hist, 3, "history");
    for (i, (nm, pts)) in paths.iter().enumerate() {
        p.line_dots(pts, i, nm);
    }
    let body = p.finish("t, period", "x", "");
    let ends: Vec<String> = paths.iter().map(|(nm, pts)| format!("{nm} {}", num(pts[pts.len() - 1].1))).collect();
    Some(s.chart("forecast", "Seasonal series and three forecasts",
                 &format!("A seasonal series of {} values ({}) and three forecasts for the next 8 periods, each step \
                           computed by the model: Holt-Winters additive and multiplicative (m = {}, α = {}, β = {}, γ = {}) \
                           and a linear trend with seasonal effects; 8-step forecasts: {}.", x.len(), lst(&x),
                          num(n(s, "m")?), num(n(s, "alpha")?), num(n(s, "beta")?), num(n(s, "gamma")?), ends.join(", ")),
                 &body))
}

pub fn hw_alpha(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let nn = x.len() as f64;
    let alpha = n(s, "alpha")?;
    let hist = idx(&x);
    let start = (nn, x[x.len() - 1]);
    let mut series = vec![];
    for a in dedup(vec![0.2, alpha, 0.8]) {
        let f = forecasts(8, nn, |hh| s.with_input(&[("alpha", fl(a)), ("h", ii(hh))]));
        if !f.is_empty() {
            series.push((a, std::iter::once(start).chain(f.into_iter()).collect::<Vec<_>>()));
        }
    }
    if series.is_empty() {
        return None;
    }
    let mut refs: Vec<&[(f64, f64)]> = series.iter().map(|v| v.1.as_slice()).collect();
    refs.push(&hist);
    let (xs, ys) = xs_ys(&refs);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.vline(nn + 0.5, "forecast");
    p.line_dots(&hist, 3, "history");
    let mut c = 1;
    let mut ends = vec![];
    for (a, pts) in &series {
        let ex = (a - alpha).abs() < 1e-12;
        let cc = if ex { 0 } else { c += 1; c - 1 };
        p.line_dots(pts, cc, &format!("α = {}{}", num(*a), if ex { " (example)" } else { "" }));
        ends.push(format!("α = {}: {}", num(*a), num(pts[pts.len() - 1].1)));
    }
    let body = p.finish("t, period", "x", "");
    Some(s.chart("forecast-2", "Holt-Winters additive: how α moves the forecast",
                 &format!("Holt-Winters additive forecasts (m = {}, β = {}, γ = {}) of the series x = {} for h = 1 to 8, at \
                           {} level-smoothing constants α (each point a run of the model); 8-step forecasts {}.",
                          num(n(s, "m")?), num(n(s, "beta")?), num(n(s, "gamma")?), lst(&x), series.len(), ends.join(", ")),
                 &body))
}

pub fn holt_update(s: &Solved) -> Option<Figure> {
    let (l, b, y) = (n(s, "l")?, n(s, "b")?, n(s, "y")?);
    let (lv, tr) = (s.out("level")?, s.out("trend")?);
    let f = forecasts(4, 1.0, |hh| s.with_input(&[("h", ii(hh))]));
    let before = vec![(0.0, l), (1.0, l + b)];
    let mut after = vec![(1.0, lv)];
    after.extend(f.iter().copied());
    let (xs, ys) = xs_ys(&[&before, &after, &[(1.0, y)]]);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.dashed(&before, 3, "before the update");
    p.line_dots(&after, 0, "after, with forecasts");
    p.dots(&[(1.0, y)], 1, &format!("observation y = {}", num(y)));
    p.mark(1.0, lv, &format!("level {}", num(lv)));
    let body = p.finish("t, period (update at t = 1)", "level", &format!("trend {} to {}", num(b), num(tr)));
    Some(s.chart("update", "Holt's update: level and trend after one observation",
                 &format!("One step of Holt's method with α = {}, β = {}: before the update the line level {} + trend {} \
                           predicts {} for t = 1 (dashed, from the stated inputs); the observation y = {} moves it to the \
                           returned level {} and trend {}, and the forecasts h = 1 to {} (runs of the model) continue from \
                           there{}.", num(n(s, "alpha")?), num(n(s, "beta")?), num(l), num(b), num(l + b), num(y), num(lv),
                          num(tr), f.len(), f.first().map(|q| format!(" ({} at h = 1)", num(q.1))).unwrap_or_default()),
                 &body))
}

pub fn ses_update(s: &Solved) -> Option<Figure> {
    let (ff, aa, alpha) = (n(s, "F")?, n(s, "A")?, n(s, "alpha")?);
    let mut al = k::span(0.05, 1.0, 20, false);
    al.push(alpha);
    let al = dedup(al);
    let pts = s.curve("alpha", &al, "forecast", &[]);
    if pts.len() < 3 {
        return None;
    }
    let fc = s.out("forecast")?;
    let (xs, mut ys) = xs_ys(&[&pts]);
    ys.extend([ff, aa]);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.hline(ff, &format!("previous forecast F = {}", num(ff)));
    p.hline(aa, &format!("actual A = {}", num(aa)));
    p.line(&pts, 0, "new forecast F + α(A - F)");
    p.mark(alpha, fc, &format!("α = {}: {}", num(alpha), num(fc)));
    let body = p.finish("α, smoothing constant", "new forecast", "");
    Some(s.chart("update", "An exponential-smoothing update between forecast and actual",
                 &format!("One exponential-smoothing update from the previous forecast F = {} after observing A = {}: the \
                           new forecast for α = {} to {} (each a run of the model) moves along the error {} from F to A; \
                           marked: α = {}, new forecast {}.", num(ff), num(aa), num(pts[0].0), num(pts[pts.len() - 1].0),
                          num(s.out("error").unwrap_or(aa - ff)), num(alpha), num(fc)), &body))
}

pub fn ses_forecasts(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let alpha = n(s, "alpha")?;
    if x.len() > 40 || x.len() < 2 {
        return None;
    }
    let hist = idx(&x);
    let mut series = vec![];
    for a in dedup(vec![0.1, alpha, 0.8]) {
        let pts: Vec<(f64, f64)> = (1..=x.len()).filter_map(|t| {
            s.with_input(&[("x", vj(&x[..t])), ("alpha", fl(a))]).and_then(|r| o(&r, "forecast")).map(|f| (t as f64 + 1.0, f))
        }).collect();
        if pts.len() >= 2 {
            series.push((a, pts));
        }
    }
    if series.is_empty() {
        return None;
    }
    let mut refs: Vec<&[(f64, f64)]> = series.iter().map(|v| v.1.as_slice()).collect();
    refs.push(&hist);
    let (xs, ys) = xs_ys(&refs);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.line_dots(&hist, 3, "data");
    let mut c = 1;
    let mut parts = vec![];
    for (a, pts) in &series {
        let ex = (a - alpha).abs() < 1e-12;
        let cc = if ex { 0 } else { c += 1; c - 1 };
        p.line_dots(pts, cc, &format!("α = {}{}", num(*a), if ex { " (example)" } else { "" }));
        parts.push(format!("α = {}: next {}", num(*a), num(pts[pts.len() - 1].1)));
    }
    let body = p.finish("t, period", "x", "");
    Some(s.chart("forecasts", "One-step forecasts of simple exponential smoothing",
                 &format!("Simple exponential smoothing of x = {}: after each observation t the model's forecast for period \
                           t + 1 (the run on the first t values), for {} smoothing constants α; {}. A larger α follows the \
                           data more closely.", lst(&x), series.len(), parts.join("; ")), &body))
}

pub fn ses_sse(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let cands = s.vec("alphas")?;
    let (best, sse) = (s.out("alpha")?, s.out("sse")?);
    let one = |a: f64| s.with_input(&[("alphas", vj(&[a]))]).and_then(|r| o(&r, "sse")).map(|v| (a, v));
    let curve: Vec<(f64, f64)> = k::span(0.05, 0.95, 19, false).into_iter().filter_map(one).collect();
    let dots: Vec<(f64, f64)> = cands.iter().take(40).filter_map(|a| one(*a)).collect();
    if curve.len() < 3 {
        return None;
    }
    let (xs, ys) = xs_ys(&[&curve, &dots]);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.line(&curve, 3, &format!("SSE over α = {} to {}", num(curve[0].0), num(curve[curve.len() - 1].0)));
    p.dots(&dots, 0, "candidates");
    p.mark(best, sse, &format!("best α = {}", num(best)));
    let body = p.finish("α, smoothing constant", "SSE of one-step forecasts", "");
    Some(s.chart("sse", "Choosing the smoothing constant by least squares",
                 &format!("The sum of squared one-step errors of simple exponential smoothing on x = {} as α varies (each \
                           point the model's run with a single candidate); among the candidates {} the model chooses α = {} \
                           with SSE = {} and next forecast {}.", lst(&x), lst(&cands), num(best), num(sse),
                          num(s.out("forecast").unwrap_or(f64::NAN))), &body))
}

pub fn trend_season_fit(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let nn = x.len();
    let m = n(s, "m")? as usize;
    let (b0, b1) = (s.out("intercept")?, s.out("slope")?);
    let eff = s.out_list("season_effects")?;
    let f = forecasts(8, nn as f64, |hh| s.with_input(&[("h", ii(hh))]));
    if f.is_empty() || eff.len() != m || m == 0 {
        return None;
    }
    // the fitted values from the returned coefficients, drawn only when they reproduce the model's own forecast
    let fit_at = |t: usize| b0 + b1 * t as f64 + eff[(t - 1) % m];
    let ok = (fit_at(nn + 1) - f[0].1).abs() <= 1e-6 * (1.0 + f[0].1.abs());
    let fit: Vec<(f64, f64)> = if ok { (1..=nn).map(|t| (t as f64, fit_at(t))).collect() } else { vec![] };
    let hist = idx(&x);
    let mut path = vec![(nn as f64, x[nn - 1])];
    path.extend(f.iter().copied());
    let (xs, ys) = xs_ys(&[&hist, &path, &fit]);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.vline(nn as f64 + 0.5, "forecast");
    p.line_dots(&hist, 1, "history");
    if ok {
        p.dashed(&fit, 2, "fitted trend + season");
    }
    p.line_dots(&path, 0, "forecast");
    let body = p.finish("t, period", "x", "");
    Some(s.chart("fit", "Trend-plus-season regression and its forecast",
                 &format!("A linear trend with seasonal effects fitted to x = {} (m = {m}): returned intercept {}, slope {} \
                           per period and season effects {}; {}the forecasts h = 1 to {} are runs of the model ({} at h = \
                           1).", lst(&x), num(b0), num(b1), lst(&eff),
                          if ok { "the fitted values are drawn from these coefficients (t = 1 ... n) and " } else { "" },
                          f.len(), num(f[0].1)), &body))
}

// ------------------------------------------------------------------------------------------------ smoothers
pub fn smooth(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let kk = n(s, "k")? as usize;
    let hist = idx(&x);
    let at_end = |v: &[f64], first: usize| -> Vec<(f64, f64)> {
        v.iter().enumerate().map(|(i, y)| ((first + i) as f64, *y)).collect()
    };
    let (title, cap, lines): (&str, String, Vec<(String, Vec<(f64, f64)>, bool)>) = match s.key {
        "centred_moving_average" => {
            let v = s.out_list("cma")?;
            let first = s.out("first_index")? as usize;
            ("Centred moving average over the series",
             format!("The series x = {} and its centred moving average of order k = {kk}, as returned by the model: {} \
                      values {}, the first centred on period {first} (the ends, where the window does not fit, have none); \
                      marked: the first value {}.", lst(&x), v.len(), lst(&v), num(v[0])),
             vec![(format!("centred moving average, k = {kk}"), at_end(&v, first), false)])
        }
        "simple_moving_average" => {
            let v = s.out_list("sma")?;
            let e = other(s, "exponential_moving_average", &[]).and_then(|a| ol(&a, "ema"));
            let mut l = vec![(format!("simple moving average, k = {kk}"), at_end(&v, x.len() + 1 - v.len()), false)];
            if let Some(e) = &e {
                l.push(("exponential, same span".into(), idx(e), true));
            }
            ("Simple moving average trailing the data",
             format!("The series x = {} with its simple moving average over k = {kk} periods (each value placed at the end \
                      of its window: {}){}, returned by the model.", lst(&x), lst(&v),
                     if e.is_some() { " and, for comparison, the exponential moving average with the same span (dashed)" }
                     else { "" }), l)
        }
        "linearly_weighted_moving_average" => {
            let v = s.out_list("lwma")?;
            let e = other(s, "simple_moving_average", &[]).and_then(|a| ol(&a, "sma"));
            let mut l = vec![("linearly weighted".to_string(), at_end(&v, x.len() + 1 - v.len()), false)];
            if let Some(e) = &e {
                l.push(("simple (equal weights)".into(), at_end(e, x.len() + 1 - e.len()), true));
            }
            ("Weighted against simple moving average",
             format!("The series x = {} with its linearly weighted moving average over k = {kk} periods (weights 1 to \
                      {kk}, newest heaviest: {}){}, each placed at the end of its window; all from the model.", lst(&x),
                     lst(&v), e.as_ref().map(|e| format!(" and the equally weighted simple moving average (dashed: {})",
                                                         lst(e))).unwrap_or_default()), l)
        }
        _ => {
            let mut ks: Vec<usize> = vec![2, kk, (2 * kk).max(5)];
            ks.sort();
            ks.dedup();
            let mut l = vec![];
            let mut parts = vec![];
            for kv in ks.into_iter().filter(|kv| *kv >= 1) {
                if let Some(a) = s.with_input(&[("k", ii(kv as i64))]) {
                    if let (Some(v), Some(al)) = (ol(&a, "ema"), o(&a, "alpha")) {
                        parts.push(format!("k = {kv}: α = {}, last {}", num(al), num(v[v.len() - 1])));
                        l.push((format!("k = {kv} (α = {})", num(al)), idx(&v), false));
                    }
                }
            }
            ("Exponential moving averages of three spans",
             format!("The series x = {} with exponential moving averages of {} spans k, each returned by the model: {}. A \
                      longer span means a smaller α and a smoother, slower average.", lst(&x), l.len(), parts.join("; ")),
             l)
        }
    };
    if lines.is_empty() || lines[0].1.is_empty() {
        return None;
    }
    let mut refs: Vec<&[(f64, f64)]> = lines.iter().map(|v| v.1.as_slice()).collect();
    refs.push(&hist);
    let (xs, ys) = xs_ys(&refs);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.line_dots(&hist, 3, "data");
    for (i, (nm, pts, dashed)) in lines.iter().enumerate() {
        if *dashed {
            p.dashed(pts, 2, nm);
        } else {
            p.line_dots(pts, if i == 0 { 0 } else { i }, nm);
        }
    }
    if s.key == "centred_moving_average" {
        let q = lines[0].1[0];
        p.mark(q.0, q.1, &num(q.1));
    }
    let body = p.finish("t, period", "x", "");
    Some(s.chart("smooth", title, &cap, &body))
}

pub fn season_bars(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let v = s.out_list("indices")?;
    let mult = s.key.ends_with("multiplicative");
    let labels: Vec<String> = (1..=v.len()).map(|i| i.to_string()).collect();
    let body = k::bars(&labels, &v, &[], "season", "seasonal index",
                       "");
    Some(s.chart("bars", if mult { "Multiplicative seasonal indices" } else { "Additive seasonal indices" },
                 &format!("The {} seasonal indices of x = {} for its m = {} seasons, as returned by the model: {} (relative \
                           to {}).", if mult { "multiplicative" } else { "additive" }, lst(&x), v.len(), lst(&v),
                          if mult { 1 } else { 0 }), &body))
}

pub fn adjusted(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let v = s.out_list("adjusted")?;
    let mult = s.key.ends_with("multiplicative");
    let (a, b) = (idx(&x), idx(&v));
    let (xs, ys) = xs_ys(&[&a, &b]);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.line_dots(&a, 3, "original series");
    p.line_dots(&b, 0, &format!("adjusted ({})", if mult { "multiplicative" } else { "additive" }));
    let last = b[b.len() - 1];
    p.mark(last.0, last.1, &num(last.1));
    let body = p.finish("t, period", "x", "");
    let w = if mult { "multiplicative" } else { "additive" };
    Some(s.chart("adjusted", &format!("Seasonally adjusted series ({w})"),
                 &format!("The series x = {} and its {w} seasonally adjusted values (m = {}), as returned by the model: the \
                           seasonal swing is removed and the trend remains; marked: last adjusted value {}.", lst(&x),
                          num(n(s, "m")?), num(last.1)), &body))
}

pub fn growth(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let r = s.out("rate")?;
    let nn = x.len() as f64;
    let hist = idx(&x);
    let path: Vec<(f64, f64)> = k::span(1.0, nn, 41, false).into_iter().map(|t| (t, x[0] * (1.0 + r).powf(t - 1.0)))
        .collect();
    let (xs, ys) = xs_ys(&[&hist, &path]);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.dashed(&path, 0, &format!("growth {} % per period", num(100.0 * r)));
    p.dots(&hist, 3, "data");
    let tg = s.out("total_growth").unwrap_or(f64::NAN);
    let body = p.finish("t, period", "x", &format!("total growth {} %", num(100.0 * tg)));
    Some(s.chart("growth", "Compound average growth between the first and last value",
                 &format!("The series x = {} and the constant-growth path x₁(1 + r)^(t-1) with the returned compound average \
                           growth rate r = {} ({} % per period over {} periods), drawn from that value: it starts and ends on \
                           the data; total growth {}.", lst(&x), num(r), num(100.0 * r), num(nn - 1.0), num(tg)), &body))
}

pub fn differences(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let d = n(s, "d").unwrap_or(1.0) as i64;
    let mut orders = vec![1, d.max(2)];
    orders.dedup();
    let mut sets = vec![];
    for o_ in orders {
        if let Some(v) = s.with_input(&[("d", ii(o_))]).and_then(|a| ol(&a, "differenced")) {
            let off = x.len() - v.len();
            sets.push((o_, v.iter().enumerate().map(|(i, y)| ((off + i + 1) as f64, *y)).collect::<Vec<_>>(), v));
        }
    }
    if sets.is_empty() {
        return None;
    }
    let hist = idx(&x);
    let (xs, ys) = xs_ys(&[&hist]);
    let mut a = Plot::new(&xs, &ys, false, false)?;
    a.line_dots(&hist, 3, "series");
    let a = a.finish("t, period", "x", "");
    let refs: Vec<&[(f64, f64)]> = sets.iter().map(|v| v.1.as_slice()).collect();
    let (xs2, ys2) = xs_ys(&refs);
    let mut b = Plot::new(&xs2, &ys2, true, false)?;
    b.hline(0.0, "");
    for (i, (o_, pts, _)) in sets.iter().enumerate() {
        b.line_dots(pts, i, &format!("order d = {o_}"));
    }
    let b = b.finish("t, period", "difference", "");
    let parts: Vec<String> = sets.iter().map(|(o_, _, v)| format!("d = {o_}: {}", lst(v))).collect();
    Some(two(s, "differences", "Differencing removes the trend",
             &format!("The series x = {} (top) and its differences (bottom) as returned by the model: {}. Each difference is \
                       placed at the later of its periods; constant second differences show a quadratic trend.", lst(&x),
                      parts.join("; ")), a, b))
}

pub fn interpolation(s: &Solved) -> Option<Figure> {
    let (t, y) = (s.vec("t")?, s.vec("y")?);
    let t0 = n(s, "t0")?;
    let v = s.out("value")?;
    let lo = t.iter().cloned().fold(f64::INFINITY, f64::min).min(t0);
    let hi = t.iter().cloned().fold(f64::NEG_INFINITY, f64::max).max(t0);
    let xs = k::span(lo, hi, 41, false);
    let own = s.curve("t0", &xs, "value", &[]);
    let expo = s.key == "exponential_interpolation";
    let okey = if expo { "linear_interpolation" } else { "exponential_interpolation" };
    let oth: Vec<(f64, f64)> = xs.iter().filter_map(|x| other(s, okey, &[("t0", fl(*x))]).and_then(|a| o(&a, "value"))
        .map(|w| (*x, w))).collect();
    if own.len() < 3 {
        return None;
    }
    let pts: Vec<(f64, f64)> = t.iter().copied().zip(y.iter().copied()).collect();
    let (xs2, ys2) = xs_ys(&[&own, &oth, &pts]);
    let mut p = Plot::new(&xs2, &ys2, false, false)?;
    let (own_n, oth_n) = if expo { ("exponential interpolation", "linear interpolation") } else {
        ("linear interpolation", "exponential interpolation")
    };
    if oth.len() >= 2 {
        p.dashed(&oth, 3, oth_n);
    }
    p.line(&own, 0, own_n);
    p.dots(&pts, 1, "given points");
    p.mark(t0, v, &format!("t₀ = {}: {}", num(t0), num(v)));
    let body = p.finish("t", "y", "");
    Some(s.chart("interpolation", &format!("{} between data points", if expo { "Exponential interpolation" } else {
                     "Linear interpolation"
                 }),
                 &format!("{} between the given points t = {}, y = {}: the model's value at {} times t₀ across the range{}; \
                           marked: t₀ = {}, value {}.", if expo { "Exponential interpolation" } else { "Linear interpolation" },
                          lst(&t), lst(&y), own.len(),
                          if oth.len() >= 2 { format!(", with {oth_n} dashed for comparison") } else { String::new() },
                          num(t0), num(v)), &body))
}

pub fn error_fan(s: &Solved) -> Option<Figure> {
    let h = n(s, "h")?;
    let hmax = (h as i64).max(10).min(40);
    let sd: Vec<(f64, f64)> = (1..=hmax).filter_map(|hh| s.with_input(&[("h", ii(hh))]).and_then(|a| o(&a, "sd"))
        .map(|v| (hh as f64, v))).collect();
    if sd.len() < 2 {
        return None;
    }
    let up: Vec<(f64, f64)> = sd.iter().map(|q| (q.0, 1.96 * q.1)).collect();
    let dn: Vec<(f64, f64)> = sd.iter().map(|q| (q.0, -1.96 * q.1)).collect();
    let (xs, ys) = xs_ys(&[&up, &dn]);
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.band(&dn, &up, 3, "±1.96 × forecast error sd");
    p.hline(0.0, "");
    p.line_dots(&sd, 0, "sd");
    let v = s.out("sd")?;
    p.mark(h, v, &format!("sd = {}", num(v)));
    let body = p.finish("h, steps ahead", "forecast error", "");
    Some(s.chart("fan", "Forecast uncertainty grows with the horizon",
                 &format!("The standard deviation of the h-step forecast error of the ARMA model φ = ({}), θ = ({}), σ² = {} \
                           for h = 1 to {hmax} (runs of the model) and the 95 % band ±1.96·sd computed from it; it grows \
                           towards the process standard deviation. Marked: h = {h}, sd = {} (variance {}).",
                          lst(&s.vec("phi")?), lst(&s.vec("theta")?), num(n(s, "sigma2")?), num(v),
                          num(s.out("variance").unwrap_or(f64::NAN))), &body))
}
