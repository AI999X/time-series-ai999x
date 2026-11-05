//! Time Series' own figures: forecast errors, unit-root and portmanteau tests over their settings, change points and
//! cusums, trend tests, Kalman filters and smoothers (see figures.rs).

use crate::figures::{fl, idx, ii, lst, n, o, ol, other, two, vj};
use crate::figures_plot::{xs_ys, Plot};
use ax_engine::figkit::{self as k, num, Solved};
use ax_engine::figures::Figure;

// ------------------------------------------------------------------------------------------------ forecast errors
fn error_lines(p: &mut Plot, y: &[(f64, f64)], f: &[(f64, f64)]) {
    for (a, b) in y.iter().zip(f.iter()) {
        p.line(&[*a, *b], 1, "");
        let e = b.1 - a.1;
        if e != 0.0 {
            p.note_right(a.0, (a.1 + b.1) / 2.0, &num(e));
        }
    }
}

pub fn errors(s: &Solved) -> Option<Figure> {
    let (y, f) = (s.vec("y")?, s.vec("f")?);
    if y.len() != f.len() || y.len() > 60 {
        return None;
    }
    let (sym, name, unit) = match s.key {
        "mean_absolute_error" => ("mae", "MAE", ""),
        "mean_absolute_percentage_error" => ("mape", "MAPE", " %"),
        "mean_forecast_error" => ("mean_error", "mean forecast error", ""),
        "mean_squared_error" => ("mse", "MSE", ""),
        "root_mean_squared_error" => ("rmse", "RMSE", ""),
        _ => ("smape", "sMAPE", " %"),
    };
    let v = s.out(sym)?;
    let (a, b) = (idx(&y), idx(&f));
    let (xs, ys) = xs_ys(&[&a, &b]);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.line(&[(f64::NAN, f64::NAN)], 1, "error f - y");
    error_lines(&mut p, &a, &b);
    p.line_dots(&a, 3, "actual y");
    p.line_dots(&b, 0, "forecast f");
    let body = p.finish("t, period", "value", &format!("{name} = {}{unit}", num(v)));
    Some(s.chart("errors", &format!("Forecast errors behind the {name}"),
                 &format!("Actual values y = {} and forecasts f = {}, with each error f - y drawn between them (differences of \
                           the stated inputs); the model returns {name} = {}{unit} over n = {} periods.", lst(&y), lst(&f),
                          num(v), y.len()), &body))
}

pub fn mase(s: &Solved) -> Option<Figure> {
    let (tr, y, f) = (s.vec("y_train")?, s.vec("y")?, s.vec("f")?);
    if y.len() != f.len() {
        return None;
    }
    let n1 = tr.len() as f64;
    let train = idx(&tr);
    let shiftv = |v: &[f64]| -> Vec<(f64, f64)> { v.iter().enumerate().map(|(i, x)| (n1 + 1.0 + i as f64, *x)).collect() };
    let (a, b) = (shiftv(&y), shiftv(&f));
    let (xs, ys) = xs_ys(&[&train, &a, &b]);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.vline(n1 + 0.5, "test period");
    error_lines(&mut p, &a, &b);
    p.line_dots(&train, 3, "training data");
    p.line_dots(&a, 2, "actual (test)");
    p.line_dots(&b, 0, "forecast");
    let (m, mae, sc) = (s.out("mase")?, s.out("mae")?, s.out("scale")?);
    let body = p.finish("t, period", "value", &format!("MASE = {}", num(m)));
    Some(s.chart("errors", "Scaled error: forecast against the naive benchmark",
                 &format!("Training data {}, test values y = {} and forecasts f = {}, errors drawn between them: the model \
                           scales the forecast MAE {} by the in-sample naive error {}, giving MASE = {} ({} 1: {} than the \
                           naive forecast).", lst(&tr), lst(&y), lst(&f), num(mae), num(sc), num(m),
                          if m < 1.0 { "below" } else { "not below" }, if m < 1.0 { "better" } else { "no better" }), &body))
}

// ------------------------------------------------------------------------------------------------ tests
pub fn stat_by_setting(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let (stat, title) = match s.key {
        "augmented_dickey_fuller_test" => ("tau", "Augmented Dickey-Fuller statistic by lag order"),
        "dickey_fuller_test" => ("tau", "Dickey-Fuller statistic by deterministic terms"),
        _ => ("kpss", "KPSS statistic by number of lags"),
    };
    let mut pts: Vec<(f64, f64)> = vec![];
    let mut labels: Vec<String> = vec![];
    let (mine, xlab): (f64, &str);
    if s.key == "dickey_fuller_test" {
        let reg = s.txt("regression").unwrap_or_else(|| "c".into());
        let mut at = 0.0;
        for (i, r) in ["n", "c", "ct"].iter().enumerate() {
            if let Some(v) = s.with_input(&[("regression", ax_engine::Input::Text(r.to_string()))]).and_then(|a| o(&a, stat)) {
                pts.push((i as f64 + 1.0, v));
            } else {
                pts.push((i as f64 + 1.0, f64::NAN));
            }
            labels.push(r.to_string());
            if *r == reg {
                at = i as f64 + 1.0;
            }
        }
        mine = at;
        xlab = "deterministic terms";
    } else {
        let sym = if s.key == "kpss_test" { "nlags" } else { "p" };
        let given = n(s, sym)? as i64;
        for lag in 0..=given.max(if s.key == "kpss_test" { 4 } else { 3 }).min(12) {
            if let Some(v) = s.with_input(&[(sym, ii(lag))]).and_then(|a| o(&a, stat)) {
                pts.push((lag as f64, v));
            }
        }
        mine = given as f64;
        xlab = if s.key == "kpss_test" { "number of lags" } else { "lag order p" };
    }
    let good: Vec<(f64, f64)> = pts.iter().copied().filter(|q| q.1.is_finite()).collect();
    if good.is_empty() {
        return None;
    }
    let hist = idx(&x);
    let (xs, ys) = xs_ys(&[&hist]);
    let mut a = Plot::new(&xs, &ys, false, false)?;
    a.line_dots(&hist, 3, "series");
    let a = a.finish("t, period", "x", "");
    let (mut xs2, ys2) = xs_ys(&[&good]);
    if !labels.is_empty() {
        xs2.extend([1.0, labels.len() as f64]);
    }
    let mut b = Plot::new(&xs2, &ys2, true, false)?;
    if !labels.is_empty() {
        b = b.cats(&labels);
    }
    b.hline(0.0, "");
    if labels.is_empty() {
        b.line_dots(&good, 0, "");
    } else {
        b.stems(&good, 0, "");
    }
    for q in &good {
        let lab = if labels.is_empty() { format!("{}: {}", q.0, num(q.1)) } else { num(q.1) };
        b.note(q.0, q.1, if q.1 >= 0.0 { -10.0 } else { 18.0 }, &lab);
    }
    let v = s.out(stat)?;
    b.mark(mine, v, "");
    let b = b.finish(xlab, &format!("{stat} statistic"), "");
    let parts: Vec<String> = pts.iter().enumerate().filter(|q| q.1 .1.is_finite())
        .map(|(i, q)| format!("{}: {}", labels.get(i).cloned().unwrap_or_else(|| num(q.0)), num(q.1))).collect();
    let rest: Vec<String> = s.a.values.iter().filter_map(|(kk, val, _)| val.as_f64().map(|x| format!("{kk} = {}", num(x))))
        .collect();
    Some(two(s, "series", title,
             &format!("The series x = {} (top) and the {stat} statistic the model returns for each {} (bottom; other inputs \
                       as given): {}. For these inputs: {}.", lst(&x), xlab, parts.join(", "), rest.join(", ")), a, b))
}

pub fn q_by_lag(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let h = n(s, "h")? as i64;
    let lb = s.key == "ljung_box_test";
    let okey = if lb { "box_pierce_test" } else { "ljung_box_test" };
    let (own_n, oth_n) = if lb { ("Ljung-Box", "Box-Pierce") } else { ("Box-Pierce", "Ljung-Box") };
    let top = h.max(8).min(((x.len() as i64) - 1).max(h));
    let mut q = vec![];
    let mut pv = vec![];
    let mut q2 = vec![];
    for hh in 1..=top {
        if let Some(a) = s.with_input(&[("h", ii(hh))]) {
            if let (Some(v), Some(pp)) = (o(&a, "q"), o(&a, "p_value")) {
                q.push((hh as f64, v));
                pv.push((hh as f64, pp));
            }
        }
        if let Some(v) = other(s, okey, &[("h", ii(hh))]).and_then(|a| o(&a, "q")) {
            q2.push((hh as f64, v));
        }
    }
    if q.len() < 2 {
        return None;
    }
    let (xs, ys) = xs_ys(&[&q, &q2]);
    let mut a = Plot::new(&xs, &ys, true, false)?;
    a.line_dots(&q, 0, own_n);
    if q2.len() >= 2 {
        a.line_dots(&q2, 3, oth_n);
    }
    let qq = s.out("q")?;
    a.mark(h as f64, qq, &format!("Q = {}", num(qq)));
    let a = a.finish("h, number of lags", "Q statistic", "");
    let (xs, mut ys) = xs_ys(&[&pv]);
    ys.extend([0.0, 1.0]);
    let mut b = Plot::new(&xs, &ys, false, false)?;
    b.hline(0.05, "0.05");
    b.line_dots(&pv, 0, &format!("{own_n} p-value"));
    let pp = s.out("p_value")?;
    b.mark(h as f64, pp, &num(pp));
    let b = b.finish("h, number of lags", "p-value", "");
    Some(two(s, "q-by-lag", &format!("{own_n} test over the number of lags"),
             &format!("Portmanteau tests for autocorrelation in x = {} using h = 1 to {top} lags, every value a run of the \
                       model: the {own_n} and {oth_n} statistics (top) and the {own_n} p-values with the 0.05 level \
                       (bottom). For h = {h}: Q = {}, {} degrees of freedom, p-value {}.", lst(&x), num(qq),
                      num(s.out("df").unwrap_or(f64::NAN)), num(pp)), a, b))
}

// ------------------------------------------------------------------------------------------------ change points
/// The segment means as a step line over periods 1 ... n split after the change indices.
fn steps(cuts: &[usize], means: &[f64], nn: usize) -> Vec<(f64, f64)> {
    let mut b = vec![0usize];
    b.extend(cuts.iter().copied());
    b.push(nn);
    let mut out = vec![];
    for (i, m) in means.iter().enumerate() {
        if i + 1 >= b.len() {
            break;
        }
        out.push((b[i] as f64 + 0.5, *m));
        out.push((b[i + 1] as f64 + 0.5, *m));
        out.push((f64::NAN, f64::NAN));
    }
    out
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn seg_plot(x: &[f64], lines: &[(String, Vec<(f64, f64)>)], cuts: &[usize]) -> Option<Plot> {
    let hist = idx(x);
    let mut refs: Vec<&[(f64, f64)]> = lines.iter().map(|v| v.1.as_slice()).collect();
    refs.push(&hist);
    let (xs, ys) = xs_ys(&refs);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    for c in cuts {
        p.vline(*c as f64 + 0.5, "");
    }
    p.dots(&hist, 3, "data");
    for (i, (nm, pts)) in lines.iter().enumerate() {
        p.line(pts, i, nm);
    }
    Some(p)
}

pub fn one_change(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let c = s.out("change_index")? as usize;
    let (mb, ma) = (s.out("mean_before")?, s.out("mean_after")?);
    if c == 0 || c >= x.len() {
        return None;
    }
    let st = steps(&[c], &[mb, ma], x.len());
    let mut p = seg_plot(&x, &[("returned means".into(), st)], &[c])?;
    p.vline(c as f64 + 0.5, &format!("change after t = {c}"));
    let body = p.finish("t, index", "x", "");
    Some(s.chart("segments", "A single change in the mean",
                 &format!("The single mean shift the model finds in x = {}: after observation {c}, with mean {} before and {} \
                           after (squared error {}, a reduction of {} against one common mean).", lst(&x), num(mb), num(ma),
                          num(s.out("cost").unwrap_or(f64::NAN)), num(s.out("reduction").unwrap_or(f64::NAN))), &body))
}

pub fn two_changes(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let (c1, c2) = (s.out("change_1")? as usize, s.out("change_2")? as usize);
    let m = s.out_list("means")?;
    if !(c1 < c2 && c2 < x.len()) || m.len() != 3 {
        return None;
    }
    let mut p = seg_plot(&x, &[("segment means".into(), steps(&[c1, c2], &m, x.len()))], &[c1, c2])?;
    p.vline(c1 as f64 + 0.5, &format!("after t = {c1}"));
    p.vline(c2 as f64 + 0.5, &format!("after t = {c2}"));
    let body = p.finish("t, index", "x", "");
    Some(s.chart("segments", "Two change points and the segment means",
                 &format!("The two change points the model finds in x = {}: after observations {c1} and {c2}, splitting the \
                           series into segments with means {} (total squared error {}).", lst(&x), lst(&m),
                          num(s.out("cost").unwrap_or(f64::NAN))), &body))
}

pub fn change_comparison(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let (c1, c2) = (s.out("change_1")? as usize, s.out("change_2")? as usize);
    let m = s.out_list("means")?;
    let one = other(s, "change_point_mean_shift", &[])?;
    let c = o(&one, "change_index")? as usize;
    let (mb, ma) = (o(&one, "mean_before")?, o(&one, "mean_after")?);
    if !(c1 < c2 && c2 < x.len()) || m.len() != 3 || c == 0 || c >= x.len() {
        return None;
    }
    let (cost1, cost2) = (o(&one, "cost").unwrap_or(f64::NAN), s.out("cost").unwrap_or(f64::NAN));
    let lines = vec![(format!("one change (cost {})", num(cost1)), steps(&[c], &[mb, ma], x.len())),
                     (format!("two changes (cost {})", num(cost2)), steps(&[c1, c2], &m, x.len()))];
    let p = seg_plot(&x, &lines, &[])?;
    let body = p.finish("t, index", "x", "");
    Some(s.chart("comparison", "One change against two changes",
                 &format!("Data x = {} split by the model at one change (after t = {c}, means {} and {}, squared error {}) \
                           and at two changes (after t = {c1} and {c2}, means {}, squared error {}).", lst(&x), num(mb),
                          num(ma), num(cost1), lst(&m), num(cost2)), &body))
}

pub fn penalised(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let pen = n(s, "penalty")?;
    let cuts: Vec<usize> = s.out_list("change_indices").unwrap_or_default().iter().map(|c| *c as usize).collect();
    if cuts.iter().any(|c| *c == 0 || *c >= x.len()) {
        return None;
    }
    let mut b = vec![0usize];
    b.extend(cuts.iter().copied());
    b.push(x.len());
    let means: Vec<f64> = b.windows(2).map(|w| mean(&x[w[0]..w[1]])).collect();
    let a = seg_plot(&x, &[("segment means".into(), steps(&cuts, &means, x.len()))], &cuts)?;
    let a_body = a.finish("t, index", "x", &format!("{} change(s)", cuts.len()));
    let pens: Vec<f64> = if pen > 0.0 { k::span(pen / 10.0, pen * 50.0, 9, true) } else { vec![] };
    let mut counts = vec![];
    for pv in pens.iter().copied().chain(std::iter::once(pen)) {
        if let Some(c) = s.with(&[("penalty", pv)]).and_then(|r| o(&r, "n_changes")) {
            counts.push((pv, c));
        }
    }
    counts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    counts.dedup_by(|a, b| a.0 == b.0);
    if counts.len() < 2 {
        return None;
    }
    let (xs, mut ys) = xs_ys(&[&counts]);
    ys.push(0.0);
    let mut bp = Plot::new(&xs, &ys, true, pen > 0.0)?;
    bp.line_dots(&counts, 0, "");
    bp.mark(pen, cuts.len() as f64, &format!("penalty {}: {}", num(pen), cuts.len()));
    let b_body = bp.finish("penalty (log scale)", "changes found", "");
    let pc: Vec<String> = counts.iter().map(|q| format!("{} gives {}", num(q.0), q.1)).collect();
    Some(two(s, "segments", "Penalised change points and the penalty",
             &format!("Penalised change-point search on x = {}: with penalty {} the model returns {} change(s){} (cost {}); \
                       the segment means drawn (top) are the means of the stated data between them. Bottom: the number of \
                       changes the model returns for penalties {}.", lst(&x), num(pen), cuts.len(),
                      if cuts.is_empty() { String::new() } else {
                          format!(", after observations {}", cuts.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", "))
                      }, num(s.out("cost").unwrap_or(f64::NAN)), pc.join(", ")), a_body, b_body))
}

pub fn cusum_path(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let c = s.out("change_index")? as usize;
    let mx = s.out("max_cusum")?;
    let m = mean(&x);
    let mut acc = 0.0;
    let mut pts = vec![(0.0, 0.0)];
    for (i, v) in x.iter().enumerate() {
        acc += v - m;
        pts.push((i as f64 + 1.0, acc));
    }
    // drawn only when the path reproduces the model's own change point and maximum
    let at = pts.get(c)?.1;
    if (at.abs() - mx).abs() > 1e-6 * (1.0 + mx.abs()) {
        return None;
    }
    let (xs, ys) = xs_ys(&[&pts]);
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.hline(0.0, "");
    p.line_dots(&pts, 0, "cusum S(t)");
    p.mark(c as f64, at, &format!("|S| max {} at t = {c}", num(mx)));
    let body = p.finish("t, index", "cumulative sum of x - mean", "");
    Some(s.chart("cusum", "Cusum path locating a change in mean",
                 &format!("The cumulative sum of deviations from the overall mean ({}, computed from the stated data x = {}): \
                           it peaks in size where the mean changes. The model returns the change after t = {c}, maximum \
                           |cusum| {}, mean {} before and {} after.", num(m), lst(&x), num(mx),
                          num(s.out("mean_before").unwrap_or(f64::NAN)), num(s.out("mean_after").unwrap_or(f64::NAN))),
                 &body))
}

pub fn tabular_cusum(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let h = n(s, "h")?;
    if x.len() > 60 {
        return None;
    }
    let mut up = vec![(0.0, 0.0)];
    let mut dn = vec![(0.0, 0.0)];
    for t in 1..=x.len() {
        if let Some(a) = s.with_input(&[("x", vj(&x[..t]))]) {
            if let (Some(cp), Some(cm)) = (o(&a, "c_plus"), o(&a, "c_minus")) {
                up.push((t as f64, cp));
                dn.push((t as f64, -cm));
            }
        }
    }
    if up.len() < 3 {
        return None;
    }
    let (xs, mut ys) = xs_ys(&[&up, &dn]);
    ys.extend([h, -h]);
    let mut p = Plot::new(&xs, &ys, true, false)?;
    p.hline_at(h, &format!("decision limit h = {}", num(h)), true);
    p.hline(-h, "");
    p.hline(0.0, "");
    p.line_dots(&up, 0, "C⁺ (upward shifts)");
    p.line_dots(&dn, 2, "-C⁻ (downward shifts)");
    let alarm = s.out("alarm_index");
    if let Some(al) = alarm {
        p.vline(al, &format!("alarm at t = {}", num(al)));
    }
    let body = p.finish("t, observation", "cumulative sum", "");
    Some(s.chart("cusum", "Tabular cusum against its decision limit",
                 &format!("Tabular cusum of x = {} against target μ₀ = {} with allowance k = {}: the upper and lower sums \
                           after each observation (the model's run on the first t values), and the decision limit ±{}; {}.",
                          lst(&x), num(n(s, "mu0")?), num(n(s, "k")?), num(h),
                          match alarm {
                              Some(al) => format!("the model signals on the {} side at t = {} (C⁺ = {}, C⁻ = {})",
                                                  s.out_text("side").unwrap_or_default(), num(al),
                                                  num(s.out("c_plus").unwrap_or(f64::NAN)),
                                                  num(s.out("c_minus").unwrap_or(f64::NAN))),
                              None => "the model gives no alarm".into(),
                          }), &body))
}

pub fn trend(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let nn = x.len() as f64;
    let hist = idx(&x);
    let line = |b0: f64, b1: f64| vec![(1.0, b0 + b1), (nn, b0 + b1 * nn)];
    let mk = s.key == "mann_kendall_test";
    let (b1, b0) = if mk {
        let sen = other(s, "sens_slope", &[])?;
        (o(&sen, "slope")?, o(&sen, "intercept")?)
    } else {
        (s.out("slope")?, s.out("intercept")?)
    };
    let ls = other(s, "linear_trend_forecast", &[]);
    let (xs, ys) = xs_ys(&[&hist, &line(b0, b1)]);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.line_dots(&hist, 3, "data");
    p.dashed(&line(b0, b1), 0, &format!("Sen: {} + {}·t", num(b0), num(b1)));
    let mut ls_t = String::new();
    if !mk {
        if let Some(a) = &ls {
            if let (Some(c0), Some(c1)) = (o(a, "intercept"), o(a, "slope")) {
                p.dashed(&line(c0, c1), 2, &format!("least squares: {} + {}·t", num(c0), num(c1)));
                ls_t = format!("; for comparison the least-squares trend (the model's linear trend fit) has slope {}", num(c1));
            }
        }
    }
    let header = if mk {
        format!("S = {}, z = {}, p = {}", num(s.out("s")?), num(s.out("z")?), num(s.out("p_value")?))
    } else { String::new() };
    let body = p.finish("t, period", "x", &header);
    if mk {
        let pv = s.out("p_value")?;
        Some(s.chart("trend", "Mann-Kendall trend test with Sen's slope",
                     &format!("The Mann-Kendall trend test on x = {}: the model returns S = {} (concordant minus discordant \
                               pairs), variance {}, z = {} and p-value {}{}; the dashed line is the model's Sen slope {} with \
                               intercept {} (t = 1 ... n).", lst(&x), num(s.out("s")?), num(s.out("var_s").unwrap_or(f64::NAN)),
                              num(s.out("z")?), num(pv), if pv < 0.05 { ", a significant trend at the 0.05 level" } else {
                                  ", not significant at the 0.05 level"
                              }, num(b1), num(b0)), &body))
    } else {
        Some(s.chart("trend", "Sen's robust slope against least squares",
                     &format!("Sen's slope of x = {}, the median of all pairwise slopes and robust to outlying values, is {} \
                               per period with intercept {}{ls_t}. Both lines drawn from the returned values (t = 1 ... n).",
                              lst(&x), num(b1), num(b0)), &body))
    }
}

// ------------------------------------------------------------------------------------------------ Kalman
pub fn kf_cv(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    if x.len() > 40 {
        return None;
    }
    let (mut pos, mut lo, mut hi, mut vel) = (vec![], vec![], vec![], vec![]);
    for t in 1..=x.len() {
        if let Some(a) = s.with_input(&[("x", vj(&x[..t]))]) {
            if let (Some(p), Some(v), Some(p11)) = (o(&a, "position"), o(&a, "velocity"), o(&a, "p11")) {
                let tt = t as f64;
                pos.push((tt, p));
                vel.push((tt, v));
                lo.push((tt, p - 1.96 * p11.max(0.0).sqrt()));
                hi.push((tt, p + 1.96 * p11.max(0.0).sqrt()));
            }
        }
    }
    if pos.len() < 2 {
        return None;
    }
    let obs = idx(&x);
    let (xs, ys) = xs_ys(&[&pos, &obs, &lo, &hi]);
    let mut a = Plot::new(&xs, &ys, false, false)?;
    a.band(&lo, &hi, 3, "±1.96·√p11");
    a.dots(&obs, 1, "measurements");
    a.line_dots(&pos, 0, "filtered position");
    let a = a.finish("t, observation", "position", "");
    let (xs, ys) = xs_ys(&[&vel]);
    let mut b = Plot::new(&xs, &ys, false, false)?;
    b.line_dots(&vel, 0, "filtered velocity");
    let b = b.finish("t, observation", "velocity", "");
    Some(two(s, "filter", "Tracking position and velocity with a Kalman filter",
             &format!("A constant-velocity Kalman filter (dt = {}, q = {}, r = {}, start position {}, velocity {}, variance {}) \
                       on the measurements x = {}: after each measurement t the model's run on the first t values gives the \
                       position (with the band ±1.96·√p11 from its returned variance) and the velocity; final position {}, \
                       velocity {}.", num(n(s, "dt")?), num(n(s, "q")?), num(n(s, "r")?), num(n(s, "p0")?),
                      num(n(s, "v0")?), num(n(s, "var0")?), lst(&x), num(s.out("position")?), num(s.out("velocity")?)),
             a, b))
}

fn ll_words(s: &Solved) -> Option<String> {
    Some(format!("q = {}, r = {}, m₀ = {}, p₀ = {}", num(n(s, "q")?), num(n(s, "r")?), num(n(s, "m0")?), num(n(s, "p0")?)))
}

pub fn kf_band(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    if x.len() > 40 {
        return None;
    }
    let (mut lv, mut lo, mut hi, mut gain) = (vec![], vec![], vec![], vec![]);
    for t in 1..=x.len() {
        if let Some(a) = s.with_input(&[("x", vj(&x[..t]))]) {
            if let (Some(e), Some(v), Some(g)) = (o(&a, "estimate"), o(&a, "variance"), o(&a, "gain")) {
                let tt = t as f64;
                lv.push((tt, e));
                lo.push((tt, e - 1.96 * v.max(0.0).sqrt()));
                hi.push((tt, e + 1.96 * v.max(0.0).sqrt()));
                gain.push((tt, g));
            }
        }
    }
    if lv.len() < 2 {
        return None;
    }
    let obs = idx(&x);
    let (xs, ys) = xs_ys(&[&lv, &obs, &lo, &hi]);
    let mut a = Plot::new(&xs, &ys, false, false)?;
    a.band(&lo, &hi, 3, "±1.96·√variance");
    a.dots(&obs, 1, "observations");
    a.line_dots(&lv, 0, "filtered level");
    let a = a.finish("t, observation", "level", "");
    let (xs, mut ys) = xs_ys(&[&gain]);
    ys.extend([0.0, 1.0]);
    let mut b = Plot::new(&xs, &ys, false, false)?;
    b.line_dots(&gain, 0, "gain");
    let b = b.finish("t, observation", "Kalman gain", "");
    Some(two(s, "band", "Local-level filter with its uncertainty and gain",
             &format!("The local-level Kalman filter ({}) on x = {}: after each observation t the model's run on the first t \
                       values gives the level, its variance (the band ±1.96·√variance) and the gain, which settles as the \
                       variance shrinks; final level {}, variance {}, gain {}.", ll_words(s)?, lst(&x),
                      num(s.out("estimate")?), num(s.out("variance")?), num(s.out("gain")?)), a, b))
}

pub fn kf_filter(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let est = s.out_list("estimates")?;
    let sm = other(s, "rts_smoother_local_level", &[]).and_then(|a| ol(&a, "smoothed"));
    let (obs, f) = (idx(&x), idx(&est));
    let smp = sm.as_ref().map(|v| idx(v)).unwrap_or_default();
    let (xs, ys) = xs_ys(&[&obs, &f, &smp]);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.dots(&obs, 3, "observations");
    p.line_dots(&f, 0, "Kalman filter");
    if !smp.is_empty() {
        p.line_dots(&smp, 1, "smoother (backward pass)");
    }
    let last = f[f.len() - 1];
    p.mark(last.0, last.1, &num(last.1));
    let body = p.finish("t, time step", "level", "");
    Some(s.chart("filter", "Kalman filter and smoother against noisy observations",
                 &format!("A local-level model ({}) run by the model on the {} observations x = {}: the filtered level after \
                           each observation{}; marked: t = {}, filtered level = {} (variance {}, gain {}).", ll_words(s)?,
                          x.len(), lst(&x), if smp.is_empty() { "" } else { " and the smoothed level from the backward pass" },
                          x.len(), num(last.1), num(s.out("variance")?), num(s.out("gain")?)), &body))
}

pub fn rts_family(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let q = n(s, "q")?;
    let obs = idx(&x);
    let mut series = vec![];
    for qq in [q / 10.0, q, q * 10.0] {
        if let Some(v) = s.with(&[("q", qq)]).and_then(|a| ol(&a, "smoothed")) {
            series.push((qq, idx(&v)));
        }
    }
    if series.is_empty() {
        return None;
    }
    let mut refs: Vec<&[(f64, f64)]> = series.iter().map(|v| v.1.as_slice()).collect();
    refs.push(&obs);
    let (xs, ys) = xs_ys(&refs);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.dots(&obs, 3, "observations");
    let mut parts = vec![];
    let mut c = 1;
    for (qq, pts) in &series {
        let ex = *qq == q;
        let cc = if ex { 0 } else { c += 1; c - 1 };
        p.line_dots(pts, cc, &format!("q = {}{}", num(*qq), if ex { " (example)" } else { "" }));
        parts.push(format!("q = {}: {} to {}", num(*qq), num(pts[0].1), num(pts[pts.len() - 1].1)));
    }
    let body = p.finish("t, observation", "level", "");
    Some(s.chart("smoothing", "Smoothed level for three state-noise variances",
                 &format!("Smoothed levels (forward filter and backward pass) of x = {} with r = {}, m₀ = {}, p₀ = {}, for {} \
                           state-noise variances q, each returned by the model: {}. Small q gives a nearly constant level; \
                           large q follows the data.", lst(&x), num(n(s, "r")?), num(n(s, "m0")?), num(n(s, "p0")?),
                          series.len(), parts.join("; ")), &body))
}

pub fn cv_gains(s: &Solved) -> Option<Figure> {
    let q = n(s, "q")?;
    if !(q > 0.0) {
        return None;
    }
    let qs = k::span(q / 100.0, q * 1000.0, 31, true);
    let kp = s.curve("q", &qs, "k_position", &[]);
    let kv = s.curve("q", &qs, "k_velocity", &[]);
    if kp.len() < 3 {
        return None;
    }
    let (xs, ys) = xs_ys(&[&kp, &kv]);
    let mut p = Plot::new(&xs, &ys, true, true)?;
    p.line(&kp, 0, "position gain");
    p.line(&kv, 1, "velocity gain");
    let (a, b) = (s.out("k_position")?, s.out("k_velocity")?);
    p.mark(q, a, &num(a));
    p.dots(&[(q, b)], 1, "");
    let body = p.finish("q, acceleration noise variance (log)", "steady-state gain", "");
    Some(s.chart("gains", "Constant-velocity tracker: steady-state gains",
                 &format!("Steady-state Kalman gains of a constant-velocity tracker (dt = {}, measurement noise r = {}) as the \
                           acceleration noise q runs over {} to {} ({} runs of the model): more process noise, more trust in \
                           the measurements. Marked: q = {}, position gain {}, velocity gain {}.", num(n(s, "dt")?),
                          num(n(s, "r")?), num(qs[0]), num(qs[qs.len() - 1]), kp.len(), num(q), num(a), num(b)), &body))
}

pub fn gain_convergence(s: &Solved) -> Option<Figure> {
    let (q, r) = (n(s, "q")?, n(s, "r")?);
    let g = s.out("gain")?;
    let prior = s.out("prior_variance")?;
    let mut p0 = (4.0 * prior).max(1e-12);
    let start = p0;
    let mut pts = vec![];
    for step in 1..=12 {
        let a = other(s, "kalman_step", &[("m", fl(0.0)), ("p", fl(p0)), ("z", fl(0.0)), ("q", fl(q)), ("r", fl(r))])?;
        let (gg, v) = (o(&a, "gain")?, o(&a, "variance")?);
        pts.push((step as f64, gg));
        p0 = v;
    }
    let (xs, mut ys) = xs_ys(&[&pts]);
    ys.push(g);
    let mut p = Plot::new(&xs, &ys, false, false)?;
    p.hline(g, &format!("steady-state gain {}", num(g)));
    p.line_dots(&pts, 0, "gain at each step");
    let body = p.finish("step", "Kalman gain", "");
    Some(s.chart("convergence", "Kalman gain converging to its steady state",
                 &format!("The Kalman gain step by step for a local-level model with q = {}, r = {}, starting from variance p = \
                           {} (each step a run of the model's Kalman step, which feeds its returned variance to the next), \
                           converging to the steady-state gain the model returns, {} (prior variance {}, posterior {}).",
                          num(q), num(r), num(start), num(g), num(prior),
                          num(s.out("posterior_variance").unwrap_or(f64::NAN))), &body))
}

pub fn gain_field(s: &Solved) -> Option<Figure> {
    let (q, r) = (n(s, "q")?, n(s, "r")?);
    if !(q > 0.0 && r > 0.0) {
        return None;
    }
    let qs = k::span(q / 10.0, q * 10.0, 21, true);
    let rs = k::span(r / 10.0, r * 10.0, 15, true);
    let mut vals = vec![];
    let mut all = vec![];
    for qq in &qs {
        let mut col = vec![];
        for rr in &rs {
            let v = s.with(&[("q", *qq), ("r", *rr)]).and_then(|a| o(&a, "gain"));
            if let Some(v) = v {
                all.push(v);
            }
            col.push(v);
        }
        vals.push(col);
    }
    if all.len() < 20 {
        return None;
    }
    let g = s.out("gain")?;
    let body = k::field(&qs, &rs, &vals, (q, r), "q, process noise variance", "r, measurement noise variance",
                        &format!("gain {}", num(g)), (true, true));
    let lo = all.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = all.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    Some(s.chart("field", "Steady-state Kalman gain over process and measurement noise",
                 &format!("The steady-state gain over q from {} to {} and r from {} to {} (logarithmic axes): each of the {} x \
                           {} cells is one run of the model, shaded by its value (light {} to dark {}). Marked: q = {}, r = {}, \
                           gain = {}.", num(qs[0]), num(qs[qs.len() - 1]), num(rs[0]), num(rs[rs.len() - 1]), qs.len(),
                          rs.len(), num(lo), num(hi), num(q), num(r), num(g)), &body))
}

pub fn densities(s: &Solved) -> Option<Figure> {
    let (m, p, z, r) = (n(s, "m")?, n(s, "p")?, n(s, "z")?, n(s, "r")?);
    let step = s.key == "kalman_step";
    let prior_v = if step { s.out("prior_variance")? } else { p };
    let (mean, var, gain) = (s.out("mean")?, s.out("variance")?, s.out("gain")?);
    if !(prior_v > 0.0 && r > 0.0 && var > 0.0) {
        return None;
    }
    let dists = [("prior", m, prior_v), ("measurement", z, r), ("posterior", mean, var)];
    let lo = dists.iter().map(|d| d.1 - 3.5 * d.2.sqrt()).fold(f64::INFINITY, f64::min);
    let hi = dists.iter().map(|d| d.1 + 3.5 * d.2.sqrt()).fold(f64::NEG_INFINITY, f64::max);
    let xs = k::span(lo, hi, 161, false);
    let curves: Vec<Vec<(f64, f64)>> = dists.iter().map(|(_, mu, v)| {
        xs.iter().map(|x| (*x, (-(x - mu).powi(2) / (2.0 * v)).exp() / (2.0 * std::f64::consts::PI * v).sqrt())).collect()
    }).collect();
    let refs: Vec<&[(f64, f64)]> = curves.iter().map(|v| v.as_slice()).collect();
    let (xs2, ys) = xs_ys(&refs);
    let mut pl = Plot::new(&xs2, &ys, true, false)?;
    for (i, ((nm, mu, v), c)) in dists.iter().zip(curves.iter()).enumerate() {
        let cls = [3, 1, 0][i];
        pl.line(c, cls, &format!("{nm} N({}, {})", num(*mu), num(*v)));
    }
    let body = pl.finish("state", "density", &format!("gain {}", num(gain)));
    let pre = if step {
        format!("One Kalman filter step: the previous estimate m = {} with variance p = {} is predicted forward with process \
                 noise q = {} to the returned prior variance {}; the measurement z = {} has noise variance r = {}.", num(m),
                num(p), num(n(s, "q")?), num(prior_v), num(z), num(r))
    } else {
        format!("A Kalman update of the estimate m = {} with variance p = {}; the measurement z = {} has noise variance r = \
                 {}.", num(m), num(p), num(z), num(r))
    };
    Some(s.chart("densities", if step { "Kalman step: predict, then update" } else {
                     "Kalman update: prior and measurement combine"
                 },
                 &format!("{pre} The model returns the gain {}, the posterior mean {} and variance {}. The normal curves are \
                           drawn from these means and variances; the posterior is narrower than both.", num(gain), num(mean),
                          num(var)), &body))
}
