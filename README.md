# time-series-ai999x — offline, verified time series solver with step-by-step answers

[![tests](https://github.com/AI999X/time-series-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/time-series-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/time-series-ai999x)](https://pypi.org/project/time-series-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/time-series-ai999x)

**[Install](https://pypi.org/project/time-series-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/time-series-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/time-series-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

Field: **Mathematics & Statistics** · Tasks: Question Answering, Token Classification, Text Classification, Time Series Forecasting, Sentence Similarity, Feature Extraction

time-series-ai999x is an offline, deterministic Python solver that answers time series questions with verified results from 77 certified capabilities, gives 403 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**time-series-ai999x** is a compact, deterministic time series engine for Python. It reads a question in plain English (or a typed call), picks a certified time series capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified time series capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 403 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** moving averages, exponential smoothing and Holt-Winters, autocorrelation, ARIMA basics, stationarity tests, change points, forecast accuracy and Kalman filtering.

## Model overview

| Property | Value |
|---|---|
| Package | `time-series-ai999x` 1.0.0 (import `time_series_ai999x`) |
| Type | Deterministic neuro-symbolic time series engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 77 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 403 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install time-series-ai999x
```

```python
import time_series_ai999x as time_series

time_series.ask("Find the 3-period simple moving average of [10, 12, 14, 13, 15].").show()
# -> SOLVED — sma = [12.0, 13.0, 14.0], last = 14, count = 3  (simple_moving_average)
time_series.ask("Apply Holt's linear trend method with alpha = 0.5 and beta = 0.3 to [10, 12, 15, 16, 19, 21].").show()
# -> SOLVED — forecast = 22.6533, level = 20.6553, trend = 1.99795, sse = 7.49276  (holt_linear_trend)
time_series.ask("Perform the Ljung-Box test with 3 lags on [4, 6, 5, 8, 7, 9, 8, 10, 9, 12, 11, 13].").show()
# -> SOLVED — q = 9.3191, p_value = 0.0253358, df = 3  (ljung_box_test)
time_series.ask("Find the MAPE for actual values [10, 12, 14, 13] and forecasts [11, 12, 13, 15].").show()
# -> SOLVED — mape = 8.13187, n = 4  (mean_absolute_percentage_error)
time_series.solve("kalman_filter_local_level", x="10.4, 11.1, 10.7, 11.9, 12.3", q=0.5, r=2, m0=10, p0=5).show()
# -> SOLVED — estimate = 11.6241, variance = 0.79064, gain = 0.39532, estimates = [10.293333333333333, 10.693277310924369, 10.696149217809866, 11.182224215246636, 11.624103148133084]  (kalman_filter_local_level)
time_series.ask("Who created you?").message
# -> time-series-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `time-series-ai999x "<question>"`, `time-series-ai999x --capabilities`. The outputs above were produced by running this exact code on time-series-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

77 certified procedures. A few of them:

- **Procedures:** AR(1) forecast towards the mean; AR(p) estimation by Yule-Walker; AR(p) estimation by conditional least squares; Augmented Dickey-Fuller statistic; Bartlett standard error of a sample autocorrelation; Best smoothing constant among candidates; Box-Pierce test; CUSUM estimate of a change point; Centred moving average; Compound average growth rate of a series; Damped-trend Holt method; Dickey-Fuller unit-root statistic; Exponential (log-linear) interpolation; Exponential moving average; Exponential smoothing forecast update; Exponential trend fit and forecast; Forecasts of an AR(p) model; Forecasts of an ARIMA(p,1,0) model; Holt's linear trend method; Holt-Winters additive method; Holt-Winters multiplicative method; Invertibility of an MA(2) process; KPSS stationarity statistic; Kalman filter for a constant-velocity model …

`time_series_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified time series capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (moving averages, exponential smoothing and Holt-Winters, autocorrelation, ARIMA basics, stationarity tests, change points, forecast accuracy and Kalman filtering); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{time_series_ai999x_1_0_0,
  title  = {time-series-ai999x: an offline, verified time series solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/time-series-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/time-series-ai999x/discussions or visit https://ai999x.com.

## Support the project

time-series-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/time-series-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/time-series-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/time-series-ai999x/tree/main/onnx).
