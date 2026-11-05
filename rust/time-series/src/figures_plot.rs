//! A small plotting surface for Time Series' own figures: the generic 520 x 340 frame (the site's axp-* classes) with
//! lines, dashed lines, dots, stems (correlograms), shaded bands, guide lines, a marked point and a legend. Several
//! panels are stacked with figkit::stack.

use ax_engine::figkit::num;

pub const X0: f64 = 96.0;
pub const X1: f64 = 504.0;
pub const Y0: f64 = 44.0;
pub const Y1: f64 = 278.0;

const LINE: [&str; 4] = ["axp-c", "axp-c2", "axp-c3", "axp-c4"];
const DOT: [&str; 4] = ["axp-d", "axp-d2", "axp-d3", "axp-d4"];
const BAND: [&str; 4] = ["axp-s", "axp-s2", "axp-s3", "axp-s4"];
const SWATCH: [&str; 4] = ["axp-b", "axp-b2", "axp-b3", "axp-b4"];

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

#[derive(Clone, Copy)]
struct Ax {
    lo: f64,
    hi: f64,
    log: bool,
    a: f64,
    b: f64,
    int: bool,
}

impl Ax {
    fn px(&self, v: f64) -> f64 {
        let f = if self.log { (v.log10() - self.lo.log10()) / (self.hi.log10() - self.lo.log10()) } else {
            (v - self.lo) / (self.hi - self.lo)
        };
        self.a + f * (self.b - self.a)
    }
    fn ticks(&self) -> Vec<f64> {
        if self.log {
            let (e0, e1) = (self.lo.log10().ceil() as i32, self.hi.log10().floor() as i32);
            let n = (e1 - e0 + 1).max(1);
            let step = ((n as f64) / 6.0).ceil().max(1.0) as usize;
            let v: Vec<f64> = (e0..=e1).step_by(step).map(|e| 10f64.powi(e)).collect();
            return if v.is_empty() { vec![(self.lo * self.hi).sqrt()] } else { v };
        }
        let span = self.hi - self.lo;
        if !(span > 0.0) || !span.is_finite() {
            return vec![self.lo];
        }
        let raw = span / 5.0;
        let mag = 10f64.powf(raw.log10().floor());
        let nrm = raw / mag;
        let mut step = mag * if nrm < 1.5 { 1.0 } else if nrm < 3.0 { 2.0 } else if nrm < 7.0 { 5.0 } else { 10.0 };
        if self.int && step < 1.0 {
            step = 1.0;
        }
        let mut v = vec![];
        let mut t = (self.lo / step).ceil() * step;
        while t <= self.hi + step * 1e-9 && v.len() < 12 {
            v.push(if t.abs() < step * 1e-9 { 0.0 } else { t });
            t += step;
        }
        v
    }
}

fn range(v: &[f64]) -> Option<(f64, f64)> {
    let f: Vec<f64> = v.iter().copied().filter(|x| x.is_finite()).collect();
    if f.is_empty() {
        return None;
    }
    let lo = f.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = f.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    Some((lo, hi))
}

fn axis(lo: f64, hi: f64, log: bool, a: f64, b: f64, pad: f64) -> Ax {
    let (mut lo, mut hi) = (lo, hi);
    if log {
        if !(lo > 0.0) {
            lo = hi / 10.0;
        }
        if hi <= lo {
            hi = lo * 10.0;
        }
        let f = 1.0 + pad * 3.0;
        return Ax { lo: lo / f, hi: hi * f, log, a, b, int: false };
    }
    if !(hi > lo) || !(hi - lo).is_finite() {
        let d = if lo == 0.0 { 1.0 } else { lo.abs() * 0.1 };
        lo -= d;
        hi += d;
    }
    let d = (hi - lo) * pad;
    Ax { lo: lo - d, hi: hi + d, log, a, b, int: false }
}

/// One plot panel: data ranges fixed at creation, drawing calls append to the body.
pub struct Plot {
    xa: Ax,
    ya: Ax,
    s: String,
    legend: Vec<(String, usize)>,
    marks: String,
    xcats: Option<Vec<String>>,
}

impl Plot {
    /// A panel over these x and y values (ranges padded; `zero`: the y axis includes 0; `log_x`: logarithmic x).
    pub fn new(xs: &[f64], ys: &[f64], zero: bool, log_x: bool) -> Option<Plot> {
        let (xl, xh) = range(xs)?;
        let (mut yl, mut yh) = range(ys)?;
        if zero {
            yl = yl.min(0.0);
            yh = yh.max(0.0);
        }
        let xp = if log_x { 0.02 } else if xh > xl { 0.04 } else { 0.5 };
        let mut xa = axis(xl, xh, log_x && xl > 0.0, X0, X1, xp);
        let mut ya = axis(yl, yh, false, Y1, Y0, 0.08);
        // whole-number data (periods, lags, counts) get whole-number ticks
        let whole = |v: &[f64], tol: f64| v.iter().filter(|x| x.is_finite()).all(|x| (x - x.round()).abs() <= tol);
        xa.int = whole(xs, 0.15);
        ya.int = whole(ys, 1e-9);
        Some(Plot { xa, ya, s: String::new(), legend: vec![], marks: String::new(), xcats: None })
    }
    /// Category names under x = 1, 2, ... (instead of numeric ticks).
    pub fn cats(mut self, names: &[String]) -> Plot {
        self.xa = Ax { lo: 0.5, hi: names.len() as f64 + 0.5, log: false, a: X0, b: X1, int: true };
        self.xcats = Some(names.to_vec());
        self
    }
    fn p(&self, x: f64, y: f64) -> (f64, f64) {
        (self.xa.px(x), self.ya.px(y))
    }
    fn ok(&self, x: f64, y: f64) -> bool {
        x.is_finite() && y.is_finite() && (!self.xa.log || x > 0.0)
    }
    fn name(&mut self, name: &str, c: usize) {
        if !name.is_empty() && !self.legend.iter().any(|l| l.0 == name) {
            self.legend.push((name.to_string(), c));
        }
    }
    fn path(&self, pts: &[(f64, f64)]) -> String {
        let mut d = String::new();
        let mut pen = false;
        for &(x, y) in pts {
            if !self.ok(x, y) {
                pen = false;
                continue;
            }
            let (px, py) = self.p(x, y);
            d.push_str(&format!("{}{px:.1},{py:.1} ", if pen { "L" } else { "M" }));
            pen = true;
        }
        d.trim_end().to_string()
    }
    /// A line through the points (a NaN point breaks it).
    pub fn line(&mut self, pts: &[(f64, f64)], c: usize, name: &str) {
        let d = self.path(pts);
        if !d.is_empty() {
            self.s.push_str(&format!("<path class=\"{}\" d=\"{d}\"/>", LINE[c % 4]));
        }
        self.name(name, c);
    }
    /// A dashed line.
    pub fn dashed(&mut self, pts: &[(f64, f64)], c: usize, name: &str) {
        let d = self.path(pts);
        if !d.is_empty() {
            self.s.push_str(&format!("<path class=\"{}\" stroke-dasharray=\"6 4\" d=\"{d}\"/>", LINE[c % 4]));
        }
        self.name(name, c);
    }
    /// Dots at the points.
    pub fn dots(&mut self, pts: &[(f64, f64)], c: usize, name: &str) {
        for &(x, y) in pts.iter().take(400) {
            if self.ok(x, y) {
                let (px, py) = self.p(x, y);
                self.s.push_str(&format!("<circle class=\"{}\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"3.2\"/>", DOT[c % 4]));
            }
        }
        self.name(name, c);
    }
    /// A line with a dot at each point.
    pub fn line_dots(&mut self, pts: &[(f64, f64)], c: usize, name: &str) {
        self.line(pts, c, name);
        if pts.len() <= 60 {
            self.dots(pts, c, "");
        }
    }
    /// Stems from y = 0 to each point, a dot on top (a correlogram).
    pub fn stems(&mut self, pts: &[(f64, f64)], c: usize, name: &str) {
        let y0 = if self.ya.lo > 0.0 { self.ya.lo } else if self.ya.hi < 0.0 { self.ya.hi } else { 0.0 };
        for &(x, y) in pts {
            if self.ok(x, y) {
                let (px, py) = self.p(x, y);
                let (_, pz) = self.p(x, y0);
                self.s.push_str(&format!("<path class=\"{}\" d=\"M{px:.1},{pz:.1} L{px:.1},{py:.1}\"/>", LINE[c % 4]));
            }
        }
        self.dots(pts, c, name);
    }
    /// A shaded band between a lower and an upper edge at the same x values.
    pub fn band(&mut self, lo: &[(f64, f64)], hi: &[(f64, f64)], c: usize, name: &str) {
        let pts: Vec<(f64, f64)> = hi.iter().chain(lo.iter().rev()).copied().filter(|p| self.ok(p.0, p.1)).collect();
        if pts.len() >= 3 {
            let mut d = String::new();
            for (i, &(x, y)) in pts.iter().enumerate() {
                let (px, py) = self.p(x, y);
                d.push_str(&format!("{}{px:.1},{py:.1} ", if i == 0 { "M" } else { "L" }));
            }
            self.s.push_str(&format!("<path class=\"{}\" d=\"{}Z\"/>", BAND[c % 4], d));
        }
        self.name(name, c);
    }
    /// A horizontal guide line across the panel with a label at its right end.
    pub fn hline(&mut self, y: f64, label: &str) {
        self.hline_at(y, label, false);
    }
    /// A horizontal guide line, its label at the left end (`left`) or the right end.
    pub fn hline_at(&mut self, y: f64, label: &str, left: bool) {
        if !y.is_finite() || y < self.ya.lo || y > self.ya.hi {
            return;
        }
        let py = self.ya.px(y);
        self.s.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M{X0},{py:.1} L{X1},{py:.1}\"/>"));
        if !label.is_empty() {
            let ty = if py - 5.0 < Y0 + 12.0 { py + 15.0 } else { py - 5.0 };
            let (tx, anchor) = if left { (X0 + 4.0, "start") } else { (X1 - 4.0, "end") };
            self.marks.push_str(&format!("<text class=\"axp-lt\" x=\"{tx:.1}\" y=\"{ty:.1}\" text-anchor=\"{anchor}\">{}</text>",
                                         esc(label)));
        }
    }
    /// A vertical guide line with a label at its top.
    pub fn vline(&mut self, x: f64, label: &str) {
        if !x.is_finite() || x < self.xa.lo || x > self.xa.hi {
            return;
        }
        let px = self.xa.px(x);
        self.s.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M{px:.1},{Y0} L{px:.1},{Y1}\"/>"));
        if !label.is_empty() {
            let w = 7.4 * label.chars().count() as f64;
            let (tx, anchor) = if px + 6.0 + w <= X1 { (px + 6.0, "start") } else { (px - 6.0, "end") };
            self.marks.push_str(&format!("<text class=\"axp-lt\" x=\"{tx:.1}\" y=\"{:.1}\" text-anchor=\"{anchor}\">{}</text>",
                                         Y0 + 16.0, esc(label)));
        }
    }
    /// A short label at a data point (a value printed beside a bar or a dot).
    pub fn note(&mut self, x: f64, y: f64, dy: f64, t: &str) {
        if !self.ok(x, y) {
            return;
        }
        let (px, py) = self.p(x, y);
        let w = 7.6 * t.chars().count() as f64;
        let tx = px.clamp(X0 + w / 2.0 + 2.0, X1 - w / 2.0 - 2.0);
        let ty = (py + dy).clamp(Y0 + 12.0, Y1 - 4.0);
        self.marks.push_str(&format!("<text class=\"axp-vt\" x=\"{tx:.1}\" y=\"{ty:.1}\" text-anchor=\"middle\">{}</text>",
                                     esc(t)));
    }
    /// A short label just right of a data point.
    pub fn note_right(&mut self, x: f64, y: f64, t: &str) {
        if !self.ok(x, y) {
            return;
        }
        let (px, py) = self.p(x, y);
        let w = 7.6 * t.chars().count() as f64;
        let (tx, anchor) = if px + 6.0 + w <= X1 { (px + 6.0, "start") } else { (px - 6.0, "end") };
        self.marks.push_str(&format!("<text class=\"axp-vt\" x=\"{tx:.1}\" y=\"{:.1}\" text-anchor=\"{anchor}\">{}</text>",
                                     (py + 5.0).clamp(Y0 + 12.0, Y1 - 4.0), esc(t)));
    }
    /// The visitor's point, marked and labelled.
    pub fn mark(&mut self, x: f64, y: f64, label: &str) {
        if !self.ok(x, y) {
            return;
        }
        let (px, py) = self.p(x, y);
        let w = 9.0 * label.chars().count() as f64;
        let (tx, anchor) = if px + 10.0 + w <= X1 { (px + 10.0, "start") } else if px - 10.0 - w >= X0 {
            (px - 10.0, "end")
        } else {
            (((X0 + X1) / 2.0).clamp(X0 + w / 2.0, X1 - w / 2.0), "middle")
        };
        let ty = if py < Y0 + 30.0 { py + 22.0 } else { py - 10.0 };
        self.marks.push_str(&format!("<circle class=\"axp-m\" cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"6\"/>\
                                      <text class=\"axp-mt\" x=\"{tx:.1}\" y=\"{ty:.1}\" text-anchor=\"{anchor}\">{}</text>",
                                     esc(label)));
    }
    /// The panel: grid, ticks, frame, axis names, header, the drawing (clipped to the plot), marks and legend.
    pub fn finish(self, xlab: &str, ylab: &str, header: &str) -> String {
        let mut out = String::new();
        match &self.xcats {
            Some(names) => {
                for (i, n) in names.iter().enumerate() {
                    let x = self.xa.px(i as f64 + 1.0);
                    out.push_str(&format!("<text class=\"axp-t\" x=\"{x:.1}\" y=\"{}\" text-anchor=\"middle\">{}</text>",
                                          Y1 + 20.0, esc(n)));
                }
            }
            None => {
                for t in self.xa.ticks() {
                    let x = self.xa.px(t);
                    out.push_str(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{Y0}\" x2=\"{x:.1}\" y2=\"{Y1}\"/>\
                                           <text class=\"axp-t\" x=\"{x:.1}\" y=\"{}\" text-anchor=\"middle\">{}</text>",
                                          Y1 + 20.0, esc(&num(t))));
                }
            }
        }
        for t in self.ya.ticks() {
            let y = self.ya.px(t);
            out.push_str(&format!("<line class=\"axp-g\" x1=\"{X0}\" y1=\"{y:.1}\" x2=\"{X1}\" y2=\"{y:.1}\"/>\
                                   <text class=\"axp-t\" x=\"{}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>",
                                  X0 - 6.0, y + 5.0, esc(&num(t))));
        }
        out.push_str(&format!("<rect class=\"axp-f\" x=\"{X0}\" y=\"{Y0}\" width=\"{}\" height=\"{}\"/>", X1 - X0, Y1 - Y0));
        if !xlab.is_empty() {
            out.push_str(&format!("<text class=\"axp-l\" x=\"{:.1}\" y=\"326\" text-anchor=\"middle\">{}</text>",
                                  (X0 + X1) / 2.0, esc(&clip(xlab, 56))));
        }
        if !ylab.is_empty() {
            out.push_str(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" \
                                   text-anchor=\"middle\">{}</text>", (Y0 + Y1) / 2.0, esc(&clip(ylab, 32))));
        }
        if !header.is_empty() {
            out.push_str(&format!("<text class=\"axp-mt\" x=\"{X0}\" y=\"20\">{}</text>", esc(&clip(header, 44))));
        }
        out.push_str(&format!("<clipPath id=\"CLIPID\"><rect x=\"{X0}\" y=\"{Y0}\" width=\"{}\" height=\"{}\"/></clipPath>\
                               <g clip-path=\"url(#CLIPID)\">{}</g>", X1 - X0, Y1 - Y0, self.s));
        out.push_str(&self.marks);
        // the legend: one row above the plot, right-aligned; when too long for that, a column in the plot's top left
        let items: Vec<(String, usize)> = self.legend.iter().take(5).map(|(n, c)| (clip(n, 30), *c)).collect();
        let width: f64 = items.iter().map(|(t, _)| 7.0 * t.chars().count() as f64 + 22.0).sum();
        if width <= X1 - 12.0 {
            let mut x = X1;
            for (t, c) in items.iter().rev() {
                x -= 7.0 * t.chars().count() as f64 + 22.0;
                out.push_str(&format!("<rect class=\"{}\" x=\"{x:.1}\" y=\"28\" width=\"12\" height=\"10\"/>\
                                       <text class=\"axp-lt\" x=\"{:.1}\" y=\"37\">{}</text>", SWATCH[c % 4], x + 16.0,
                                      esc(t)));
            }
        } else {
            let w = items.iter().map(|(t, _)| 7.0 * t.chars().count() as f64 + 30.0).fold(0.0, f64::max);
            out.push_str(&format!("<rect class=\"axp-ka\" x=\"{}\" y=\"{}\" width=\"{w:.1}\" height=\"{}\"/>", X0 + 4.0,
                                  Y0 + 4.0, 16.0 * items.len() as f64 + 6.0));
            for (i, (t, c)) in items.iter().enumerate() {
                let y = Y0 + 8.0 + 16.0 * i as f64;
                out.push_str(&format!("<rect class=\"{}\" x=\"{}\" y=\"{y:.1}\" width=\"12\" height=\"10\"/>\
                                       <text class=\"axp-lt\" x=\"{}\" y=\"{:.1}\">{}</text>", SWATCH[c % 4], X0 + 10.0,
                                      X0 + 26.0, y + 9.0, esc(t)));
            }
        }
        out
    }
}

pub fn clip(s: &str, n: usize) -> String {
    let c: Vec<char> = s.chars().collect();
    if c.len() <= n { s.to_string() } else { format!("{}...", c[..n.saturating_sub(3)].iter().collect::<String>().trim_end()) }
}

/// The x values and y values of point lists (for Plot::new).
pub fn xs_ys(sets: &[&[(f64, f64)]]) -> (Vec<f64>, Vec<f64>) {
    let mut xs = vec![];
    let mut ys = vec![];
    for s in sets {
        for p in s.iter() {
            if p.0.is_finite() && p.1.is_finite() {
                xs.push(p.0);
                ys.push(p.1);
            }
        }
    }
    (xs, ys)
}
