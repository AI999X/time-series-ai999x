"""Time series procedures: deterministic algorithms over data series (vectors) with several outputs. Domain code,
loaded by the pack; a procedure is exposed only once tools/certify_procedures.py has certified it.

Every procedure states its inputs, outputs, conditions, method and LaTeX, the documented question forms it reads,
worked examples, an INDEPENDENT reference computation that takes a different route (an FFT autocorrelation, a
cumulative-sum or closed-form evaluation of a recursion, a state-space or Riccati solution, a library's own fitting
routine; numpy, scipy, statsmodels, ruptures, filterpy, scikit-learn, sktime imported inside the reference only and
used by certification only) and a deliberately wrong variant that certification shows the checks reject. The shipped
code is plain Python (math): least squares by Householder QR, Durbin-Levinson recursions, the chi-square tail from
the regularized incomplete gamma function.

Conventions (stated once here and again in each capability):
  * a series x_1..x_n is written oldest first; time indices are t = 1..n;
  * sample autocovariances divide by n (not n - k) and are taken about the sample mean;
  * exponential smoothing states l_0, b_0, s_{1-m..0} are the states BEFORE the first observation; the one-step
    fitted value of x_t uses the states at t - 1; forecast errors are e_t = actual - forecast;
  * a list of seasonal indices starts at the season of the FIRST observation;
  * a change index k means the first segment is x_1..x_k; ties go to the smallest k;
  * outputs built from exp, log or powers are rounded to 12 significant digits (identical on every platform).
"""
from __future__ import annotations

import ast
import math
import re
from pathlib import Path

PROCEDURES: dict = {}
LEVELS: dict = {}
_ROUTES: list = []

_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:e[-+]?\d+)?"


def _load_expected():
    """Example outputs computed once by the independent reference libraries (models/time_series/_dev/gen_expected.py)
    and frozen in _expected.py; a run must reproduce them. Without the file the examples carry no outputs."""
    try:
        text = Path(__file__).with_name("_expected.py").read_text(encoding="utf-8")
        return ast.literal_eval(text.split("=", 1)[1].strip())
    except Exception:
        return {}


_EXPECTED = _load_expected()


def _r12(v):
    return float(f"{v:.12g}") + 0.0


# ================================================================================================ input checks ====
def _fin(x, name="the series"):
    out = []
    for v in x:
        if isinstance(v, bool) or not isinstance(v, (int, float)) or not math.isfinite(v):
            raise ValueError(f"{name} must contain finite numbers only")
        out.append(float(v))
    return out


def _need(x, n, name="the series"):
    if len(x) < n:
        raise ValueError(f"{name} needs at least {n} values")


def _int(v, name, lo=None, hi=None):
    if isinstance(v, bool) or not isinstance(v, (int, float)) or not math.isfinite(v) or float(v) != int(v):
        raise ValueError(f"{name} must be a whole number")
    v = int(v)
    if lo is not None and v < lo:
        raise ValueError(f"{name} must be at least {lo}")
    if hi is not None and v > hi:
        raise ValueError(f"{name} must be at most {hi}")
    return v


def _num(v, name):
    if isinstance(v, bool) or not isinstance(v, (int, float)) or not math.isfinite(v):
        raise ValueError(f"{name} must be a finite number")
    return float(v)


def _pos(v, name):
    v = _num(v, name)
    if v <= 0:
        raise ValueError(f"{name} must be positive")
    return v


def _unit(v, name):
    v = _num(v, name)
    if not 0 < v <= 1:
        raise ValueError(f"{name} must lie in (0, 1]")
    return v


def _positive(x, name="the series"):
    if any(v <= 0 for v in x):
        raise ValueError(f"{name} must contain positive values only")


def _nonconstant(x, name="the series"):
    if max(x) == min(x):
        raise ValueError(f"{name} must not be constant")


def _same_len(a, b, na="the actual values", nb="the forecasts"):
    if len(a) != len(b):
        raise ValueError(f"{na} and {nb} must have the same length")


# ============================================================================================ numeric helpers ====
def _mean(x):
    return math.fsum(x) / len(x)


def _median(x):
    s = sorted(x)
    n = len(s)
    return s[n // 2] if n % 2 else (s[n // 2 - 1] + s[n // 2]) / 2.0


def _sse(x):
    m = _mean(x)
    return math.fsum((v - m) ** 2 for v in x)


def _acov(x, k, m=None):
    """Sample autocovariance at lag k, divided by n, about the sample mean."""
    n = len(x)
    m = _mean(x) if m is None else m
    return math.fsum((x[t] - m) * (x[t + k] - m) for t in range(n - k)) / n


def _acf_list(x, kmax):
    m = _mean(x)
    g0 = _acov(x, 0, m)
    if g0 == 0:
        raise ValueError("the series must not be constant")
    return [1.0] + [_acov(x, k, m) / g0 for k in range(1, kmax + 1)]


def _levinson(r, p):
    """Durbin-Levinson on autocorrelations r[0..p] (r[0] = 1): (AR coefficients of order p, partial autocorrelations
    1..p, innovation variance ratio prod(1 - kappa^2)). Raises when the sequence is not positive definite."""
    phi, pacf, v = [], [], 1.0
    for m in range(1, p + 1):
        a = (r[m] - math.fsum(phi[j] * r[m - 1 - j] for j in range(m - 1))) / v
        if not abs(a) < 1:
            raise ValueError("the autocorrelations are not a valid (positive definite) sequence")
        phi = [phi[j] - a * phi[m - 2 - j] for j in range(m - 1)] + [a]
        pacf.append(a)
        v *= (1 - a * a)
    return phi, pacf, v


def _reflection(coefs):
    """Step-down (inverse Levinson) of phi_1..phi_p in y_t = sum phi_j y_{t-j} + e_t: its reflection coefficients.
    Every root of the AR polynomial lies outside the unit circle iff every |kappa| < 1."""
    a = list(coefs)
    ks = []
    for m in range(len(a), 0, -1):
        k = a[m - 1]
        ks.append(k)
        if abs(k) >= 1:
            return ks
        a = [(a[j] + k * a[m - 2 - j]) / (1 - k * k) for j in range(m - 1)]
    return ks


def _stationary(phi) -> bool:
    return all(abs(k) < 1 for k in _reflection(phi))


def _solve(A, b):
    """Gaussian elimination with partial pivoting."""
    n = len(A)
    M = [list(map(float, A[i])) + [float(b[i])] for i in range(n)]
    for c in range(n):
        piv = max(range(c, n), key=lambda r: abs(M[r][c]))
        if abs(M[piv][c]) < 1e-300:
            raise ValueError("the linear system is singular")
        M[c], M[piv] = M[piv], M[c]
        for r in range(c + 1, n):
            f = M[r][c] / M[c][c]
            if f:
                for j in range(c, n + 1):
                    M[r][j] -= f * M[c][j]
    x = [0.0] * n
    for i in range(n - 1, -1, -1):
        x[i] = (M[i][n] - math.fsum(M[i][j] * x[j] for j in range(i + 1, n))) / M[i][i]
    return x


def _lstsq(X, y):
    """Ordinary least squares by Householder QR: (beta, residuals, diagonal of (X'X)^-1)."""
    n, p = len(X), len(X[0])
    if n <= p:
        raise ValueError("too few observations for the regression")
    A = [list(map(float, r)) for r in X]
    b = list(map(float, y))
    for j in range(p):
        norm = math.sqrt(math.fsum(A[i][j] ** 2 for i in range(j, n)))
        if norm == 0:
            raise ValueError("the regressors are collinear")
        alpha = -norm if A[j][j] >= 0 else norm
        v = [0.0] * n
        v[j] = A[j][j] - alpha
        for i in range(j + 1, n):
            v[i] = A[i][j]
        vn = math.fsum(v[i] ** 2 for i in range(j, n))
        if vn == 0:
            continue
        for k in range(j, p):
            f = 2 * math.fsum(v[i] * A[i][k] for i in range(j, n)) / vn
            for i in range(j, n):
                A[i][k] -= f * v[i]
        f = 2 * math.fsum(v[i] * b[i] for i in range(j, n)) / vn
        for i in range(j, n):
            b[i] -= f * v[i]
    big = max(abs(A[i][i]) for i in range(p))
    if any(abs(A[i][i]) <= 1e-11 * big for i in range(p)):
        raise ValueError("the regressors are collinear")
    beta = [0.0] * p
    for i in range(p - 1, -1, -1):
        beta[i] = (b[i] - math.fsum(A[i][j] * beta[j] for j in range(i + 1, p))) / A[i][i]
    Rinv = [[0.0] * p for _ in range(p)]
    for c in range(p):
        for i in range(p - 1, -1, -1):
            s = (1.0 if i == c else 0.0) - math.fsum(A[i][j] * Rinv[j][c] for j in range(i + 1, p))
            Rinv[i][c] = s / A[i][i]
    diag = [math.fsum(Rinv[i][c] ** 2 for c in range(p)) for i in range(p)]
    resid = [y[i] - math.fsum(X[i][j] * beta[j] for j in range(p)) for i in range(n)]
    return beta, resid, diag


def _gammq(a, x):
    """Regularized upper incomplete gamma Q(a, x) (series or continued fraction)."""
    if x <= 0:
        return 1.0
    lf = a * math.log(x) - x - math.lgamma(a)
    if x < a + 1.0:
        ap, s = a, 1.0 / a
        d = s
        for _ in range(100000):
            ap += 1.0
            d *= x / ap
            s += d
            if abs(d) < abs(s) * 1e-17:
                break
        return 1.0 - s * math.exp(lf)
    tiny = 1e-300
    b = x + 1.0 - a
    c, d = 1.0 / tiny, 1.0 / b
    h = d
    for i in range(1, 100000):
        an = -i * (i - a)
        b += 2.0
        d = an * d + b
        d = tiny if abs(d) < tiny else d
        c = b + an / c
        c = tiny if abs(c) < tiny else c
        d = 1.0 / d
        de = d * c
        h *= de
        if abs(de - 1.0) < 4e-16:
            break
    return math.exp(lf) * h


def _chi2_sf(x, k):
    return _gammq(k / 2.0, x / 2.0)


_Z975 = 1.959963984540054          # the standard normal 97.5 % quantile


# ====================================================================================== verify (residual) ====
def _d(a, b):
    if isinstance(a, str) or isinstance(b, str):
        return 0.0 if a == b else 1.0
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return 1.0
        return max((_d(u, v) for u, v in zip(a, b)), default=0.0)
    if a is None or b is None:
        return 0.0 if a is b else 1.0
    a, b = complex(a), complex(b)
    r = abs(a - b) / max(1.0, abs(a), abs(b))
    return r if r == r else 1.0


def _diff(want, got):
    return max((_d(want[k], got[k]) for k in got if k in want), default=0.0)


# ========================================================================================= question reading ====
_LIST = re.compile(r"\[[^\[\]]*\]")
_WORD_NUMBER = re.compile(r"\b(?:two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|twenty|thirty|hundred|"
                          r"half)\b")
_GLOBAL_IGNORE = [r"\b(?:s?arima|arma|ar|ma)\s*\(\s*[\d\s,]*\)", r"\b[12]-?d\b", r"holt-winters"]


def _norm(q: str) -> str:
    q = q.lower()
    for a, b in (("−", "-"), ("–", "-"), ("—", "-"), ("α", " alpha "), ("β", " beta "), ("γ", " gamma "),
                 ("φ", " phi "), ("ϕ", " phi "), ("θ", " theta "), ("σ²", " sigma2 "), ("σ^2", " sigma2 "),
                 ("σ", " sigma "), ("ρ", " rho "), ("λ", " lambda "), ("μ", " mu "), ("’", "'")):
        q = q.replace(a, b)
    return " ".join(q.split())


def _route(q: str) -> set:
    hits = [(prio, key) for key, need, avoid, prio in _ROUTES
            if all(n.search(q) for n in need) and not any(a.search(q) for a in avoid)]
    if not hits:
        return set()
    best = max(p for p, _k in hits)
    return {k for p, k in hits if p == best}


def _make_parse(key, name, inputs, aliases, labels, extract, ignore, word_ok=None):
    vecs = [i for i in inputs if i.get("type") == "vector"]
    scal = [i for i in inputs if i.get("type", "number") in ("number", "integer")]

    def parse(question):
        if not isinstance(question, str):
            return None
        q = _norm(question)
        if key not in _route(q):
            return None
        if _WORD_NUMBER.search(re.sub(word_ok, " ", _LIST.sub(" ", q)) if word_ok else _LIST.sub(" ", q)):
            return {"_out_of_scope": "a number written in words; write every number in digits"}
        d = {}
        lists = list(_LIST.finditer(q))
        if len(lists) > len(vecs):
            return {"_out_of_scope": f"{len(lists)} lists were given; {name} takes {len(vecs)}"}
        if len(vecs) == 1 and lists:
            d[vecs[0]["symbol"]] = lists[0].group(0)
        elif lists:
            prev, used = 0, set()
            for m in lists:
                seg = q[max(prev, m.start() - 70):m.start()]
                prev = m.end()
                best = None
                for v in vecs:
                    for mm in re.finditer(labels[v["symbol"]], seg):
                        if best is None or mm.end() > best[0]:
                            best = (mm.end(), v["symbol"])
                if best is None or best[1] in used:
                    return None                      # which list is which is not stated: never guess
                used.add(best[1])
                d[best[1]] = m.group(0)
        rest = _LIST.sub(lambda m: " " * len(m.group(0)), q)
        spans = []
        for i in scal:
            s = i["symbol"]
            for pat in aliases.get(s, [rf"\b{re.escape(s)}\s*(?:=|:)\s*({_NUM})"]):
                m = re.search(pat, rest)
                if m and not any(a < m.end() and m.start() < b for a, b in spans):
                    d[s] = float(m.group(1))
                    spans.append((m.start(), m.end()))
                    break
        if extract is not None:
            extra = extract(rest, d, spans)
            if isinstance(extra, dict) and "_out_of_scope" in extra:
                return extra
        left = list(rest)
        for a, b in spans:
            left[a:b] = " " * (b - a)
        left = "".join(left)
        for pat in list(ignore) + _GLOBAL_IGNORE:
            left = re.sub(pat, " ", left)
        stray = re.search(r"\d+(?:\.\d+)?", left)
        if stray:
            return {"_out_of_scope": f"the number {stray.group(0)} is not an input of {name}"}
        return d
    return parse


# ================================================================================================ the builder ====
def _vin(sym, meaning, min_length=2, required=True, max_length=None):
    d = {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
         "min_length": min_length}
    if max_length:
        d["max_length"] = max_length
    return d


def _nin(sym, meaning, required=True):
    return {"symbol": sym, "kind": "number", "meaning": meaning, "required": required}


def _iin(sym, meaning, required=True, lo=0, hi=None):
    d = {"symbol": sym, "type": "integer", "kind": "number", "meaning": meaning, "required": required, "min": lo}
    if hi is not None:
        d["max"] = hi
    return d


def _cin(sym, meaning, choices, required=False):
    return {"symbol": sym, "type": "choice", "kind": "number", "meaning": meaning, "required": required,
            "choices": list(choices)}


def _out(sym, meaning):
    return {"symbol": sym, "kind": "number", "meaning": meaning}


def _q(text, ex=None, outcome=None, outputs=None):
    return {"q": text, "ex": ex, "outcome": outcome, "outputs": outputs}


def _cap(key, *, level, name, topic, purpose, inputs, outputs, primary, method, latex, run, reference, wrong, rnd,
         need, templates, examples, questions, conditions=(), avoid=(), prio=5, aliases=None, labels=None,
         extract=None, ignore=(), keywords=(), word_ok=None):
    exp = _EXPECTED.get(key, [])
    exs = [{"inputs": e, "outputs": dict(exp[i]) if i < len(exp) else {}} for i, e in enumerate(examples)]
    qs = []
    for q in questions:
        if q["outcome"]:
            qs.append({"q": q["q"], "outcome": q["outcome"]})
        elif q["outputs"] is not None:
            qs.append({"q": q["q"], "outputs": q["outputs"]})
        else:
            qs.append({"q": q["q"], "outputs": dict(exs[q["ex"]]["outputs"]) if q["ex"] is not None else {}})

    def safe_wrong(**kw):
        try:
            return wrong(**kw)
        except (ValueError, ZeroDivisionError, OverflowError, ArithmeticError):
            return {primary: 1.0e300}
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": list(conditions), "method": method, "latex": latex, "run": run,
        "verify": (lambda i, o, _r=run: _diff(_r(**i), o)),
        "parse": _make_parse(key, name, inputs, aliases or {}, labels or {}, extract, ignore, word_ok),
        "reference": reference, "wrong": safe_wrong, "random_inputs": rnd, "templates": list(templates),
        "examples": exs, "questions": qs, "keywords": list(keywords)}
    LEVELS[key] = level
    _ROUTES.append((key, [re.compile(p) for p in ([need] if isinstance(need, str) else need)],
                    [re.compile(p) for p in ([avoid] if isinstance(avoid, str) else avoid)], prio))


# ================================================================================================ random data ====
def _walk(rng, n, dec=2):
    start = rng.uniform(20, 500)
    drift = rng.uniform(-0.02, 0.04) * start
    sd = rng.uniform(0.01, 0.06) * start
    x, v = [], start
    for _ in range(n):
        v += drift + rng.gauss(0, sd)
        if v <= 0.05 * start:
            v = 0.05 * start + abs(rng.gauss(0, sd))
        x.append(round(v, dec))
    return x


def _ar1(rng, n, phi=None, dec=3):
    phi = rng.uniform(-0.8, 0.9) if phi is None else phi
    mu, sd = rng.uniform(-10, 50), rng.uniform(0.5, 5)
    v, x = mu, []
    for _ in range(n):
        v = mu + phi * (v - mu) + rng.gauss(0, sd)
        x.append(round(v, dec))
    return x


def _seasonal(rng, n, m, mult=False, dec=2):
    level = rng.uniform(50, 400)
    slope = rng.uniform(-0.01, 0.02) * level
    amp = [rng.uniform(-0.25, 0.25) for _ in range(m)]
    ma = sum(amp) / m
    amp = [a - ma for a in amp]
    out = []
    for t in range(n):
        base = level + slope * t
        v = base * (1 + amp[t % m]) if mult else base + level * amp[t % m]
        v += rng.gauss(0, 0.02 * level)
        out.append(round(max(v, 1.0), dec))
    return out


def _any(rng, lo=8, hi=40, positive=False):
    n = rng.randint(lo, hi)
    kinds = [lambda: _walk(rng, n), lambda: _seasonal(rng, n, rng.choice([4, 12]))]
    if not positive:
        kinds.append(lambda: _ar1(rng, n))
    return rng.choice(kinds)()


def _rand_pacf_rho(rng, p):
    """A valid autocorrelation sequence rho_1..rho_p (from partial autocorrelations in (-0.85, 0.85)), 3 decimals."""
    for _ in range(100):
        ks = [rng.uniform(-0.85, 0.85) for _ in range(p)]
        rho, phi, v = [1.0], [], 1.0
        for m, k in enumerate(ks, 1):
            rho.append(math.fsum(phi[j] * rho[m - 1 - j] for j in range(m - 1)) + k * v)
            phi = [phi[j] - k * phi[m - 2 - j] for j in range(m - 1)] + [k]
            v *= 1 - k * k
        r = [round(x, 3) for x in rho[1:]]
        try:
            _levinson([1.0] + r, p)
            return r
        except ValueError:
            continue
    return [0.5] + [0.0] * (p - 1)


def _rand_stationary(rng, p, lo=-0.8, hi=0.8, dec=2):
    for _ in range(200):
        phi = [round(rng.uniform(lo, hi), dec) for _ in range(p)]
        ks = _reflection(phi)
        if all(abs(k) < 0.92 for k in ks):
            return phi
    return [0.5] + [0.0] * (p - 1)


# ================================================================================ shared question vocabulary ====
_W = r"(?:=|:|of|is|was|were|are|equals?|equal to)?\s*"
A_WINDOW = [r"(\d+)[- ](?:period|point|term|day|month|week|year|quarter|observation|value)s?\b",
            r"window(?: size| length| width)?\s*" + _W + r"(\d+)", r"\bk\s*=\s*(\d+)", r"\bspan\s*" + _W + r"(\d+)"]
A_LAG = [r"\blag\s*k?\s*(?:=|:|of)?\s*(-?\d+)", r"\bk\s*=\s*(\d+)", r"(\d+)(?:st|nd|rd|th)?[- ]lag\b"]
A_LAGS = [r"(\d+)\s+lags\b", r"\bh\s*=\s*(\d+)", r"\blags?\s*(?:=|:|of)?\s*(\d+)", r"up to lag\s*(\d+)",
          r"\bm\s*=\s*(\d+)"]
A_H = [r"(\d+)[- ](?:steps?|periods?|years?|months?|quarters?|weeks?|days?)[- ]ahead", r"\bh\s*=\s*(\d+)",
       r"horizon\s*" + _W + r"(\d+)", r"next\s+(\d+)\s+(?:periods|values|steps|observations|months|quarters|years)"]
A_SEASON = [r"season(?:al)? (?:length|period|cycle)\s*" + _W + r"(\d+)", r"\bperiod\s*" + _W + r"(\d+)",
            r"\bm\s*=\s*(\d+)", r"(\d+)\s+seasons\b", r"\bs\s*=\s*(\d+)"]


def _nm(*names):
    """Aliases for a scalar written as 'name = value' (also 'name of value', 'name is value', 'name value')."""
    return [r"\b" + n + r"\s*" + _W + "(" + _NUM + ")" for n in names]


def _season_words(rest, d, spans, key="m"):
    if key not in d:
        if re.search(r"\bquarterly\b", rest):
            d[key] = 4.0
        elif re.search(r"\bmonthly\b", rest):
            d[key] = 12.0


_SERIES_LABEL = r"\bof\b|series|data|values|observations|history|measurements|sales|demand"


# =================================================================================== moving averages ====
_MA = "moving averages"


def run_sma(x, k):
    x = _fin(x)
    k = _int(k, "the window k", 1)
    if k > len(x):
        raise ValueError("the window k must not exceed the number of values")
    s = [math.fsum(x[i:i + k]) / k for i in range(len(x) - k + 1)]
    return {"sma": s, "last": s[-1], "count": len(s)}


def ref_sma(x, k):
    """Cumulative-sum route: window sums as differences of running totals."""
    import numpy as np
    c = np.concatenate([[0.0], np.cumsum(np.asarray(x, dtype=float))])
    s = (c[k:] - c[:-k]) / k
    return {"sma": [float(v) for v in s], "last": float(s[-1]), "count": len(s)}


def wrong_sma(x, k):          # the window ends one value too early (the latest value left out)
    y = x[:-1] if len(x) > k else x
    s = [sum(y[i:i + k]) / k for i in range(len(y) - k + 1)] or [sum(x) / len(x)]
    return {"sma": s, "last": s[-1], "count": len(s)}


_cap("simple_moving_average", level="beginner", name="Simple moving average", topic=_MA,
     purpose="The k-period simple (trailing) moving average of a series; its last value is the moving-average "
             "forecast of the next period.",
     inputs=[_vin("x", "the series, oldest first"), _iin("k", "the window length (number of periods)", lo=1)],
     outputs=[_out("sma", "the moving averages, one per complete window (the first covers x_1..x_k)"),
              _out("last", "the latest moving average (forecast of the next period)"),
              _out("count", "number of moving averages (n - k + 1)")],
     primary="last", conditions=["1 <= k <= n"], method="mean of each window of k consecutive values",
     latex=r"\mathrm{SMA}_t = \frac{1}{k}\sum_{i=0}^{k-1} x_{t-i}", run=run_sma, reference=ref_sma, wrong=wrong_sma,
     rnd=lambda rng: (lambda x: {"x": x, "k": rng.randint(1, len(x))})(_any(rng, 3, 40)),
     need=r"\bmoving average|\bsma\b|rolling (?:mean|average)",
     avoid=r"weighted|exponential|cent(?:red|ered)|\bema\b|process|model|\bma\(|theta|arima|arma|2\s*x",
     aliases={"k": A_WINDOW}, prio=4,
     templates=["Find the 3-period simple moving average of [10, 12, 14, 13, 15].",
                "Compute the moving average with window 4 of [5, 7, 6, 8, 9, 10]."],
     examples=[dict(x="10, 12, 14, 13, 15", k=3), dict(x=[5, 7, 6, 8, 9, 10], k=4), dict(x="2.5, 3.5", k=1)],
     questions=[_q("Find the 3-period simple moving average of [10, 12, 14, 13, 15].", 0),
                _q("Compute the moving average with window 4 of [5, 7, 6, 8, 9, 10].", 1),
                _q("Find the 6-period moving average of [1, 2, 3].", outcome="INVALID_INPUT"),
                _q("Find the 3-period moving average.", outcome="NEEDS_INPUT"),
                _q("Find the moving average of [1, 2, 3, 4].", outcome="NEEDS_INPUT")])


def run_lwma(x, k):
    x = _fin(x)
    k = _int(k, "the window k", 1)
    if k > len(x):
        raise ValueError("the window k must not exceed the number of values")
    den = k * (k + 1) / 2
    s = [math.fsum((j + 1) * x[i + j] for j in range(k)) / den for i in range(len(x) - k + 1)]
    return {"lwma": s, "last": s[-1]}


def ref_lwma(x, k):
    """Recursive route: numerator N_t = N_{t-1} + k x_t - T_{t-1}, T the running window total."""
    import numpy as np
    x = np.asarray(x, dtype=float)
    num = float(np.dot(np.arange(1, k + 1), x[:k]))
    tot = float(x[:k].sum())
    out = [num]
    for t in range(k, len(x)):
        num = num + k * x[t] - tot
        tot = tot + x[t] - x[t - k]
        out.append(num)
    s = [v / (k * (k + 1) / 2) for v in out]
    return {"lwma": s, "last": s[-1]}


def wrong_lwma(x, k):         # weights reversed: the oldest value gets weight k
    den = k * (k + 1) / 2
    s = [sum((k - j) * x[i + j] for j in range(k)) / den for i in range(len(x) - k + 1)]
    return {"lwma": s, "last": s[-1]}


_cap("linearly_weighted_moving_average", level="beginner", name="Linearly weighted moving average", topic=_MA,
     purpose="The k-period linearly weighted moving average: weights 1, 2, ..., k from the oldest to the newest "
             "value of each window.",
     inputs=[_vin("x", "the series, oldest first"), _iin("k", "the window length", lo=1)],
     outputs=[_out("lwma", "the weighted moving averages, one per complete window"),
              _out("last", "the latest weighted moving average (forecast of the next period)")],
     primary="last", conditions=["1 <= k <= n", "the newest value of a window has weight k, the oldest weight 1"],
     method="weighted mean of each window with weights 1..k divided by k(k+1)/2",
     latex=r"\mathrm{WMA}_t = \frac{\sum_{i=0}^{k-1} (k-i)\,x_{t-i}}{k(k+1)/2}", run=run_lwma, reference=ref_lwma,
     wrong=wrong_lwma, rnd=lambda rng: (lambda x: {"x": x, "k": rng.randint(2, len(x))})(_any(rng, 3, 40)),
     need=r"linear(?:ly)?[- ]weighted moving average|\blwma\b", aliases={"k": A_WINDOW},
     templates=["Find the 3-period linearly weighted moving average of [10, 12, 14, 13, 15]."],
     examples=[dict(x="10, 12, 14, 13, 15", k=3), dict(x=[20, 22, 21, 25], k=4)],
     questions=[_q("Find the 3-period linearly weighted moving average of [10, 12, 14, 13, 15].", 0),
                _q("Compute the linearly weighted moving average with window 4 of [20, 22, 21, 25].", 1),
                _q("Find the linearly weighted moving average of [1, 2, 3].", outcome="NEEDS_INPUT")])


def run_ema(x, k):
    x = _fin(x)
    k = _int(k, "the span k", 1)
    a = 2.0 / (k + 1)
    e, out = x[0], [x[0]]
    for v in x[1:]:
        e = a * v + (1 - a) * e
        out.append(e)
    return {"ema": out, "last": out[-1], "alpha": a}


def ref_ema(x, k):
    """Closed form: e_t = (1-a)^(t-1) x_1 + sum_{j=2..t} a (1-a)^(t-j) x_j."""
    import numpy as np
    x = np.asarray(x, dtype=float)
    a = 2.0 / (k + 1)
    out = []
    for t in range(len(x)):
        w = a * (1 - a) ** np.arange(t - 1, -1, -1.0)
        out.append(float((1 - a) ** t * x[0] + np.dot(w, x[1:t + 1])))
    return {"ema": out, "last": out[-1], "alpha": a}


def wrong_ema(x, k):          # alpha = 1/k instead of 2/(k+1)
    a = 1.0 / k
    e, out = x[0], [x[0]]
    for v in x[1:]:
        e = a * v + (1 - a) * e
        out.append(e)
    return {"ema": out, "last": out[-1], "alpha": a}


_cap("exponential_moving_average", level="beginner", name="Exponential moving average", topic=_MA,
     purpose="The exponential moving average with span k (alpha = 2/(k+1)), started at the first value.",
     inputs=[_vin("x", "the series, oldest first"), _iin("k", "the span (number of periods)", lo=1)],
     outputs=[_out("ema", "the exponential moving average at every time (EMA_1 = x_1)"),
              _out("last", "the latest exponential moving average"), _out("alpha", "the weight 2/(k+1)")],
     primary="last", conditions=["k >= 1", "EMA_1 = x_1 (no averaging of a start-up window)"],
     method="EMA_t = alpha x_t + (1 - alpha) EMA_{t-1}, alpha = 2/(k+1)",
     latex=r"\mathrm{EMA}_t = \alpha x_t + (1-\alpha)\,\mathrm{EMA}_{t-1},\quad \alpha = \frac{2}{k+1}",
     run=run_ema, reference=ref_ema, wrong=wrong_ema,
     rnd=lambda rng: {"x": _any(rng, 2, 40), "k": rng.randint(2, 30)},
     need=r"exponential(?:ly weighted)? moving average|\bema\b|\bewma\b", avoid=r"smoothing",
     aliases={"k": A_WINDOW},
     templates=["Find the 3-period exponential moving average of [10, 12, 14, 13, 15]."],
     examples=[dict(x="10, 12, 14, 13, 15", k=3), dict(x=[100, 102, 101, 105, 107, 106], k=5)],
     questions=[_q("Find the 3-period exponential moving average of [10, 12, 14, 13, 15].", 0),
                _q("Compute the EMA with span 5 of [100, 102, 101, 105, 107, 106].", 1),
                _q("Find the exponential moving average of [1, 2, 3].", outcome="NEEDS_INPUT")])


def run_cma(x, k):
    x = _fin(x)
    k = _int(k, "the window k", 2)
    h = k // 2
    if (k % 2 and k > len(x)) or (k % 2 == 0 and k + 1 > len(x)):
        raise ValueError("the series is too short for a centred moving average of this length")
    if k % 2:
        vals = [math.fsum(x[i - h:i + h + 1]) / k for i in range(h, len(x) - h)]
    else:
        vals = [(0.5 * x[i - h] + math.fsum(x[i - h + 1:i + h]) + 0.5 * x[i + h]) / k for i in range(h, len(x) - h)]
    return {"cma": vals, "first_index": h + 1, "first": vals[0]}


def ref_cma(x, k):
    """Odd k: running-total windows; even k: the mean of two consecutive trailing k-term moving averages."""
    import numpy as np
    c = np.concatenate([[0.0], np.cumsum(np.asarray(x, dtype=float))])
    ma = (c[k:] - c[:-k]) / k
    vals = ma if k % 2 else (ma[:-1] + ma[1:]) / 2
    return {"cma": [float(v) for v in vals], "first_index": k // 2 + 1, "first": float(vals[0])}


def wrong_cma(x, k):          # a trailing k-term average reported as if centred
    vals = [sum(x[i:i + k]) / k for i in range(len(x) - k + 1)]
    return {"cma": vals, "first_index": k, "first": vals[0]}


def _x_cma(rest, d, spans):
    m = re.search(r"2\s*x\s*(\d+)", rest)
    if m and "k" not in d:
        d["k"] = float(m.group(1))
        spans.append((m.start(), m.end()))


_cap("centred_moving_average", level="beginner", name="Centred moving average", topic=_MA,
     purpose="The centred moving average of length k: a symmetric k-term mean for odd k, the 2 x k moving average "
             "(half weights at both ends) for even k, as used in classical decomposition.",
     inputs=[_vin("x", "the series, oldest first", 3), _iin("k", "the length of the moving average", lo=2)],
     outputs=[_out("cma", "the centred moving averages, from time first_index on"),
              _out("first_index", "the time t (1-based) of the first centred value (k//2 + 1)"),
              _out("first", "the first centred moving average")],
     primary="first", conditions=["k >= 2", "odd k: k <= n; even k: k + 1 <= n"],
     method="odd k: mean of x_{t-h}..x_{t+h}; even k: (x_{t-h}/2 + x_{t-h+1} + ... + x_{t+h-1} + x_{t+h}/2)/k, h = k//2",
     latex=r"\mathrm{CMA}_t = \frac{1}{k}\Big(\tfrac12 x_{t-h} + \sum_{i=-h+1}^{h-1} x_{t+i} + \tfrac12 x_{t+h}\Big)",
     run=run_cma, reference=ref_cma, wrong=wrong_cma,
     rnd=lambda rng: (lambda x: {"x": x, "k": rng.randint(2, len(x) - 1)})(_any(rng, 5, 40)),
     need=r"cent(?:red|ered) moving average|2\s*x\s*\d+\s*(?:moving average|ma)\b", avoid=r"process|model",
     aliases={"k": A_WINDOW}, extract=_x_cma,
     templates=["Find the 4-period centred moving average of [12, 15, 14, 18, 20, 19, 23, 25].",
                "Compute the 3-period centered moving average of [5, 8, 6, 9, 7]."],
     examples=[dict(x="12, 15, 14, 18, 20, 19, 23, 25", k=4), dict(x=[5, 8, 6, 9, 7], k=3)],
     questions=[_q("Find the 4-period centred moving average of [12, 15, 14, 18, 20, 19, 23, 25].", 0),
                _q("Compute the 3-period centered moving average of [5, 8, 6, 9, 7].", 1),
                _q("Find the 2x4 moving average of [12, 15, 14, 18, 20, 19, 23, 25].", 0),
                _q("Find the 4-period centred moving average of [1, 2, 3, 4].", outcome="INVALID_INPUT")])


# ============================================================================ exponential smoothing ====
_ES = "exponential smoothing"
_ALPHA = _nm("alpha", r"level smoothing (?:constant|parameter)", r"smoothing (?:constant|parameter|factor)")
_BETA = _nm("beta", r"trend smoothing (?:constant|parameter)")
_GAMMA = _nm("gamma", r"seasonal smoothing (?:constant|parameter)")
_L0 = _nm("initial level", "l0", "l_0", "initial forecast")
_B0 = _nm("initial trend", "b0", "b_0")


def run_ses(x, alpha, l0=None):
    x = _fin(x)
    _need(x, 1)
    alpha = _unit(alpha, "alpha")
    lev = x[0] if l0 is None else _num(l0, "the initial level")
    sse = 0.0
    for v in x:
        sse += (v - lev) ** 2
        lev = lev + alpha * (v - lev)
    return {"forecast": lev, "sse": sse}


def ref_ses(x, alpha, l0=None):
    import numpy as np
    from statsmodels.tsa.holtwinters import SimpleExpSmoothing
    y = np.asarray(x, dtype=float)
    lev = float(y[0]) if l0 is None else float(l0)
    f = SimpleExpSmoothing(y, initialization_method="known", initial_level=lev).fit(
        smoothing_level=alpha, optimized=False)
    return {"forecast": float(f.forecast(1)[0]), "sse": float(f.sse)}


def wrong_ses(x, alpha, l0=None):  # alpha and 1 - alpha swapped
    lev = x[0] if l0 is None else l0
    sse = 0.0
    for v in x:
        sse += (v - lev) ** 2
        lev = alpha * lev + (1 - alpha) * v
    return {"forecast": lev, "sse": sse}


def _rnd_ses(rng):
    d = {"x": _any(rng, 3, 40), "alpha": round(rng.uniform(0.05, 0.95), 2)}
    if rng.random() < 0.4:
        d["l0"] = round(d["x"][0] + rng.gauss(0, 2), 2)
    return d


_cap("simple_exponential_smoothing", level="beginner", name="Simple exponential smoothing", topic=_ES,
     purpose="Simple exponential smoothing with smoothing constant alpha: the forecast of the next period and the "
             "sum of squared one-step errors. The initial level l_0 is the first observation unless stated.",
     inputs=[_vin("x", "the series, oldest first", 1), _nin("alpha", "the smoothing constant, 0 < alpha <= 1"),
             _nin("l0", "the initial level l_0 before the first observation (default: x_1)", required=False)],
     outputs=[_out("forecast", "the forecast of the next period (the final level l_n)"),
              _out("sse", "sum of squared one-step errors, e_t = x_t - l_{t-1}")],
     primary="forecast", conditions=["0 < alpha <= 1", "l_0 = x_1 unless an initial level is stated"],
     method="l_t = l_{t-1} + alpha (x_t - l_{t-1}); forecast = l_n",
     latex=r"\ell_t = \alpha x_t + (1-\alpha)\,\ell_{t-1},\qquad \hat x_{n+1} = \ell_n",
     run=run_ses, reference=ref_ses, wrong=wrong_ses, rnd=_rnd_ses,
     need=r"(?:simple |single )?exponential smoothing|\bses\b",
     avoid=r"holt|winters|trend|season|double|triple|moving average|damp|beta|gamma|previous forecast|last forecast|"
           r"smallest|best|minimi|optimal|lowest",
     aliases={"alpha": _ALPHA, "l0": _L0},
     templates=["Apply simple exponential smoothing with alpha = 0.3 to [20, 22, 21, 24, 23]."],
     examples=[dict(x="20, 22, 21, 24, 23", alpha=0.3), dict(x=[50, 52, 49, 55, 53, 56], alpha=0.5, l0=51)],
     questions=[_q("Apply simple exponential smoothing with alpha = 0.3 to [20, 22, 21, 24, 23].", 0),
                _q("Use exponential smoothing with alpha = 0.5 and initial level 51 on [50, 52, 49, 55, 53, 56].", 1),
                _q("Apply simple exponential smoothing with alpha = 1.5 to [1, 2, 3].", outcome="INVALID_INPUT"),
                _q("Apply simple exponential smoothing to [1, 2, 3].", outcome="NEEDS_INPUT")])


def _holt_core(x, alpha, beta, l0, b0, h, phi=1.0, wrong=False):
    lev, tr, sse = l0, b0, 0.0
    for v in x:
        f = lev + phi * tr
        sse += (v - f) ** 2
        new = alpha * v + (1 - alpha) * f
        tr = beta * ((v if wrong else new) - lev) + (1 - beta) * phi * tr
        lev = new
    damp = math.fsum(phi ** j for j in range(1, h + 1))
    return {"forecast": lev + damp * tr, "level": lev, "trend": tr, "sse": sse}


def _holt_inputs(x, alpha, beta, l0, b0, h):
    x = _fin(x)
    _need(x, 2)
    alpha, beta = _unit(alpha, "alpha"), _unit(beta, "beta")
    l0 = x[0] if l0 is None else _num(l0, "the initial level")
    b0 = x[1] - x[0] if b0 is None else _num(b0, "the initial trend")
    return x, alpha, beta, l0, b0, _int(h, "the horizon h", 1, 1000)


def run_holt(x, alpha, beta, l0=None, b0=None, h=1):
    return _holt_core(*_holt_inputs(x, alpha, beta, l0, b0, h))


def ref_holt(x, alpha, beta, l0=None, b0=None, h=1, phi=None):
    import numpy as np
    from statsmodels.tsa.holtwinters import Holt
    y = np.asarray(x, dtype=float)
    l0 = float(y[0]) if l0 is None else float(l0)
    b0 = float(y[1] - y[0]) if b0 is None else float(b0)
    m = Holt(y, damped_trend=phi is not None, initialization_method="known", initial_level=l0, initial_trend=b0)
    kw = {"damping_trend": phi} if phi is not None else {}
    f = m.fit(smoothing_level=alpha, smoothing_trend=beta, optimized=False, **kw)
    return {"forecast": float(f.forecast(int(h))[-1]), "level": float(f.level[-1]), "trend": float(f.trend[-1]),
            "sse": float(f.sse)}


def _rnd_holt(rng, damped=False):
    d = {"x": _walk(rng, rng.randint(4, 40)), "alpha": round(rng.uniform(0.05, 0.95), 2),
         "beta": round(rng.uniform(0.02, 0.6), 2), "h": rng.randint(1, 8)}
    if damped:
        d["phi"] = round(rng.uniform(0.7, 0.98), 2)
    if rng.random() < 0.3:
        d["l0"], d["b0"] = round(d["x"][0], 1), round(rng.gauss(0, 3), 2)
    return d


_HOLT_IN = [_vin("x", "the series, oldest first"), _nin("alpha", "level smoothing constant, 0 < alpha <= 1"),
            _nin("beta", "trend smoothing constant, 0 < beta <= 1 (b_t = beta (l_t - l_{t-1}) + (1 - beta) b_{t-1})")]
_HOLT_OPT = [_nin("l0", "initial level l_0 (default x_1)", required=False),
             _nin("b0", "initial trend b_0 (default x_2 - x_1)", required=False),
             _iin("h", "forecast horizon in periods (default 1)", required=False, lo=1)]
_HOLT_OUT = [_out("forecast", "the h-step-ahead forecast"), _out("level", "the final level l_n"),
             _out("trend", "the final trend b_n"), _out("sse", "sum of squared one-step errors")]

_cap("holt_linear_trend", level="intermediate", name="Holt's linear trend method", topic=_ES,
     purpose="Holt's linear (double) exponential smoothing: level and trend updated each period, h-step forecast "
             "l_n + h b_n. Initial states l_0 = x_1 and b_0 = x_2 - x_1 (before the first observation) unless stated.",
     inputs=_HOLT_IN + _HOLT_OPT, outputs=_HOLT_OUT, primary="forecast",
     conditions=["0 < alpha, beta <= 1", "l_0 = x_1, b_0 = x_2 - x_1 unless stated", "trend form: beta multiplies "
                 "(l_t - l_{t-1}) (the error-correction beta* of many texts)"],
     method="l_t = alpha x_t + (1-alpha)(l_{t-1}+b_{t-1}); b_t = beta (l_t - l_{t-1}) + (1-beta) b_{t-1}",
     latex=r"\ell_t=\alpha x_t+(1-\alpha)(\ell_{t-1}+b_{t-1}),\ b_t=\beta(\ell_t-\ell_{t-1})+(1-\beta)b_{t-1},\ "
           r"\hat x_{n+h}=\ell_n+h\,b_n",
     run=run_holt, reference=ref_holt,
     wrong=lambda x, alpha, beta, l0=None, b0=None, h=1: _holt_core(*_holt_inputs(x, alpha, beta, l0, b0, h), wrong=True),
     rnd=_rnd_holt, need=r"\bholt\b|double exponential smoothing",
     avoid=r"winters|season|damp|triple|(?:previous|last|current) level",
     aliases={"alpha": _ALPHA, "beta": _BETA, "l0": _L0, "b0": _B0, "h": A_H},
     templates=["Apply Holt's linear trend method with alpha = 0.5 and beta = 0.3 to [10, 12, 15, 16, 19, 21]."],
     examples=[dict(x="10, 12, 15, 16, 19, 21", alpha=0.5, beta=0.3),
               dict(x=[100, 104, 109, 111, 118, 121, 127], alpha=0.4, beta=0.2, h=3),
               dict(x="30, 33, 35, 39", alpha=0.6, beta=0.1, l0=29, b0=2)],
     questions=[_q("Apply Holt's linear trend method with alpha = 0.5 and beta = 0.3 to [10, 12, 15, 16, 19, 21].", 0),
                _q("Use Holt's method with alpha = 0.4 and beta = 0.2 to forecast 3 periods ahead for "
                   "[100, 104, 109, 111, 118, 121, 127].", 1),
                _q("Apply Holt's method with alpha = 0.6, beta = 0.1, initial level 29 and initial trend 2 to "
                   "[30, 33, 35, 39].", 2),
                _q("Apply Holt's linear trend method with alpha = 0.5 to [10, 12, 15].", outcome="NEEDS_INPUT")])


def run_holt_damped(x, alpha, beta, phi, l0=None, b0=None, h=1):
    args = _holt_inputs(x, alpha, beta, l0, b0, h)
    return _holt_core(*args, phi=_unit(phi, "the damping parameter phi"))


_cap("holt_damped_trend", level="advanced", name="Damped-trend Holt method", topic=_ES,
     purpose="Holt's method with a damped trend (damping parameter phi): the h-step forecast is "
             "l_n + (phi + phi^2 + ... + phi^h) b_n. Initial states l_0 = x_1, b_0 = x_2 - x_1 unless stated.",
     inputs=_HOLT_IN + [_nin("phi", "damping parameter, 0 < phi <= 1")] + _HOLT_OPT, outputs=_HOLT_OUT,
     primary="forecast", conditions=["0 < alpha, beta, phi <= 1", "l_0 = x_1, b_0 = x_2 - x_1 unless stated"],
     method="l_t = alpha x_t + (1-alpha)(l_{t-1} + phi b_{t-1}); b_t = beta (l_t - l_{t-1}) + (1-beta) phi b_{t-1}",
     latex=r"\hat x_{n+h} = \ell_n + (\phi+\phi^2+\dots+\phi^h)\,b_n",
     run=run_holt_damped, reference=lambda x, alpha, beta, phi, l0=None, b0=None, h=1: ref_holt(x, alpha, beta, l0, b0, h, phi),
     wrong=lambda x, alpha, beta, phi, l0=None, b0=None, h=1: _holt_core(*_holt_inputs(x, alpha, beta, l0, b0, h), phi=1.0),
     rnd=lambda rng: _rnd_holt(rng, True), need=[r"\bholt\b|exponential smoothing", r"damp"], avoid=r"winters|season",
     aliases={"alpha": _ALPHA, "beta": _BETA, "phi": _nm("phi", r"damping (?:parameter|factor|constant)"),
              "l0": _L0, "b0": _B0, "h": A_H},
     templates=["Apply Holt's damped trend method with alpha = 0.5, beta = 0.3 and phi = 0.9 to [10, 12, 15, 16, 19, 21]."],
     examples=[dict(x="10, 12, 15, 16, 19, 21", alpha=0.5, beta=0.3, phi=0.9),
               dict(x=[100, 104, 109, 111, 118, 121, 127], alpha=0.4, beta=0.2, phi=0.85, h=5)],
     questions=[_q("Apply Holt's damped trend method with alpha = 0.5, beta = 0.3 and phi = 0.9 to "
                   "[10, 12, 15, 16, 19, 21].", 0),
                _q("Forecast 5 periods ahead with damped Holt smoothing, alpha = 0.4, beta = 0.2, phi = 0.85, for "
                   "[100, 104, 109, 111, 118, 121, 127].", 1),
                _q("Apply Holt's damped trend method with alpha = 0.5, beta = 0.3 to [10, 12, 15].", outcome="NEEDS_INPUT")])


def _hw_init(x, m, mult):
    l0 = math.fsum(x[:m]) / m
    b0 = (math.fsum(x[m:2 * m]) / m - l0) / m
    s = [v / l0 for v in x[:m]] if mult else [v - l0 for v in x[:m]]
    return l0, b0, s


def _hw(x, m, alpha, beta, gamma, h, mult, wrong=False):
    x = _fin(x)
    m = _int(m, "the season length m", 2, 52)
    _need(x, 2 * m, "the series (two full seasons)")
    alpha, beta, gamma = _unit(alpha, "alpha"), _unit(beta, "beta"), _unit(gamma, "gamma")
    h = _int(h, "the horizon h", 1, 1000)
    if mult:
        _positive(x)
    lev, tr, s = _hw_init(x, m, mult)
    if mult and lev <= 0:
        raise ValueError("the first season's mean must be positive")
    sse = 0.0
    for t, v in enumerate(x):
        base = lev + tr
        f = base * s[t] if mult else base + s[t]
        sse += (v - f) ** 2
        if wrong:
            new = alpha * v + (1 - alpha) * base
        else:
            new = alpha * (v / s[t] if mult else v - s[t]) + (1 - alpha) * base
        s.append(gamma * (v / base if mult else v - base) + (1 - gamma) * s[t])
        tr = beta * (new - lev) + (1 - beta) * tr
        lev = new
    n = len(x)
    seas = s[n + (h - 1) % m]
    fc = (lev + h * tr) * seas if mult else lev + h * tr + seas
    return {"forecast": fc, "level": lev, "trend": tr, "sse": sse}


def _ref_hw(x, m, alpha, beta, gamma, h, mult):
    import numpy as np
    from statsmodels.tsa.holtwinters import ExponentialSmoothing
    y = np.asarray(x, dtype=float)
    m = int(m)
    l0 = y[:m].mean()
    b0 = (y[m:2 * m].mean() - l0) / m
    s0 = y[:m] / l0 if mult else y[:m] - l0
    mod = ExponentialSmoothing(y, trend="add", seasonal="mul" if mult else "add", seasonal_periods=m,
                               initialization_method="known", initial_level=l0, initial_trend=b0, initial_seasonal=s0)
    f = mod.fit(smoothing_level=alpha, smoothing_trend=beta, smoothing_seasonal=gamma, optimized=False)
    # statsmodels' own forecast() reuses the PREVIOUS cycle's seasonal when h = m; the textbook forecast uses the
    # latest seasonal state of that season, built here from statsmodels' fitted states
    lev, tr, seas = float(f.level[-1]), float(f.trend[-1]), np.asarray(f.season, dtype=float)
    s = float(seas[len(y) - m + (int(h) - 1) % m])
    fc = (lev + int(h) * tr) * s if mult else lev + int(h) * tr + s
    return {"forecast": fc, "level": lev, "trend": tr, "sse": float(f.sse)}


def _rnd_hw(rng, mult):
    m = rng.choice([4, 4, 12, 3, 6])
    a = round(rng.uniform(0.05, 0.8), 2)
    return {"x": _seasonal(rng, rng.randint(2 * m, 4 * m + 4), m, mult), "m": m, "alpha": a,
            "beta": round(rng.uniform(0.02, 0.5), 2), "gamma": round(rng.uniform(0.02, max(0.03, 0.95 - a)), 2),
            "h": rng.randint(1, 2 * m)}


_HW_IN = [_vin("x", "the series, oldest first (at least two full seasons)", 4), _iin("m", "season length", lo=2),
          _nin("alpha", "level smoothing constant"), _nin("beta", "trend smoothing constant"),
          _nin("gamma", "seasonal smoothing constant"),
          _iin("h", "forecast horizon in periods (default 1)", required=False, lo=1)]
_HW_INIT = ("initialization: l_0 = mean of the first season, b_0 = (mean of season 2 - mean of season 1)/m, "
            "s_j = x_j - l_0 (additive) or x_j / l_0 (multiplicative) for the first m observations")
_HW_ALIASES = {"alpha": _ALPHA, "beta": _BETA, "gamma": _GAMMA, "m": A_SEASON, "h": A_H}

for _mult, _key, _lvl, _word in ((False, "holt_winters_additive", "advanced", "additive"),
                                 (True, "holt_winters_multiplicative", "advanced", "multiplicative")):
    _cap(_key, level=_lvl, name=f"Holt-Winters {_word} method", topic=_ES,
         purpose=f"Holt-Winters exponential smoothing with additive trend and {_word} seasonality: the h-step "
                 f"forecast, final level and trend, and the sum of squared one-step errors. {_HW_INIT}.",
         inputs=_HW_IN, outputs=[_out("forecast", "the h-step-ahead forecast"), _out("level", "final level l_n"),
                                 _out("trend", "final trend b_n"), _out("sse", "sum of squared one-step errors")],
         primary="forecast", conditions=["n >= 2m", "0 < alpha, beta, gamma <= 1", _HW_INIT] +
                                        (["positive data"] if _mult else []),
         method=("l_t = alpha x_t/s_{t-m} + (1-alpha)(l_{t-1}+b_{t-1}); s_t = gamma x_t/(l_{t-1}+b_{t-1}) + (1-gamma) s_{t-m}"
                 if _mult else
                 "l_t = alpha (x_t - s_{t-m}) + (1-alpha)(l_{t-1}+b_{t-1}); s_t = gamma (x_t - l_{t-1} - b_{t-1}) + (1-gamma) s_{t-m}")
                + "; b_t = beta (l_t - l_{t-1}) + (1-beta) b_{t-1}",
         latex=(r"\hat x_{n+h} = (\ell_n + h b_n)\, s_{n+h-m(\lfloor (h-1)/m\rfloor+1)}" if _mult else
                r"\hat x_{n+h} = \ell_n + h b_n + s_{n+h-m(\lfloor (h-1)/m\rfloor+1)}"),
         run=(lambda x, m, alpha, beta, gamma, h=1, _m=_mult: _hw(x, m, alpha, beta, gamma, h, _m)),
         reference=(lambda x, m, alpha, beta, gamma, h=1, _m=_mult: _ref_hw(x, m, alpha, beta, gamma, h, _m)),
         wrong=(lambda x, m, alpha, beta, gamma, h=1, _m=_mult: _hw(x, m, alpha, beta, gamma, h, _m, wrong=True)),
         rnd=(lambda rng, _m=_mult: _rnd_hw(rng, _m)),
         need=[r"holt[- ]winters|triple exponential smoothing", r"\b" + _word],
         avoid=r"multiplicative" if not _mult else r"\badditive", aliases=_HW_ALIASES, extract=_season_words,
         templates=[f"Apply the {_word} Holt-Winters method with m = 4, alpha = 0.5, beta = 0.3 and gamma = 0.2 to "
                    "[30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51]."],
         examples=[dict(x="30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51", m=4, alpha=0.5, beta=0.3, gamma=0.2),
                   dict(x=[12, 18, 25, 15, 14, 20, 28, 17, 16, 23, 31, 19], m=4, alpha=0.3, beta=0.1, gamma=0.4, h=4)],
         questions=[_q(f"Apply the {_word} Holt-Winters method with m = 4, alpha = 0.5, beta = 0.3 and gamma = 0.2 "
                       "to [30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51].", 0),
                    _q(f"Forecast 4 steps ahead with {_word} Holt-Winters for quarterly data, alpha = 0.3, beta = 0.1, "
                       "gamma = 0.4: [12, 18, 25, 15, 14, 20, 28, 17, 16, 23, 31, 19].", 1),
                    _q(f"Apply the {_word} Holt-Winters method with m = 4, alpha = 0.5, beta = 0.3 and gamma = 0.2 "
                       "to [30, 21, 29, 31, 40].", outcome="INVALID_INPUT"),
                    _q(f"Apply the {_word} Holt-Winters method with alpha = 0.5, beta = 0.3 and gamma = 0.2 to "
                       "[30, 21, 29, 31, 40, 30, 39, 41].", outcome="NEEDS_INPUT")])


# ===================================================================== classical decomposition ====
_DEC = "classical decomposition"


def _decompose(x, m, mult):
    x = _fin(x)
    m = _int(m, "the season length m", 2, 52)
    _need(x, 2 * m, "the series (two full seasons)")
    if mult:
        _positive(x)
    n, h = len(x), m // 2
    trend = [None] * n
    for t in range(h, n - h):
        if m % 2:
            trend[t] = math.fsum(x[t - h:t + h + 1]) / m
        else:
            trend[t] = (0.5 * x[t - h] + math.fsum(x[t - h + 1:t + h]) + 0.5 * x[t + h]) / m
    det = [None if trend[t] is None else (x[t] / trend[t] if mult else x[t] - trend[t]) for t in range(n)]
    avg = []
    for i in range(m):
        vals = [det[t] for t in range(i, n, m) if det[t] is not None]
        avg.append(math.fsum(vals) / len(vals))
    c = math.fsum(avg) / m
    idx = [a / c for a in avg] if mult else [a - c for a in avg]
    return x, idx, avg


def run_seasonal_indices(x, m, mult):
    _x, idx, _avg = _decompose(x, m, mult)
    return {"indices": idx, "first": idx[0]}


def ref_seasonal_indices(x, m, mult):
    import numpy as np
    from statsmodels.tsa.seasonal import seasonal_decompose
    r = seasonal_decompose(np.asarray(x, dtype=float), model="multiplicative" if mult else "additive", period=int(m))
    idx = [float(v) for v in r.seasonal[:int(m)]]
    return {"indices": idx, "first": idx[0]}


def run_adjusted(x, m, mult):
    x, idx, _avg = _decompose(x, m, mult)
    m = len(idx)
    adj = [v / idx[t % m] if mult else v - idx[t % m] for t, v in enumerate(x)]
    return {"adjusted": adj, "last": adj[-1]}


def ref_adjusted(x, m, mult):
    import numpy as np
    from statsmodels.tsa.seasonal import seasonal_decompose
    y = np.asarray(x, dtype=float)
    r = seasonal_decompose(y, model="multiplicative" if mult else "additive", period=int(m))
    adj = y / r.seasonal if mult else y - r.seasonal
    return {"adjusted": [float(v) for v in adj], "last": float(adj[-1])}


def _wrong_idx(x, m, mult):    # raw period averages, not normalised to sum 0 (mean 1)
    _x, _idx, avg = _decompose(x, m, mult)
    return {"indices": avg, "first": avg[0]}


def _wrong_adj(x, m, mult):    # seasonal effect added instead of removed
    x, idx, _avg = _decompose(x, m, mult)
    adj = [v * idx[t % len(idx)] if mult else v + idx[t % len(idx)] for t, v in enumerate(x)]
    return {"adjusted": adj, "last": adj[-1]}


_DEC_COND = ["n >= 2m", "trend: centred moving average of length m (2 x m for even m)",
             "index i belongs to the season of observation i (the list starts at the first observation's season)"]

for _mult, _word in ((False, "additive"), (True, "multiplicative")):
    _norm_word = "sum to 0" if not _mult else "average 1"
    _cap(f"seasonal_indices_{_word}", level="intermediate", name=f"Seasonal indices ({_word} decomposition)",
         topic=_DEC,
         purpose=f"Seasonal indices of classical {_word} decomposition: detrend by the centred moving average, "
                 f"average each season, normalise the indices to {_norm_word}.",
         inputs=[_vin("x", "the series, oldest first (at least two full seasons)", 4), _iin("m", "season length", lo=2)],
         outputs=[_out("indices", "the m seasonal indices, starting with the season of x_1"),
                  _out("first", "the index of the first observation's season")],
         primary="first", conditions=_DEC_COND + (["positive data"] if _mult else []),
         method=("ratios x_t / CMA_t averaged by season, divided by their mean" if _mult else
                 "differences x_t - CMA_t averaged by season, minus their mean"),
         latex=(r"S_i = \bar r_i \Big/ \tfrac1m\sum_j \bar r_j,\quad r_t = x_t/\mathrm{CMA}_t" if _mult else
                r"S_i = \bar d_i - \tfrac1m\sum_j \bar d_j,\quad d_t = x_t-\mathrm{CMA}_t"),
         run=(lambda x, m, _m=_mult: run_seasonal_indices(x, m, _m)),
         reference=(lambda x, m, _m=_mult: ref_seasonal_indices(x, m, _m)),
         wrong=(lambda x, m, _m=_mult: _wrong_idx(x, m, _m)),
         rnd=(lambda rng, _m=_mult: (lambda m: {"x": _seasonal(rng, rng.randint(2 * m, 5 * m), m, _m), "m": m})(
             rng.choice([3, 4, 4, 6, 7, 12]))),
         need=[r"seasonal (?:indices|index|factors?|components?|effects?|relatives?)",
               r"\bmultiplicative|ratio[- ]to[- ]moving[- ]average" if _mult else r"\badditive"],
         avoid=r"adjust|holt|deseason" + ("" if _mult else r"|multiplicative"),
         aliases={"m": A_SEASON}, extract=_season_words,
         templates=[f"Find the {_word} seasonal indices of the quarterly series [30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51]."],
         examples=[dict(x="30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51", m=4),
                   dict(x=[10, 14, 9, 11, 15, 10, 12, 16, 11], m=3)],
         questions=[_q(f"Find the {_word} seasonal indices of the quarterly series "
                       "[30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51].", 0),
                    _q(f"Compute the {_word} seasonal indices with season length 3 of [10, 14, 9, 11, 15, 10, 12, 16, 11].", 1),
                    _q(f"Find the {_word} seasonal indices with m = 4 of [1, 2, 3, 4, 5].", outcome="INVALID_INPUT"),
                    _q(f"Find the {_word} seasonal indices of [1, 2, 3, 4, 5, 6, 7, 8].", outcome="NEEDS_INPUT")])
    _cap(f"seasonally_adjusted_{_word}", level="intermediate", name=f"Seasonally adjusted series ({_word})",
         topic=_DEC,
         purpose=f"The seasonally adjusted series of classical {_word} decomposition: each value "
                 + ("divided by" if _mult else "minus") + " its seasonal index.",
         inputs=[_vin("x", "the series, oldest first (at least two full seasons)", 4), _iin("m", "season length", lo=2)],
         outputs=[_out("adjusted", "the seasonally adjusted series"), _out("last", "the latest adjusted value")],
         primary="last", conditions=_DEC_COND + (["positive data"] if _mult else []),
         method="seasonal indices of classical decomposition, then x_t " + ("/" if _mult else "-") + " S_{season(t)}",
         latex=r"x_t^{\mathrm{SA}} = x_t / S_{t}" if _mult else r"x_t^{\mathrm{SA}} = x_t - S_{t}",
         run=(lambda x, m, _m=_mult: run_adjusted(x, m, _m)),
         reference=(lambda x, m, _m=_mult: ref_adjusted(x, m, _m)),
         wrong=(lambda x, m, _m=_mult: _wrong_adj(x, m, _m)),
         rnd=(lambda rng, _m=_mult: (lambda m: {"x": _seasonal(rng, rng.randint(2 * m, 5 * m), m, _m), "m": m})(
             rng.choice([3, 4, 4, 6, 12]))),
         need=[r"seasonally adjust|deseasonali[sz]", r"\bmultiplicative" if _mult else r"\badditive"],
         avoid=r"holt" + ("" if _mult else r"|multiplicative"), aliases={"m": A_SEASON}, extract=_season_words,
         templates=[f"Seasonally adjust the quarterly series [30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51] using {_word} decomposition."],
         examples=[dict(x="30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51", m=4)],
         questions=[_q(f"Seasonally adjust the quarterly series [30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51] "
                       f"using {_word} decomposition.", 0),
                    _q(f"Seasonally adjust [1, 2, 3, 4, 5, 6] using {_word} decomposition.", outcome="NEEDS_INPUT")])


# =========================================================================== differencing and growth ====
_TR = "transformations and growth"


def run_diff(x, d=1, D=0, s=None):
    x = _fin(x)
    d = _int(d, "the differencing order d", 0, 3)
    D = _int(D, "the seasonal differencing order D", 0, 2)
    if d + D == 0:
        raise ValueError("at least one difference (d or D) is needed")
    if D:
        if s is None:
            raise ValueError("a seasonal difference needs the seasonal lag s")
        s = _int(s, "the seasonal lag s", 2, 52)
    y = list(x)
    for _ in range(D):
        y = [y[t] - y[t - s] for t in range(s, len(y))]
    for _ in range(d):
        y = [y[t] - y[t - 1] for t in range(1, len(y))]
    if len(y) < 1:
        raise ValueError("the series is too short for these differences")
    return {"differenced": y, "last": y[-1], "length": len(y)}


def ref_diff(x, d=1, D=0, s=None):
    """Polynomial route: the coefficients of (1 - L)^d (1 - L^s)^D applied as one filter."""
    import numpy as np
    from numpy.polynomial import polynomial as P
    c = np.array([1.0])
    for _ in range(int(d)):
        c = P.polymul(c, [1.0, -1.0])
    for _ in range(int(D)):
        c = P.polymul(c, np.r_[1.0, np.zeros(int(s) - 1), -1.0])
    y = np.convolve(np.asarray(x, dtype=float), c, "valid")
    return {"differenced": [float(v) for v in y], "last": float(y[-1]), "length": len(y)}


def wrong_diff(x, d=1, D=0, s=None):  # an order-d difference taken as one lag-d difference
    y = list(x)
    if D:
        y = [y[t] - y[t - int(s)] for t in range(int(s), len(y))]
    if d:
        y = [y[t] - y[t - int(d)] for t in range(int(d), len(y))]
    return {"differenced": y, "last": y[-1], "length": len(y)}


def _x_diff(rest, d, spans):
    words = {"first": 1, "second": 2, "third": 3}
    m = re.search(r"\b(first|second|third)[- ](?:order )?(?:regular |non-seasonal )?differenc", rest)
    if m and "d" not in d:
        d["d"] = float(words[m.group(1)])
    if "d" not in d and re.search(r"\btwice\b", rest):
        d["d"] = 2.0
    seasonal = re.search(r"seasonal(?:ly)? differenc", rest)
    if seasonal:
        _season_words(rest, d, spans, "s")
    if (seasonal or "s" in d) and "D" not in d:
        d["D"] = 1.0
        if "d" not in d:
            d["d"] = 0.0


def _rnd_diff(rng):
    x = _any(rng, 12, 40)
    if rng.random() < 0.65:
        return {"x": x, "d": rng.choice([2, 2, 3])}
    return {"x": x, "d": rng.choice([0, 1, 2]), "D": 1, "s": rng.choice([4, 12, 3])}


_cap("differencing", level="beginner", name="Regular and seasonal differencing", topic=_TR,
     purpose="Difference a series d times (and seasonally D times at lag s): the series that an ARIMA(p, d, q) or "
             "seasonal ARIMA model works on.",
     inputs=[_vin("x", "the series, oldest first"),
             _iin("d", "number of regular (lag-1) differences, 0..3 (default 1)", required=False, lo=0, hi=3),
             _iin("D", "number of seasonal differences, 0..2 (default 0)", required=False, lo=0, hi=2),
             _iin("s", "seasonal lag (needed when D > 0)", required=False, lo=2)],
     outputs=[_out("differenced", "the differenced series"), _out("last", "its latest value"),
              _out("length", "its length n - d - D s")],
     primary="last", conditions=["d + D >= 1", "the series is longer than d + D s"],
     method="seasonal differences y_t = x_t - x_{t-s} (D times), then regular differences y_t - y_{t-1} (d times)",
     latex=r"\nabla^d \nabla_s^D x_t = (1-L)^d (1-L^s)^D x_t", run=run_diff, reference=ref_diff, wrong=wrong_diff,
     rnd=_rnd_diff,
     need=r"differenc", avoid=r"dickey|\bmean\b|absolute|successive|log|change|growth|cross|percent",
     aliases={"d": [r"\bd\s*=\s*(\d+)", r"(?<!seasonal )\border\s*(?:d\s*)?" + _W + r"(\d+)"],
              "D": [r"seasonal order\s*" + _W + r"(\d+)"],
              "s": [r"(?:seasonal )?lag\s*(?:s\s*)?" + _W + r"(\d+)", r"season(?:al)? (?:length|period)\s*" + _W + r"(\d+)",
                    r"\bs\s*=\s*(\d+)", r"\bperiod\s*" + _W + r"(\d+)"]},
     extract=_x_diff,
     templates=["Find the second differences of [3, 5, 9, 15, 23, 33].",
                "Take the seasonal difference with lag 4 of [10, 14, 9, 12, 11, 16, 10, 13]."],
     examples=[dict(x="3, 5, 9, 15, 23, 33", d=2), dict(x=[10, 14, 9, 12, 11, 16, 10, 13], d=0, D=1, s=4),
               dict(x="5, 8, 6, 9, 7, 10", d=1),
               dict(x=[10, 14, 9, 12, 11, 16, 10, 13, 12, 18, 11, 15], d=1, D=1, s=4)],
     questions=[_q("Find the second differences of [3, 5, 9, 15, 23, 33].", 0),
                _q("Take the seasonal difference with lag 4 of [10, 14, 9, 12, 11, 16, 10, 13].", 1),
                _q("Compute the first differences of [5, 8, 6, 9, 7, 10].", 2),
                _q("Apply a first difference and a seasonal difference with lag 4 to "
                   "[10, 14, 9, 12, 11, 16, 10, 13, 12, 18, 11, 15].", 3),
                _q("Take the seasonal difference with lag 12 of [1, 2, 3, 4, 5].", outcome="INVALID_INPUT")])


def run_avg_growth(x):
    x = _fin(x)
    _need(x, 2)
    _positive(x)
    n = len(x) - 1
    g = (x[-1] / x[0]) ** (1.0 / n)
    return {"rate": _r12(g - 1), "rate_percent": _r12(100 * (g - 1)), "total_growth": _r12(x[-1] / x[0] - 1),
            "periods": n}


def ref_avg_growth(x):
    """Geometric mean of the period-to-period growth factors (scipy.stats.gmean)."""
    import numpy as np
    from scipy.stats import gmean
    y = np.asarray(x, dtype=float)
    g = float(gmean(y[1:] / y[:-1]))
    return {"rate": g - 1, "rate_percent": 100 * (g - 1), "total_growth": float(np.prod(y[1:] / y[:-1]) - 1),
            "periods": len(y) - 1}


def wrong_avg_growth(x):      # the arithmetic mean of the growth rates
    r = [x[i] / x[i - 1] - 1 for i in range(1, len(x))]
    a = sum(r) / len(r)
    return {"rate": a, "rate_percent": 100 * a, "total_growth": x[-1] / x[0] - 1, "periods": len(r)}


_cap("compound_average_growth_rate", level="beginner", name="Compound average growth rate of a series",
     topic=_TR,
     purpose="The compound (geometric) average growth rate per period of a positive series (CAGR when the periods "
             "are years), and the total growth from the first to the last value.",
     inputs=[_vin("x", "the series, oldest first (positive values)")],
     outputs=[_out("rate", "compound average growth rate per period, as a fraction"),
              _out("rate_percent", "the same rate in percent"),
              _out("total_growth", "total growth x_n/x_1 - 1, as a fraction"), _out("periods", "number of periods n - 1")],
     primary="rate", conditions=["all values positive", "equally spaced periods"],
     method="(x_n / x_1)^(1/(n-1)) - 1", latex=r"g = \left(\frac{x_n}{x_1}\right)^{1/(n-1)} - 1",
     run=run_avg_growth, reference=ref_avg_growth, wrong=wrong_avg_growth,
     rnd=lambda rng: {"x": _walk(rng, rng.randint(2, 30))},
     need=[r"compound(?:ed)? (?:average |annual )?growth|geometric (?:mean|average) (?:growth|rate)|\bcagr\b|"
           r"average compound", r"\["],
     templates=["Find the compound average growth rate of [100, 110, 125, 133, 150]."],
     examples=[dict(x="100, 110, 125, 133, 150"), dict(x=[2000, 2300, 2100, 2600])],
     questions=[_q("Find the compound average growth rate of [100, 110, 125, 133, 150].", 0),
                _q("What is the CAGR of the yearly revenues [2000, 2300, 2100, 2600]?", 1),
                _q("Find the compound average growth rate of [100, -5, 120].", outcome="INVALID_INPUT")])


# ======================================================================================= autocorrelation ====
_AC = "autocorrelation"
_NOT_THEORY = (r"partial|pacf|ljung|box|cross|bartlett|standard error|theoretical|\bar\s*\(|\bma\s*\(|arma|\bphi\b|"
               r"theta|yule|levinson|model|process")


def _acf_in(x, k, lo=1):
    x = _fin(x)
    _need(x, 3)
    _nonconstant(x)
    k = _int(k, "the lag k", lo, len(x) - 1)
    return x, k


def run_acf(x, k):
    x, k = _acf_in(x, k)
    r = _acf_list(x, k)
    return {"acf": r[k], "acf_list": r[1:]}


def ref_acf(x, k):
    """FFT route (Wiener-Khinchin) of statsmodels' acf."""
    from statsmodels.tsa.stattools import acf
    r = acf(x, nlags=int(k), adjusted=False, fft=True)
    return {"acf": float(r[int(k)]), "acf_list": [float(v) for v in r[1:]]}


def wrong_acf(x, k):          # n - k in the denominator of each autocovariance
    n = len(x)
    r = [v * n / (n - j) for j, v in enumerate(_acf_list(x, k))]
    return {"acf": r[k], "acf_list": r[1:]}


def _rnd_acf(rng):
    x = _any(rng, 8, 60)
    return {"x": x, "k": rng.randint(1, min(len(x) - 1, 12))}


_cap("sample_autocorrelation", level="intermediate", name="Sample autocorrelation at lag k", topic=_AC,
     purpose="The sample autocorrelation r_k of a series (and r_1..r_k), with autocovariances divided by n and taken "
             "about the sample mean.",
     inputs=[_vin("x", "the series, oldest first", 3), _iin("k", "the lag k (1 <= k <= n - 1)", lo=1)],
     outputs=[_out("acf", "the sample autocorrelation r_k"), _out("acf_list", "r_1, ..., r_k")],
     primary="acf", conditions=["the series is not constant", "1 <= k <= n - 1", "denominator n (not n - k)"],
     method="r_k = c_k / c_0, c_k = (1/n) sum_{t=1}^{n-k} (x_t - xbar)(x_{t+k} - xbar)",
     latex=r"r_k = \frac{\sum_{t=1}^{n-k}(x_t-\bar x)(x_{t+k}-\bar x)}{\sum_{t=1}^{n}(x_t-\bar x)^2}",
     run=run_acf, reference=ref_acf, wrong=wrong_acf, rnd=_rnd_acf,
     need=r"auto-?correlation|\bacf\b", avoid=_NOT_THEORY, aliases={"k": A_LAG},
     templates=["Find the lag 1 autocorrelation of [4, 6, 5, 8, 7, 9, 8, 10]."],
     examples=[dict(x="4, 6, 5, 8, 7, 9, 8, 10", k=1), dict(x=[12, 15, 11, 18, 14, 19, 13, 20, 16], k=3)],
     questions=[_q("Find the lag 1 autocorrelation of [4, 6, 5, 8, 7, 9, 8, 10].", 0),
                _q("Compute the sample autocorrelation at lag 3 of [12, 15, 11, 18, 14, 19, 13, 20, 16].", 1),
                _q("Find the lag 9 autocorrelation of [1, 2, 3].", outcome="INVALID_INPUT"),
                _q("Find the autocorrelation of [1, 2, 3, 5].", outcome="NEEDS_INPUT")])


def run_acov(x, k):
    x = _fin(x)
    _need(x, 2)
    k = _int(k, "the lag k", 0, len(x) - 1)
    m = _mean(x)
    return {"acov": _acov(x, k, m), "gamma0": _acov(x, 0, m)}


def ref_acov(x, k):
    from statsmodels.tsa.stattools import acovf
    g = acovf(x, adjusted=False, fft=True, nlag=int(k))
    return {"acov": float(g[int(k)]), "gamma0": float(g[0])}


_cap("sample_autocovariance", level="intermediate", name="Sample autocovariance at lag k", topic=_AC,
     purpose="The sample autocovariance c_k of a series, divided by n and taken about the sample mean.",
     inputs=[_vin("x", "the series, oldest first"), _iin("k", "the lag k (0 <= k <= n - 1)", lo=0)],
     outputs=[_out("acov", "the sample autocovariance c_k"), _out("gamma0", "c_0, the variance divided by n")],
     primary="acov", conditions=["0 <= k <= n - 1", "denominator n (not n - k)"],
     method="c_k = (1/n) sum_{t=1}^{n-k} (x_t - xbar)(x_{t+k} - xbar)",
     latex=r"c_k = \frac1n\sum_{t=1}^{n-k}(x_t-\bar x)(x_{t+k}-\bar x)", run=run_acov, reference=ref_acov,
     wrong=lambda x, k: (lambda o: {"acov": o["acov"] * len(x) / (len(x) - k), "gamma0": o["gamma0"]})(run_acov(x, k)),
     rnd=lambda rng: (lambda x: {"x": x, "k": rng.randint(1, min(len(x) - 1, 12))})(_any(rng, 6, 60)),
     need=r"auto-?covariance", avoid=_NOT_THEORY, aliases={"k": A_LAG},
     templates=["Find the lag 2 autocovariance of [4, 6, 5, 8, 7, 9, 8, 10]."],
     examples=[dict(x="4, 6, 5, 8, 7, 9, 8, 10", k=2), dict(x=[1.5, 2.5, 2.0, 3.5, 3.0], k=1)],
     questions=[_q("Find the lag 2 autocovariance of [4, 6, 5, 8, 7, 9, 8, 10].", 0),
                _q("Compute the sample autocovariance at lag 1 of [1.5, 2.5, 2.0, 3.5, 3.0].", 1),
                _q("Find the autocovariance at lag 7 of [1, 2, 3].", outcome="INVALID_INPUT")])


def run_pacf(x, k):
    x = _fin(x)
    _need(x, 4)
    _nonconstant(x)
    k = _int(k, "the lag k", 1)
    if k >= len(x) // 2:
        raise ValueError("the lag k must be less than n/2")
    _phi, p, _v = _levinson(_acf_list(x, k), k)
    return {"pacf": p[-1], "pacf_list": p}


def ref_pacf(x, k):
    """FFT autocorrelations, then the Yule-Walker system of every order solved directly (no recursion)."""
    import numpy as np
    from scipy.linalg import toeplitz
    from statsmodels.tsa.stattools import acf
    r = acf(x, nlags=int(k), adjusted=False, fft=True)
    p = [float(np.linalg.solve(toeplitz(r[:j]), r[1:j + 1])[-1]) for j in range(1, int(k) + 1)]
    return {"pacf": p[-1], "pacf_list": p}


def wrong_pacf(x, k):         # Durbin-Levinson on n - k adjusted autocorrelations
    n = len(x)
    r = [v * n / (n - j) for j, v in enumerate(_acf_list(x, k))]
    try:
        p = _levinson(r, k)[1]
    except ValueError:
        p = r[1:]
    return {"pacf": p[-1], "pacf_list": p}


_LEV_FROM = r"from (?:the )?(?:theoretical )?autocorrelations|given (?:the )?autocorrelations|\brho\b"

_cap("sample_partial_autocorrelation", level="advanced", name="Sample partial autocorrelation at lag k", topic=_AC,
     purpose="The sample partial autocorrelation phi_kk of a series (and phi_11..phi_kk) by the Durbin-Levinson "
             "recursion on the sample autocorrelations (denominator n).",
     inputs=[_vin("x", "the series, oldest first", 4), _iin("k", "the lag k (1 <= k < n/2)", lo=1)],
     outputs=[_out("pacf", "the partial autocorrelation at lag k"), _out("pacf_list", "partial autocorrelations at lags 1..k")],
     primary="pacf", conditions=["the series is not constant", "1 <= k < n/2", "autocovariances divided by n"],
     method="Durbin-Levinson: phi_kk = (r_k - sum phi_{k-1,j} r_{k-j}) / (1 - sum phi_{k-1,j} r_j)",
     latex=r"\phi_{kk} = \frac{r_k - \sum_{j=1}^{k-1}\phi_{k-1,j}\,r_{k-j}}{1-\sum_{j=1}^{k-1}\phi_{k-1,j}\,r_j}",
     run=run_pacf, reference=ref_pacf, wrong=wrong_pacf,
     rnd=lambda rng: (lambda x: {"x": x, "k": rng.randint(2, min(len(x) // 2 - 1, 8))})(_any(rng, 10, 60)),
     need=r"partial auto-?correlation|\bpacf\b", avoid=_LEV_FROM + r"|theoretical|model|process|\bar\s*\(|\bma\s*\(",
     aliases={"k": A_LAG},
     templates=["Find the partial autocorrelation at lag 2 of [4, 6, 5, 8, 7, 9, 8, 10, 9, 12]."],
     examples=[dict(x="4, 6, 5, 8, 7, 9, 8, 10, 9, 12", k=2), dict(x=[3.1, 2.4, 3.9, 3.3, 4.6, 3.8, 5.0, 4.1, 5.6, 4.9, 6.2, 5.1], k=3)],
     questions=[_q("Find the partial autocorrelation at lag 2 of [4, 6, 5, 8, 7, 9, 8, 10, 9, 12].", 0),
                _q("Compute the PACF at lag 3 of [3.1, 2.4, 3.9, 3.3, 4.6, 3.8, 5.0, 4.1, 5.6, 4.9, 6.2, 5.1].", 1),
                _q("Find the partial autocorrelation at lag 3 of [1, 2, 4, 3, 5].", outcome="INVALID_INPUT")])


def run_bartlett(x, k):
    x, k = _acf_in(x, k)
    r = _acf_list(x, k)
    se = math.sqrt((1 + 2 * math.fsum(v * v for v in r[1:k])) / len(x))
    return {"se": se, "half_width_95": _Z975 * se, "acf": r[k]}


def ref_bartlett(x, k):
    from scipy.stats import norm
    from statsmodels.tsa.stattools import acf
    r, ci = acf(x, nlags=int(k), adjusted=False, fft=True, alpha=0.05)
    half = float(ci[int(k), 1] - r[int(k)])
    return {"se": half / float(norm.ppf(0.975)), "half_width_95": half, "acf": float(r[int(k)])}


_cap("bartlett_standard_error", level="advanced", name="Bartlett standard error of a sample autocorrelation",
     topic=_AC,
     purpose="Bartlett's standard error of r_k under the hypothesis that the autocorrelations beyond lag k - 1 are "
             "zero, and the 95 % band half-width used in correlograms.",
     inputs=[_vin("x", "the series, oldest first", 3), _iin("k", "the lag k", lo=1)],
     outputs=[_out("se", "Bartlett standard error of r_k"), _out("half_width_95", "1.959964 x se"),
              _out("acf", "the sample autocorrelation r_k")],
     primary="se", conditions=["1 <= k <= n - 1", "autocorrelations with denominator n"],
     method="se_k = sqrt((1 + 2 sum_{j<k} r_j^2) / n)", latex=r"\mathrm{se}(r_k) = \sqrt{\frac{1 + 2\sum_{j=1}^{k-1} r_j^2}{n}}",
     run=run_bartlett, reference=ref_bartlett,
     wrong=lambda x, k: {"se": 1 / math.sqrt(len(x)), "half_width_95": _Z975 / math.sqrt(len(x)), "acf": run_acf(x, k)["acf"]},
     rnd=lambda rng: (lambda x: {"x": x, "k": rng.randint(2, min(len(x) - 1, 10))})(_any(rng, 8, 60)),
     need=r"bartlett(?:'s)? (?:standard error|formula|bands?|confidence)|standard error of (?:the )?(?:sample )?auto-?correlation",
     aliases={"k": A_LAG},
     templates=["Find the Bartlett standard error of the lag 3 autocorrelation of [4, 6, 5, 8, 7, 9, 8, 10, 9, 12]."],
     examples=[dict(x="4, 6, 5, 8, 7, 9, 8, 10, 9, 12", k=3), dict(x=[12, 15, 11, 18, 14, 19, 13, 20, 16], k=1)],
     questions=[_q("Find the Bartlett standard error of the lag 3 autocorrelation of [4, 6, 5, 8, 7, 9, 8, 10, 9, 12].", 0),
                _q("What is the standard error of the sample autocorrelation at lag 1 of [12, 15, 11, 18, 14, 19, 13, 20, 16]?", 1)])


def _portmanteau(x, h, ljung, model_df=0):
    x = _fin(x)
    _need(x, 3)
    _nonconstant(x)
    h = _int(h, "the number of lags h", 1, len(x) - 1)
    md = _int(model_df, "the fitted-model degrees of freedom", 0)
    if md >= h:
        raise ValueError("the number of lags h must exceed the fitted-model degrees of freedom")
    n = len(x)
    r = _acf_list(x, h)
    q = n * (n + 2) * math.fsum(r[k] ** 2 / (n - k) for k in range(1, h + 1)) if ljung else \
        n * math.fsum(r[k] ** 2 for k in range(1, h + 1))
    return {"q": q, "p_value": _r12(_chi2_sf(q, h - md)), "df": h - md}


def _ref_portmanteau(x, h, ljung, model_df=0):
    from statsmodels.stats.diagnostic import acorr_ljungbox
    d = acorr_ljungbox(x, lags=[int(h)], boxpierce=True, model_df=int(model_df), return_df=True)
    c = ("lb_stat", "lb_pvalue") if ljung else ("bp_stat", "bp_pvalue")
    return {"q": float(d[c[0]].iloc[0]), "p_value": float(d[c[1]].iloc[0]), "df": int(h) - int(model_df)}


def _rnd_port(rng):
    n = rng.randint(15, 80)
    x = _ar1(rng, n, phi=rng.uniform(-0.4, 0.5)) if rng.random() < 0.7 else _walk(rng, n)
    h = rng.randint(1, min(12, n // 3))
    return {"x": x, "h": h, "model_df": rng.choice([0, 0, 1, 2]) if h > 2 else 0}


for _lj, _key, _lvl, _nm_, _need_, _av in (
        (True, "ljung_box_test", "intermediate", "Ljung-Box test", r"ljung[- ]box", None),
        (False, "box_pierce_test", "intermediate", "Box-Pierce test", r"box[- ]pierce", r"ljung")):
    _cap(_key, level=_lvl, name=_nm_, topic=_AC,
         purpose=f"The {_nm_} portmanteau statistic for no autocorrelation up to lag h and its chi-square p-value "
                 "(h - model_df degrees of freedom; autocorrelations with denominator n).",
         inputs=[_vin("x", "the series (or residuals), oldest first", 3), _iin("h", "the number of lags h", lo=1),
                 _iin("model_df", "parameters of a fitted ARMA model subtracted from the degrees of freedom (default 0)",
                      required=False, lo=0)],
         outputs=[_out("q", "the test statistic"), _out("p_value", "upper-tail chi-square probability, df = h - model_df"),
                  _out("df", "degrees of freedom h - model_df")],
         primary="q", conditions=["1 <= h <= n - 1", "model_df < h (default 0: raw series)"],
         method=("Q = n(n+2) sum_{k=1}^h r_k^2/(n-k)" if _lj else "Q = n sum_{k=1}^h r_k^2") + "; p = P(chi2_{h-model_df} > Q)",
         latex=(r"Q_{LB} = n(n+2)\sum_{k=1}^{h}\frac{r_k^2}{n-k}" if _lj else r"Q_{BP} = n\sum_{k=1}^{h} r_k^2"),
         run=(lambda x, h, model_df=0, _l=_lj: _portmanteau(x, h, _l, model_df)),
         reference=(lambda x, h, model_df=0, _l=_lj: _ref_portmanteau(x, h, _l, model_df)),
         wrong=(lambda x, h, model_df=0, _l=_lj: _portmanteau(x, h, not _l, model_df)), rnd=_rnd_port,
         need=_need_, avoid=_av or (),
         aliases={"h": A_LAGS, "model_df": [r"(\d+)\s+(?:fitted|estimated) (?:model )?parameters",
                                            r"model[_ ]df\s*" + _W + r"(\d+)"]},
         templates=[f"Perform the {_nm_} with 3 lags on [4, 6, 5, 8, 7, 9, 8, 10, 9, 12, 11, 13]."],
         examples=[dict(x="4, 6, 5, 8, 7, 9, 8, 10, 9, 12, 11, 13", h=3),
                   dict(x=[0.5, -0.3, 0.8, -0.1, 0.2, -0.6, 0.4, 0.1, -0.2, 0.7, -0.4, 0.3, 0.0, -0.5, 0.6], h=5),
                   dict(x=[0.5, -0.3, 0.8, -0.1, 0.2, -0.6, 0.4, 0.1, -0.2, 0.7, -0.4, 0.3, 0.0, -0.5, 0.6], h=6,
                        model_df=2)],
         questions=[_q(f"Perform the {_nm_} with 3 lags on [4, 6, 5, 8, 7, 9, 8, 10, 9, 12, 11, 13].", 0),
                    _q(f"Compute the {_nm_} statistic with h = 5 for the residuals "
                       "[0.5, -0.3, 0.8, -0.1, 0.2, -0.6, 0.4, 0.1, -0.2, 0.7, -0.4, 0.3, 0.0, -0.5, 0.6].", 1),
                    _q(f"Perform the {_nm_} with 6 lags and 2 fitted parameters on the residuals "
                       "[0.5, -0.3, 0.8, -0.1, 0.2, -0.6, 0.4, 0.1, -0.2, 0.7, -0.4, 0.3, 0.0, -0.5, 0.6].", 2),
                    _q(f"Perform the {_nm_} on [1, 3, 2, 4, 3].", outcome="NEEDS_INPUT")])


# =============================================================================== autoregressive models ====
_ARM = "ARIMA models"
_AR_RE = r"\bar\s*\(\s*(\d+)\s*\)"


def _x_ar_order(vec=None, key="p"):
    """Read the order from 'AR(p)'; with a coefficient list it must agree with the list's length."""
    def f(rest, d, spans):
        m = re.search(_AR_RE, rest)
        if m:
            p = float(m.group(1))
            if vec and vec in d:
                n = len([t for t in re.split(r"[,\s;]+", d[vec].strip("[]")) if t])
                if n != p:
                    return {"_out_of_scope": f"AR({int(p)}) was named but {n} coefficients were given"}
            elif key and key not in d:
                d[key] = p
    return f


def run_ar_yw(x, p):
    x = _fin(x)
    _need(x, 4)
    _nonconstant(x)
    p = _int(p, "the order p", 1, 10)
    if p >= len(x) - 1:
        raise ValueError("the order p must be less than n - 1")
    r = _acf_list(x, p)
    phi, _pacf, v = _levinson(r, p)
    return {"phi": phi, "phi1": phi[0], "sigma2": _acov(x, 0) * v}


def ref_ar_yw(x, p):
    from statsmodels.regression.linear_model import yule_walker
    rho, sigma = yule_walker(x, order=int(p), method="mle", demean=True)
    return {"phi": [float(v) for v in rho], "phi1": float(rho[0]), "sigma2": float(sigma) ** 2}


def wrong_ar_yw(x, p):        # n - k adjusted autocovariances
    n = len(x)
    r = [v * n / (n - j) for j, v in enumerate(_acf_list(x, p))]
    try:
        phi, _pacf, v = _levinson(r, p)
    except ValueError:
        phi, v = r[1:], 1.0
    return {"phi": phi, "phi1": phi[0], "sigma2": _acov(x, 0) * v}


_cap("ar_yule_walker", level="advanced", name="AR(p) estimation by Yule-Walker", topic=_ARM,
     purpose="Estimate the coefficients phi_1..phi_p and the innovation variance of an AR(p) model from data by the "
             "Yule-Walker equations (sample autocovariances divided by n, mean removed).",
     inputs=[_vin("x", "the series, oldest first", 4), _iin("p", "the AR order p", lo=1, hi=10)],
     outputs=[_out("phi", "the coefficients phi_1..phi_p"), _out("phi1", "phi_1"),
              _out("sigma2", "the innovation variance c_0 (1 - sum phi_j r_j)")],
     primary="phi1", conditions=["the series is not constant", "1 <= p < n - 1", "denominator n"],
     method="solve the Toeplitz system R phi = r (Durbin-Levinson); sigma^2 = c_0 prod(1 - phi_kk^2)",
     latex=r"\begin{pmatrix}1&r_1&\cdots\\ r_1&1&\cdots\\ \vdots&&\ddots\end{pmatrix}\phi = \begin{pmatrix}r_1\\ \vdots\\ r_p\end{pmatrix}",
     run=run_ar_yw, reference=ref_ar_yw, wrong=wrong_ar_yw,
     rnd=lambda rng: {"x": _ar1(rng, rng.randint(15, 80)), "p": rng.randint(1, 4)},
     need=[r"yule[- ]walker", r"\bar\s*\(|autoregress"], avoid=r"autocorrelations|\brho\b|forecast",
     aliases={"p": [r"\border\s*(?:p\s*)?" + _W + r"(\d+)", r"\bp\s*=\s*(\d+)"]}, extract=_x_ar_order(),
     templates=["Estimate an AR(2) model by Yule-Walker for the series [4, 6, 5, 8, 7, 9, 8, 10, 9, 12]."],
     examples=[dict(x="4, 6, 5, 8, 7, 9, 8, 10, 9, 12", p=2), dict(x=[1.2, 0.8, 1.5, 1.1, 1.9, 1.4, 2.0, 1.6], p=1)],
     questions=[_q("Estimate an AR(2) model by Yule-Walker for the series [4, 6, 5, 8, 7, 9, 8, 10, 9, 12].", 0),
                _q("Fit an AR(1) model with the Yule-Walker equations to the data [1.2, 0.8, 1.5, 1.1, 1.9, 1.4, 2.0, 1.6].", 1),
                _q("Fit an autoregressive model by Yule-Walker to the data [1, 3, 2, 4].", outcome="NEEDS_INPUT")])


def run_ar_ols(x, p):
    x = _fin(x)
    p = _int(p, "the order p", 1, 10)
    if len(x) - p <= p + 2:
        raise ValueError("too few observations: need n - p > p + 2")
    X = [[1.0] + [x[t - j] for j in range(1, p + 1)] for t in range(p, len(x))]
    y = x[p:]
    beta, res, _dg = _lstsq(X, y)
    return {"const": beta[0], "phi": beta[1:], "phi1": beta[1], "sigma2": math.fsum(e * e for e in res) / len(y)}


def ref_ar_ols(x, p):
    import numpy as np
    from statsmodels.tsa.ar_model import AutoReg
    f = AutoReg(np.asarray(x, dtype=float), lags=int(p), trend="c").fit()
    pr = [float(v) for v in f.params]
    return {"const": pr[0], "phi": pr[1:], "phi1": pr[1], "sigma2": float(f.sigma2)}


def wrong_ar_ols(x, p):       # regression through the origin (constant left out)
    X = [[x[t - j] for j in range(1, p + 1)] for t in range(p, len(x))]
    beta, res, _dg = _lstsq(X, x[p:])
    return {"const": 0.0, "phi": beta, "phi1": beta[0], "sigma2": sum(e * e for e in res) / len(res)}


_cap("ar_least_squares", level="intermediate", name="AR(p) estimation by conditional least squares", topic=_ARM,
     purpose="Fit x_t = c + phi_1 x_{t-1} + ... + phi_p x_{t-p} + e_t by ordinary least squares on t = p+1..n; "
             "the innovation variance is the residual sum of squares divided by n - p.",
     inputs=[_vin("x", "the series, oldest first", 5), _iin("p", "the AR order p", lo=1, hi=10)],
     outputs=[_out("const", "the constant c"), _out("phi", "the coefficients phi_1..phi_p"), _out("phi1", "phi_1"),
              _out("sigma2", "residual sum of squares / (n - p)")],
     primary="phi1", conditions=["n - p > p + 2", "regressors not collinear", "the first p values are conditioned on"],
     method="least squares (Householder QR) of x_t on (1, x_{t-1}, ..., x_{t-p}), t = p+1..n",
     latex=r"\min_{c,\phi}\sum_{t=p+1}^{n}\Big(x_t - c - \sum_{j=1}^{p}\phi_j x_{t-j}\Big)^2",
     run=run_ar_ols, reference=ref_ar_ols, wrong=wrong_ar_ols,
     rnd=lambda rng: {"x": _ar1(rng, rng.randint(15, 80)), "p": rng.randint(1, 4)},
     need=[r"\bar\s*\(|autoregress", r"least squares|\bols\b|regression"], avoid=r"yule|forecast|predict",
     aliases={"p": [r"\border\s*(?:p\s*)?" + _W + r"(\d+)", r"\bp\s*=\s*(\d+)"]}, extract=_x_ar_order(),
     templates=["Fit an AR(1) model by least squares to [4, 6, 5, 8, 7, 9, 8, 10, 9, 12]."],
     examples=[dict(x="4, 6, 5, 8, 7, 9, 8, 10, 9, 12", p=1), dict(x=[3.1, 2.4, 3.9, 3.3, 4.6, 3.8, 5.0, 4.1, 5.6, 4.9, 6.2, 5.1], p=2)],
     questions=[_q("Fit an AR(1) model by least squares to [4, 6, 5, 8, 7, 9, 8, 10, 9, 12].", 0),
                _q("Estimate an AR(2) by OLS for [3.1, 2.4, 3.9, 3.3, 4.6, 3.8, 5.0, 4.1, 5.6, 4.9, 6.2, 5.1].", 1),
                _q("Fit an AR(3) model by least squares to [1, 2, 3, 2, 1].", outcome="INVALID_INPUT")])


def run_yw_acf(rho):
    rho = _fin(rho, "the autocorrelations")
    p = len(rho)
    phi, _pacf, v = _levinson([1.0] + rho, p)
    return {"phi": phi, "phi1": phi[0], "variance_ratio": v}


def ref_yw_acf(rho):
    import numpy as np
    from scipy.linalg import toeplitz
    r = np.asarray(rho, dtype=float)
    phi = np.linalg.solve(toeplitz(np.r_[1.0, r[:-1]]), r)
    return {"phi": [float(v) for v in phi], "phi1": float(phi[0]), "variance_ratio": float(1 - phi @ r)}


_cap("yule_walker_from_autocorrelations", level="intermediate",
     name="Yule-Walker equations from autocorrelations", topic=_ARM,
     purpose="Solve the Yule-Walker equations for the AR(p) coefficients given the autocorrelations rho_1..rho_p, and "
             "the ratio of the innovation variance to the process variance.",
     inputs=[_vin("rho", "the autocorrelations rho_1..rho_p", 1, max_length=10)],
     outputs=[_out("phi", "the AR coefficients phi_1..phi_p"), _out("phi1", "phi_1"),
              _out("variance_ratio", "sigma^2 / gamma_0 = 1 - sum phi_j rho_j")],
     primary="phi1", conditions=["the autocorrelations form a positive definite sequence"],
     method="Durbin-Levinson recursion on rho_1..rho_p", latex=r"\rho_k = \sum_{j=1}^{p}\phi_j\,\rho_{|k-j|},\ k = 1..p",
     run=run_yw_acf, reference=ref_yw_acf,
     wrong=lambda rho: {"phi": list(rho), "phi1": rho[0], "variance_ratio": 1 - sum(v * v for v in rho)},
     rnd=lambda rng: {"rho": _rand_pacf_rho(rng, rng.randint(2, 6))},
     need=[r"yule[- ]walker", r"autocorrelations|\brho\b"], avoid=r"series|\bdata\b|observations|partial",
     extract=_x_ar_order("rho", None),
     templates=["Solve the Yule-Walker equations for the autocorrelations [0.6, 0.2]."],
     examples=[dict(rho="0.6, 0.2"), dict(rho=[0.8, 0.5, 0.3]), dict(rho="0.5")],
     questions=[_q("Solve the Yule-Walker equations for the autocorrelations [0.6, 0.2].", 0),
                _q("Find the AR(3) coefficients from the Yule-Walker equations with autocorrelations [0.8, 0.5, 0.3].", 1),
                _q("Solve the Yule-Walker equations for the autocorrelations [0.9, 0.1].", outcome="INVALID_INPUT"),
                _q("Find the AR(2) coefficients from the Yule-Walker equations with autocorrelations [0.5].",
                   outcome="OUT_OF_SCOPE")])


def run_lev_pacf(rho):
    rho = _fin(rho, "the autocorrelations")
    _phi, p, _v = _levinson([1.0] + rho, len(rho))
    return {"pacf": p, "last": p[-1]}


def ref_lev_pacf(rho):
    import numpy as np
    from scipy.linalg import toeplitz
    r = np.r_[1.0, np.asarray(rho, dtype=float)]
    p = [float(np.linalg.solve(toeplitz(r[:j]), r[1:j + 1])[-1]) for j in range(1, len(r))]
    return {"pacf": p, "last": p[-1]}


_cap("partial_autocorrelations_from_autocorrelations", level="advanced",
     name="Partial autocorrelations from autocorrelations", topic=_ARM,
     purpose="The partial autocorrelations phi_11..phi_pp implied by autocorrelations rho_1..rho_p (Durbin-Levinson).",
     inputs=[_vin("rho", "the autocorrelations rho_1..rho_p", 1, max_length=12)],
     outputs=[_out("pacf", "partial autocorrelations at lags 1..p"), _out("last", "phi_pp")],
     primary="last", conditions=["the autocorrelations form a positive definite sequence"],
     method="Durbin-Levinson recursion", latex=r"\phi_{kk} = \frac{\rho_k-\sum_{j=1}^{k-1}\phi_{k-1,j}\rho_{k-j}}{1-\sum_{j=1}^{k-1}\phi_{k-1,j}\rho_j}",
     run=run_lev_pacf, reference=ref_lev_pacf, wrong=lambda rho: {"pacf": list(rho), "last": rho[-1]},
     rnd=lambda rng: {"rho": _rand_pacf_rho(rng, rng.randint(2, 7))},
     need=[r"partial auto-?correlations?|\bpacf\b", _LEV_FROM],
     templates=["Find the partial autocorrelations from the autocorrelations [0.6, 0.3, 0.1]."],
     examples=[dict(rho="0.6, 0.3, 0.1"), dict(rho=[0.8, 0.64])],
     questions=[_q("Find the partial autocorrelations from the autocorrelations [0.6, 0.3, 0.1].", 0),
                _q("Compute the PACF given autocorrelations [0.8, 0.64].", 1),
                _q("Find the partial autocorrelations from the autocorrelations [0.9, 0.1].", outcome="INVALID_INPUT")])


def run_ar_forecast(x, phi, c=0.0, h=1):
    x = _fin(x)
    phi = _fin(phi, "the coefficients")
    c = _num(c, "the constant")
    h = _int(h, "the horizon h", 1, 500)
    p = len(phi)
    if len(x) < p:
        raise ValueError("the series needs at least p values")
    hist = list(x)
    out = []
    for _ in range(h):
        v = c + math.fsum(phi[j] * hist[-1 - j] for j in range(p))
        hist.append(v)
        out.append(v)
    return {"forecasts": out, "forecast": out[-1]}


def ref_ar_forecast(x, phi, c=0.0, h=1):
    """Companion (state-space) matrix raised to the power j."""
    import numpy as np
    p = len(phi)
    F = np.zeros((p + 1, p + 1))
    F[0, :p], F[0, p] = phi, c
    for i in range(1, p):
        F[i, i - 1] = 1.0
    F[p, p] = 1.0
    s = np.r_[np.asarray(x, dtype=float)[::-1][:p], 1.0]
    out = [float((np.linalg.matrix_power(F, j) @ s)[0]) for j in range(1, int(h) + 1)]
    return {"forecasts": out, "forecast": out[-1]}


def _rnd_arf(rng):
    p = rng.choice([1, 2, 2, 3, 3, 4])
    return {"x": _ar1(rng, rng.randint(p, 20)), "phi": _rand_stationary(rng, p),
            "c": round(rng.uniform(-5, 5), 2), "h": rng.randint(1, 10)}


_cap("ar_forecast", level="advanced", name="Forecasts of an AR(p) model", topic=_ARM,
     purpose="h-step forecasts of x_t = c + phi_1 x_{t-1} + ... + phi_p x_{t-p} + e_t from the last p observations "
             "(future shocks set to 0).",
     inputs=[_vin("x", "the series, oldest first (at least p values)", 1), _vin("phi", "the coefficients phi_1..phi_p", 1),
             _nin("c", "the constant c (default 0)", required=False),
             _iin("h", "forecast horizon (default 1)", required=False, lo=1)],
     outputs=[_out("forecasts", "the forecasts for steps 1..h"), _out("forecast", "the h-step forecast")],
     primary="forecast", conditions=["phi_1 multiplies the latest observation"],
     method="recursion xhat_{n+j} = c + sum_i phi_i xhat_{n+j-i} (observed values where available)",
     latex=r"\hat x_{n+j} = c + \sum_{i=1}^{p}\phi_i\,\hat x_{n+j-i}", run=run_ar_forecast, reference=ref_ar_forecast,
     wrong=lambda x, phi, c=0.0, h=1: run_ar_forecast(x, list(phi)[::-1], c, h), rnd=_rnd_arf,
     need=[r"forecast|predict", r"\bar\s*\(|autoregress", r"coefficients|\bphi\b", r"\["],
     avoid=r"arima|least squares|yule|\bfit\b|estimate|variance",
     aliases={"c": _nm("constant", "intercept"), "h": A_H},
     labels={"x": _SERIES_LABEL, "phi": r"coefficients?|\bphi\b"}, extract=_x_ar_order("phi", None),
     templates=["Forecast 3 steps ahead with the AR(2) model with coefficients [0.5, 0.2] and constant 1 from the series [10, 11, 12]."],
     examples=[dict(x="10, 11, 12", phi="0.5, 0.2", c=1, h=3), dict(x=[4.0, 5.5], phi=[0.8], h=2)],
     questions=[_q("Forecast 3 steps ahead with the AR(2) model with coefficients [0.5, 0.2] and constant 1 from the "
                   "series [10, 11, 12].", 0),
                _q("Forecast 2 steps ahead for the series [4.0, 5.5] with the AR(1) model with coefficients [0.8].", 1),
                _q("Forecast 3 steps ahead with the AR(3) model with coefficients [0.5, 0.2] from the series [10, 11, 12].",
                   outcome="OUT_OF_SCOPE")])


def _arima_order_check(rest, d, spans):
    m = re.search(r"\barima\s*\(\s*(\d+)\s*,\s*1\s*,\s*0\s*\)", rest)
    if m and "phi" in d:
        n = len([t for t in re.split(r"[,\s;]+", d["phi"].strip("[]")) if t])
        if n != int(m.group(1)):
            return {"_out_of_scope": f"ARIMA({m.group(1)},1,0) was named but {n} coefficients were given"}


def run_arima_forecast(x, phi, h=1):
    x = _fin(x)
    phi = _fin(phi, "the coefficients")
    h = _int(h, "the horizon h", 1, 500)
    p = len(phi)
    if len(x) < p + 2:
        raise ValueError("the series needs at least p + 2 values")
    if not _stationary(phi):
        raise ValueError("the AR part must be stationary (every root outside the unit circle)")
    dx = [x[t] - x[t - 1] for t in range(1, len(x))]
    level, out = x[-1], []
    for _ in range(h):
        v = math.fsum(phi[j] * dx[-1 - j] for j in range(p))
        dx.append(v)
        level += v
        out.append(level)
    return {"forecasts": out, "forecast": out[-1]}


def ref_arima_forecast(x, phi, h=1):
    """Kalman filter of the state-space ARIMA(p,1,0) model (statsmodels), then its forecasts."""
    import numpy as np
    from statsmodels.tsa.arima.model import ARIMA
    m = ARIMA(np.asarray(x, dtype=float), order=(len(phi), 1, 0), trend="n")
    f = m.filter(np.r_[np.asarray(phi, dtype=float), 1.0])
    out = [float(v) for v in f.forecast(int(h))]
    return {"forecasts": out, "forecast": out[-1]}


_cap("arima_p10_forecast", level="advanced", name="Forecasts of an ARIMA(p,1,0) model", topic=_ARM,
     purpose="h-step forecasts of an ARIMA(p,1,0) model without constant: the differences follow a stationary AR(p); "
             "forecast differences are accumulated onto the last observation.",
     inputs=[_vin("x", "the series, oldest first (at least p + 2 values)", 3), _vin("phi", "AR coefficients of the differences", 1),
             _iin("h", "forecast horizon (default 1)", required=False, lo=1)],
     outputs=[_out("forecasts", "the forecasts for steps 1..h"), _out("forecast", "the h-step forecast")],
     primary="forecast", conditions=["stationary AR part", "no constant (no drift)", "n >= p + 2"],
     method="forecast d_t = x_t - x_{t-1} by the AR(p) recursion, xhat_{n+j} = xhat_{n+j-1} + dhat_{n+j}",
     latex=r"(1-\phi_1L-\dots-\phi_pL^p)(1-L)x_t = e_t", run=run_arima_forecast, reference=ref_arima_forecast,
     wrong=lambda x, phi, h=1: run_ar_forecast(x, phi, 0.0, h),
     rnd=lambda rng: (lambda p: {"x": _walk(rng, rng.randint(p + 4, 40)), "phi": _rand_stationary(rng, p),
                                 "h": rng.randint(1, 8)})(rng.randint(1, 3)),
     need=[r"\barima\s*\(\s*\d\s*,\s*1\s*,\s*0\s*\)", r"forecast|predict"],
     aliases={"h": A_H}, labels={"x": _SERIES_LABEL, "phi": r"coefficients?|\bphi\b"}, extract=_arima_order_check,
     templates=["Forecast 3 steps ahead with an ARIMA(1,1,0) model with coefficients [0.4] for the series [100, 102, 105, 104, 108]."],
     examples=[dict(x="100, 102, 105, 104, 108", phi="0.4", h=3), dict(x=[50, 51, 53, 52, 55, 57], phi=[0.5, -0.2], h=2)],
     questions=[_q("Forecast 3 steps ahead with an ARIMA(1,1,0) model with coefficients [0.4] for the series "
                   "[100, 102, 105, 104, 108].", 0),
                _q("Forecast 2 steps ahead with an ARIMA(2,1,0) model with coefficients [0.5, -0.2] for the series "
                   "[50, 51, 53, 52, 55, 57].", 1),
                _q("Forecast with an ARIMA(1,1,0) model with coefficients [1.2] for the series [1, 2, 4, 7].",
                   outcome="INVALID_INPUT")])


# ======================================================================= ARMA theoretical properties ====
_ART = "ARMA theory"


def _psi(phi, theta, K):
    psi = [1.0]
    for j in range(1, K + 1):
        v = theta[j - 1] if j <= len(theta) else 0.0
        v += math.fsum(phi[i - 1] * psi[j - i] for i in range(1, min(j, len(phi)) + 1))
        psi.append(v)
    return psi


def _arma_acov(phi, theta, sigma2, K):
    """Exact autocovariances gamma_0..gamma_K of a stationary ARMA(p, q) (linear system for gamma_0..gamma_p,
    then the AR recursion)."""
    p, q = len(phi), len(theta)
    th = [1.0] + list(theta)
    psi = _psi(phi, theta, q)
    rhs = [sigma2 * math.fsum(th[j] * psi[j - k] for j in range(k, q + 1)) for k in range(max(p, q) + 1 + K + 1)]
    A = [[0.0] * (p + 1) for _ in range(p + 1)]
    for k in range(p + 1):
        A[k][k] += 1.0
        for i in range(1, p + 1):
            A[k][abs(k - i)] -= phi[i - 1]
    g = _solve(A, rhs[:p + 1]) if p else [rhs[0]]
    for k in range(len(g), K + 1):
        g.append(math.fsum(phi[i - 1] * g[k - i] for i in range(1, p + 1)) + (rhs[k] if k < len(rhs) else 0.0))
    return g[:K + 1]


def _arma_check(phi, theta, need_invertible=False):
    phi = _fin(phi, "the AR coefficients")
    theta = _fin(theta, "the MA coefficients")
    if not _stationary(phi):
        raise ValueError("the AR part must be stationary (every root outside the unit circle)")
    if need_invertible and not _stationary([-t for t in theta]):
        raise ValueError("the MA part must be invertible (every root outside the unit circle)")
    while phi and phi[-1] == 0:
        phi = phi[:-1]
    while theta and theta[-1] == 0:
        theta = theta[:-1]
    return phi, theta


def _snap(v, scale):
    return 0.0 if abs(v) <= 1e-13 * max(1.0, scale) else v


def run_ma_acf(theta, K=None):
    _phi, th = _arma_check([], theta)
    q = len(theta)
    K = q + 1 if K is None else _int(K, "the number of lags K", 1, 200)
    g = _arma_acov([], th, 1.0, K)
    acf = [v / g[0] for v in g[1:]]
    return {"acf": acf, "rho1": acf[0]}


def ref_ma_acf(theta, K=None):
    import numpy as np
    from statsmodels.tsa.arima_process import arma_acf
    K = len(theta) + 1 if K is None else int(K)
    r = arma_acf(np.r_[1.0], np.r_[1.0, np.asarray(theta, dtype=float)], lags=K + 1)
    return {"acf": [float(v) for v in r[1:]], "rho1": float(r[1])}


def _rnd_theta(rng, q=None):
    q = rng.randint(1, 4) if q is None else q
    return [round(rng.uniform(-0.9, 0.9), 2) for _ in range(q)]


_K_LAGS = [r"(?:up to|through|first)\s+lag\s*(\d+)", r"\bk\s*=\s*(\d+)", r"(?:first\s+)?(\d+)\s+lags\b"]

_cap("ma_theoretical_acf", level="advanced", name="Theoretical autocorrelations of an MA(q) process", topic=_ART,
     purpose="The autocorrelations rho_1..rho_K of x_t = e_t + theta_1 e_{t-1} + ... + theta_q e_{t-q}; they vanish "
             "beyond lag q.",
     inputs=[_vin("theta", "the MA coefficients theta_1..theta_q (plus-sign convention)", 1, max_length=12),
             _iin("K", "number of lags (default q + 1)", required=False, lo=1)],
     outputs=[_out("acf", "rho_1..rho_K"), _out("rho1", "rho_1")], primary="rho1",
     conditions=["x_t = e_t + sum theta_j e_{t-j} (plus signs)"],
     method="rho_k = (theta_k + sum_{j=1}^{q-k} theta_j theta_{j+k}) / (1 + sum theta_j^2), 0 for k > q",
     latex=r"\rho_k = \frac{\theta_k + \sum_{j=1}^{q-k}\theta_j\theta_{j+k}}{1+\sum_{j=1}^{q}\theta_j^2}",
     run=run_ma_acf, reference=ref_ma_acf,
     wrong=lambda theta, K=None: (lambda o: {"acf": [v * (1 + sum(t * t for t in theta)) for v in o["acf"]],
                                              "rho1": o["rho1"] * (1 + sum(t * t for t in theta))})(run_ma_acf(theta, K)),
     rnd=lambda rng: {"theta": _rnd_theta(rng), "K": rng.randint(1, 8)},
     need=[r"\bma\s*\(\s*\d+\s*\)|moving[- ]average (?:process|model)", r"auto-?correlation|\bacf\b"],
     avoid=r"arma|arima|\bphi\b|partial", aliases={"K": _K_LAGS},
     extract=lambda rest, d, spans: _x_ma_order(rest, d),
     templates=["Find the autocorrelations of the MA(2) process with theta = [0.5, 0.3]."],
     examples=[dict(theta="0.5, 0.3"), dict(theta=[0.8], K=3), dict(theta="-0.4, 0.2, 0.6", K=4)],
     questions=[_q("Find the autocorrelations of the MA(2) process with theta = [0.5, 0.3].", 0),
                _q("Compute the theoretical ACF up to lag 3 of the MA(1) model with theta = [0.8].", 1),
                _q("Find the autocorrelations up to lag 4 of the MA(3) process with theta = [-0.4, 0.2, 0.6].", 2),
                _q("Find the autocorrelations of the MA(2) process with theta = [0.5].", outcome="OUT_OF_SCOPE")])


def _x_ma_order(rest, d):
    m = re.search(r"\bma\s*\(\s*(\d+)\s*\)", rest)
    if m and "theta" in d:
        n = len([t for t in re.split(r"[,\s;]+", d["theta"].strip("[]")) if t])
        if n != int(m.group(1)):
            return {"_out_of_scope": f"MA({m.group(1)}) was named but {n} coefficients were given"}


def run_arma11_acf(phi, theta, K=5):
    phi, theta = _num(phi, "phi"), _num(theta, "theta")
    if not abs(phi) < 1:
        raise ValueError("|phi| must be less than 1 (stationarity)")
    K = _int(K, "the number of lags K", 1, 200)
    r1 = (1 + phi * theta) * (phi + theta) / (1 + 2 * phi * theta + theta * theta)
    acf = [r1]
    for _ in range(K - 1):
        acf.append(phi * acf[-1])
    return {"acf": acf, "rho1": r1}


def ref_arma11_acf(phi, theta, K=5):
    from statsmodels.tsa.arima_process import arma_acf
    r = arma_acf([1.0, -phi], [1.0, theta], lags=int(K) + 1)
    return {"acf": [float(v) for v in r[1:]], "rho1": float(r[1])}


_cap("arma11_theoretical_acf", level="advanced", name="Theoretical autocorrelations of an ARMA(1,1) process",
     topic=_ART,
     purpose="The autocorrelations rho_1..rho_K of x_t = phi x_{t-1} + e_t + theta e_{t-1}.",
     inputs=[_nin("phi", "AR coefficient, |phi| < 1"), _nin("theta", "MA coefficient (plus-sign convention)"),
             _iin("K", "number of lags (default 5)", required=False, lo=1)],
     outputs=[_out("acf", "rho_1..rho_K"), _out("rho1", "rho_1")], primary="rho1",
     conditions=["|phi| < 1", "x_t = phi x_{t-1} + e_t + theta e_{t-1}"],
     method="rho_1 = (1 + phi theta)(phi + theta)/(1 + 2 phi theta + theta^2); rho_k = phi rho_{k-1}",
     latex=r"\rho_1 = \frac{(1+\phi\theta)(\phi+\theta)}{1+2\phi\theta+\theta^2},\ \rho_k = \phi\,\rho_{k-1}",
     run=run_arma11_acf, reference=ref_arma11_acf,
     wrong=lambda phi, theta, K=5: (lambda r1: {"acf": [r1 * phi ** j for j in range(int(K))], "rho1": r1})(phi + theta),
     rnd=lambda rng: {"phi": round(rng.uniform(-0.9, 0.9), 2), "theta": round(rng.uniform(-0.9, 0.9), 2),
                      "K": rng.randint(1, 8)},
     need=[r"\barma\s*\(\s*1\s*,\s*1\s*\)", r"auto-?correlation|\bacf\b"], avoid=r"\[|variance|partial",
     aliases={"phi": _nm("phi"), "theta": _nm("theta"), "K": _K_LAGS},
     templates=["Find the autocorrelations of the ARMA(1,1) process with phi = 0.6 and theta = 0.3."],
     examples=[dict(phi=0.6, theta=0.3), dict(phi=-0.5, theta=0.8, K=3)],
     questions=[_q("Find the autocorrelations of the ARMA(1,1) process with phi = 0.6 and theta = 0.3.", 0),
                _q("Compute the theoretical ACF up to lag 3 of the ARMA(1,1) model with phi = -0.5 and theta = 0.8.", 1),
                _q("Find the autocorrelations of the ARMA(1,1) process with phi = 1.2 and theta = 0.3.",
                   outcome="INVALID_INPUT")])


_PHI_LABEL = r"\bphi\b|\bar (?:coefficients|part|parameters)|autoregressive (?:coefficients|part)"
_THETA_LABEL = r"\btheta\b|\bma (?:coefficients|part|parameters)|moving[- ]average (?:coefficients|part)"


def _arma_order(rest, d, spans):
    m = re.search(r"\barma\s*\(\s*(\d+)\s*,\s*(\d+)\s*\)", rest)
    if m:
        for sym, want in (("phi", m.group(1)), ("theta", m.group(2))):
            if sym in d:
                n = len([t for t in re.split(r"[,\s;]+", d[sym].strip("[]")) if t])
                if n != int(want) and not (want == "0" and d[sym].strip("[] ") in ("0", "0.0")):
                    return {"_out_of_scope": f"ARMA({m.group(1)},{m.group(2)}) was named but the coefficient lists differ"}


def _rnd_arma(rng):
    return {"phi": _rand_stationary(rng, rng.randint(1, 3), -0.7, 0.7), "theta": _rnd_theta(rng, rng.randint(1, 3))}


def _psi_roots(phi, theta, K):
    """Reference route: 1/phi(L) as the product of geometric series in the inverse roots, times theta(L)."""
    import numpy as np
    roots = np.roots(np.r_[1.0, -np.asarray(phi, dtype=float)]) if len(phi) else np.array([])
    c = np.zeros(K + 1, dtype=complex)
    c[0] = 1.0
    for r in roots:                       # multiply by 1/(1 - r L): c'_j = c_j + r c'_{j-1}
        for j in range(1, K + 1):
            c[j] += r * c[j - 1]
    return np.real(np.convolve(c, np.r_[1.0, np.asarray(theta, dtype=float)])[:K + 1])


def run_arma_acf(phi, theta, K=5):
    phi, theta = _arma_check(phi, theta)
    K = _int(K, "the number of lags K", 1, 200)
    g = _arma_acov(phi, theta, 1.0, K)
    acf = [v / g[0] for v in g[1:]]
    return {"acf": acf, "rho1": acf[0]}


def ref_arma_acf(phi, theta, K=5):
    """MA(infinity) route: gamma_k = sum_j psi_j psi_{j+k} over 6000 psi weights."""
    import numpy as np
    psi = _psi_roots(phi, theta, 6000 + int(K))
    g = [float(np.dot(psi[:6000], psi[k:6000 + k])) for k in range(int(K) + 1)]
    acf = [v / g[0] for v in g[1:]]
    return {"acf": acf, "rho1": acf[0]}


_cap("arma_theoretical_acf", level="expert", name="Theoretical autocorrelations of an ARMA(p,q) process", topic=_ART,
     purpose="The autocorrelations rho_1..rho_K of a stationary ARMA(p,q) process "
             "x_t = sum phi_i x_{t-i} + e_t + sum theta_j e_{t-j}.",
     inputs=[_vin("phi", "AR coefficients phi_1..phi_p (stationary; [0] for none)", 1, max_length=8),
             _vin("theta", "MA coefficients theta_1..theta_q ([0] for none)", 1, max_length=8),
             _iin("K", "number of lags (default 5)", required=False, lo=1)],
     outputs=[_out("acf", "rho_1..rho_K"), _out("rho1", "rho_1")], primary="rho1",
     conditions=["stationary AR part", "plus-sign MA convention"],
     method="solve the linear equations for gamma_0..gamma_p, then gamma_k = sum phi_i gamma_{k-i} + sigma^2 sum theta_j psi_{j-k}",
     latex=r"\gamma_k - \sum_{i=1}^{p}\phi_i\gamma_{k-i} = \sigma^2\sum_{j=k}^{q}\theta_j\psi_{j-k}",
     run=run_arma_acf, reference=ref_arma_acf,
     wrong=lambda phi, theta, K=5: run_arma_acf(phi, [0.0], K),
     rnd=lambda rng: dict(_rnd_arma(rng), K=rng.randint(1, 8)),
     need=[r"\barma\s*\(|autoregressive moving[- ]average", r"auto-?correlation|\bacf\b", r"\["],
     avoid=r"variance|partial|psi|forecast", aliases={"K": _K_LAGS},
     labels={"phi": _PHI_LABEL, "theta": _THETA_LABEL}, extract=_arma_order,
     templates=["Find the autocorrelations of the ARMA(2,1) process with phi = [0.5, 0.2] and theta = [0.4]."],
     examples=[dict(phi="0.5, 0.2", theta="0.4"), dict(phi=[0.7], theta=[0.2, -0.3], K=4)],
     questions=[_q("Find the autocorrelations of the ARMA(2,1) process with phi = [0.5, 0.2] and theta = [0.4].", 0),
                _q("Compute the theoretical ACF up to lag 4 of the ARMA(1,2) model with phi = [0.7] and "
                   "theta = [0.2, -0.3].", 1),
                _q("Find the autocorrelations of the ARMA(1,1) process with phi = [1.1] and theta = [0.4].",
                   outcome="INVALID_INPUT")])


def run_arma_var(phi, theta, sigma2):
    phi, theta = _arma_check(phi, theta)
    s2 = _pos(sigma2, "the innovation variance")
    g = _arma_acov(phi, theta, s2, 1)
    return {"gamma0": g[0], "gamma1": g[1]}


def ref_arma_var(phi, theta, sigma2):
    import numpy as np
    from statsmodels.tsa.arima_process import arma_acovf
    g = arma_acovf(np.r_[1.0, -np.asarray(phi, dtype=float)], np.r_[1.0, np.asarray(theta, dtype=float)], nobs=2,
                   sigma2=sigma2)
    return {"gamma0": float(g[0]), "gamma1": float(g[1])}


_cap("arma_variance", level="expert", name="Variance and lag-1 autocovariance of an ARMA(p,q) process", topic=_ART,
     purpose="The process variance gamma_0 and lag-1 autocovariance gamma_1 of a stationary ARMA(p,q) process with "
             "innovation variance sigma^2.",
     inputs=[_vin("phi", "AR coefficients ([0] for none)", 1, max_length=8),
             _vin("theta", "MA coefficients ([0] for none)", 1, max_length=8),
             _nin("sigma2", "innovation (white-noise) variance sigma^2")],
     outputs=[_out("gamma0", "the variance of the process"), _out("gamma1", "the lag-1 autocovariance")],
     primary="gamma0", conditions=["stationary AR part", "sigma^2 > 0"],
     method="exact linear equations for the autocovariances", latex=r"\gamma_0 = \sigma^2\sum_{j\ge0}\psi_j^2",
     run=run_arma_var, reference=ref_arma_var,
     wrong=lambda phi, theta, sigma2: {"gamma0": sigma2 * (1 + sum(t * t for t in theta)) / (1 - sum(f * f for f in phi)),
                                       "gamma1": 0.0},
     rnd=lambda rng: dict(_rnd_arma(rng), sigma2=round(rng.uniform(0.2, 9), 2)),
     need=[r"\barma\s*\(|autoregressive moving[- ]average", r"variance|autocovariance", r"\["],
     avoid=r"forecast|partial", labels={"phi": _PHI_LABEL, "theta": _THETA_LABEL},
     aliases={"sigma2": _nm("sigma2", r"sigma\^2", "sigma squared", r"(?:white[- ]noise |innovation |noise |error )variance")},
     extract=_arma_order,
     templates=["Find the variance of the ARMA(1,1) process with phi = [0.5], theta = [0.4] and sigma2 = 2."],
     examples=[dict(phi="0.5", theta="0.4", sigma2=2), dict(phi=[0.6, -0.2], theta=[0.3], sigma2=1)],
     questions=[_q("Find the variance of the ARMA(1,1) process with phi = [0.5], theta = [0.4] and sigma2 = 2.", 0),
                _q("Compute the autocovariance of the ARMA(2,1) model with phi = [0.6, -0.2], theta = [0.3] and "
                   "white noise variance 1.", 1),
                _q("Find the variance of the ARMA(1,1) process with phi = [0.5] and theta = [0.4].", outcome="NEEDS_INPUT")])


def run_psi(phi, theta, K=10):
    phi, theta = _arma_check(phi, theta)
    K = _int(K, "the number of weights K", 1, 500)
    psi = _psi(phi, theta, K)[1:]
    return {"psi": psi, "psi1": psi[0]}


def ref_psi(phi, theta, K=10):
    psi = _psi_roots(phi, theta, int(K))
    return {"psi": [float(v) for v in psi[1:]], "psi1": float(psi[1])}


def wrong_psi(phi, theta, K=10):  # psi_j = phi_j + theta_j (the recursion left out)
    w = [(phi[j] if j < len(phi) else 0.0) + (theta[j] if j < len(theta) else 0.0) for j in range(int(K))]
    return {"psi": w, "psi1": w[0]}


_cap("psi_weights", level="advanced", name="Psi weights (MA-infinity representation) of an ARMA process", topic=_ART,
     purpose="The weights psi_1..psi_K of x_t = sum_{j>=0} psi_j e_{t-j} (psi_0 = 1) for a stationary ARMA(p,q); "
             "they are also the impulse response.",
     inputs=[_vin("phi", "AR coefficients ([0] for none)", 1, max_length=8),
             _vin("theta", "MA coefficients ([0] for none)", 1, max_length=8),
             _iin("K", "number of weights (default 10)", required=False, lo=1)],
     outputs=[_out("psi", "psi_1..psi_K"), _out("psi1", "psi_1 = phi_1 + theta_1")], primary="psi1",
     conditions=["stationary AR part", "plus-sign MA convention"],
     method="psi_j = theta_j + sum_{i=1}^{min(j,p)} phi_i psi_{j-i}",
     latex=r"\psi_j = \theta_j + \sum_{i=1}^{\min(j,p)}\phi_i\,\psi_{j-i},\ \psi_0 = 1",
     run=run_psi, reference=ref_psi, wrong=wrong_psi,
     rnd=lambda rng: dict(_rnd_arma(rng), K=rng.randint(2, 15)),
     need=r"psi[- ]weights|\bpsi\b|ma\s*\(\s*(?:infinity|inf|∞)\s*\)|wold|impulse response",
     labels={"phi": _PHI_LABEL, "theta": _THETA_LABEL}, aliases={"K": [r"first\s+(\d+)", r"\bk\s*=\s*(\d+)", r"(\d+)\s+(?:psi )?weights"]},
     extract=_arma_order,
     templates=["Find the first 5 psi weights of the ARMA(1,1) process with phi = [0.5] and theta = [0.3]."],
     examples=[dict(phi="0.5", theta="0.3", K=5), dict(phi=[0.6, -0.2], theta=[0.4, 0.1], K=6)],
     questions=[_q("Find the first 5 psi weights of the ARMA(1,1) process with phi = [0.5] and theta = [0.3].", 0),
                _q("Compute the first 6 psi weights of the ARMA(2,2) model with phi = [0.6, -0.2] and theta = [0.4, 0.1].", 1)])


def run_pi(phi, theta, K=10):
    phi, theta = _arma_check(phi, theta, need_invertible=True)
    K = _int(K, "the number of weights K", 1, 500)
    c = [1.0]
    for j in range(1, K + 1):
        a = -phi[j - 1] if j <= len(phi) else 0.0
        c.append(a - math.fsum(theta[i - 1] * c[j - i] for i in range(1, min(j, len(theta)) + 1)))
    pi = [-v for v in c[1:]]
    return {"pi": pi, "pi1": pi[0]}


def ref_pi(phi, theta, K=10):
    import numpy as np
    K = int(K)
    m = np.roots(np.r_[1.0, np.asarray(theta, dtype=float)]) if len(theta) else np.array([])
    c = np.zeros(K + 1, dtype=complex)
    c[0] = 1.0
    for r in m:
        c = np.convolve(c, (-1.0) ** 0 * r ** np.arange(K + 1))[:K + 1]
    c = np.real(np.convolve(c, np.r_[1.0, -np.asarray(phi, dtype=float)])[:K + 1])
    return {"pi": [float(-v) for v in c[1:]], "pi1": float(-c[1])}


def wrong_pi(phi, theta, K=10):   # pi_j = phi_j - theta_j (the recursion left out)
    w = [(phi[j] if j < len(phi) else 0.0) - (theta[j] if j < len(theta) else 0.0) for j in range(int(K))]
    return {"pi": w, "pi1": w[0]}


_cap("pi_weights", level="advanced", name="Pi weights (AR-infinity representation) of an ARMA process", topic=_ART,
     purpose="The weights pi_1..pi_K of x_t = sum_{j>=1} pi_j x_{t-j} + e_t for an invertible ARMA(p,q).",
     inputs=[_vin("phi", "AR coefficients ([0] for none)", 1, max_length=8),
             _vin("theta", "MA coefficients ([0] for none), invertible", 1, max_length=8),
             _iin("K", "number of weights (default 10)", required=False, lo=1)],
     outputs=[_out("pi", "pi_1..pi_K"), _out("pi1", "pi_1 = phi_1 + theta_1")], primary="pi1",
     conditions=["invertible MA part", "stationary AR part", "plus-sign MA convention"],
     method="coefficients c_j of phi(L)/theta(L); pi_j = -c_j",
     latex=r"\pi(L) = 1 - \sum_{j\ge1}\pi_j L^j = \frac{\phi(L)}{\theta(L)}", run=run_pi, reference=ref_pi,
     wrong=wrong_pi, rnd=lambda rng: dict(_rnd_arma(rng), K=rng.randint(2, 15)),
     need=r"pi[- ]weights|\bar\s*\(\s*(?:infinity|inf|∞)\s*\)|autoregressive representation",
     labels={"phi": _PHI_LABEL, "theta": _THETA_LABEL}, aliases={"K": [r"first\s+(\d+)", r"\bk\s*=\s*(\d+)", r"(\d+)\s+(?:pi )?weights"]},
     extract=_arma_order,
     templates=["Find the first 5 pi weights of the ARMA(1,1) process with phi = [0.5] and theta = [0.3]."],
     examples=[dict(phi="0.5", theta="0.3", K=5), dict(phi=[0], theta=[0.6], K=4)],
     questions=[_q("Find the first 5 pi weights of the ARMA(1,1) process with phi = [0.5] and theta = [0.3].", 0),
                _q("Compute the first 4 pi weights of the MA(1) model with phi = [0] and theta = [0.6].", 1),
                _q("Find the first 4 pi weights of the ARMA(1,1) process with phi = [0.5] and theta = [1.5].",
                   outcome="INVALID_INPUT")])


def run_fev(phi, theta, sigma2, h):
    phi, theta = _arma_check(phi, theta)
    s2 = _pos(sigma2, "the innovation variance")
    h = _int(h, "the horizon h", 1, 1000)
    psi = _psi(phi, theta, h - 1)
    v = s2 * math.fsum(w * w for w in psi)
    return {"variance": v, "sd": math.sqrt(v)}


def ref_fev(phi, theta, sigma2, h):
    import numpy as np
    psi = _psi_roots(phi, theta, int(h) - 1)
    v = float(sigma2 * np.sum(psi ** 2))
    return {"variance": v, "sd": float(np.sqrt(v))}


_cap("forecast_error_variance", level="advanced", name="h-step forecast error variance of an ARMA model",
     topic=_ART,
     purpose="The variance (and standard deviation) of the h-step-ahead forecast error of a stationary ARMA(p,q) "
             "with known parameters: sigma^2 (psi_0^2 + ... + psi_{h-1}^2).",
     inputs=[_vin("phi", "AR coefficients ([0] for none)", 1, max_length=8),
             _vin("theta", "MA coefficients ([0] for none)", 1, max_length=8),
             _nin("sigma2", "innovation variance sigma^2"), _iin("h", "forecast horizon", lo=1)],
     outputs=[_out("variance", "forecast error variance"), _out("sd", "forecast error standard deviation")],
     primary="variance", conditions=["stationary AR part", "known parameters (no estimation error)"],
     method="psi weights, then sigma^2 sum_{j=0}^{h-1} psi_j^2",
     latex=r"\operatorname{Var}(e_n(h)) = \sigma^2\sum_{j=0}^{h-1}\psi_j^2", run=run_fev, reference=ref_fev,
     wrong=lambda phi, theta, sigma2, h: (lambda v: {"variance": v, "sd": math.sqrt(v)})(
         sigma2 * sum(w * w for w in _psi(phi, theta, int(h))[1:])),
     rnd=lambda rng: dict(_rnd_arma(rng), sigma2=round(rng.uniform(0.2, 9), 2), h=rng.randint(1, 12)),
     need=r"forecast error variance|variance of the (?:\d+[- ]step[- ]ahead |h[- ]step[- ]ahead )?forecast errors?",
     labels={"phi": _PHI_LABEL, "theta": _THETA_LABEL},
     aliases={"sigma2": _nm("sigma2", r"sigma\^2", "sigma squared", r"(?:white[- ]noise |innovation |noise )variance"),
              "h": A_H + [r"(\d+)[- ]step"]},
     extract=_arma_order,
     templates=["Find the 3-step forecast error variance of the ARMA(1,1) model with phi = [0.5], theta = [0.3] and sigma2 = 2."],
     examples=[dict(phi="0.5", theta="0.3", sigma2=2, h=3), dict(phi=[0.8], theta=[0], sigma2=1, h=5)],
     questions=[_q("Find the 3-step forecast error variance of the ARMA(1,1) model with phi = [0.5], theta = [0.3] "
                   "and sigma2 = 2.", 0),
                _q("What is the variance of the 5-step-ahead forecast error of the ARMA(1,0) model with phi = [0.8], "
                   "theta = [0] and sigma2 = 1?", 1)])


def run_ar1_props(phi, sigma2, k=1, c=0.0):
    phi = _num(phi, "phi")
    if not abs(phi) < 1:
        raise ValueError("|phi| must be less than 1 (stationarity)")
    s2 = _pos(sigma2, "the innovation variance")
    k = _int(k, "the lag k", 0, 500)
    c = _num(c, "the constant")
    g0 = s2 / (1 - phi * phi)
    return {"mean": c / (1 - phi), "variance": g0, "acf_k": phi ** k, "acov_k": g0 * phi ** k}


def ref_ar1_props(phi, sigma2, k=1, c=0.0):
    """statsmodels' ARMA autocovariances; the mean as the fixed point of m <- c + phi m iterated."""
    from statsmodels.tsa.arima_process import arma_acovf
    g = arma_acovf([1.0, -phi], [1.0], nobs=int(k) + 1, sigma2=sigma2)
    m = 0.0
    for _ in range(20000):
        m = c + phi * m
    return {"mean": m, "variance": float(g[0]), "acf_k": float(g[int(k)] / g[0]), "acov_k": float(g[int(k)])}


_SIG2 = _nm("sigma2", r"sigma\^2", "sigma squared", r"(?:white[- ]noise |innovation |noise |error )variance")

_cap("ar1_properties", level="intermediate", name="Mean, variance and autocorrelation of an AR(1) process",
     topic=_ART,
     purpose="For a stationary AR(1) x_t = c + phi x_{t-1} + e_t: the mean c/(1 - phi), the variance "
             "sigma^2/(1 - phi^2), and the lag-k autocorrelation phi^k and autocovariance.",
     inputs=[_nin("phi", "AR coefficient, |phi| < 1"), _nin("sigma2", "innovation variance sigma^2"),
             _iin("k", "the lag k (default 1)", required=False, lo=0), _nin("c", "the constant c (default 0)", required=False)],
     outputs=[_out("mean", "c / (1 - phi)"), _out("variance", "sigma^2 / (1 - phi^2)"), _out("acf_k", "phi^k"),
              _out("acov_k", "variance x phi^k")],
     primary="variance", conditions=["|phi| < 1", "sigma^2 > 0"],
     method="stationary moments of the AR(1) recursion",
     latex=r"\mu = \frac{c}{1-\phi},\ \gamma_0 = \frac{\sigma^2}{1-\phi^2},\ \rho_k = \phi^k",
     run=run_ar1_props, reference=ref_ar1_props,
     wrong=lambda phi, sigma2, k=1, c=0.0: {"mean": c / (1 - phi), "variance": sigma2 / (1 - phi),
                                            "acf_k": phi ** k, "acov_k": sigma2 / (1 - phi) * phi ** k},
     rnd=lambda rng: {"phi": round(rng.uniform(-0.95, 0.95), 2), "sigma2": round(rng.uniform(0.1, 10), 2),
                      "k": rng.randint(0, 8), "c": round(rng.uniform(-10, 10), 2)},
     need=[r"\bar\s*\(\s*1\s*\)|first[- ]order autoregress", r"variance|\bmean\b|autocorrelation|autocovariance|propert"],
     avoid=r"\[|forecast|yule|least squares|stationar|estimat|\bfit\b",
     aliases={"phi": _nm("phi"), "sigma2": _SIG2, "k": A_LAG, "c": _nm("constant", "intercept")},
     templates=["Find the mean and variance of the AR(1) process with constant 2, phi = 0.6 and sigma2 = 4."],
     examples=[dict(phi=0.6, sigma2=4, c=2), dict(phi=-0.5, sigma2=1, k=3)],
     questions=[_q("Find the mean and variance of the AR(1) process with constant 2, phi = 0.6 and sigma2 = 4.", 0),
                _q("Find the lag 3 autocorrelation and variance of the AR(1) process with phi = -0.5 and white noise "
                   "variance 1.", 1),
                _q("Find the variance of the AR(1) process with phi = 1.1 and sigma2 = 1.", outcome="INVALID_INPUT"),
                _q("Find the variance of the AR(1) process with phi = 0.5.", outcome="NEEDS_INPUT")])


def _quad_inverse_roots(b, c):
    """Inverse roots of z^2 + b z + c = 0: (max modulus, 'real' or 'complex')."""
    disc = b * b - 4 * c
    if disc >= 0:
        s = math.sqrt(disc)
        return max(abs((-b + s) / 2), abs((-b - s) / 2)), "real"
    return math.sqrt(c), "complex"


def run_ar2_stat(phi1, phi2):
    phi1, phi2 = _num(phi1, "phi1"), _num(phi2, "phi2")
    m, kind = _quad_inverse_roots(-phi1, -phi2)
    return {"stationary": "yes" if m < 1 else "no", "max_modulus": m, "roots": kind}


def ref_ar2_stat(phi1, phi2):
    import numpy as np
    from statsmodels.tsa.arima_process import ArmaProcess
    r = np.roots([1.0, -phi1, -phi2])
    return {"stationary": "yes" if ArmaProcess(np.r_[1.0, -phi1, -phi2]).isstationary else "no",
            "max_modulus": float(np.max(np.abs(r))), "roots": "complex" if np.iscomplexobj(r) and np.any(np.abs(r.imag) > 0) else "real"}


def run_ma2_inv(theta1, theta2):
    t1, t2 = _num(theta1, "theta1"), _num(theta2, "theta2")
    m, kind = _quad_inverse_roots(t1, t2)
    return {"invertible": "yes" if m < 1 else "no", "max_modulus": m, "roots": kind}


def ref_ma2_inv(theta1, theta2):
    import numpy as np
    from statsmodels.tsa.arima_process import ArmaProcess
    r = np.roots([1.0, theta1, theta2])
    return {"invertible": "yes" if ArmaProcess([1.0], np.r_[1.0, theta1, theta2]).isinvertible else "no",
            "max_modulus": float(np.max(np.abs(r))), "roots": "complex" if np.any(np.abs(r.imag) > 0) else "real"}


def _rnd_pair(rng, ma=False):
    for _ in range(100):
        a, b = round(rng.uniform(-2, 2), 2), round(rng.uniform(-1.2, 1.2), 2)
        disc = a * a - 4 * b if ma else a * a + 4 * b
        mod = (_quad_inverse_roots(a, b) if ma else _quad_inverse_roots(-a, -b))[0]
        if abs(disc) > 1e-6 and abs(mod - 1) > 1e-3:
            return a, b
    return 0.5, 0.2


def _pair_alias(name):
    return [r"\b" + name + r"\s*_?\s*" + i + r"\b\s*" + _W + "(" + _NUM + ")" for i in ("1", "2")]


_cap("ar2_stationarity", level="intermediate", name="Stationarity of an AR(2) process", topic=_ART,
     purpose="Whether x_t = phi1 x_{t-1} + phi2 x_{t-2} + e_t is stationary: the largest modulus of the inverse "
             "characteristic roots (roots of z^2 - phi1 z - phi2) must be below 1.",
     inputs=[_nin("phi1", "phi_1"), _nin("phi2", "phi_2")],
     outputs=[_out("stationary", "yes or no"), _out("max_modulus", "largest modulus of the inverse roots"),
              _out("roots", "real or complex")],
     primary="max_modulus", conditions=["a modulus of exactly 1 (unit root) counts as not stationary"],
     method="roots of z^2 - phi1 z - phi2 by the quadratic formula",
     latex=r"z^2 - \phi_1 z - \phi_2 = 0,\quad \text{stationary} \iff \max|z_i| < 1",
     run=run_ar2_stat, reference=ref_ar2_stat,
     wrong=lambda phi1, phi2: {"stationary": "yes" if phi1 + phi2 < 1 else "no", "max_modulus": abs(phi1) + abs(phi2),
                               "roots": "real"},
     rnd=lambda rng: dict(zip(("phi1", "phi2"), _rnd_pair(rng))),
     need=[r"\bar\s*\(\s*2\s*\)", r"stationar|unit circle|characteristic roots"], avoid=r"\[",
     aliases={"phi1": _pair_alias("phi")[:1], "phi2": _pair_alias("phi")[1:]},
     templates=["Is the AR(2) process with phi1 = 0.5 and phi2 = 0.3 stationary?"],
     examples=[dict(phi1=0.5, phi2=0.3), dict(phi1=1.2, phi2=-0.8), dict(phi1=0.6, phi2=0.5)],
     questions=[_q("Is the AR(2) process with phi1 = 0.5 and phi2 = 0.3 stationary?", 0),
                _q("Check the stationarity of the AR(2) model with phi1 = 1.2 and phi2 = -0.8.", 1),
                _q("Is the AR(2) process with phi1 = 0.6 and phi2 = 0.5 stationary?", 2),
                _q("Is the AR(2) process with phi1 = 0.5 stationary?", outcome="NEEDS_INPUT")])

_cap("ma2_invertibility", level="advanced", name="Invertibility of an MA(2) process", topic=_ART,
     purpose="Whether x_t = e_t + theta1 e_{t-1} + theta2 e_{t-2} is invertible: the largest modulus of the inverse "
             "roots (roots of z^2 + theta1 z + theta2) must be below 1.",
     inputs=[_nin("theta1", "theta_1"), _nin("theta2", "theta_2")],
     outputs=[_out("invertible", "yes or no"), _out("max_modulus", "largest modulus of the inverse roots"),
              _out("roots", "real or complex")],
     primary="max_modulus", conditions=["plus-sign MA convention", "a modulus of exactly 1 counts as not invertible"],
     method="roots of z^2 + theta1 z + theta2 by the quadratic formula",
     latex=r"z^2 + \theta_1 z + \theta_2 = 0,\quad \text{invertible} \iff \max|z_i| < 1",
     run=run_ma2_inv, reference=ref_ma2_inv,
     wrong=lambda theta1, theta2: {"invertible": "yes" if theta1 + theta2 < 1 else "no",
                                   "max_modulus": abs(theta1) + abs(theta2), "roots": "real"},
     rnd=lambda rng: dict(zip(("theta1", "theta2"), _rnd_pair(rng, True))),
     need=[r"\bma\s*\(\s*2\s*\)", r"invertib"], avoid=r"\[",
     aliases={"theta1": _pair_alias("theta")[:1], "theta2": _pair_alias("theta")[1:]},
     templates=["Is the MA(2) process with theta1 = 0.5 and theta2 = 0.3 invertible?"],
     examples=[dict(theta1=0.5, theta2=0.3), dict(theta1=-1.5, theta2=0.9)],
     questions=[_q("Is the MA(2) process with theta1 = 0.5 and theta2 = 0.3 invertible?", 0),
                _q("Check the invertibility of the MA(2) model with theta1 = -1.5 and theta2 = 0.9.", 1),
                _q("Is the MA(2) process with theta1 = 0.5 invertible?", outcome="NEEDS_INPUT")])


# ===================================================================================== cross-correlation ====
_XC = "cross-correlation"
_XL, _YL = r"\bx\b|first series", r"\by\b|second series"


def _xc_in(x, y):
    x, y = _fin(x, "x"), _fin(y, "y")
    _same_len(x, y, "x", "y")
    _need(x, 3, "x")
    _nonconstant(x, "x")
    _nonconstant(y, "y")
    return x, y


def _ccf(x, y, k):
    """r_xy(k) = corr(x_{t+k}, y_t), divided by n, about the sample means."""
    n = len(x)
    mx, my = _mean(x), _mean(y)
    sx = math.sqrt(math.fsum((v - mx) ** 2 for v in x) / n)
    sy = math.sqrt(math.fsum((v - my) ** 2 for v in y) / n)
    if k >= 0:
        c = math.fsum((x[t + k] - mx) * (y[t] - my) for t in range(n - k))
    else:
        c = math.fsum((x[t] - mx) * (y[t - k] - my) for t in range(n + k))
    return c / n / (sx * sy)


def run_ccf(x, y, k):
    x, y = _xc_in(x, y)
    k = _int(k, "the lag k", 0, len(x) - 1)
    return {"ccf": _ccf(x, y, k)}


def ref_ccf(x, y, k):
    from statsmodels.tsa.stattools import ccf
    return {"ccf": float(ccf(x, y, adjusted=False, fft=True)[int(k)])}


def _rnd_xc(rng):
    n = rng.randint(10, 50)
    s = rng.randint(-4, 4)
    base = _ar1(rng, n + 10, phi=rng.uniform(0, 0.8))
    x = base[5:5 + n]
    y = [round(base[5 + t + s] + rng.gauss(0, 1.5), 3) for t in range(n)]
    return {"x": x, "y": y}


_cap("cross_correlation_at_lag", level="intermediate", name="Sample cross-correlation at lag k", topic=_XC,
     purpose="The sample cross-correlation r_xy(k) = corr(x_{t+k}, y_t) of two equally long series (y leads x by k).",
     inputs=[_vin("x", "the first series", 3), _vin("y", "the second series (same length)", 3),
             _iin("k", "the lag k >= 0", lo=0)],
     outputs=[_out("ccf", "the cross-correlation r_xy(k)")], primary="ccf",
     conditions=["equal lengths", "0 <= k <= n - 1", "denominator n, standard deviations with divisor n"],
     method="r_xy(k) = (1/n) sum_t (x_{t+k} - xbar)(y_t - ybar) / (s_x s_y)",
     latex=r"r_{xy}(k) = \frac{\frac1n\sum_{t=1}^{n-k}(x_{t+k}-\bar x)(y_t-\bar y)}{s_x s_y}",
     run=run_ccf, reference=ref_ccf,
     wrong=lambda x, y, k: {"ccf": _ccf(y, x, int(k))},
     rnd=lambda rng: dict(_rnd_xc(rng), k=rng.randint(1, 5)),
     need=r"cross[- ]?correlation", avoid=r"which lag|best lag|maxim|strongest|peak|largest|lag at which|\bleads?\b",
     aliases={"k": A_LAG}, labels={"x": _XL, "y": _YL},
     templates=["Compute the cross-correlation at lag 1 between x = [1, 3, 2, 5, 4, 6] and y = [2, 1, 4, 3, 6, 5]."],
     examples=[dict(x="1, 3, 2, 5, 4, 6", y="2, 1, 4, 3, 6, 5", k=1),
               dict(x=[10, 12, 11, 15, 14, 18, 17], y=[8, 11, 10, 12, 16, 15, 19], k=2)],
     questions=[_q("Compute the cross-correlation at lag 1 between x = [1, 3, 2, 5, 4, 6] and y = [2, 1, 4, 3, 6, 5].", 0),
                _q("Find the sample cross-correlation at lag 2 of x = [10, 12, 11, 15, 14, 18, 17] and "
                   "y = [8, 11, 10, 12, 16, 15, 19].", 1),
                _q("Compute the cross-correlation at lag 1 between x = [1, 3, 2] and y = [2, 1, 4, 3].",
                   outcome="INVALID_INPUT")])


def run_best_lag(x, y, K):
    x, y = _xc_in(x, y)
    K = _int(K, "the largest lag K", 0, len(x) - 1)
    best = None
    for k in sorted(range(-K, K + 1), key=lambda j: (abs(j), -j)):
        r = _ccf(x, y, k)
        if best is None or abs(r) > abs(best[1]):
            best = (k, r)
    return {"lag": best[0], "ccf": best[1]}


def ref_best_lag(x, y, K):
    """FFT cross-correlation of the centred series (scipy.signal.correlate) over every lag."""
    import numpy as np
    from scipy.signal import correlate, correlation_lags
    a, b = np.asarray(x, dtype=float), np.asarray(y, dtype=float)
    a, b = a - a.mean(), b - b.mean()
    c = correlate(a, b, mode="full", method="fft") / (len(a) * a.std() * b.std())
    lags = correlation_lags(len(a), len(b), mode="full")
    sel = [(int(l), float(v)) for l, v in zip(lags, c) if abs(l) <= int(K)]
    sel.sort(key=lambda t: (-round(abs(t[1]), 12), abs(t[0]), -t[0]))
    return {"lag": sel[0][0], "ccf": sel[0][1]}


_cap("cross_correlation_best_lag", level="advanced", name="Lag of the strongest cross-correlation", topic=_XC,
     purpose="The lag k in -K..K with the largest |r_xy(k)|, r_xy(k) = corr(x_{t+k}, y_t): a positive k means y "
             "leads x by k periods.",
     inputs=[_vin("x", "the first series", 3), _vin("y", "the second series (same length)", 3),
             _iin("K", "the largest lag searched", lo=0)],
     outputs=[_out("lag", "the lag with the largest absolute cross-correlation"), _out("ccf", "r_xy at that lag")],
     primary="ccf", conditions=["equal lengths", "K <= n - 1", "ties: the smallest |k|, then the positive k"],
     method="r_xy(k) for every k in -K..K, the largest in absolute value",
     latex=r"k^* = \arg\max_{|k|\le K}|r_{xy}(k)|", run=run_best_lag, reference=ref_best_lag,
     wrong=lambda x, y, K: (lambda o: {"lag": -o["lag"], "ccf": o["ccf"] if o["lag"] == 0 else _ccf(x, y, -o["lag"])})(
         run_best_lag(x, y, K)),
     rnd=lambda rng: dict(_rnd_xc(rng), K=rng.randint(4, 8)),
     need=[r"cross[- ]?correlation", r"which lag|best lag|maxim|strongest|peak|largest|lag at which|\bleads?\b"],
     aliases={"K": [r"(?:max(?:imum)? lag|up to lag|lags? up to|largest lag)\s*" + _W + r"(\d+)", r"\bk\s*=\s*(\d+)"]},
     labels={"x": _XL, "y": _YL},
     templates=["At which lag up to 3 is the cross-correlation strongest between x = [1, 3, 2, 5, 4, 6, 5, 8] and y = [3, 2, 5, 4, 6, 5, 8, 7]?"],
     examples=[dict(x="1, 3, 2, 5, 4, 6, 5, 8", y="3, 2, 5, 4, 6, 5, 8, 7", K=3),
               dict(x=[10, 12, 11, 15, 14, 18, 17, 20], y=[12, 11, 15, 14, 18, 17, 20, 19], K=2)],
     questions=[_q("At which lag up to 3 is the cross-correlation strongest between x = [1, 3, 2, 5, 4, 6, 5, 8] and "
                   "y = [3, 2, 5, 4, 6, 5, 8, 7]?", 0),
                _q("Find the lag with the largest cross-correlation, max lag 2, of x = [10, 12, 11, 15, 14, 18, 17, 20] "
                   "and y = [12, 11, 15, 14, 18, 17, 20, 19].", 1)])


# ====================================================================================== forecast accuracy ====
_FA = "forecast accuracy"
_ACT = r"actual|observ|true values|realis|realiz|test values|outcomes"
_FC = r"forecasts?|predict|fitted"
_TRAIN = r"training|\btrain\b|in-sample|histor|past"
_FA_IN = [_vin("y", "the actual values", 1), _vin("f", "the forecasts (same length)", 1)]


def _errors(y, f):
    y, f = _fin(y, "the actual values"), _fin(f, "the forecasts")
    _same_len(y, f)
    return y, f, [a - b for a, b in zip(y, f)]


def _rnd_fa(rng, positive=False):
    y = _walk(rng, rng.randint(3, 30)) if positive or rng.random() < 0.7 else _ar1(rng, rng.randint(3, 30))
    sd = (max(y) - min(y) + 1) * rng.uniform(0.05, 0.4)
    return {"y": y, "f": [round(v + rng.gauss(rng.uniform(-0.5, 0.5) * sd, sd), 2) for v in y]}


def run_mae(y, f):
    y, f, e = _errors(y, f)
    return {"mae": math.fsum(abs(v) for v in e) / len(e), "n": len(e)}


def ref_mae(y, f):
    """Identity route: |e| = 2 max(e, 0) - e, so MAE = 2 mean(e+) - mean(e)."""
    import numpy as np
    e = np.asarray(y, dtype=float) - np.asarray(f, dtype=float)
    return {"mae": float(2 * np.maximum(e, 0).mean() - e.mean()), "n": len(e)}


def run_mse(y, f):
    y, f, e = _errors(y, f)
    return {"mse": math.fsum(v * v for v in e) / len(e), "n": len(e)}


def _ref_mse(y, f):
    """Bias-variance route: MSE = (mean error)^2 + population variance of the errors."""
    import numpy as np
    e = np.asarray(y, dtype=float) - np.asarray(f, dtype=float)
    return float(e.mean() ** 2 + e.var())


def run_mape(y, f):
    y, f, e = _errors(y, f)
    if any(v == 0 for v in y):
        raise ValueError("MAPE is undefined when an actual value is 0")
    return {"mape": 100 * math.fsum(abs(a / b) for a, b in zip(e, y)) / len(e), "n": len(e)}


def ref_mape(y, f):
    import numpy as np
    y = np.asarray(y, dtype=float)
    p = (y - np.asarray(f, dtype=float)) / y
    return {"mape": float(100 * (2 * np.maximum(p, 0).mean() - p.mean())), "n": len(y)}


def run_me(y, f):
    y, f, e = _errors(y, f)
    return {"mean_error": math.fsum(e) / len(e), "n": len(e)}


def run_smape(y, f):
    y, f, e = _errors(y, f)
    if any(abs(a) + abs(b) == 0 for a, b in zip(y, f)):
        raise ValueError("sMAPE is undefined when an actual value and its forecast are both 0")
    return {"smape": 100 * math.fsum(2 * abs(a - b) / (abs(a) + abs(b)) for a, b in zip(y, f)) / len(y), "n": len(y)}


def ref_smape(y, f):
    import numpy as np
    from sktime.performance_metrics.forecasting import mean_absolute_percentage_error
    v = mean_absolute_percentage_error(np.asarray(y, dtype=float), np.asarray(f, dtype=float), symmetric=True)
    return {"smape": 100 * float(v), "n": len(y)}


def run_mase(y_train, y, f, sp=1):
    tr = _fin(y_train, "the training data")
    y, f, e = _errors(y, f)
    sp = _int(sp, "the seasonal period sp", 1)
    if len(tr) <= sp:
        raise ValueError("the training data must be longer than the seasonal period")
    scale = math.fsum(abs(tr[t] - tr[t - sp]) for t in range(sp, len(tr))) / (len(tr) - sp)
    if scale == 0:
        raise ValueError("the naive in-sample error is 0 (constant training data): MASE is undefined")
    mae = math.fsum(abs(v) for v in e) / len(e)
    return {"mase": mae / scale, "mae": mae, "scale": scale}


def ref_mase(y_train, y, f, sp=1):
    import numpy as np
    from sktime.performance_metrics.forecasting import mean_absolute_scaled_error
    v = mean_absolute_scaled_error(np.asarray(y, dtype=float), np.asarray(f, dtype=float), sp=int(sp),
                                   y_train=np.asarray(y_train, dtype=float))
    return {"mase": float(v)}


_FA_Q = "actual values [10, 12, 14, 13] and forecasts [11, 12, 13, 15]"
_FA_Q2 = "actual values [102.5, 98.0, 110.2, 105.7, 99.9] and forecasts [100.0, 101.0, 107.5, 104.0, 103.2]"
_FA_EX = [dict(y="10, 12, 14, 13", f="11, 12, 13, 15"),
          dict(y=[102.5, 98.0, 110.2, 105.7, 99.9], f=[100.0, 101.0, 107.5, 104.0, 103.2])]


def _metric(key, level, name, abbr, need, avoid, purpose, out, run, ref, wrong, method, latex, cond=(), positive=False):
    _cap(key, level=level, name=name, topic=_FA, purpose=purpose, inputs=_FA_IN,
         outputs=[_out(out, name), _out("n", "number of forecasts")], primary=out,
         conditions=["equal lengths", "errors e_t = actual - forecast"] + list(cond), method=method, latex=latex,
         run=run, reference=ref, wrong=wrong, rnd=(lambda rng, _p=positive: _rnd_fa(rng, _p)), need=need, avoid=avoid,
         labels={"y": _ACT, "f": _FC},
         templates=[f"Find the {abbr} for {_FA_Q}."], examples=list(_FA_EX),
         questions=[_q(f"Find the {abbr} for {_FA_Q}.", 0), _q(f"Compute the {name.lower()} for {_FA_Q2}.", 1),
                    _q(f"Find the {abbr} for actual values [1, 2, 3] and forecasts [1, 2].", outcome="INVALID_INPUT"),
                    _q(f"Find the {abbr} for forecasts [1, 2, 3].", outcome="NEEDS_INPUT")])


_metric("mean_absolute_error", "beginner", "Mean absolute error", "MAE", r"\bmae\b|mean absolute error",
        r"percent|scaled|median|squared", "The mean absolute error of forecasts against actual values.", "mae",
        run_mae, ref_mae, lambda y, f: {"mae": abs(sum(a - b for a, b in zip(y, f))) / len(y), "n": len(y)},
        "mean of |actual - forecast|", r"\mathrm{MAE} = \frac1n\sum_{t=1}^{n}|y_t - f_t|")
_metric("mean_squared_error", "beginner", "Mean squared error", "MSE", r"\bmse\b|mean squared? error",
        r"\broot\b|rmse", "The mean squared error of forecasts against actual values.", "mse", run_mse,
        lambda y, f: {"mse": _ref_mse(y, f), "n": len(y)},
        lambda y, f: {"mse": sum((a - b) ** 2 for a, b in zip(y, f)) / max(1, len(y) - 1), "n": len(y)},
        "mean of (actual - forecast)^2", r"\mathrm{MSE} = \frac1n\sum_{t=1}^{n}(y_t - f_t)^2")
_metric("root_mean_squared_error", "beginner", "Root mean squared error", "RMSE", r"\brmse\b|root mean squared? error",
        (), "The root mean squared error of forecasts against actual values.", "rmse",
        lambda y, f: (lambda o: {"rmse": math.sqrt(o["mse"]), "n": o["n"]})(run_mse(y, f)),
        lambda y, f: {"rmse": math.sqrt(_ref_mse(y, f)), "n": len(y)},
        lambda y, f: {"rmse": math.sqrt(sum((a - b) ** 2 for a, b in zip(y, f)) / max(1, len(y) - 1)), "n": len(y)},
        "square root of the mean squared error", r"\mathrm{RMSE} = \sqrt{\frac1n\sum_{t=1}^{n}(y_t - f_t)^2}")
_metric("mean_absolute_percentage_error", "beginner", "Mean absolute percentage error", "MAPE",
        r"\bmape\b|mean absolute percent(?:age)? error", r"symmetric|smape",
        "The mean absolute percentage error (in percent) of forecasts against actual values.", "mape", run_mape,
        ref_mape, lambda y, f: {"mape": 100 * sum(abs((a - b) / b) for a, b in zip(y, f)) / len(y), "n": len(y)},
        "100 x mean of |actual - forecast| / |actual|", r"\mathrm{MAPE} = \frac{100}{n}\sum_{t=1}^{n}\left|\frac{y_t - f_t}{y_t}\right|",
        cond=["no actual value is 0", "the result is in percent"], positive=True)
_metric("mean_forecast_error", "beginner", "Mean forecast error (bias)", "mean forecast error",
        r"mean (?:forecast )?error\b|forecast bias|\bbias\b", r"absolute|squared|square|percent|scaled|variance",
        "The mean forecast error (bias): positive when the forecasts are too low on average.", "mean_error", run_me,
        lambda y, f: {"mean_error": float(__import__("numpy").mean(y) - __import__("numpy").mean(f)), "n": len(y)},
        lambda y, f: {"mean_error": sum(b - a for a, b in zip(y, f)) / len(y), "n": len(y)},
        "mean of actual - forecast (computed by the reference as mean(actual) - mean(forecast))",
        r"\mathrm{ME} = \frac1n\sum_{t=1}^{n}(y_t - f_t)")
_metric("symmetric_mape", "intermediate", "Symmetric mean absolute percentage error", "sMAPE",
        r"\bsmape\b|symmetric mean absolute percent(?:age)? error", (),
        "The symmetric MAPE in percent: 100/n sum 2|y - f| / (|y| + |f|).", "smape", run_smape, ref_smape,
        lambda y, f: {"smape": 100 * sum(abs(a - b) / (abs(a) + abs(b)) for a, b in zip(y, f)) / len(y), "n": len(y)},
        "100 x mean of 2|actual - forecast| / (|actual| + |forecast|)",
        r"\mathrm{sMAPE} = \frac{100}{n}\sum_{t=1}^{n}\frac{2|y_t - f_t|}{|y_t| + |f_t|}",
        cond=["no pair with actual and forecast both 0", "the result is in percent (range 0 to 200)"])


def _rnd_mase(rng):
    d = _rnd_fa(rng)
    n = rng.randint(4, 30)
    tr = _walk(rng, n)
    sp = rng.choice([1, 1, 4, 2])
    return {"y_train": tr, "y": d["y"], "f": d["f"], "sp": sp if n > sp + 1 else 1}


_cap("mean_absolute_scaled_error", level="intermediate", name="Mean absolute scaled error", topic=_FA,
     purpose="MASE: the forecasts' mean absolute error divided by the in-sample mean absolute error of the (seasonal) "
             "naive forecast on the training data.",
     inputs=[_vin("y_train", "the training (in-sample) data", 2), _vin("y", "the actual values", 1),
             _vin("f", "the forecasts (same length as y)", 1),
             _iin("sp", "seasonal period of the naive benchmark (default 1)", required=False, lo=1)],
     outputs=[_out("mase", "the mean absolute scaled error"), _out("mae", "the forecasts' MAE"),
              _out("scale", "in-sample MAE of the lag-sp naive forecast")],
     primary="mase", conditions=["the training data are longer than sp and not constant at lag sp"],
     method="MAE(actual, forecast) / mean |y_train_t - y_train_{t-sp}|",
     latex=r"\mathrm{MASE} = \frac{\frac1h\sum|y_t - f_t|}{\frac{1}{T-m}\sum_{t=m+1}^{T}|y_t - y_{t-m}|}",
     run=run_mase, reference=ref_mase,
     wrong=lambda y_train, y, f, sp=1: (lambda o: dict(o, mase=o["mae"] / (sum(abs(a - b) for a, b in zip(y_train[1:], y_train[:-1])) / len(y_train))))(run_mase(y_train, y, f, sp)),
     rnd=_rnd_mase, need=r"\bmase\b|mean absolute scaled error",
     aliases={"sp": [r"seasonal period\s*" + _W + r"(\d+)", r"\bsp\s*=\s*(\d+)", r"\bm\s*=\s*(\d+)"]},
     labels={"y_train": _TRAIN, "y": _ACT, "f": _FC},
     templates=["Find the MASE for training data [10, 12, 11, 14, 13], actual values [15, 14, 16] and forecasts [14, 15, 15]."],
     examples=[dict(y_train="10, 12, 11, 14, 13", y="15, 14, 16", f="14, 15, 15"),
               dict(y_train=[20, 30, 25, 22, 31, 27], y=[24, 33], f=[23, 30], sp=3)],
     questions=[_q("Find the MASE for training data [10, 12, 11, 14, 13], actual values [15, 14, 16] and forecasts "
                   "[14, 15, 15].", 0),
                _q("Compute the mean absolute scaled error with seasonal period 3 for training data "
                   "[20, 30, 25, 22, 31, 27], actual values [24, 33] and forecasts [23, 30].", 1),
                _q("Find the MASE for actual values [15, 14, 16] and forecasts [14, 15, 15].", outcome="NEEDS_INPUT")])


# ============================================================================================ trend fitting ====
_TF = "trend fitting"
_TREND_H = {"h": A_H}


def _t(n):
    return [float(i) for i in range(1, n + 1)]


def run_linear_trend(x, h=1):
    x = _fin(x)
    _need(x, 3)
    _nonconstant(x)
    h = _int(h, "the horizon h", 1, 10000)
    n = len(x)
    beta, res, _dg = _lstsq([[1.0, t] for t in _t(n)], x)
    r2 = 1 - math.fsum(e * e for e in res) / _sse(x)
    return {"slope": beta[1], "intercept": beta[0], "forecast": beta[0] + beta[1] * (n + h), "r_squared": r2}


def ref_linear_trend(x, h=1):
    from scipy.stats import linregress
    n = len(x)
    r = linregress(_t(n), x)
    return {"slope": float(r.slope), "intercept": float(r.intercept),
            "forecast": float(r.intercept + r.slope * (n + int(h))), "r_squared": float(r.rvalue ** 2)}


_cap("linear_trend_forecast", level="beginner", name="Linear trend fit and forecast", topic=_TF,
     purpose="Fit the least-squares trend line x_t = a + b t (t = 1..n) and forecast period n + h.",
     inputs=[_vin("x", "the series, oldest first", 3), _iin("h", "periods ahead to forecast (default 1)", required=False, lo=1)],
     outputs=[_out("slope", "trend per period b"), _out("intercept", "a (the trend value at t = 0)"),
              _out("forecast", "a + b (n + h)"), _out("r_squared", "coefficient of determination")],
     primary="forecast", conditions=["t = 1..n", "the series is not constant"],
     method="ordinary least squares of x_t on (1, t)", latex=r"\hat x_t = a + b\,t,\quad \hat x_{n+h} = a + b(n+h)",
     run=run_linear_trend, reference=ref_linear_trend,
     wrong=lambda x, h=1: (lambda o: dict(o, forecast=o["intercept"] + o["slope"] * (len(x) - 1 + h)))(run_linear_trend(x, h)),
     rnd=lambda rng: {"x": _walk(rng, rng.randint(3, 40)), "h": rng.randint(1, 10)},
     need=r"linear trend|trend line|least[- ]squares (?:linear )?trend|straight[- ]line trend",
     avoid=r"holt|smoothing|quadratic|exponential|seasonal|dumm|sen'?s|theil|mann|durbin|dickey|kpss|residual", aliases=_TREND_H,
     templates=["Fit a linear trend to [12, 15, 17, 20, 22, 25] and forecast 2 periods ahead."],
     examples=[dict(x="12, 15, 17, 20, 22, 25", h=2), dict(x=[5.2, 5.0, 5.9, 6.3, 6.1, 7.0, 7.4])],
     questions=[_q("Fit a linear trend to [12, 15, 17, 20, 22, 25] and forecast 2 periods ahead.", 0),
                _q("Find the least-squares trend line of [5.2, 5.0, 5.9, 6.3, 6.1, 7.0, 7.4].", 1),
                _q("Fit a linear trend to [4, 4, 4, 4].", outcome="INVALID_INPUT")])


def run_quad_trend(x, h=1):
    x = _fin(x)
    _need(x, 4)
    h = _int(h, "the horizon h", 1, 10000)
    n = len(x)
    beta, _res, _dg = _lstsq([[1.0, t, t * t] for t in _t(n)], x)
    T = n + h
    return {"b0": beta[0], "b1": beta[1], "b2": beta[2], "forecast": beta[0] + beta[1] * T + beta[2] * T * T}


def ref_quad_trend(x, h=1):
    import numpy as np
    c2, c1, c0 = np.polyfit(np.arange(1, len(x) + 1, dtype=float), np.asarray(x, dtype=float), 2)
    T = len(x) + int(h)
    return {"b0": float(c0), "b1": float(c1), "b2": float(c2), "forecast": float(c0 + c1 * T + c2 * T * T)}


_cap("quadratic_trend_forecast", level="intermediate", name="Quadratic trend fit and forecast", topic=_TF,
     purpose="Fit x_t = b0 + b1 t + b2 t^2 (t = 1..n) by least squares and forecast period n + h.",
     inputs=[_vin("x", "the series, oldest first", 4), _iin("h", "periods ahead (default 1)", required=False, lo=1)],
     outputs=[_out("b0", "constant"), _out("b1", "coefficient of t"), _out("b2", "coefficient of t^2"),
              _out("forecast", "the trend value at t = n + h")],
     primary="forecast", conditions=["t = 1..n", "n >= 4"], method="least squares (QR) on (1, t, t^2)",
     latex=r"\hat x_t = b_0 + b_1 t + b_2 t^2", run=run_quad_trend, reference=ref_quad_trend,
     wrong=lambda x, h=1: (lambda o: dict(o, forecast=o["b0"] + o["b1"] * (len(x) + h - 1) + o["b2"] * (len(x) + h - 1) ** 2))(run_quad_trend(x, h)),
     rnd=lambda rng: {"x": _walk(rng, rng.randint(4, 40)), "h": rng.randint(1, 8)},
     need=r"quadratic trend", aliases=_TREND_H,
     templates=["Fit a quadratic trend to [3, 5, 9, 15, 22, 32] and forecast 1 period ahead."],
     examples=[dict(x="3, 5, 9, 15, 22, 32", h=1), dict(x=[100, 98, 99, 103, 110, 120, 133], h=3)],
     questions=[_q("Fit a quadratic trend to [3, 5, 9, 15, 22, 32] and forecast 1 period ahead.", 0),
                _q("Fit a quadratic trend to [100, 98, 99, 103, 110, 120, 133] and forecast 3 periods ahead.", 1)])


def run_exp_trend(x, h=1):
    x = _fin(x)
    _need(x, 3)
    _positive(x)
    h = _int(h, "the horizon h", 1, 10000)
    n = len(x)
    beta, _res, _dg = _lstsq([[1.0, t] for t in _t(n)], [math.log(v) for v in x])
    a = math.exp(beta[0])
    return {"a": _r12(a), "b": _r12(beta[1]), "growth_rate": _r12(math.expm1(beta[1])),
            "forecast": _r12(a * math.exp(beta[1] * (n + h)))}


def ref_exp_trend(x, h=1):
    import numpy as np
    b, c = np.polyfit(np.arange(1, len(x) + 1, dtype=float), np.log(np.asarray(x, dtype=float)), 1)
    return {"a": float(np.exp(c)), "b": float(b), "growth_rate": float(np.exp(b) - 1),
            "forecast": float(np.exp(c + b * (len(x) + int(h))))}


_cap("exponential_trend_forecast", level="intermediate", name="Exponential trend fit and forecast", topic=_TF,
     purpose="Fit x_t = a e^(b t) (t = 1..n) by least squares on ln x_t, the growth rate per period e^b - 1, and the "
             "forecast of period n + h.",
     inputs=[_vin("x", "the series, oldest first (positive)", 3), _iin("h", "periods ahead (default 1)", required=False, lo=1)],
     outputs=[_out("a", "the scale a"), _out("b", "the exponent rate b"), _out("growth_rate", "e^b - 1 per period"),
              _out("forecast", "a e^(b (n + h))")],
     primary="forecast", conditions=["positive values", "least squares on the logarithms (not on the values)"],
     method="ordinary least squares of ln x_t on (1, t)", latex=r"\ln x_t = \ln a + b\,t,\quad \hat x_{n+h} = a\,e^{b(n+h)}",
     run=run_exp_trend, reference=ref_exp_trend,
     wrong=lambda x, h=1: (lambda o: dict(o, growth_rate=o["b"], forecast=x[-1] * (1 + o["b"]) ** h))(run_exp_trend(x, h)),
     rnd=lambda rng: {"x": _walk(rng, rng.randint(3, 40)), "h": rng.randint(1, 8)},
     need=r"exponential (?:growth )?trend|log[- ]linear trend", aliases=_TREND_H,
     templates=["Fit an exponential trend to [100, 112, 124, 140, 155] and forecast 1 period ahead."],
     examples=[dict(x="100, 112, 124, 140, 155", h=1), dict(x=[2.0, 2.3, 2.9, 3.1, 3.8, 4.4], h=2)],
     questions=[_q("Fit an exponential trend to [100, 112, 124, 140, 155] and forecast 1 period ahead.", 0),
                _q("Fit an exponential trend to [2.0, 2.3, 2.9, 3.1, 3.8, 4.4] and forecast 2 periods ahead.", 1),
                _q("Fit an exponential trend to [5, -1, 3].", outcome="INVALID_INPUT")])


def run_sen(x):
    x = _fin(x)
    _need(x, 3)
    n = len(x)
    s = [(x[j] - x[i]) / (j - i) for i in range(n) for j in range(i + 1, n)]
    b = _median(s)
    return {"slope": b, "intercept": _median(x) - b * (n + 1) / 2}


def ref_sen(x):
    from scipy.stats import theilslopes
    r = theilslopes(x, [float(t) for t in range(1, len(x) + 1)])
    return {"slope": float(r[0]), "intercept": float(r[1])}


_cap("sens_slope", level="intermediate", name="Sen's slope (Theil-Sen trend)", topic=_TF,
     purpose="Sen's robust trend slope (median of all pairwise slopes, t = 1..n) and the intercept "
             "median(x) - slope median(t).",
     inputs=[_vin("x", "the series, oldest first", 3)],
     outputs=[_out("slope", "median of (x_j - x_i)/(j - i), i < j"), _out("intercept", "median(x) - slope (n+1)/2")],
     primary="slope", conditions=["t = 1..n"], method="median of the n(n-1)/2 pairwise slopes",
     latex=r"\hat\beta = \operatorname{median}_{i<j}\frac{x_j - x_i}{j - i}", run=run_sen, reference=ref_sen,
     wrong=lambda x: (lambda s: {"slope": s, "intercept": _median(x) - s * (len(x) + 1) / 2})(
         sum((x[j] - x[i]) / (j - i) for i in range(len(x)) for j in range(i + 1, len(x))) / (len(x) * (len(x) - 1) / 2)),
     rnd=lambda rng: {"x": _walk(rng, rng.randint(3, 40))},
     need=r"sen'?s slope|theil[- ]sen", aliases={},
     templates=["Find Sen's slope of [12, 14, 13, 17, 16, 19, 25]."],
     examples=[dict(x="12, 14, 13, 17, 16, 19, 25"), dict(x=[3.2, 3.0, 3.9, 4.1, 3.8, 4.6])],
     questions=[_q("Find Sen's slope of [12, 14, 13, 17, 16, 19, 25].", 0),
                _q("Compute the Theil-Sen trend of [3.2, 3.0, 3.9, 4.1, 3.8, 4.6].", 1)])


def run_mk(x):
    x = _fin(x)
    _need(x, 4)
    n = len(x)
    s = sum((x[j] > x[i]) - (x[j] < x[i]) for i in range(n) for j in range(i + 1, n))
    counts = {}
    for v in x:
        counts[v] = counts.get(v, 0) + 1
    var = (n * (n - 1) * (2 * n + 5) - sum(c * (c - 1) * (2 * c + 5) for c in counts.values())) / 18
    if var <= 0:
        raise ValueError("the series must not be constant")
    z = (s - 1) / math.sqrt(var) if s > 0 else ((s + 1) / math.sqrt(var) if s < 0 else 0.0)
    return {"s": s, "var_s": var, "z": z, "p_value": _r12(math.erfc(abs(z) / math.sqrt(2)))}


def ref_mk(x):
    import pymannkendall as mk
    r = mk.original_test(list(map(float, x)))
    return {"s": float(r.s), "var_s": float(r.var_s), "z": float(r.z), "p_value": float(r.p)}


def _rnd_mk(rng):
    n = rng.randint(6, 40)
    slope = rng.uniform(-0.15, 0.15)
    return {"x": [round(slope * t + rng.gauss(0, 1), rng.choice([1, 2])) for t in range(n)]}


_cap("mann_kendall_test", level="advanced", name="Mann-Kendall trend test", topic=_TF,
     purpose="The Mann-Kendall statistic S, its variance (with the tie correction), the continuity-corrected normal "
             "score z and the two-sided p-value for a monotonic trend.",
     inputs=[_vin("x", "the series, oldest first", 4)],
     outputs=[_out("s", "S = sum_{i<j} sign(x_j - x_i)"), _out("var_s", "variance of S with tie correction"),
              _out("z", "(S - sign S) / sqrt(var S)"), _out("p_value", "two-sided normal p-value")],
     primary="z", conditions=["n >= 4", "ties corrected in the variance", "continuity correction of 1 in z"],
     method="count concordant minus discordant pairs; normal approximation",
     latex=r"S=\sum_{i<j}\operatorname{sgn}(x_j-x_i),\ \operatorname{Var}S=\frac{n(n-1)(2n+5)-\sum t(t-1)(2t+5)}{18}",
     run=run_mk, reference=ref_mk,
     wrong=lambda x: (lambda o: dict(o, z=o["s"] / math.sqrt(o["var_s"]),
                                     p_value=math.erfc(abs(o["s"]) / math.sqrt(2 * o["var_s"]))))(run_mk(x)),
     rnd=_rnd_mk, need=r"mann[- ]kendall",
     templates=["Perform the Mann-Kendall trend test on [12, 14, 13, 17, 16, 19, 25, 21, 24]."],
     examples=[dict(x="12, 14, 13, 17, 16, 19, 25, 21, 24"), dict(x=[5, 3, 5, 6, 4, 7, 7, 8, 6, 9])],
     questions=[_q("Perform the Mann-Kendall trend test on [12, 14, 13, 17, 16, 19, 25, 21, 24].", 0),
                _q("Compute the Mann-Kendall statistic for [5, 3, 5, 6, 4, 7, 7, 8, 6, 9].", 1),
                _q("Perform the Mann-Kendall trend test on [3, 3, 3, 3, 3].", outcome="INVALID_INPUT")])


# ========================================================================================= interpolation ====
_IP = "interpolation of missing values"
_T_LABEL, _Y_LABEL = r"times?|\bt\s*=|positions?|periods|days|years|dates", r"values?|observations|readings|y\s*="


def _interp_in(t, y, t0):
    t, y = _fin(t, "the times"), _fin(y, "the values")
    _same_len(t, y, "the times", "the values")
    _need(t, 2, "the times")
    if any(b <= a for a, b in zip(t, t[1:])):
        raise ValueError("the times must be strictly increasing")
    t0 = _num(t0, "the time t0")
    if not t[0] <= t0 <= t[-1]:
        raise ValueError("t0 must lie between the first and last time (no extrapolation)")
    i = max(j for j in range(len(t) - 1) if t[j] <= t0)
    return t, y, t0, i


def run_lin_interp(t, y, t0):
    t, y, t0, i = _interp_in(t, y, t0)
    w = (t0 - t[i]) / (t[i + 1] - t[i])
    return {"value": y[i] + w * (y[i + 1] - y[i])}


def ref_lin_interp(t, y, t0):
    """Degree-1 B-spline through the points (de Boor evaluation)."""
    from scipy.interpolate import make_interp_spline
    return {"value": float(make_interp_spline(t, y, k=1)(t0))}


def run_exp_interp(t, y, t0):
    t, y, t0, i = _interp_in(t, y, t0)
    _positive(y, "the values")
    w = (t0 - t[i]) / (t[i + 1] - t[i])
    return {"value": _r12(y[i] * (y[i + 1] / y[i]) ** w)}


def ref_exp_interp(t, y, t0):
    import numpy as np
    from scipy.interpolate import make_interp_spline
    return {"value": float(np.exp(make_interp_spline(t, np.log(y), k=1)(t0)))}


def _rnd_interp(rng):
    n = rng.randint(2, 12)
    t = sorted(rng.sample(range(0, 60), n))
    y = _walk(rng, n)
    return {"t": [float(v) for v in t], "y": y, "t0": round(rng.uniform(t[0], t[-1]), 2)}


_IP_IN = [_vin("t", "the times (strictly increasing)", 2), _vin("y", "the values at those times", 2),
          _nin("t0", "the time of the missing value")]
_IP_AL = {"t0": [r"\bat t\s*=\s*(" + _NUM + ")", r"\bt0\s*=\s*(" + _NUM + ")", r"\bat (?:time|t)\s+(" + _NUM + ")"]}

_cap("linear_interpolation", level="beginner", name="Linear interpolation of a missing value", topic=_IP,
     purpose="Fill a missing value at time t0 by the straight line between the neighbouring observations.",
     inputs=_IP_IN, outputs=[_out("value", "the interpolated value at t0")], primary="value",
     conditions=["times strictly increasing", "t0 within the observed times (no extrapolation)"],
     method="y_i + (t0 - t_i)(y_{i+1} - y_i)/(t_{i+1} - t_i) with t_i <= t0 <= t_{i+1}",
     latex=r"\hat y(t_0) = y_i + \frac{t_0 - t_i}{t_{i+1}-t_i}(y_{i+1}-y_i)", run=run_lin_interp,
     reference=ref_lin_interp,
     wrong=lambda t, y, t0: (lambda a: {"value": (y[a[3]] + y[a[3] + 1]) / 2})(_interp_in(t, y, t0)),
     rnd=_rnd_interp, need=r"interpolat", avoid=r"exponential|log[- ]linear|geometric|spline|cubic",
     aliases=_IP_AL, labels={"t": _T_LABEL, "y": _Y_LABEL},
     templates=["Linearly interpolate the value at t = 2.5 from times [1, 2, 3, 4] and values [10, 14, 13, 18]."],
     examples=[dict(t="1, 2, 3, 4", y="10, 14, 13, 18", t0=2.5), dict(t=[0, 5, 10], y=[3.2, 4.7, 4.1], t0=8)],
     questions=[_q("Linearly interpolate the value at t = 2.5 from times [1, 2, 3, 4] and values [10, 14, 13, 18].", 0),
                _q("Interpolate the missing value at t = 8 given times [0, 5, 10] and values [3.2, 4.7, 4.1].", 1),
                _q("Interpolate the value at t = 12 given times [0, 5, 10] and values [3.2, 4.7, 4.1].",
                   outcome="INVALID_INPUT")])

_cap("exponential_interpolation", level="intermediate", name="Exponential (log-linear) interpolation", topic=_IP,
     purpose="Fill a missing value at time t0 assuming constant growth between the neighbouring observations "
             "(linear interpolation of the logarithms).",
     inputs=_IP_IN, outputs=[_out("value", "the interpolated value at t0")], primary="value",
     conditions=["positive values", "times strictly increasing", "t0 within the observed times"],
     method="y_i (y_{i+1}/y_i)^((t0 - t_i)/(t_{i+1} - t_i))",
     latex=r"\hat y(t_0) = y_i\left(\frac{y_{i+1}}{y_i}\right)^{\frac{t_0-t_i}{t_{i+1}-t_i}}", run=run_exp_interp,
     reference=ref_exp_interp, wrong=lambda t, y, t0: run_lin_interp(t, y, t0), rnd=_rnd_interp,
     need=r"exponential(?:ly)? interpolat|log[- ]linear interpolat|geometric(?:ally)? interpolat|interpolat\w* exponentially",
     aliases=_IP_AL, labels={"t": _T_LABEL, "y": _Y_LABEL},
     templates=["Exponentially interpolate the value at t = 2.5 from times [1, 2, 3, 4] and values [100, 120, 150, 170]."],
     examples=[dict(t="1, 2, 3, 4", y="100, 120, 150, 170", t0=2.5), dict(t=[2000, 2010], y=[6.1, 6.9], t0=2004)],
     questions=[_q("Exponentially interpolate the value at t = 2.5 from times [1, 2, 3, 4] and values [100, 120, 150, 170].", 0),
                _q("Use exponential interpolation for the value at t = 2004 given years [2000, 2010] and values [6.1, 6.9].", 1),
                _q("Exponentially interpolate the value at t = 1.5 from times [1, 2] and values [-1, 4].",
                   outcome="INVALID_INPUT")])


# =========================================================================================== change points ====
_CP = "change points"


def _seg_cost(x):
    return _sse(x) if x else 0.0


def run_cp1(x, min_size=2):
    x = _fin(x)
    ms = _int(min_size, "the minimum segment length", 1)
    if len(x) < 2 * ms:
        raise ValueError("the series is too short for two segments of the minimum length")
    best = None
    for k in range(ms, len(x) - ms + 1):
        c = _seg_cost(x[:k]) + _seg_cost(x[k:])
        if best is None or c < best[1]:
            best = (k, c)
    k, c = best
    return {"change_index": k, "mean_before": _mean(x[:k]), "mean_after": _mean(x[k:]), "cost": c,
            "reduction": _sse(x) - c}


def ref_cp1(x, min_size=2):
    import numpy as np
    import ruptures as rpt
    y = np.asarray(x, dtype=float)
    k = int(rpt.Dynp(model="l2", min_size=int(min_size), jump=1).fit(y).predict(n_bkps=1)[0])
    c = float(y[:k].var() * k + y[k:].var() * (len(y) - k))
    return {"change_index": k, "mean_before": float(y[:k].mean()), "mean_after": float(y[k:].mean()), "cost": c,
            "reduction": float(y.var() * len(y) - c)}


def wrong_cp1(x, min_size=2):  # the first index of the NEW segment reported (off by one), its means and cost
    k = run_cp1(x, min_size)["change_index"] + 1
    k = k if k < len(x) else k - 2
    c = _seg_cost(x[:k]) + _seg_cost(x[k:])
    return {"change_index": k, "mean_before": _mean(x[:k]), "mean_after": _mean(x[k:]), "cost": c, "reduction": _sse(x) - c}


def _shifted(rng, n, nseg):
    cuts = sorted(rng.sample(range(2, n - 1), nseg - 1)) if nseg > 1 else []
    edges = [0] + cuts + [n]
    sd = rng.uniform(0.5, 2)
    out = []
    for a, b in zip(edges, edges[1:]):
        mu = rng.uniform(-10, 10)
        out += [round(mu + rng.gauss(0, sd), 3) for _ in range(b - a)]
    return out


_MS = {"min_size": [r"minimum segment (?:length|size)\s*" + _W + r"(\d+)", r"\bmin_size\s*=\s*(\d+)"]}

_cap("change_point_mean_shift", level="advanced", name="Single change point in the mean (least squares)", topic=_CP,
     purpose="The single change point that minimises the total within-segment sum of squares (a shift in the mean), "
             "with the means before and after.",
     inputs=[_vin("x", "the series, oldest first", 2), _iin("min_size", "minimum segment length (default 2)", required=False, lo=1)],
     outputs=[_out("change_index", "k: the first segment is x_1..x_k"), _out("mean_before", "mean of x_1..x_k"),
              _out("mean_after", "mean of x_{k+1}..x_n"), _out("cost", "total within-segment sum of squares"),
              _out("reduction", "sum of squares removed by the split")],
     primary="cost", conditions=["each segment has at least min_size values", "ties: the smallest k"],
     method="exhaustive search over every admissible split", latex=r"\hat k = \arg\min_k\Big[\sum_{t\le k}(x_t-\bar x_{1:k})^2 + \sum_{t>k}(x_t-\bar x_{k+1:n})^2\Big]",
     run=run_cp1, reference=ref_cp1, wrong=wrong_cp1,
     rnd=lambda rng: {"x": _shifted(rng, rng.randint(6, 50), 2), "min_size": rng.choice([1, 2, 2, 3])},
     need=r"change[- ]?point|structural break|mean shift|break ?point|shift in (?:the )?mean",
     avoid=r"cusum|cumulative|\btwo\b|penal|pelt|multiple|tabular|optimal partition",
     aliases=_MS,
     templates=["Find the change point in the mean of [5, 6, 5, 7, 6, 12, 13, 12, 14]."],
     examples=[dict(x="5, 6, 5, 7, 6, 12, 13, 12, 14"), dict(x=[1.2, 0.8, 1.1, 3.9, 4.2, 4.0, 4.4], min_size=3)],
     questions=[_q("Find the change point in the mean of [5, 6, 5, 7, 6, 12, 13, 12, 14].", 0),
                _q("Locate the mean shift with minimum segment length 3 in [1.2, 0.8, 1.1, 3.9, 4.2, 4.0, 4.4].", 1),
                _q("Find the change point in the mean of [5, 6, 7].", outcome="INVALID_INPUT")])


def run_cp2(x, min_size=2):
    x = _fin(x)
    ms = _int(min_size, "the minimum segment length", 1)
    n = len(x)
    if n < 3 * ms:
        raise ValueError("the series is too short for three segments of the minimum length")
    cost = {}

    def c(a, b):
        if (a, b) not in cost:
            cost[(a, b)] = _seg_cost(x[a:b])
        return cost[(a, b)]
    best = None
    for i in range(ms, n - 2 * ms + 1):
        for j in range(i + ms, n - ms + 1):
            v = c(0, i) + c(i, j) + c(j, n)
            if best is None or v < best[2]:
                best = (i, j, v)
    i, j, v = best
    return {"change_1": i, "change_2": j, "means": [_mean(x[:i]), _mean(x[i:j]), _mean(x[j:])], "cost": v}


def ref_cp2(x, min_size=2):
    import numpy as np
    import ruptures as rpt
    y = np.asarray(x, dtype=float)
    i, j, _n = rpt.Dynp(model="l2", min_size=int(min_size), jump=1).fit(y).predict(n_bkps=2)
    segs = [y[:i], y[i:j], y[j:]]
    return {"change_1": int(i), "change_2": int(j), "means": [float(s.mean()) for s in segs],
            "cost": float(sum(s.var() * len(s) for s in segs))}


def wrong_cp2(x, min_size=2):  # first indices of the new segments reported (off by one), their means and cost
    o = run_cp2(x, min_size)
    i, j = o["change_1"] + 1, o["change_2"] + 1
    if j >= len(x):
        i, j = i - 2, j - 2
    return {"change_1": i, "change_2": j, "means": [_mean(x[:i]), _mean(x[i:j]), _mean(x[j:])],
            "cost": _seg_cost(x[:i]) + _seg_cost(x[i:j]) + _seg_cost(x[j:])}


_cap("two_change_points", level="expert", name="Two change points in the mean (optimal segmentation)", topic=_CP,
     purpose="The two change points that minimise the total within-segment sum of squares (three mean levels), found "
             "by exhaustive dynamic search.",
     inputs=[_vin("x", "the series, oldest first", 3), _iin("min_size", "minimum segment length (default 2)", required=False, lo=1)],
     outputs=[_out("change_1", "end of the first segment"), _out("change_2", "end of the second segment"),
              _out("means", "the three segment means"), _out("cost", "total within-segment sum of squares")],
     primary="cost", conditions=["each segment has at least min_size values", "ties: the smallest first, then second index"],
     method="exhaustive search over all pairs of admissible splits",
     latex=r"\min_{i<j}\ \mathrm{SS}(1{:}i)+\mathrm{SS}(i{+}1{:}j)+\mathrm{SS}(j{+}1{:}n)", run=run_cp2, reference=ref_cp2,
     wrong=wrong_cp2,
     rnd=lambda rng: {"x": _shifted(rng, rng.randint(9, 40), 3), "min_size": rng.choice([1, 2, 2, 3])},
     need=r"two change[- ]?points|two (?:mean )?shifts|two breaks|two structural breaks", aliases=_MS,
     word_ok=r"\btwo (?=change|mean|shifts|breaks|structural)",
     templates=["Find two change points in the mean of [5, 6, 5, 12, 13, 12, 20, 21, 19]."],
     examples=[dict(x="5, 6, 5, 12, 13, 12, 20, 21, 19"), dict(x=[1.0, 1.2, 0.9, 4.1, 3.8, 4.3, 4.0, 1.5, 1.1, 1.3], min_size=3)],
     questions=[_q("Find two change points in the mean of [5, 6, 5, 12, 13, 12, 20, 21, 19].", 0),
                _q("Locate two mean shifts with minimum segment length 3 in [1.0, 1.2, 0.9, 4.1, 3.8, 4.3, 4.0, 1.5, 1.1, 1.3].", 1)])


def run_pelt(x, penalty):
    x = _fin(x)
    _need(x, 2)
    pen = _num(penalty, "the penalty")
    if pen <= 0:
        raise ValueError("the penalty must be positive")
    n = len(x)
    F = [0.0] * (n + 1)
    last = [0] * (n + 1)
    for b in range(1, n + 1):
        best = None
        for a in range(0, b):
            v = (F[a] if a else 0.0) + _seg_cost(x[a:b]) + pen
            if best is None or v < best[0]:
                best = (v, a)
        F[b], last[b] = best
    cps, b = [], n
    while last[b] > 0:
        cps.append(last[b])
        b = last[b]
    cps.reverse()
    edges = [0] + cps + [n]
    cost = math.fsum(_seg_cost(x[a:b]) for a, b in zip(edges, edges[1:]))
    return {"n_changes": len(cps), "change_indices": cps, "cost": cost}


def ref_pelt(x, penalty):
    import numpy as np
    import ruptures as rpt
    y = np.asarray(x, dtype=float)
    b = rpt.Pelt(model="l2", min_size=1, jump=1).fit(y).predict(pen=float(penalty))
    edges = [0] + list(b)
    return {"n_changes": len(b) - 1, "change_indices": [int(v) for v in b[:-1]],
            "cost": float(sum(y[a:c].var() * (c - a) for a, c in zip(edges, edges[1:])))}


_cap("penalised_change_points", level="expert", name="Multiple change points with a penalty (PELT)", topic=_CP,
     purpose="The segmentation minimising the within-segment sum of squares plus a penalty per change point "
             "(optimal partitioning, as found by PELT): the number and positions of the changes in the mean.",
     inputs=[_vin("x", "the series, oldest first", 2), _nin("penalty", "penalty per change point (> 0)")],
     outputs=[_out("n_changes", "number of change points"), _out("change_indices", "ends of all segments but the last"),
              _out("cost", "within-segment sum of squares of the chosen segmentation")],
     primary="n_changes", conditions=["penalty > 0", "segments of any length >= 1", "ties: the earlier last change"],
     method="optimal partitioning F(b) = min_a F(a) + SS(a+1:b) + penalty",
     latex=r"\min_{K,\tau}\sum_{i=0}^{K}\mathrm{SS}(\tau_i{+}1{:}\tau_{i+1}) + \beta K", run=run_pelt, reference=ref_pelt,
     wrong=lambda x, penalty: run_pelt(x, 2 * penalty) if run_pelt(x, 2 * penalty)["n_changes"] != run_pelt(x, penalty)["n_changes"] else
     dict(run_pelt(x, penalty), n_changes=run_pelt(x, penalty)["n_changes"] + 1),
     rnd=lambda rng: (lambda x: {"x": x, "penalty": round(rng.uniform(2, 30), 1)})(_shifted(rng, rng.randint(8, 40), rng.randint(1, 4))),
     need=r"\bpelt\b|penali[sz]ed (?:change|segmentation)|optimal partitioning",
     aliases={"penalty": _nm("penalty", "pen")},
     templates=["Find the change points with PELT and penalty 10 in [5, 6, 5, 12, 13, 12, 20, 21, 19, 20]."],
     examples=[dict(x="5, 6, 5, 12, 13, 12, 20, 21, 19, 20", penalty=10),
               dict(x=[1.0, 1.2, 0.9, 1.1, 1.3, 1.0], penalty=5)],
     questions=[_q("Find the change points with PELT and penalty 10 in [5, 6, 5, 12, 13, 12, 20, 21, 19, 20].", 0),
                _q("Run penalized change point detection with penalty 5 on [1.0, 1.2, 0.9, 1.1, 1.3, 1.0].", 1),
                _q("Find the change points with PELT in [5, 6, 5, 12].", outcome="NEEDS_INPUT")])


def run_cusum_cp(x):
    x = _fin(x)
    _need(x, 3)
    m = _mean(x)
    s, best = 0.0, None
    for k in range(1, len(x)):
        s += x[k - 1] - m
        if best is None or abs(s) > best[1]:
            best = (k, abs(s))
    k = best[0]
    return {"change_index": k, "max_cusum": best[1], "mean_before": _mean(x[:k]), "mean_after": _mean(x[k:])}


def ref_cusum_cp(x):
    """|S_k| = k (n - k) / n |mean(x_1..k) - mean(x_k+1..n)|: the segment-means form."""
    import numpy as np
    y = np.asarray(x, dtype=float)
    n = len(y)
    d = np.array([k * (n - k) / n * abs(y[:k].mean() - y[k:].mean()) for k in range(1, n)])
    k = int(np.argmax(d)) + 1
    return {"change_index": k, "max_cusum": float(d[k - 1]), "mean_before": float(y[:k].mean()),
            "mean_after": float(y[k:].mean())}


_cap("cusum_change_point", level="intermediate", name="CUSUM estimate of a change point", topic=_CP,
     purpose="The change point estimated by the cumulative sum of deviations from the overall mean, "
             "S_k = sum_{t<=k}(x_t - xbar): the k with the largest |S_k|.",
     inputs=[_vin("x", "the series, oldest first", 3)],
     outputs=[_out("change_index", "k with the largest |S_k| (first segment x_1..x_k)"), _out("max_cusum", "max |S_k|"),
              _out("mean_before", "mean of x_1..x_k"), _out("mean_after", "mean of x_{k+1}..x_n")],
     primary="max_cusum", conditions=["k = 1..n-1", "ties: the smallest k"],
     method="running sum of deviations from the mean; argmax of its absolute value",
     latex=r"S_k = \sum_{t=1}^{k}(x_t - \bar x),\quad \hat k = \arg\max_k |S_k|", run=run_cusum_cp, reference=ref_cusum_cp,
     wrong=lambda x: (lambda m: (lambda k: {"change_index": k, "max_cusum": abs(sum(x[:k]) - k * x[0]),
                                            "mean_before": _mean(x[:k]), "mean_after": _mean(x[k:])})(
         max(range(1, len(x)), key=lambda j: abs(sum(v - x[0] for v in x[:j])))))(None),
     rnd=lambda rng: {"x": _shifted(rng, rng.randint(6, 50), 2)},
     need=[r"cusum|cumulative sum", r"change[- ]?point|shift|break"],
     avoid=r"tabular|allowance|decision interval|reference value|target",
     templates=["Estimate the change point with CUSUM for [5, 6, 5, 7, 6, 12, 13, 12, 14]."],
     examples=[dict(x="5, 6, 5, 7, 6, 12, 13, 12, 14"), dict(x=[20.1, 19.8, 20.3, 17.2, 16.9, 17.4, 17.0])],
     questions=[_q("Estimate the change point with CUSUM for [5, 6, 5, 7, 6, 12, 13, 12, 14].", 0),
                _q("Use the cumulative sum of deviations to locate the shift in [20.1, 19.8, 20.3, 17.2, 16.9, 17.4, 17.0].", 1)])


def run_cusum_tab(x, mu0, k, h):
    x = _fin(x)
    _need(x, 1)
    mu0, k, h = _num(mu0, "the target"), _num(k, "the allowance k"), _num(h, "the decision interval h")
    if k < 0 or h <= 0:
        raise ValueError("the allowance k must be >= 0 and the decision interval h > 0")
    cp = cm = 0.0
    alarm, side = 0, "none"
    for t, v in enumerate(x, 1):
        cp = max(0.0, cp + v - mu0 - k)
        cm = max(0.0, cm + mu0 - k - v)
        if not alarm and (cp > h or cm > h):
            alarm = t
            side = "both" if cp > h and cm > h else ("upper" if cp > h else "lower")
    return {"c_plus": cp, "c_minus": cm, "alarm_index": alarm, "side": side}


def ref_cusum_tab(x, mu0, k, h):
    """Lindley form: C_t = S_t - min(0, min_{j<=t} S_j) with S the running sum of the increments."""
    import numpy as np
    y = np.asarray(x, dtype=float)
    sp = np.cumsum(y - mu0 - k)
    sm = np.cumsum(mu0 - k - y)
    cp = sp - np.minimum(0.0, np.minimum.accumulate(sp))
    cm = sm - np.minimum(0.0, np.minimum.accumulate(sm))
    hit = np.nonzero((cp > h) | (cm > h))[0]
    if len(hit):
        t = int(hit[0])
        side = "both" if cp[t] > h and cm[t] > h else ("upper" if cp[t] > h else "lower")
        alarm = t + 1
    else:
        alarm, side = 0, "none"
    return {"c_plus": float(cp[-1]), "c_minus": float(cm[-1]), "alarm_index": alarm, "side": side}


def wrong_cusum_tab(x, mu0, k, h):  # no reset at zero: plain cumulative sums
    cp = cm = 0.0
    alarm, side = 0, "none"
    for t, v in enumerate(x, 1):
        cp += v - mu0 - k
        cm += mu0 - k - v
        if not alarm and (cp > h or cm > h):
            alarm, side = t, "upper" if cp > h else "lower"
    return {"c_plus": cp, "c_minus": cm, "alarm_index": alarm, "side": side}


def _rnd_ctab(rng):
    mu0 = round(rng.uniform(-5, 50), 1)
    sd = rng.uniform(0.5, 3)
    n = rng.randint(5, 40)
    shift = rng.choice([0, 1, -1]) * rng.uniform(0.5, 2) * sd
    cut = rng.randint(1, n)
    x = [round(mu0 + (shift if t >= cut else 0) + rng.gauss(0, sd), 2) for t in range(n)]
    return {"x": x, "mu0": mu0, "k": round(0.5 * sd, 2), "h": round(rng.uniform(3, 6) * sd, 2)}


_cap("tabular_cusum", level="advanced", name="Tabular CUSUM control chart", topic=_CP,
     purpose="The upper and lower tabular CUSUM statistics for a target mean mu0 with allowance k, and the first "
             "time either exceeds the decision interval h.",
     inputs=[_vin("x", "the observations, oldest first", 1), _nin("mu0", "target (in-control) mean"),
             _nin("k", "allowance (reference value) k >= 0"), _nin("h", "decision interval h > 0")],
     outputs=[_out("c_plus", "final upper CUSUM C+_n"), _out("c_minus", "final lower CUSUM C-_n"),
              _out("alarm_index", "first t with C+_t > h or C-_t > h (0 if none)"),
              _out("side", "upper, lower, both or none")],
     primary="c_plus", conditions=["C+_0 = C-_0 = 0", "an alarm needs a value strictly above h"],
     method="C+_t = max(0, C+_{t-1} + x_t - mu0 - k); C-_t = max(0, C-_{t-1} + mu0 - k - x_t)",
     latex=r"C_t^+ = \max(0,\,C_{t-1}^+ + x_t - \mu_0 - k),\quad C_t^- = \max(0,\,C_{t-1}^- + \mu_0 - k - x_t)",
     run=run_cusum_tab, reference=ref_cusum_tab, wrong=wrong_cusum_tab, rnd=_rnd_ctab,
     need=[r"cusum|cumulative sum", r"allowance|decision interval|reference value|target"],
     aliases={"mu0": _nm(r"target(?: mean)?", "mu0", "mu_0", r"mu", r"in-control mean"),
              "k": _nm("allowance", "reference value", "slack", "k"), "h": _nm("decision interval", "threshold", "h")},
     templates=["Run a tabular CUSUM with target 10, allowance 0.5 and decision interval 4 on [10.2, 9.8, 10.5, 11.2, 11.8, 12.1, 11.9]."],
     examples=[dict(x="10.2, 9.8, 10.5, 11.2, 11.8, 12.1, 11.9", mu0=10, k=0.5, h=4),
               dict(x=[50, 49, 48, 47.5, 47, 46.8], mu0=50, k=1, h=5)],
     questions=[_q("Run a tabular CUSUM with target 10, allowance 0.5 and decision interval 4 on "
                   "[10.2, 9.8, 10.5, 11.2, 11.8, 12.1, 11.9].", 0),
                _q("Compute the CUSUM statistics with target mean 50, k = 1 and h = 5 for [50, 49, 48, 47.5, 47, 46.8].", 1),
                _q("Run a tabular CUSUM with target 10 and allowance 0.5 on [10, 11].", outcome="NEEDS_INPUT")])


# ============================================================================================ Kalman filter ====
_KF = "Kalman filtering"


def run_kf_update(m, p, z, r):
    m, z = _num(m, "the prior mean"), _num(z, "the measurement")
    p, r = _pos(p, "the prior variance"), _pos(r, "the measurement variance")
    k = p / (p + r)
    return {"mean": m + k * (z - m), "variance": (1 - k) * p, "gain": k}


def ref_kf_update(m, p, z, r):
    """Information (precision-weighted) form."""
    v = 1.0 / (1.0 / p + 1.0 / r)
    return {"mean": v * (m / p + z / r), "variance": v, "gain": v / r}


_cap("kalman_update", level="beginner", name="Kalman measurement update (scalar)", topic=_KF,
     purpose="Combine a prior estimate (mean, variance) with a measurement (value, noise variance): the posterior "
             "mean and variance and the Kalman gain.",
     inputs=[_nin("m", "prior mean"), _nin("p", "prior variance (> 0)"), _nin("z", "the measurement"),
             _nin("r", "measurement noise variance (> 0)")],
     outputs=[_out("mean", "posterior mean"), _out("variance", "posterior variance"), _out("gain", "Kalman gain p/(p+r)")],
     primary="mean", conditions=["p > 0, r > 0", "the measurement observes the state directly (H = 1)"],
     method="K = p/(p+r); m' = m + K(z - m); p' = (1 - K) p",
     latex=r"K = \frac{P}{P+R},\ \hat x^+ = \hat x + K(z - \hat x),\ P^+ = (1-K)P", run=run_kf_update,
     reference=ref_kf_update,
     wrong=lambda m, p, z, r: (lambda k: {"mean": m + k * (z - m), "variance": (1 - k) * p, "gain": k})(r / (p + r)),
     rnd=lambda rng: {"m": round(rng.uniform(-50, 50), 2), "p": round(rng.uniform(0.1, 20), 2),
                      "z": round(rng.uniform(-50, 50), 2), "r": round(rng.uniform(0.1, 20), 2)},
     need=[r"kalman", r"\bprior\b"], avoid=r"process (?:noise )?variance|\[|steady|velocity|smooth",
     aliases={"m": _nm(r"prior (?:mean|estimate)"), "p": _nm("prior variance"),
              "z": [r"(?<!variance )\bmeasurement\s*(?:=|:|of|is)?\s*(" + _NUM + ")", r"measured value\s*" + _W + "(" + _NUM + ")"],
              "r": _nm(r"measurement (?:noise )?variance")},
     templates=["Do a Kalman update with prior mean = 10, prior variance = 4, measurement = 12 and measurement variance = 1."],
     examples=[dict(m=10, p=4, z=12, r=1), dict(m=-3.5, p=0.5, z=-2.0, r=2.5)],
     questions=[_q("Do a Kalman update with prior mean = 10, prior variance = 4, measurement = 12 and measurement variance = 1.", 0),
                _q("Kalman filter: prior estimate -3.5, prior variance 0.5, measurement -2.0, measurement noise variance 2.5. "
                   "Find the posterior.", 1),
                _q("Do a Kalman update with prior mean = 10, prior variance = -4, measurement = 12 and measurement variance = 1.",
                   outcome="INVALID_INPUT")])


def run_kf_step(m, p, z, q, r):
    m, z = _num(m, "the previous estimate"), _num(z, "the measurement")
    p = _num(p, "the previous variance")
    q = _num(q, "the process variance")
    r = _pos(r, "the measurement variance")
    if p < 0 or q < 0 or p + q <= 0:
        raise ValueError("the variances must be non-negative and p + q > 0")
    pp = p + q
    k = pp / (pp + r)
    return {"mean": m + k * (z - m), "variance": (1 - k) * pp, "gain": k, "prior_variance": pp}


def ref_kf_step(m, p, z, q, r):
    from filterpy.kalman import KalmanFilter
    kf = KalmanFilter(dim_x=1, dim_z=1)
    kf.x[:] = m
    kf.P[:] = p
    kf.F[:] = 1.0
    kf.H[:] = 1.0
    kf.Q[:] = q
    kf.R[:] = r
    kf.predict()
    pp = float(kf.P[0, 0])
    kf.update(z)
    return {"mean": float(kf.x[0, 0]), "variance": float(kf.P[0, 0]), "gain": float(kf.K[0, 0]), "prior_variance": pp}


_cap("kalman_step", level="intermediate", name="One Kalman filter step (random-walk model)", topic=_KF,
     purpose="One predict-and-update step of the scalar Kalman filter for a random-walk state x_t = x_{t-1} + w_t "
             "observed as z_t = x_t + v_t.",
     inputs=[_nin("m", "previous estimate"), _nin("p", "previous estimate variance"), _nin("z", "the new measurement"),
             _nin("q", "process noise variance"), _nin("r", "measurement noise variance")],
     outputs=[_out("mean", "updated estimate"), _out("variance", "updated variance"), _out("gain", "Kalman gain"),
              _out("prior_variance", "predicted variance p + q")],
     primary="mean", conditions=["p, q >= 0, r > 0", "F = H = 1"],
     method="predict P- = P + Q; K = P-/(P- + R); x = x + K(z - x); P = (1 - K) P-",
     latex=r"P^- = P + Q,\ K = \frac{P^-}{P^-+R},\ \hat x = \hat x^- + K(z - \hat x^-),\ P = (1-K)P^-",
     run=run_kf_step, reference=ref_kf_step,
     wrong=lambda m, p, z, q, r: (lambda k: {"mean": m + k * (z - m), "variance": (1 - k) * p, "gain": k,
                                             "prior_variance": p})(p / (p + r)),
     rnd=lambda rng: {"m": round(rng.uniform(-50, 50), 2), "p": round(rng.uniform(0.1, 20), 2),
                      "z": round(rng.uniform(-50, 50), 2), "q": round(rng.uniform(0.1, 10), 2),
                      "r": round(rng.uniform(0.1, 20), 2)},
     need=[r"kalman", r"process (?:noise )?variance"], avoid=r"\[|steady|velocity|smooth",
     aliases={"m": _nm(r"(?:previous|prior|current) (?:mean|estimate)"), "p": _nm(r"(?:previous|prior|current) variance"),
              "q": _nm(r"process (?:noise )?variance"),
              "z": [r"(?<!variance )\b(?:new )?measurement\s*(?:=|:|of|is)?\s*(" + _NUM + ")"],
              "r": _nm(r"measurement (?:noise )?variance")},
     templates=["Do one Kalman filter step with previous estimate = 10, previous variance = 4, process variance = 1, "
                "measurement = 12 and measurement variance = 2."],
     examples=[dict(m=10, p=4, z=12, q=1, r=2), dict(m=0.5, p=0.2, z=1.4, q=0.05, r=0.3)],
     questions=[_q("Do one Kalman filter step with previous estimate = 10, previous variance = 4, process variance = 1, "
                   "measurement = 12 and measurement variance = 2.", 0),
                _q("Kalman filter step: previous estimate 0.5, previous variance 0.2, process noise variance 0.05, new "
                   "measurement 1.4, measurement variance 0.3.", 1),
                _q("Do one Kalman filter step with previous estimate = 10, process variance = 1, measurement = 12 and "
                   "measurement variance = 2.", outcome="NEEDS_INPUT")])


def _ll_filter(x, q, r, m0, p0):
    x = _fin(x, "the measurements")
    _need(x, 1, "the measurements")
    q = _num(q, "the process variance")
    r = _pos(r, "the measurement variance")
    m, p = _num(m0, "the initial estimate"), _num(p0, "the initial variance")
    if q < 0 or p < 0:
        raise ValueError("the variances must be non-negative")
    ms, ps, ll, k = [], [], 0.0, 0.0
    for z in x:
        pp = p + q
        s = pp + r
        k = pp / s
        v = z - m
        ll += -0.5 * (math.log(2 * math.pi * s) + v * v / s)
        m = m + k * v
        p = (1 - k) * pp
        ms.append(m)
        ps.append(p)
    return ms, ps, ll, k, q


def run_kf_ll(x, q, r, m0, p0):
    ms, ps, ll, k, _q_ = _ll_filter(x, q, r, m0, p0)
    return {"estimate": ms[-1], "variance": ps[-1], "gain": k, "estimates": ms, "log_likelihood": _r12(ll)}


def _ref_filter(x, q, r, m0, p0):
    from filterpy.kalman import KalmanFilter
    kf = KalmanFilter(dim_x=1, dim_z=1)
    kf.x[:] = m0
    kf.P[:] = p0
    kf.F[:] = 1.0
    kf.H[:] = 1.0
    kf.Q[:] = q
    kf.R[:] = r
    ms, ps, ll = [], [], 0.0
    for z in x:
        kf.predict()
        kf.update(z)
        ms.append(kf.x.copy())
        ps.append(kf.P.copy())
        ll += float(kf.log_likelihood)
    return kf, ms, ps, ll


def ref_kf_ll(x, q, r, m0, p0):
    kf, ms, ps, ll = _ref_filter(x, q, r, m0, p0)
    return {"estimate": float(kf.x[0, 0]), "variance": float(kf.P[0, 0]), "gain": float(kf.K[0, 0]),
            "estimates": [float(v[0, 0]) for v in ms], "log_likelihood": ll}


def _rnd_kf_ll(rng):
    n = rng.randint(2, 30)
    q, r = round(rng.uniform(0.05, 5), 2), round(rng.uniform(0.1, 10), 2)
    lev, x = rng.uniform(-20, 20), []
    for _ in range(n):
        lev += rng.gauss(0, math.sqrt(q))
        x.append(round(lev + rng.gauss(0, math.sqrt(r)), 2))
    return {"x": x, "q": q, "r": r, "m0": round(x[0] + rng.gauss(0, 2), 1), "p0": round(rng.uniform(0.5, 100), 1)}


_KF_AL = {"q": _nm(r"process (?:noise )?variance"), "r": _nm(r"measurement (?:noise )?variance"),
          "m0": _nm(r"initial (?:estimate|mean|state)", "m0", "x0"), "p0": _nm(r"initial (?:estimate )?variance", "p0")}
_KF_IN = [_vin("x", "the measurements, oldest first", 1), _nin("q", "process noise variance"),
          _nin("r", "measurement noise variance"), _nin("m0", "initial state estimate"), _nin("p0", "initial variance")]
_KF_Q = ("process variance = 0.5, measurement variance = 2, initial estimate = 10 and initial variance = 5 "
         "for the measurements [10.4, 11.1, 10.7, 11.9, 12.3]")

_cap("kalman_filter_local_level", level="advanced", name="Kalman filter for a random-walk (local level) model",
     topic=_KF,
     purpose="Filter a sequence of measurements with the scalar Kalman filter for x_t = x_{t-1} + w_t, z_t = x_t + v_t "
             "(predict, then update, at every step): the final estimate, variance, gain, all filtered estimates and the "
             "Gaussian log-likelihood of the one-step prediction errors.",
     inputs=_KF_IN,
     outputs=[_out("estimate", "final filtered estimate"), _out("variance", "final variance"), _out("gain", "final gain"),
              _out("estimates", "filtered estimates for every measurement"),
              _out("log_likelihood", "sum of log N(v_t; 0, S_t)")],
     primary="estimate", conditions=["q >= 0, r > 0, p0 >= 0", "the initial estimate is the state before the first measurement"],
     method="P- = P + q; S = P- + r; K = P-/S; x = x + K(z - x); P = (1 - K) P-",
     latex=r"P_t^- = P_{t-1}+q,\ K_t = \frac{P_t^-}{P_t^-+r},\ \hat x_t = \hat x_{t-1} + K_t(z_t - \hat x_{t-1})",
     run=run_kf_ll, reference=ref_kf_ll,
     wrong=lambda x, q, r, m0, p0: run_kf_ll(x, 0.0, r, m0, p0) if q else run_kf_ll(x, 1.0, r, m0, p0),
     rnd=_rnd_kf_ll, need=[r"kalman", r"\["], avoid=r"smooth|\brts\b|velocity|steady", aliases=_KF_AL,
     templates=[f"Run a Kalman filter with {_KF_Q}."],
     examples=[dict(x="10.4, 11.1, 10.7, 11.9, 12.3", q=0.5, r=2, m0=10, p0=5),
               dict(x=[0.2, -0.1, 0.4, 0.3], q=0.01, r=0.25, m0=0, p0=1)],
     questions=[_q(f"Run a Kalman filter with {_KF_Q}.", 0),
                _q("Apply the Kalman filter with process variance 0.01, measurement variance 0.25, initial estimate 0 and "
                   "initial variance 1 to the measurements [0.2, -0.1, 0.4, 0.3].", 1),
                _q("Run a Kalman filter with process variance = 0.5 for the measurements [1, 2].", outcome="NEEDS_INPUT")])


def run_rts(x, q, r, m0, p0):
    ms, ps, _ll, _k, q = _ll_filter(x, q, r, m0, p0)
    n = len(ms)
    sm, sp = ms[:], ps[:]
    for t in range(n - 2, -1, -1):
        pp = ps[t] + q
        c = ps[t] / pp if pp > 0 else 0.0
        sm[t] = ms[t] + c * (sm[t + 1] - ms[t])
        sp[t] = ps[t] + c * c * (sp[t + 1] - pp)
    return {"smoothed": sm, "first": sm[0], "first_variance": sp[0]}


def ref_rts(x, q, r, m0, p0):
    import numpy as np
    kf, ms, ps, _ll = _ref_filter(x, q, r, m0, p0)
    xs, Ps, _K, _Pp = kf.rts_smoother(np.array(ms), np.array(ps))
    return {"smoothed": [float(v[0, 0]) for v in xs], "first": float(xs[0][0, 0]), "first_variance": float(Ps[0][0, 0])}


_cap("rts_smoother_local_level", level="expert", name="Rauch-Tung-Striebel smoother (random-walk model)", topic=_KF,
     purpose="Smoothed state estimates of the random-walk (local level) model: the Kalman filter forward, then the "
             "Rauch-Tung-Striebel recursion backward.",
     inputs=_KF_IN,
     outputs=[_out("smoothed", "smoothed estimates for every time"), _out("first", "smoothed estimate at time 1"),
              _out("first_variance", "its variance")],
     primary="first", conditions=["q > 0, r > 0, p0 >= 0"],
     method="C_t = P_t/(P_t + q); xs_t = x_t + C_t (xs_{t+1} - x_t); Ps_t = P_t + C_t^2 (Ps_{t+1} - P_t - q)",
     latex=r"\hat x_{t|n} = \hat x_{t} + C_t(\hat x_{t+1|n} - \hat x_t),\ C_t = \frac{P_t}{P_t+q}",
     run=run_rts, reference=ref_rts,
     wrong=lambda x, q, r, m0, p0: (lambda o: {"smoothed": o["estimates"], "first": o["estimates"][0],
                                               "first_variance": _ll_filter(x, q, r, m0, p0)[1][0]})(run_kf_ll(x, q, r, m0, p0)),
     rnd=lambda rng: (lambda d: d if len(d["x"]) > 1 else dict(d, x=d["x"] + [d["x"][0]]))(_rnd_kf_ll(rng)),
     need=[r"\brts\b|rauch|kalman smooth|smoother", r"\["], avoid=r"velocity", aliases=_KF_AL,
     templates=[f"Apply the RTS smoother with {_KF_Q}."],
     examples=[dict(x="10.4, 11.1, 10.7, 11.9, 12.3", q=0.5, r=2, m0=10, p0=5),
               dict(x=[0.2, -0.1, 0.4, 0.3], q=0.01, r=0.25, m0=0, p0=1)],
     questions=[_q(f"Apply the RTS smoother with {_KF_Q}.", 0),
                _q("Smooth the measurements [0.2, -0.1, 0.4, 0.3] with the Rauch-Tung-Striebel smoother, process variance "
                   "0.01, measurement variance 0.25, initial estimate 0 and initial variance 1.", 1)])


def run_kf_ss(q, r):
    q, r = _pos(q, "the process variance"), _pos(r, "the measurement variance")
    pp = (q + math.sqrt(q * q + 4 * q * r)) / 2
    k = pp / (pp + r)
    return {"gain": k, "prior_variance": pp, "posterior_variance": (1 - k) * pp}


def ref_kf_ss(q, r):
    import numpy as np
    from scipy.linalg import solve_discrete_are
    pp = float(solve_discrete_are(np.array([[1.0]]), np.array([[1.0]]), np.array([[q]]), np.array([[r]]))[0, 0])
    k = pp / (pp + r)
    return {"gain": k, "prior_variance": pp, "posterior_variance": (1 - k) * pp}


_cap("kalman_steady_state_gain", level="advanced", name="Steady-state Kalman gain (random-walk model)", topic=_KF,
     purpose="The limiting Kalman gain and variances of the random-walk (local level) model with process variance q "
             "and measurement variance r (the solution of the scalar Riccati equation).",
     inputs=[_nin("q", "process noise variance (> 0)"), _nin("r", "measurement noise variance (> 0)")],
     outputs=[_out("gain", "steady-state gain"), _out("prior_variance", "steady-state predicted variance P-"),
              _out("posterior_variance", "steady-state filtered variance")],
     primary="gain", conditions=["q > 0, r > 0"], method="P- = (q + sqrt(q^2 + 4 q r))/2, K = P-/(P- + r)",
     latex=r"P^- = \frac{q + \sqrt{q^2 + 4qr}}{2},\quad K = \frac{P^-}{P^- + r}", run=run_kf_ss, reference=ref_kf_ss,
     wrong=lambda q, r: {"gain": q / (q + r), "prior_variance": q, "posterior_variance": q * r / (q + r)},
     rnd=lambda rng: {"q": round(10 ** rng.uniform(-2, 1.5), 3), "r": round(10 ** rng.uniform(-2, 1.5), 3)},
     need=[r"kalman", r"steady[- ]state|limiting|asymptotic"], avoid=r"velocity|\[",
     aliases={"q": _KF_AL["q"], "r": _KF_AL["r"]},
     templates=["Find the steady-state Kalman gain with process variance 1 and measurement variance 4."],
     examples=[dict(q=1, r=4), dict(q=0.05, r=2.5)],
     questions=[_q("Find the steady-state Kalman gain with process variance 1 and measurement variance 4.", 0),
                _q("What is the steady-state Kalman gain for a random walk with process noise variance 0.05 and "
                   "measurement noise variance 2.5?", 1),
                _q("Find the steady-state Kalman gain with process variance 1.", outcome="NEEDS_INPUT")])


def _cv_mats(dt, q):
    return [[1.0, dt], [0.0, 1.0]], [[q * dt ** 4 / 4, q * dt ** 3 / 2], [q * dt ** 3 / 2, q * dt ** 2]]


def _cv_predict(P, F, Q):
    a, b, c = P[0][0], P[0][1], P[1][1]
    dt = F[0][1]
    p00 = a + 2 * dt * b + dt * dt * c + Q[0][0]
    p01 = b + dt * c + Q[0][1]
    p11 = c + Q[1][1]
    return [[p00, p01], [p01, p11]]


def _cv_update(P, r):
    s = P[0][0] + r
    k0, k1 = P[0][0] / s, P[0][1] / s
    return [[(1 - k0) * P[0][0], (1 - k0) * P[0][1]], [(1 - k0) * P[0][1], P[1][1] - k1 * P[0][1]]], (k0, k1)


def run_kf_cv(x, dt, q, r, p0, v0, var0):
    x = _fin(x, "the measured positions")
    dt, q, r = _pos(dt, "the time step"), _num(q, "the acceleration variance"), _pos(r, "the measurement variance")
    var0 = _num(var0, "the initial variance")
    if q < 0 or var0 < 0:
        raise ValueError("the variances must be non-negative")
    F, Q = _cv_mats(dt, q)
    s = [_num(p0, "the initial position"), _num(v0, "the initial velocity")]
    P = [[var0, 0.0], [0.0, var0]]
    for z in x:
        s = [s[0] + dt * s[1], s[1]]
        P = _cv_predict(P, F, Q)
        Pn, (k0, k1) = _cv_update(P, r)
        v = z - s[0]
        s = [s[0] + k0 * v, s[1] + k1 * v]
        P = Pn
    return {"position": s[0], "velocity": s[1], "p11": P[0][0], "p12": P[0][1], "p22": P[1][1]}


def ref_kf_cv(x, dt, q, r, p0, v0, var0):
    import numpy as np
    from filterpy.common import Q_discrete_white_noise
    from filterpy.kalman import KalmanFilter
    kf = KalmanFilter(dim_x=2, dim_z=1)
    kf.x = np.array([[p0], [v0]], dtype=float)
    kf.F = np.array([[1.0, dt], [0.0, 1.0]])
    kf.H = np.array([[1.0, 0.0]])
    kf.P = np.eye(2) * var0
    kf.R = np.array([[r]], dtype=float)
    kf.Q = Q_discrete_white_noise(dim=2, dt=dt, var=q)
    for z in x:
        kf.predict()
        kf.update(z)
    return {"position": float(kf.x[0, 0]), "velocity": float(kf.x[1, 0]), "p11": float(kf.P[0, 0]),
            "p12": float(kf.P[0, 1]), "p22": float(kf.P[1, 1])}


def _wrong_cv(x, dt, q, r, p0, v0, var0):  # continuous white-noise Q (dt^3/3, dt^2/2, dt) mistaken for the discrete one
    s, P = [p0, v0], [[var0, 0.0], [0.0, var0]]
    Q = [[q * dt ** 3 / 3, q * dt ** 2 / 2], [q * dt ** 2 / 2, q * dt]]
    F = [[1.0, dt], [0.0, 1.0]]
    for z in x:
        s = [s[0] + dt * s[1], s[1]]
        P = _cv_predict(P, F, Q)
        P, (k0, k1) = _cv_update(P, r)
        v = z - s[0]
        s = [s[0] + k0 * v, s[1] + k1 * v]
    return {"position": s[0], "velocity": s[1], "p11": P[0][0], "p12": P[0][1], "p22": P[1][1]}


def _rnd_cv(rng):
    dt = rng.choice([0.1, 0.5, 1.0, 1.0, 2.0])
    n = rng.randint(1, 25)
    pos, vel = rng.uniform(-10, 10), rng.uniform(-3, 3)
    r = round(rng.uniform(0.1, 5), 2)
    x = []
    for _ in range(n):
        pos += vel * dt
        x.append(round(pos + rng.gauss(0, math.sqrt(r)), 2))
    return {"x": x, "dt": dt, "q": round(rng.uniform(0.01, 2), 2), "r": r, "p0": round(rng.uniform(-10, 10), 1),
            "v0": round(rng.uniform(-3, 3), 1), "var0": round(rng.uniform(1, 100), 1)}


_cap("kalman_filter_constant_velocity", level="expert", name="Kalman filter for a constant-velocity model", topic=_KF,
     purpose="Track position and velocity from noisy position measurements with the constant-velocity Kalman filter "
             "(discrete white-noise acceleration with variance q, time step dt): final state and covariance.",
     inputs=[_vin("x", "the measured positions, oldest first", 1), _nin("dt", "time step"),
             _nin("q", "acceleration (process) noise variance"), _nin("r", "position measurement noise variance"),
             _nin("p0", "initial position estimate"), _nin("v0", "initial velocity estimate"),
             _nin("var0", "initial variance of position and of velocity (uncorrelated)")],
     outputs=[_out("position", "final position estimate"), _out("velocity", "final velocity estimate"),
              _out("p11", "final position variance"), _out("p12", "final position-velocity covariance"),
              _out("p22", "final velocity variance")],
     primary="position",
     conditions=["F = [[1, dt], [0, 1]], H = [1, 0]", "Q = q [[dt^4/4, dt^3/2], [dt^3/2, dt^2]]",
                 "predict then update at every measurement"],
     method="2-state Kalman filter recursion", latex=r"F=\begin{pmatrix}1&\Delta t\\0&1\end{pmatrix},\ Q = q\begin{pmatrix}\Delta t^4/4&\Delta t^3/2\\ \Delta t^3/2&\Delta t^2\end{pmatrix}",
     run=run_kf_cv, reference=ref_kf_cv, wrong=_wrong_cv, rnd=_rnd_cv,
     need=[r"kalman", r"velocity", r"\["], avoid=r"steady",
     aliases={"dt": _nm(r"time step", "dt", r"sampling interval"), "q": _nm(r"(?:acceleration|process) (?:noise )?variance"),
              "r": _nm(r"measurement (?:noise )?variance"), "p0": _nm(r"initial position"), "v0": _nm(r"initial velocity"),
              "var0": _nm(r"initial variance")},
     templates=["Run a constant-velocity Kalman filter with time step 1, process variance 0.1, measurement variance 1, "
                "initial position 0, initial velocity 1 and initial variance 10 for the positions [1.1, 1.9, 3.2, 4.0, 4.8]."],
     examples=[dict(x="1.1, 1.9, 3.2, 4.0, 4.8", dt=1, q=0.1, r=1, p0=0, v0=1, var0=10),
               dict(x=[0.0, 0.6, 0.9, 1.6], dt=0.5, q=0.5, r=0.2, p0=0, v0=0, var0=5)],
     questions=[_q("Run a constant-velocity Kalman filter with time step 1, process variance 0.1, measurement variance 1, "
                   "initial position 0, initial velocity 1 and initial variance 10 for the positions "
                   "[1.1, 1.9, 3.2, 4.0, 4.8].", 0),
                _q("Track position and velocity with a Kalman filter: time step 0.5, acceleration variance 0.5, "
                   "measurement variance 0.2, initial position 0, initial velocity 0, initial variance 5, positions "
                   "[0.0, 0.6, 0.9, 1.6].", 1)])


def run_kf_cv_ss(dt, q, r):
    dt, q, r = _pos(dt, "the time step"), _pos(q, "the acceleration variance"), _pos(r, "the measurement variance")
    F, Q = _cv_mats(dt, q)
    P = [row[:] for row in Q]
    P = [[P[0][0] + r, P[0][1]], [P[1][0], P[1][1] + r]]
    for _ in range(1000000):
        Pu, _k = _cv_update(P, r)
        Pn = _cv_predict(Pu, F, Q)
        if max(abs(Pn[i][j] - P[i][j]) for i in range(2) for j in range(2)) <= 1e-15 * max(1.0, abs(Pn[0][0])):
            P = Pn
            break
        P = Pn
    else:
        raise ValueError("the Riccati iteration did not converge")
    s = P[0][0] + r
    return {"k_position": P[0][0] / s, "k_velocity": P[0][1] / s, "prior_p11": P[0][0]}


def ref_kf_cv_ss(dt, q, r):
    import numpy as np
    from scipy.linalg import solve_discrete_are
    F = np.array([[1.0, dt], [0.0, 1.0]])
    H = np.array([[1.0, 0.0]])
    Q = q * np.array([[dt ** 4 / 4, dt ** 3 / 2], [dt ** 3 / 2, dt ** 2]])
    P = solve_discrete_are(F.T, H.T, Q, np.array([[r]]))
    s = P[0, 0] + r
    return {"k_position": float(P[0, 0] / s), "k_velocity": float(P[0, 1] / s), "prior_p11": float(P[0, 0])}


_cap("kalman_steady_state_constant_velocity", level="expert",
     name="Steady-state Kalman gains of a constant-velocity model", topic=_KF,
     purpose="The limiting position and velocity gains (and predicted position variance) of the constant-velocity "
             "Kalman filter with discrete white-noise acceleration q, time step dt and measurement variance r.",
     inputs=[_nin("dt", "time step"), _nin("q", "acceleration noise variance (> 0)"), _nin("r", "measurement noise variance (> 0)")],
     outputs=[_out("k_position", "steady-state position gain"), _out("k_velocity", "steady-state velocity gain"),
              _out("prior_p11", "steady-state predicted position variance")],
     primary="k_position", conditions=["dt, q, r > 0", "Q = q [[dt^4/4, dt^3/2], [dt^3/2, dt^2]]"],
     method="iterate the Riccati recursion to its fixed point",
     latex=r"P = F\big(P - PH^\top(HPH^\top+R)^{-1}HP\big)F^\top + Q", run=run_kf_cv_ss, reference=ref_kf_cv_ss,
     wrong=lambda dt, q, r: (lambda o: dict(o, k_velocity=o["k_velocity"] * dt))(run_kf_cv_ss(dt, q, r)),
     rnd=lambda rng: {"dt": rng.choice([0.1, 0.5, 1.0, 2.0]), "q": round(10 ** rng.uniform(-1.5, 1), 3),
                      "r": round(10 ** rng.uniform(-1, 1), 3)},
     need=[r"kalman", r"velocity", r"steady[- ]state|limiting|asymptotic"], avoid=r"\[",
     aliases={"dt": _nm(r"time step", "dt", r"sampling interval"), "q": _nm(r"(?:acceleration|process) (?:noise )?variance"),
              "r": _nm(r"measurement (?:noise )?variance")},
     templates=["Find the steady-state Kalman gains of a constant-velocity model with time step 1, process variance 0.1 and measurement variance 1."],
     examples=[dict(dt=1, q=0.1, r=1), dict(dt=0.5, q=2, r=0.3)],
     questions=[_q("Find the steady-state Kalman gains of a constant-velocity model with time step 1, process variance 0.1 "
                   "and measurement variance 1.", 0),
                _q("What are the steady-state gains of a constant velocity Kalman filter with time step 0.5, acceleration "
                   "variance 2 and measurement variance 0.3?", 1)])


# ======================================================================================== stationarity tests ====
_ST = "stationarity tests"


def _x_regression(rest, d, spans, kpss=False):
    if "regression" in d:
        return
    if kpss:
        if re.search(r"trend[- ]stationar|\btrend\b", rest):
            d["regression"] = "ct"
        elif re.search(r"level[- ]stationar|\bconstant\b|\blevel\b", rest):
            d["regression"] = "c"
        return
    if re.search(r"no constant|without (?:a )?constant|no drift|no intercept|without (?:an )?intercept", rest):
        d["regression"] = "n"
    elif re.search(r"\btrend\b", rest):
        d["regression"] = "ct"
    elif re.search(r"\bconstant\b|\bdrift\b|\bintercept\b", rest):
        d["regression"] = "c"


def _df(x, p, regression, mle=False):
    x = _fin(x)
    p = _int(p, "the number of lagged differences", 0, 12)
    reg = regression or "c"
    if reg not in ("n", "c", "ct"):
        raise ValueError("the regression must be n, c or ct")
    _nonconstant(x)
    nt = {"n": 0, "c": 1, "ct": 2}[reg]
    if p > len(x) // 2 - nt - 1:
        raise ValueError("too many lags for the series length (p <= n/2 - deterministic terms - 1)")
    dx = [x[t] - x[t - 1] for t in range(1, len(x))]
    rows, y = [], []
    for i, t in enumerate(range(p, len(dx))):
        row = [x[t]] + [dx[t - j] for j in range(1, p + 1)]
        if nt >= 1:
            row.append(1.0)
        if nt == 2:
            row.append(float(i + 1))
        rows.append(row)
        y.append(dx[t])
    if len(rows) <= len(rows[0]):
        raise ValueError("the series is too short for this regression")
    beta, res, diag = _lstsq(rows, y)
    nobs = len(y)
    s2 = math.fsum(e * e for e in res) / (nobs if mle else nobs - len(rows[0]))
    se = math.sqrt(s2 * diag[0])
    return {"tau": beta[0] / se, "gamma": beta[0], "se": se, "nobs": nobs}


def _ref_df(x, p, regression):
    import numpy as np
    from statsmodels.tsa.stattools import adfuller
    r = adfuller(np.asarray(x, dtype=float), maxlag=int(p), regression=regression or "c", autolag=None, regresults=True)
    res = r[-1].resols
    return {"tau": float(r[0]), "gamma": float(res.params[0]), "se": float(res.bse[0]), "nobs": int(res.nobs)}


_REG_IN = _cin("regression", "deterministic terms: c = constant (default), ct = constant and trend, n = none",
               ["c", "ct", "n"])


def _rnd_df(rng, lags):
    n = rng.randint(20, 80)
    x = _walk(rng, n) if rng.random() < 0.5 else _ar1(rng, n, phi=rng.uniform(0.2, 0.95))
    d = {"x": x, "regression": rng.choice(["c", "c", "ct", "n"])}
    if lags:
        d["p"] = rng.randint(1, 4)
    return d


_cap("dickey_fuller_test", level="advanced", name="Dickey-Fuller unit-root statistic", topic=_ST,
     purpose="The Dickey-Fuller tau statistic: the t-ratio of gamma in the regression Delta x_t = gamma x_{t-1} "
             "(+ constant, + trend) + e_t. The statistic only: compare it with Dickey-Fuller critical values.",
     inputs=[_vin("x", "the series, oldest first", 8), _REG_IN],
     outputs=[_out("tau", "the Dickey-Fuller statistic gamma/se"), _out("gamma", "the coefficient of x_{t-1}"),
              _out("se", "its standard error"), _out("nobs", "observations in the regression (n - 1)")],
     primary="tau", conditions=["the series is not constant", "default deterministic terms: a constant",
                                "no lagged differences (see augmented_dickey_fuller_test)"],
     method="least squares of Delta x_t on x_{t-1} (and constant / trend t = 1..n-1)",
     latex=r"\Delta x_t = \alpha + \gamma x_{t-1} + e_t,\quad \tau = \hat\gamma/\mathrm{se}(\hat\gamma)",
     run=lambda x, regression="c": _df(x, 0, regression), reference=lambda x, regression="c": _ref_df(x, 0, regression),
     wrong=lambda x, regression="c": _df(x, 0, regression, mle=True), rnd=lambda rng: _rnd_df(rng, False),
     need=r"dickey[- ]fuller", avoid=r"augmented|\badf\b|lags?\b|lagged", extract=_x_regression,
     templates=["Compute the Dickey-Fuller statistic with a constant for [10.2, 10.8, 10.5, 11.3, 11.0, 11.9, 11.6, 12.4, 12.0, 12.9]."],
     examples=[dict(x="10.2, 10.8, 10.5, 11.3, 11.0, 11.9, 11.6, 12.4, 12.0, 12.9", regression="c"),
               dict(x=[5.0, 4.1, 4.6, 3.8, 4.4, 4.0, 4.7, 4.2, 4.5, 3.9, 4.3, 4.1], regression="ct"),
               dict(x=[1.2, -0.5, 0.8, -1.1, 0.3, 0.9, -0.7, 0.4, -0.2, 0.6], regression="n")],
     questions=[_q("Compute the Dickey-Fuller statistic with a constant for "
                   "[10.2, 10.8, 10.5, 11.3, 11.0, 11.9, 11.6, 12.4, 12.0, 12.9].", 0),
                _q("Find the Dickey-Fuller test statistic with constant and trend for "
                   "[5.0, 4.1, 4.6, 3.8, 4.4, 4.0, 4.7, 4.2, 4.5, 3.9, 4.3, 4.1].", 1),
                _q("Compute the Dickey-Fuller statistic with no constant for "
                   "[1.2, -0.5, 0.8, -1.1, 0.3, 0.9, -0.7, 0.4, -0.2, 0.6].", 2),
                _q("Compute the Dickey-Fuller statistic for [3, 3, 3, 3, 3, 3, 3, 3].", outcome="INVALID_INPUT")])

_cap("augmented_dickey_fuller_test", level="expert", name="Augmented Dickey-Fuller statistic", topic=_ST,
     purpose="The augmented Dickey-Fuller tau statistic with p lagged differences: the t-ratio of gamma in "
             "Delta x_t = gamma x_{t-1} + sum_j delta_j Delta x_{t-j} (+ constant, + trend) + e_t. The statistic only.",
     inputs=[_vin("x", "the series, oldest first", 8), _iin("p", "number of lagged differences", lo=0, hi=12), _REG_IN],
     outputs=[_out("tau", "the ADF statistic"), _out("gamma", "the coefficient of x_{t-1}"), _out("se", "its standard error"),
              _out("nobs", "observations in the regression (n - 1 - p)")],
     primary="tau", conditions=["p <= n/2 - (deterministic terms) - 1", "default deterministic terms: a constant",
                                "the lag length is given, not selected"],
     method="least squares on t = p+2..n of Delta x_t on x_{t-1}, Delta x_{t-1..t-p} (and constant / trend)",
     latex=r"\Delta x_t = \alpha + \gamma x_{t-1} + \sum_{j=1}^{p}\delta_j\Delta x_{t-j} + e_t",
     run=lambda x, p, regression="c": _df(x, p, regression), reference=lambda x, p, regression="c": _ref_df(x, p, regression),
     wrong=lambda x, p, regression="c": _df(x, 0, regression), rnd=lambda rng: _rnd_df(rng, True),
     need=r"augmented dickey[- ]fuller|\badf\b|dickey[- ]fuller\b.*\blags?\b", extract=_x_regression,
     aliases={"p": [r"(\d+)\s+(?:lagged differences?|lags?)\b", r"\blags?\s*" + _W + r"(\d+)", r"\bp\s*=\s*(\d+)"]},
     templates=["Compute the augmented Dickey-Fuller statistic with 2 lags and a constant for "
                "[10.2, 10.8, 10.5, 11.3, 11.0, 11.9, 11.6, 12.4, 12.0, 12.9, 12.5, 13.4]."],
     examples=[dict(x="10.2, 10.8, 10.5, 11.3, 11.0, 11.9, 11.6, 12.4, 12.0, 12.9, 12.5, 13.4", p=2, regression="c"),
               dict(x=[5.0, 4.1, 4.6, 3.8, 4.4, 4.0, 4.7, 4.2, 4.5, 3.9, 4.3, 4.1, 4.8, 4.0], p=1, regression="ct")],
     questions=[_q("Compute the augmented Dickey-Fuller statistic with 2 lags and a constant for "
                   "[10.2, 10.8, 10.5, 11.3, 11.0, 11.9, 11.6, 12.4, 12.0, 12.9, 12.5, 13.4].", 0),
                _q("Find the ADF statistic with 1 lag, constant and trend, for "
                   "[5.0, 4.1, 4.6, 3.8, 4.4, 4.0, 4.7, 4.2, 4.5, 3.9, 4.3, 4.1, 4.8, 4.0].", 1),
                _q("Compute the augmented Dickey-Fuller statistic with 6 lags for [1, 3, 2, 4, 3, 5, 4, 6].",
                   outcome="INVALID_INPUT")])


def run_kpss(x, nlags, regression="c", flat=False):
    x = _fin(x)
    _need(x, 4)
    _nonconstant(x)
    L = _int(nlags, "the number of lags", 0)
    reg = regression or "c"
    if reg not in ("c", "ct"):
        raise ValueError("the KPSS regression must be c (level) or ct (trend)")
    n = len(x)
    if L >= n:
        raise ValueError("the number of lags must be less than n")
    if reg == "c":
        m = _mean(x)
        e = [v - m for v in x]
    else:
        _beta, e, _dg = _lstsq([[1.0, t] for t in _t(n)], x)
    s, eta = 0.0, 0.0
    for v in e:
        s += v
        eta += s * s
    eta /= n * n
    lrv = math.fsum(v * v for v in e)
    for i in range(1, L + 1):
        w = 1.0 if flat else 1.0 - i / (L + 1.0)
        lrv += 2 * w * math.fsum(e[t] * e[t - i] for t in range(i, n))
    lrv /= n
    if lrv <= 0:
        raise ValueError("the long-run variance estimate is not positive")
    return {"kpss": eta / lrv, "long_run_variance": lrv}


def ref_kpss(x, nlags, regression="c"):
    import warnings
    import numpy as np
    from statsmodels.tsa.stattools import kpss
    with warnings.catch_warnings():
        warnings.simplefilter("ignore")
        stat = kpss(np.asarray(x, dtype=float), regression=regression or "c", nlags=int(nlags))[0]
    return {"kpss": float(stat)}


_cap("kpss_test", level="expert", name="KPSS stationarity statistic", topic=_ST,
     purpose="The KPSS statistic for level (c) or trend (ct) stationarity with a Bartlett-kernel long-run variance "
             "using a given number of lags. The statistic only: compare it with KPSS critical values.",
     inputs=[_vin("x", "the series, oldest first", 4), _iin("nlags", "Bartlett lags in the long-run variance", lo=0),
             _cin("regression", "c = level stationarity (default), ct = trend stationarity", ["c", "ct"])],
     outputs=[_out("kpss", "the KPSS statistic"), _out("long_run_variance", "Bartlett long-run variance of the residuals")],
     primary="kpss", conditions=["0 <= lags < n", "residuals from the mean (c) or a linear trend t = 1..n (ct)"],
     method="eta = sum S_t^2 / n^2 over partial sums S_t of the residuals, divided by the Bartlett long-run variance",
     latex=r"\mathrm{KPSS} = \frac{n^{-2}\sum_{t=1}^{n} S_t^2}{\hat\sigma^2_{LR}},\ \hat\sigma^2_{LR} = \frac1n\Big(\sum e_t^2 + 2\sum_{i=1}^{L}\big(1-\tfrac{i}{L+1}\big)\sum_t e_te_{t-i}\Big)",
     run=lambda x, nlags, regression="c": run_kpss(x, nlags, regression), reference=ref_kpss,
     wrong=lambda x, nlags, regression="c": run_kpss(x, nlags, regression, flat=True) if int(nlags) else
     dict(run_kpss(x, nlags, regression), kpss=run_kpss(x, nlags, regression)["kpss"] * len(x)),
     rnd=lambda rng: (lambda d: dict(d, regression=rng.choice(["c", "ct"]), nlags=rng.randint(1, 6)))({"x": _any(rng, 12, 80)}),
     need=r"\bkpss\b", extract=lambda rest, d, spans: _x_regression(rest, d, spans, kpss=True),
     aliases={"nlags": [r"(\d+)\s+lags?\b", r"\blags?\s*" + _W + r"(\d+)", r"\bnlags\s*=\s*(\d+)"]},
     templates=["Compute the KPSS statistic for level stationarity with 2 lags for [10.2, 10.8, 10.5, 11.3, 11.0, 11.9, 11.6, 12.4]."],
     examples=[dict(x="10.2, 10.8, 10.5, 11.3, 11.0, 11.9, 11.6, 12.4", nlags=2, regression="c"),
               dict(x=[5.0, 4.1, 4.6, 3.8, 4.4, 4.0, 4.7, 4.2, 4.5, 3.9], nlags=1, regression="ct")],
     questions=[_q("Compute the KPSS statistic for level stationarity with 2 lags for "
                   "[10.2, 10.8, 10.5, 11.3, 11.0, 11.9, 11.6, 12.4].", 0),
                _q("Find the KPSS statistic for trend stationarity with 1 lag for [5.0, 4.1, 4.6, 3.8, 4.4, 4.0, 4.7, 4.2, 4.5, 3.9].", 1),
                _q("Compute the KPSS statistic for [1, 3, 2, 4, 3].", outcome="NEEDS_INPUT")])


# ================================================================== single updates and simple forecasts ====
def run_ses_update(F, A, alpha):
    F, A, alpha = _num(F, "the previous forecast"), _num(A, "the actual value"), _unit(alpha, "alpha")
    return {"forecast": F + alpha * (A - F), "error": A - F}


def ref_ses_update(F, A, alpha):
    import numpy as np
    from statsmodels.tsa.holtwinters import SimpleExpSmoothing
    f = SimpleExpSmoothing(np.array([float(A), float(A)]), initialization_method="known", initial_level=float(F)).fit(
        smoothing_level=alpha, optimized=False)
    return {"forecast": float(f.level[0]), "error": float(A - F)}


_cap("exponential_smoothing_update", level="beginner", name="Exponential smoothing forecast update", topic=_ES,
     purpose="The next forecast by simple exponential smoothing from the previous forecast, the actual value and the "
             "smoothing constant: F_new = F + alpha (A - F).",
     inputs=[_nin("F", "the previous forecast"), _nin("A", "the actual value of that period"),
             _nin("alpha", "smoothing constant, 0 < alpha <= 1")],
     outputs=[_out("forecast", "the new forecast"), _out("error", "the forecast error A - F")],
     primary="forecast", conditions=["0 < alpha <= 1"], method="F_new = F + alpha (A - F) = alpha A + (1 - alpha) F",
     latex=r"F_{t+1} = F_t + \alpha\,(A_t - F_t)", run=run_ses_update, reference=ref_ses_update,
     wrong=lambda F, A, alpha: {"forecast": alpha * F + (1 - alpha) * A, "error": A - F},
     rnd=lambda rng: {"F": round(rng.uniform(10, 500), 1), "A": round(rng.uniform(10, 500), 1),
                      "alpha": round(rng.uniform(0.05, 0.95), 2)},
     need=[r"exponential smoothing|smoothing constant|\bses\b", r"(?:previous|last|old) forecast"],
     avoid=r"\[|holt|trend|season",
     aliases={"F": _nm(r"(?:previous|last|old) forecast"), "A": _nm(r"actual(?: value| demand| sales)?", r"observed value"),
              "alpha": _ALPHA},
     templates=["With exponential smoothing, the previous forecast was 100, the actual value was 110 and alpha = 0.2. "
                "What is the new forecast?"],
     examples=[dict(F=100, A=110, alpha=0.2), dict(F=48.5, A=45, alpha=0.6)],
     questions=[_q("With exponential smoothing, the previous forecast was 100, the actual value was 110 and alpha = 0.2. "
                   "What is the new forecast?", 0),
                _q("Using a smoothing constant of 0.6, the last forecast was 48.5 and actual demand was 45. Find the next "
                   "forecast with exponential smoothing.", 1),
                _q("With exponential smoothing, the previous forecast was 100 and the actual value was 110. What is the "
                   "new forecast?", outcome="NEEDS_INPUT")])


def run_holt_update(l, b, y, alpha, beta, h=1):
    l, b, y = _num(l, "the previous level"), _num(b, "the previous trend"), _num(y, "the new observation")
    alpha, beta = _unit(alpha, "alpha"), _unit(beta, "beta")
    h = _int(h, "the horizon h", 1, 1000)
    new = alpha * y + (1 - alpha) * (l + b)
    tr = beta * (new - l) + (1 - beta) * b
    return {"level": new, "trend": tr, "forecast": new + h * tr}


def ref_holt_update(l, b, y, alpha, beta, h=1):
    import numpy as np
    from statsmodels.tsa.holtwinters import Holt
    f = Holt(np.array([float(y), float(y)]), initialization_method="known", initial_level=float(l),
             initial_trend=float(b)).fit(smoothing_level=alpha, smoothing_trend=beta, optimized=False)
    lev, tr = float(f.level[0]), float(f.trend[0])
    return {"level": lev, "trend": tr, "forecast": lev + int(h) * tr}


_cap("holt_update", level="intermediate", name="One update of Holt's method", topic=_ES,
     purpose="Update Holt's level and trend with one new observation and forecast h periods ahead.",
     inputs=[_nin("l", "previous level"), _nin("b", "previous trend"), _nin("y", "the new observation"),
             _nin("alpha", "level smoothing constant"), _nin("beta", "trend smoothing constant"),
             _iin("h", "forecast horizon (default 1)", required=False, lo=1)],
     outputs=[_out("level", "new level"), _out("trend", "new trend"), _out("forecast", "level + h trend")],
     primary="forecast", conditions=["0 < alpha, beta <= 1", "b_t = beta (l_t - l_{t-1}) + (1 - beta) b_{t-1}"],
     method="l = alpha y + (1 - alpha)(l + b); b = beta (l_new - l) + (1 - beta) b",
     latex=r"\ell_t=\alpha y_t+(1-\alpha)(\ell_{t-1}+b_{t-1}),\ b_t=\beta(\ell_t-\ell_{t-1})+(1-\beta)b_{t-1}",
     run=run_holt_update, reference=ref_holt_update,
     wrong=lambda l, b, y, alpha, beta, h=1: (lambda new: (lambda tr: {"level": new, "trend": tr, "forecast": new + h * tr})(
         beta * (y - l) + (1 - beta) * b))(alpha * y + (1 - alpha) * (l + b)),
     rnd=lambda rng: (lambda l: {"l": l, "b": round(rng.uniform(-5, 5), 2), "y": round(l + rng.uniform(-15, 15), 1),
                                 "alpha": round(rng.uniform(0.05, 0.95), 2), "beta": round(rng.uniform(0.02, 0.6), 2),
                                 "h": rng.randint(1, 6)})(round(rng.uniform(10, 300), 1)),
     need=[r"\bholt\b|double exponential", r"(?:previous|last|current) level"], avoid=r"\[|damp|winters",
     aliases={"l": _nm(r"(?:previous|last|current) level"), "b": _nm(r"(?:previous|last|current) trend"),
              "y": _nm(r"new (?:observation|value|actual)", r"actual(?: value)?"), "alpha": _ALPHA, "beta": _BETA, "h": A_H},
     templates=["Update Holt's method with previous level 50, previous trend 2, new observation 55, alpha = 0.3 and beta = 0.1."],
     examples=[dict(l=50, b=2, y=55, alpha=0.3, beta=0.1), dict(l=120.5, b=-1.5, y=116, alpha=0.5, beta=0.2, h=3)],
     questions=[_q("Update Holt's method with previous level 50, previous trend 2, new observation 55, alpha = 0.3 and "
                   "beta = 0.1.", 0),
                _q("Holt's method: previous level 120.5, previous trend -1.5, new observation 116, alpha = 0.5, "
                   "beta = 0.2. Forecast 3 periods ahead.", 1),
                _q("Update Holt's method with previous level 50, new observation 55, alpha = 0.3 and beta = 0.1.",
                   outcome="NEEDS_INPUT")])


def run_drift(x, h=1):
    x = _fin(x)
    _need(x, 2)
    h = _int(h, "the horizon h", 1, 100000)
    d = (x[-1] - x[0]) / (len(x) - 1)
    return {"drift": d, "forecast": x[-1] + h * d}


def ref_drift(x, h=1):
    """The drift as the mean of the period-to-period changes."""
    import numpy as np
    d = float(np.mean(np.diff(np.asarray(x, dtype=float))))
    return {"drift": d, "forecast": float(x[-1] + int(h) * d)}


_cap("drift_forecast", level="beginner", name="Random-walk-with-drift (drift method) forecast", topic=_TF,
     purpose="The drift method: the average change per period (x_n - x_1)/(n - 1) and the forecast x_n + h x drift.",
     inputs=[_vin("x", "the series, oldest first"), _iin("h", "periods ahead (default 1)", required=False, lo=1)],
     outputs=[_out("drift", "average change per period"), _out("forecast", "x_n + h drift")], primary="forecast",
     conditions=["n >= 2"], method="drift = (x_n - x_1)/(n - 1)",
     latex=r"\hat x_{n+h} = x_n + h\,\frac{x_n - x_1}{n-1}", run=run_drift, reference=ref_drift,
     wrong=lambda x, h=1: (lambda d: {"drift": d, "forecast": x[-1] + h * d})((x[-1] - x[0]) / len(x)),
     rnd=lambda rng: {"x": _walk(rng, rng.randint(2, 40)), "h": rng.randint(1, 12)},
     need=r"drift (?:method|forecast)|random walk with drift|naive (?:forecast |method )?with drift",
     aliases={"h": A_H},
     templates=["Forecast 3 periods ahead with the drift method for [100, 104, 103, 109, 112]."],
     examples=[dict(x="100, 104, 103, 109, 112", h=3), dict(x=[52.0, 50.5, 49.8, 47.1])],
     questions=[_q("Forecast 3 periods ahead with the drift method for [100, 104, 103, 109, 112].", 0),
                _q("Give the random walk with drift forecast for [52.0, 50.5, 49.8, 47.1].", 1),
                _q("Give the drift method forecast for [5].", outcome="INVALID_INPUT")])


def run_ar1_fc(x, phi, mu, h=1, sigma2=None):
    x, phi, mu = _num(x, "the last value"), _num(phi, "phi"), _num(mu, "the mean")
    if not abs(phi) < 1:
        raise ValueError("|phi| must be less than 1 (stationarity)")
    h = _int(h, "the horizon h", 1, 100000)
    out = {"forecast": _r12(mu + phi ** h * (x - mu))}
    if sigma2 is not None:
        s2 = _pos(sigma2, "the innovation variance")
        v = s2 * (1 - phi ** (2 * h)) / (1 - phi * phi)
        out.update(variance=_r12(v), lower=_r12(out["forecast"] - _Z975 * math.sqrt(v)),
                   upper=_r12(out["forecast"] + _Z975 * math.sqrt(v)))
    return out


def ref_ar1_fc(x, phi, mu, h=1, sigma2=None):
    """State-space route: the augmented transition matrix [[phi, mu(1-phi)], [0, 1]] raised to h; the variance as
    the sum of squared psi weights."""
    import numpy as np
    M = np.linalg.matrix_power(np.array([[phi, mu * (1 - phi)], [0.0, 1.0]]), int(h))
    f = float((M @ np.array([x, 1.0]))[0])
    out = {"forecast": f}
    if sigma2 is not None:
        v = float(sigma2 * np.sum(phi ** (2 * np.arange(int(h)))))
        out.update(variance=v, lower=f - 1.959963984540054 * np.sqrt(v), upper=f + 1.959963984540054 * np.sqrt(v))
    return out


def _rnd_ar1_fc(rng):
    d = {"x": round(rng.uniform(-20, 80), 2), "phi": round(rng.uniform(-0.95, 0.95), 2), "mu": round(rng.uniform(-10, 50), 2),
         "h": rng.randint(1, 15)}
    if rng.random() < 0.6:
        d["sigma2"] = round(rng.uniform(0.1, 10), 2)
    return d


_cap("ar1_forecast", level="intermediate", name="AR(1) forecast towards the mean", topic=_ARM,
     purpose="The h-step forecast mu + phi^h (x_n - mu) of a stationary AR(1) with mean mu, and (when the innovation "
             "variance is given) its forecast error variance and 95 % prediction interval.",
     inputs=[_nin("x", "the last observed value x_n"), _nin("phi", "AR coefficient, |phi| < 1"), _nin("mu", "the process mean"),
             _iin("h", "forecast horizon (default 1)", required=False, lo=1),
             _nin("sigma2", "innovation variance (optional, for the interval)", required=False)],
     outputs=[_out("forecast", "mu + phi^h (x_n - mu)"), _out("variance", "sigma^2 (1 - phi^(2h)) / (1 - phi^2)"),
              _out("lower", "lower 95 % limit"), _out("upper", "upper 95 % limit")],
     primary="forecast", conditions=["|phi| < 1", "known parameters", "normal innovations for the interval"],
     method="closed forms of the AR(1) recursion",
     latex=r"\hat x_{n+h} = \mu + \phi^h(x_n - \mu),\ \operatorname{Var} = \sigma^2\frac{1-\phi^{2h}}{1-\phi^2}",
     run=run_ar1_fc, reference=ref_ar1_fc,
     wrong=lambda x, phi, mu, h=1, sigma2=None: dict(run_ar1_fc(x, phi, mu, h, sigma2), forecast=phi ** h * x),
     rnd=_rnd_ar1_fc,
     need=[r"\bar\s*\(\s*1\s*\)|first[- ]order autoregress", r"forecast|predict"],
     avoid=r"\[|coefficients|yule|least squares|\bfit\b|estimat",
     aliases={"x": _nm(r"(?:last|latest|current) (?:value|observation)", r"x_?n"), "phi": _nm("phi"),
              "mu": _nm(r"(?:process |long[- ]run )?mean", "mu"), "h": A_H, "sigma2": _SIG2},
     templates=["Forecast 3 steps ahead for an AR(1) process with mean 50, phi = 0.7 and last value 58."],
     examples=[dict(x=58, phi=0.7, mu=50, h=3), dict(x=2.5, phi=-0.4, mu=1, h=2, sigma2=4)],
     questions=[_q("Forecast 3 steps ahead for an AR(1) process with mean 50, phi = 0.7 and last value 58.", 0),
                _q("For an AR(1) model with mean 1, phi = -0.4 and white noise variance 4, the last value is 2.5. Forecast "
                   "2 steps ahead with a prediction interval.", 1),
                _q("Forecast 3 steps ahead for an AR(1) process with mean 50, phi = 1.3 and last value 58.",
                   outcome="INVALID_INPUT")])


def run_ses_best(x, alphas):
    x = _fin(x)
    _need(x, 2)
    alphas = _fin(alphas, "the candidate alphas")
    best = None
    for a in alphas:
        r = run_ses(x, a)
        if best is None or r["sse"] < best[1]:
            best = (a, r["sse"], r["forecast"])
    return {"alpha": best[0], "sse": best[1], "forecast": best[2]}


def ref_ses_best(x, alphas):
    import numpy as np
    from statsmodels.tsa.holtwinters import SimpleExpSmoothing
    y = np.asarray(x, dtype=float)
    fits = [SimpleExpSmoothing(y, initialization_method="known", initial_level=float(y[0])).fit(
        smoothing_level=a, optimized=False) for a in alphas]
    i = int(np.argmin([f.sse for f in fits]))
    return {"alpha": float(alphas[i]), "sse": float(fits[i].sse), "forecast": float(fits[i].forecast(1)[0])}


_cap("ses_best_alpha", level="intermediate", name="Best smoothing constant among candidates", topic=_ES,
     purpose="Among candidate smoothing constants, the one whose simple exponential smoothing (l_0 = x_1) gives the "
             "smallest sum of squared one-step errors, with that SSE and its forecast.",
     inputs=[_vin("x", "the series, oldest first"), _vin("alphas", "candidate smoothing constants, each in (0, 1]", 1)],
     outputs=[_out("alpha", "the best smoothing constant"), _out("sse", "its sum of squared one-step errors"),
              _out("forecast", "its forecast of the next period")],
     primary="sse", conditions=["l_0 = x_1", "ties: the first candidate listed"],
     method="simple exponential smoothing for each candidate; the smallest SSE",
     latex=r"\alpha^* = \arg\min_{\alpha\in A}\sum_t (x_t - \ell_{t-1}(\alpha))^2", run=run_ses_best, reference=ref_ses_best,
     wrong=lambda x, alphas: (lambda a: dict(run_ses(x, a, l0=_mean(x)), alpha=a))(
         min(alphas, key=lambda a: run_ses(x, a, l0=_mean(x))["sse"])),
     rnd=lambda rng: {"x": _any(rng, 4, 40), "alphas": sorted({round(rng.uniform(0.05, 0.95), 2) for _ in range(rng.randint(2, 6))})},
     need=[r"exponential smoothing", r"smallest|best|minimi|optimal|lowest"], avoid=r"holt|winters|trend|season",
     labels={"x": _SERIES_LABEL, "alphas": r"alphas?|smoothing constants?|candidates?"},
     templates=["Which smoothing constant among [0.1, 0.3, 0.5] gives the smallest SSE for simple exponential smoothing of "
                "[20, 22, 21, 24, 23, 26]?"],
     examples=[dict(x="20, 22, 21, 24, 23, 26", alphas="0.1, 0.3, 0.5"), dict(x=[5, 9, 4, 8, 6, 7, 5], alphas=[0.2, 0.8])],
     questions=[_q("Which smoothing constant among [0.1, 0.3, 0.5] gives the smallest SSE for simple exponential smoothing "
                   "of [20, 22, 21, 24, 23, 26]?", 0),
                _q("Choose the best alpha from the candidates [0.2, 0.8] for exponential smoothing of the series "
                   "[5, 9, 4, 8, 6, 7, 5].", 1)])


def run_ts_reg(x, m, h=1):
    x = _fin(x)
    m = _int(m, "the season length m", 2, 52)
    h = _int(h, "the horizon h", 1, 10000)
    n = len(x)
    if n < m + 3:
        raise ValueError("the series needs at least m + 3 values")

    def row(t):
        s = (t - 1) % m
        return [1.0, float(t)] + [1.0 if s == j else 0.0 for j in range(1, m)]
    beta, _res, _dg = _lstsq([row(t) for t in range(1, n + 1)], x)
    eff = [0.0] + beta[2:]
    T = n + h
    return {"forecast": beta[0] + beta[1] * T + eff[(T - 1) % m], "intercept": beta[0], "slope": beta[1],
            "season_effects": eff}


def ref_ts_reg(x, m, h=1):
    import numpy as np
    import statsmodels.api as sm
    n, m = len(x), int(m)
    t = np.arange(1, n + 1)
    D = np.zeros((n, m - 1))
    for j in range(1, m):
        D[:, j - 1] = ((t - 1) % m == j)
    p = sm.OLS(np.asarray(x, dtype=float), np.column_stack([np.ones(n), t, D])).fit().params
    eff = [0.0] + [float(v) for v in p[2:]]
    T = n + int(h)
    return {"forecast": float(p[0] + p[1] * T + eff[(T - 1) % m]), "intercept": float(p[0]), "slope": float(p[1]),
            "season_effects": eff}


_cap("trend_seasonal_regression", level="intermediate", name="Linear trend with seasonal dummy variables",
     topic=_TF,
     purpose="Fit x_t = a + b t + (effect of t's season) by least squares with seasonal dummies (the first season is "
             "the baseline, effect 0; the series starts in season 1) and forecast period n + h.",
     inputs=[_vin("x", "the series, oldest first, starting in season 1", 5), _iin("m", "season length", lo=2),
             _iin("h", "periods ahead (default 1)", required=False, lo=1)],
     outputs=[_out("forecast", "the fitted value at t = n + h"), _out("intercept", "a"), _out("slope", "b per period"),
              _out("season_effects", "the m seasonal effects (season 1 = 0)")],
     primary="forecast", conditions=["n >= m + 3", "x_1 is in season 1", "season 1 is the baseline"],
     method="ordinary least squares on (1, t, D_2, ..., D_m)",
     latex=r"x_t = a + b\,t + \sum_{j=2}^{m}\gamma_j D_{j,t} + e_t", run=run_ts_reg, reference=ref_ts_reg,
     wrong=lambda x, m, h=1: (lambda o: dict(o, forecast=o["intercept"] + o["slope"] * (len(x) + h)
                                              + o["season_effects"][(len(x) + h) % int(m)]))(run_ts_reg(x, m, h)),
     rnd=lambda rng: (lambda m: {"x": _seasonal(rng, rng.randint(m + 3, 4 * m + 4), m), "m": m, "h": rng.randint(1, 2 * m)})(
         rng.choice([4, 4, 12, 3, 7])),
     need=r"seasonal dumm(?:y|ies)|dummy variables? for (?:the )?seasons|trend and seasonal (?:dummy|regression)",
     aliases={"m": A_SEASON, "h": A_H}, extract=_season_words,
     templates=["Fit a linear trend with seasonal dummies to the quarterly series [30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51] "
                "and forecast 1 period ahead."],
     examples=[dict(x="30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51", m=4, h=1),
               dict(x=[12, 18, 15, 14, 20, 17, 16, 22, 19], m=3, h=2)],
     questions=[_q("Fit a linear trend with seasonal dummies to the quarterly series "
                   "[30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51] and forecast 1 period ahead.", 0),
                _q("Using a trend and seasonal dummy regression with season length 3, forecast 2 periods ahead for "
                   "[12, 18, 15, 14, 20, 17, 16, 22, 19].", 1),
                _q("Fit a linear trend with seasonal dummies to the quarterly series [30, 21, 29].",
                   outcome="INVALID_INPUT")])


def run_ar2_acf(phi1, phi2, K=5):
    phi1, phi2 = _num(phi1, "phi1"), _num(phi2, "phi2")
    if not _stationary([phi1, phi2]):
        raise ValueError("the AR(2) process must be stationary")
    K = _int(K, "the number of lags K", 1, 500)
    r = [1.0, phi1 / (1 - phi2)]
    for _ in range(2, K + 1):
        r.append(phi1 * r[-1] + phi2 * r[-2])
    return {"acf": r[1:K + 1], "rho1": r[1]}


def ref_ar2_acf(phi1, phi2, K=5):
    from statsmodels.tsa.arima_process import arma_acf
    r = arma_acf([1.0, -phi1, -phi2], [1.0], lags=int(K) + 1)
    return {"acf": [float(v) for v in r[1:]], "rho1": float(r[1])}


_cap("ar2_theoretical_acf", level="intermediate", name="Theoretical autocorrelations of an AR(2) process", topic=_ART,
     purpose="The autocorrelations rho_1..rho_K of a stationary AR(2) process from the Yule-Walker recursion "
             "rho_1 = phi1/(1 - phi2), rho_k = phi1 rho_{k-1} + phi2 rho_{k-2}.",
     inputs=[_nin("phi1", "phi_1"), _nin("phi2", "phi_2"), _iin("K", "number of lags (default 5)", required=False, lo=1)],
     outputs=[_out("acf", "rho_1..rho_K"), _out("rho1", "rho_1")], primary="rho1", conditions=["stationary AR(2)"],
     method="Yule-Walker recursion", latex=r"\rho_1 = \frac{\phi_1}{1-\phi_2},\ \rho_k = \phi_1\rho_{k-1} + \phi_2\rho_{k-2}",
     run=run_ar2_acf, reference=ref_ar2_acf,
     wrong=lambda phi1, phi2, K=5: (lambda r: {"acf": r, "rho1": r[0]})([phi1] + [0.0] * (int(K) - 1)),
     rnd=lambda rng: dict(zip(("phi1", "phi2"), _rand_stationary(rng, 2, -0.9, 0.9)), K=rng.randint(1, 10)),
     need=[r"\bar\s*\(\s*2\s*\)", r"auto-?correlation|\bacf\b"], avoid=r"\[|stationar|partial",
     aliases={"phi1": _pair_alias("phi")[:1], "phi2": _pair_alias("phi")[1:], "K": _K_LAGS},
     templates=["Find the autocorrelations of the AR(2) process with phi1 = 0.5 and phi2 = 0.3."],
     examples=[dict(phi1=0.5, phi2=0.3), dict(phi1=1.0, phi2=-0.5, K=4)],
     questions=[_q("Find the autocorrelations of the AR(2) process with phi1 = 0.5 and phi2 = 0.3.", 0),
                _q("Compute the theoretical ACF up to lag 4 of the AR(2) model with phi1 = 1.0 and phi2 = -0.5.", 1),
                _q("Find the autocorrelations of the AR(2) process with phi1 = 0.5 and phi2 = 0.6.", outcome="INVALID_INPUT")])


def run_ma1_pacf(theta, K=5):
    th = _num(theta, "theta")
    if abs(th) == 1:
        raise ValueError("|theta| = 1 is not allowed (the partial autocorrelations do not decay)")
    K = _int(K, "the number of lags K", 1, 500)
    p = [_r12(-((-th) ** k) * (1 - th * th) / (1 - th ** (2 * (k + 1)))) for k in range(1, K + 1)]
    return {"pacf": p, "pacf1": p[0]}


def ref_ma1_pacf(theta, K=5):
    from statsmodels.tsa.arima_process import arma_pacf
    p = arma_pacf([1.0], [1.0, theta], lags=int(K) + 1)
    return {"pacf": [float(v) for v in p[1:]], "pacf1": float(p[1])}


_cap("ma1_theoretical_pacf", level="advanced", name="Theoretical partial autocorrelations of an MA(1) process",
     topic=_ART,
     purpose="The partial autocorrelations phi_11..phi_KK of x_t = e_t + theta e_{t-1} (they decay, never cut off).",
     inputs=[_nin("theta", "MA coefficient, |theta| != 1"), _iin("K", "number of lags (default 5)", required=False, lo=1)],
     outputs=[_out("pacf", "phi_11..phi_KK"), _out("pacf1", "phi_11 = theta/(1 + theta^2)")], primary="pacf1",
     conditions=["|theta| != 1", "plus-sign MA convention"],
     method="closed form phi_kk = -(-theta)^k (1 - theta^2)/(1 - theta^(2(k+1)))",
     latex=r"\phi_{kk} = -\frac{(-\theta)^k(1-\theta^2)}{1-\theta^{2(k+1)}}", run=run_ma1_pacf, reference=ref_ma1_pacf,
     wrong=lambda theta, K=5: (lambda p: {"pacf": p, "pacf1": p[0]})([theta ** k for k in range(1, int(K) + 1)]),
     rnd=lambda rng: {"theta": round(rng.choice([rng.uniform(-0.95, -0.05), rng.uniform(0.05, 0.95)]), 2), "K": rng.randint(1, 10)},
     need=[r"\bma\s*\(\s*1\s*\)", r"partial auto-?correlation|\bpacf\b"], avoid=r"\[",
     aliases={"theta": _nm("theta"), "K": _K_LAGS},
     templates=["Find the partial autocorrelations of the MA(1) process with theta = 0.6."],
     examples=[dict(theta=0.6), dict(theta=-0.8, K=3)],
     questions=[_q("Find the partial autocorrelations of the MA(1) process with theta = 0.6.", 0),
                _q("Compute the theoretical PACF up to lag 3 of the MA(1) model with theta = -0.8.", 1),
                _q("Find the partial autocorrelations of the MA(1) process with theta = 1.", outcome="INVALID_INPUT")])


def run_arma_pacf(phi, theta, K=5):
    phi, theta = _arma_check(phi, theta)
    K = _int(K, "the number of lags K", 1, 200)
    g = _arma_acov(phi, theta, 1.0, K)
    _p, pacf, _v = _levinson([v / g[0] for v in g], K)
    return {"pacf": pacf, "pacf1": pacf[0]}


def ref_arma_pacf(phi, theta, K=5):
    import numpy as np
    from statsmodels.tsa.arima_process import arma_pacf
    p = arma_pacf(np.r_[1.0, -np.asarray(phi, dtype=float)], np.r_[1.0, np.asarray(theta, dtype=float)], lags=int(K) + 1)
    return {"pacf": [float(v) for v in p[1:]], "pacf1": float(p[1])}


_cap("arma_theoretical_pacf", level="expert", name="Theoretical partial autocorrelations of an ARMA(p,q) process",
     topic=_ART,
     purpose="The partial autocorrelations phi_11..phi_KK of a stationary ARMA(p,q) process (Durbin-Levinson on its exact "
             "autocorrelations).",
     inputs=[_vin("phi", "AR coefficients ([0] for none)", 1, max_length=8),
             _vin("theta", "MA coefficients ([0] for none)", 1, max_length=8),
             _iin("K", "number of lags (default 5)", required=False, lo=1)],
     outputs=[_out("pacf", "phi_11..phi_KK"), _out("pacf1", "phi_11 = rho_1")], primary="pacf1",
     conditions=["stationary AR part", "plus-sign MA convention"],
     method="exact autocovariances, then the Durbin-Levinson recursion",
     latex=r"\phi_{kk} = \frac{\rho_k-\sum_{j=1}^{k-1}\phi_{k-1,j}\rho_{k-j}}{1-\sum_{j=1}^{k-1}\phi_{k-1,j}\rho_j}",
     run=run_arma_pacf, reference=ref_arma_pacf,
     wrong=lambda phi, theta, K=5: (lambda o: {"pacf": o["acf"], "pacf1": o["rho1"]})(
         run_arma_acf(phi, theta, K)),
     rnd=lambda rng: dict(_rnd_arma(rng), K=rng.randint(2, 8)),
     need=[r"\barma\s*\(|autoregressive moving[- ]average", r"partial auto-?correlation|\bpacf\b", r"\["],
     labels={"phi": _PHI_LABEL, "theta": _THETA_LABEL}, aliases={"K": _K_LAGS}, extract=_arma_order,
     templates=["Find the partial autocorrelations of the ARMA(1,1) process with phi = [0.5] and theta = [0.4]."],
     examples=[dict(phi="0.5", theta="0.4"), dict(phi=[0.6, -0.3], theta=[0.2], K=4)],
     questions=[_q("Find the partial autocorrelations of the ARMA(1,1) process with phi = [0.5] and theta = [0.4].", 0),
                _q("Compute the theoretical PACF up to lag 4 of the ARMA(2,1) model with phi = [0.6, -0.3] and theta = [0.2].", 1)])
