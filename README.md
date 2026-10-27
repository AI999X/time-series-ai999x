# time-series-ai999x — offline, verified time series solver with step-by-step answers

[![tests](https://github.com/AI999X/time-series-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/time-series-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/time-series-ai999x)](https://pypi.org/project/time-series-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/time-series-ai999x)

**[Install](https://pypi.org/project/time-series-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/time-series-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/time-series-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.1

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
| Package | `time-series-ai999x` 1.0.1 (import `time_series_ai999x`) |
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

Command line: `time-series-ai999x "<question>"`, `time-series-ai999x --capabilities`. The outputs above were produced by running this exact code on time-series-ai999x 1.0.1.

## Find the right capability, and draw it

`time_series.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "time-series-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import time_series_ai999x as time_series

time_series.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

Each question was put to `time-series-ai999x`; the answer is the one it returned, with the steps it took.

**Simple moving average** · _Moving averages_

> Find the 3-period simple moving average of [10, 12, 14, 13, 15].

→ **sma = [12, 13, 14], last = 14, count = 3**

<details><summary>Steps</summary>

- Simple moving average: x = [10, 12, 14, 13, 15], k = 3 → sma = [12, 13, 14], last = 14, count = 3

</details>

**Holt's linear trend method** · _Exponential smoothing_

> Apply Holt's linear trend method with alpha = 0.5 and beta = 0.3 to [10, 12, 15, 16, 19, 21].

→ **forecast = 22.65328591, level = 20.65533969, trend = 1.997946219, sse = 7.492761855**

<details><summary>Steps</summary>

- Holt's linear trend method: x = [10, 12, 15, 16, 19, 21], alpha = 0.5, beta = 0.3 → forecast = 22.65328591, level = 20.65533969, trend = 1.997946219, sse = 7.492761855

</details>

**Ljung-Box test** · _Autocorrelation_

> Perform the Ljung-Box test with 3 lags on [4, 6, 5, 8, 7, 9, 8, 10, 9, 12, 11, 13].

→ **q = 9.319101598, p_value = 0.0253357685, df = 3**

<details><summary>Steps</summary>

- Ljung-Box test: x = [4, 6, 5, 8, 7, 9, 8, 10, 9, 12, 11, 13], h = 3 → q = 9.319101598, p_value = 0.0253357685, df = 3

</details>

**Mean absolute percentage error** · _Forecast accuracy_

> Find the MAPE for actual values [10, 12, 14, 13] and forecasts [11, 12, 13, 15].

→ **mape = 8.131868132, n = 4**

<details><summary>Steps</summary>

- Mean absolute percentage error: y = [10, 12, 14, 13], f = [11, 12, 13, 15] → mape = 8.131868132, n = 4

</details>

**Kalman filter for a random-walk (local level) model** · _Kalman filtering_

`m.solve("kalman_filter_local_level", x="10.4, 11.1, 10.7, 11.9, 12.3", q=0.5, r=2, m0=10, p0=5)`

→ **estimate = 11.62410315, variance = 0.7906396594, gain = 0.3953198297, estimates = [10.29333333, 10.69327731, 10.69614922, 11.18222422, 11.62410315], log_likelihood = -8.617118062**

<details><summary>Steps</summary>

- Kalman filter for a random-walk (local level) model: x = [10.4, 11.1, 10.7, 11.9, 12.3], q = 0.5, r = 2, m0 = 10, p0 = 5 → estimate = 11.62410315, variance = 0.7906396594, gain = 0.3953198297, estimates = [10.29333333, 10.69327731, 10.69614922, 11.18222422, 11.62410315], log_likelihood = -8.617118062

</details>

**AR(1) forecast towards the mean** · _ARIMA models_

> Forecast 3 steps ahead for an AR(1) process with mean 50, phi = 0.7 and last value 58.

→ **forecast = 52.744**

<details><summary>Steps</summary>

- AR(1) forecast towards the mean: x = 58, phi = 0.7, mu = 50, h = 3 → forecast = 52.744

</details>

## Figures

Every point was computed by `time-series-ai999x`.

**Seasonal series and three forecasts**

![Seasonal series and three forecasts — computed by time-series-ai999x](https://huggingface.co/AI999X/time-series-ai999x/resolve/main/figures/holt-winters-additive-forecast.svg)

_A quarterly series of 12 values (30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51) and three forecasts for the next 8 periods, each step computed by the package: Holt–Winters additive and multiplicative (m = 4, α = 0.5, β = 0.3, γ = 0.2) and a linear trend with seasonal effects; 8-step forecasts 67.8788, 68.1074 and 69.875._

**AR(1) forecast fan reverting to the mean**

![AR(1) forecast fan reverting to the mean — computed by time-series-ai999x](https://huggingface.co/AI999X/time-series-ai999x/resolve/main/figures/ar1-forecast-forecast.svg)

_AR(1) forecasts from the last value x = 58 with φ = 0.7, μ = 50 and innovation variance σ² = 4, 1 to 12 steps ahead, with the 95 % interval the package returns at each step (it widens to [44.6223, 55.5992] at h = 12); marked: h = 3, forecast = 52.744._

**Autocorrelation function with its significance band**

![Autocorrelation function with its significance band — computed by time-series-ai999x](https://huggingface.co/AI999X/time-series-ai999x/resolve/main/figures/sample-autocorrelation-acf.svg)

_Sample autocorrelations of the series 30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51 at lags 1 to 6 (0.512567, 0.361929, 0.190263, 0.157039, -0.138822, -0.267831) and the 95 % band ±0.565793 … ±0.789621 from Bartlett's standard errors, all returned by the package._

**Kalman filter and smoother against noisy observations**

![Kalman filter and smoother against noisy observations — computed by time-series-ai999x](https://huggingface.co/AI999X/time-series-ai999x/resolve/main/figures/kalman-filter-local-level-filter.svg)

_A local-level model (state noise q = 0.5, observation noise r = 2, start m₀ = 10, p₀ = 5) run by the package on 12 observations: the filtered level after each observation and the smoothed level from the backward pass; marked: t = 12, filtered level = 13.8669 (variance 0.780786, gain 0.390393)._

**Two change points and the segment means**

![Two change points and the segment means — computed by time-series-ai999x](https://huggingface.co/AI999X/time-series-ai999x/resolve/main/figures/two-change-points-segments.svg)

_The two change points the package finds in x = 5, 6, 5, 12, 13, 12, 20, 21, 19: after observations 3 and 6, splitting the series into segments with means 5.33333, 12.3333, 20 (total squared error 3.33333)._

**Where an AR(2) model is stationary**

![Where an AR(2) model is stationary — computed by time-series-ai999x](https://huggingface.co/AI999X/time-series-ai999x/resolve/main/figures/ar2-stationarity-region.svg)

_Which AR(2) models x_t = φ₁x_(t−1) + φ₂x_(t−2) + e_t are stationary: the package's verdict at 1025 points of the (φ₁, φ₂) plane — stationary with real roots (120 points), stationary with complex roots, i.e. damped oscillation (223), or not stationary (682); together they trace the stationarity triangle. Marked: φ₁ = 0.5, φ₂ = 0.3, real roots, largest modulus 0.85208._

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
@software{time_series_ai999x_1_0_1,
  title  = {time-series-ai999x: an offline, verified time series solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.1},
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
