# Time Series (time-series-ai999x)

[![tests](https://github.com/AI999X/time-series-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/time-series-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/time-series-ai999x)](https://pypi.org/project/time-series-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/time-series-ai999x)

Time Series covers moving averages, exponential smoothing, autocorrelation, ARIMA basics, stationarity tests, change points, forecasting accuracy and Kalman filtering: 78 calculations in 14 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Time Series desktop app: Simple moving average, with its worked example and live scene](https://www.ai999x.com/?ax_shot=time-series-ai999x/hero.webp)](https://www.ai999x.com/models/time-series-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-time-series-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-time-series-1.6.6-linux-x86_64.tar.gz) |
| **Command line** `time-series` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/time-series-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install time-series-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Time Series desktop app: every scene and chart is drawn from the computed answer.

[![Time Series desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=time-series-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=time-series-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=time-series-ai999x/clip.mp4)*

| | |
|---|---|
| [![Simple moving average: the live scene in the Time Series desktop app](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-1.webp)](https://www.ai999x.com/models/time-series-ai999x/#cap-simple_moving_average)<br>Simple moving average | [![Holt's linear trend method: the live scene in the Time Series desktop app](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-2.webp)](https://www.ai999x.com/models/time-series-ai999x/#cap-holt_linear_trend)<br>Holt's linear trend method |
| [![Ljung-Box test: the live scene in the Time Series desktop app](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-3.webp)](https://www.ai999x.com/models/time-series-ai999x/#cap-ljung_box_test)<br>Ljung-Box test | [![Mean absolute percentage error: the live scene in the Time Series desktop app](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-4.webp)](https://www.ai999x.com/models/time-series-ai999x/#cap-mean_absolute_percentage_error)<br>Mean absolute percentage error |
| [![Kalman filter for a random-walk (local level) model: the live scene in the Time Series desktop app](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-5.webp)](https://www.ai999x.com/models/time-series-ai999x/#cap-kalman_filter_local_level)<br>Kalman filter for a random-walk (local level) model | [![AR(1) forecast towards the mean: the live scene in the Time Series desktop app](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-6.webp)](https://www.ai999x.com/models/time-series-ai999x/#cap-ar1_forecast)<br>AR(1) forecast towards the mean |
| [![Mean, variance and autocorrelation of an AR(1) process: the live scene in the Time Series desktop app](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-7.webp)](https://www.ai999x.com/models/time-series-ai999x/#cap-ar1_properties)<br>Mean, variance and autocorrelation of an AR(1) process | [![Single change point in the mean (least squares): the live scene in the Time Series desktop app](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-8.webp)](https://www.ai999x.com/models/time-series-ai999x/#cap-change_point_mean_shift)<br>Single change point in the mean (least squares) |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Simple moving average: a chart in the Time Series desktop app](https://www.ai999x.com/?ax_shot=time-series-ai999x/chart-1.webp)](https://www.ai999x.com/models/time-series-ai999x/#cap-simple_moving_average)<br>Simple moving average | [![Holt's linear trend method: a chart in the Time Series desktop app](https://www.ai999x.com/?ax_shot=time-series-ai999x/chart-2.webp)](https://www.ai999x.com/models/time-series-ai999x/#cap-holt_linear_trend)<br>Holt's linear trend method | [![Mean absolute percentage error: a chart in the Time Series desktop app](https://www.ai999x.com/?ax_shot=time-series-ai999x/chart-3.webp)](https://www.ai999x.com/models/time-series-ai999x/#cap-mean_absolute_percentage_error)<br>Mean absolute percentage error | [![AR(1) forecast towards the mean: a chart in the Time Series desktop app](https://www.ai999x.com/?ax_shot=time-series-ai999x/chart-4.webp)](https://www.ai999x.com/models/time-series-ai999x/#cap-ar1_forecast)<br>AR(1) forecast towards the mean |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import time_series_ai999x as time_series

a = time_series.ask(
    "Forecast 3 steps ahead for an AR(1) process with mean 50, phi = "
    "0.7 and last value 58."
)
print(a.outcome, a.values)
# SOLVED {'forecast': 52.744}

a = time_series.ask(
    "Find the 3-step forecast error variance of the ARMA(1,1) model "
    "with phi = [0.5], theta = [0.3] and sigma2 = 2."
)
print(a.outcome, a.values)
# SOLVED {'variance': 3.6000000000000005, 'sd': 1.8973665961010278}

a = time_series.ask(
    "At which lag up to 3 is the cross-correlation strongest between x "
    "= [1, 3, 2, 5, 4, 6, 5, 8] and y = [3, 2, 5, 4, 6, 5, 8, 7]?"
)
print(a.outcome, a.values)
# SOLVED {'lag': 1, 'ccf': 0.713656664657495}

a = time_series.ask(
    "Which smoothing constant among [0.1, 0.3, 0.5] gives the smallest "
    "SSE for simple exponential smoothing of [20, 22, 21, 24, 23, 26]?"
)
print(a.outcome, a.values)
# SOLVED {'alpha': 0.5, 'sse': 23.8125, 'forecast': 24.375}

a = time_series.ask(
    "Find the MAE for actual values [10, 12, 14, 13] and forecasts "
    "[11, 12, 13, 15]."
)
print(a.outcome, a.values)
# SOLVED {'mae': 1.0, 'n': 4}

a = time_series.ask(
    "Exponentially interpolate the value at t = 2.5 from times [1, 2, "
    "3, 4] and values [100, 120, 150, 170]."
)
print(a.outcome, a.values)
# SOLVED {'value': 134.16407865}

a = time_series.ask(
    "Perform the Box-Pierce test with 3 lags on [4, 6, 5, 8, 7, 9, 8, "
    "10, 9, 12, 11, 13]."
)
print(a.outcome, a.values)
# SOLVED {'q': 6.94389606619248, 'p_value': 0.0737076577117, 'df': 3}

a = time_series.ask(
    "Compute the cross-correlation at lag 1 between x = [1, 3, 2, 5, "
    "4, 6] and y = [2, 1, 4, 3, 6, 5]."
)
print(a.outcome, a.values)
# SOLVED {'ccf': 0.6428571428571429}

a = time_series.ask(
    "Find the MAPE for actual values [10, 12, 14, 13] and forecasts "
    "[11, 12, 13, 15]."
)
print(a.outcome, a.values)
# SOLVED {'mape': 8.131868131868131, 'n': 4}

a = time_series.ask(
    "Linearly interpolate the value at t = 2.5 from times [1, 2, 3, 4] "
    "and values [10, 14, 13, 18]."
)
print(a.outcome, a.values)
# SOLVED {'value': 13.5}

a = time_series.ask(
    "Find the mean and variance of the AR(1) process with constant 2, "
    "phi = 0.6 and sigma2 = 4."
)
print(a.outcome, a.values)
# SOLVED {'mean': 5.0, 'variance': 6.25, 'acf_k': 0.6, 'acov_k': 3.75}

a = time_series.ask(
    "Perform the Ljung-Box test with 3 lags on [4, 6, 5, 8, 7, 9, 8, "
    "10, 9, 12, 11, 13]."
)
print(a.outcome, a.values)
# SOLVED {'q': 9.319101598068064, 'p_value': 0.0253357684996, 'df': 3}

a = time_series.ask(
    "With exponential smoothing, the previous forecast was 100, the "
    "actual value was 110 and alpha = 0.2. What is the new forecast?"
)
print(a.outcome, a.values)
# SOLVED {'forecast': 102.0, 'error': 10.0}

a = time_series.ask(
    "Find the MASE for training data [10, 12, 11, 14, 13], actual "
    "values [15, 14, 16] and forecasts [14, 15, 15]."
)
print(a.outcome, a.values)
# SOLVED {'mase': 0.5714285714285714, 'mae': 1.0, 'scale': 1.75}

a = time_series.ask(
    "Do a Kalman update with prior mean = 10, prior variance = 4, "
    "measurement = 12 and measurement variance = 1."
)
print(a.outcome, a.values)
# SOLVED {'mean': 11.6, 'variance': 0.7999999999999998, 'gain': 0.8}
```

## Command line: 15 examples

Each command below was run with `time-series` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ time-series solve ar1_forecast x=58 phi=0.7 mu=50 h=3
forecast = 52.744

$ time-series solve forecast_error_variance "phi=[0.5]" "theta=[0.3]" sigma2=2 h=3
variance = 3.6
sd = 1.897366596

$ time-series solve bartlett_standard_error "x=[4, 6, 5, 8, 7, 9, 8, 10, 9, 12]" k=3
se = 0.4144598129
half_width_95 = 0.8123263063
acf = -0.04108527132

$ time-series solve cusum_change_point "x=[5, 6, 5, 7, 6, 12, 13, 12, 14]"
change_index = 5
max_cusum = 15.44444444
mean_before = 5.8
mean_after = 12.75

$ time-series solve seasonal_indices_additive "x=[30, 21, 29, 31, 40, 30, 39, 41, 50, 39, 48, 51]" m=4
indices = [6.484375,-6.453125,0.171875,-0.203125]
first = 6.484375

$ time-series solve cross_correlation_best_lag "x=[1, 3, 2, 5, 4, 6, 5, 8]" "y=[3, 2, 5, 4, 6, 5, 8, 7]" K=3
lag = 1
ccf = 0.7136566647

$ time-series solve ses_best_alpha "alphas=[0.1, 0.3, 0.5]" "x=[20, 22, 21, 24, 23, 26]"
alpha = 0.5
sse = 23.8125
forecast = 24.375

$ time-series solve mean_absolute_error "y=[10, 12, 14, 13]" "f=[11, 12, 13, 15]"
mae = 1
n = 4

$ time-series solve exponential_interpolation "t=[1, 2, 3, 4]" "y=[100, 120, 150, 170]" t0=2.5
value = 134.1640786

$ time-series solve kalman_filter_constant_velocity "x=[1.1, 1.9, 3.2, 4, 4.8]" dt=1 q=0.1 r=1 p0=0 v0=1 var0=10
position = 4.896949573
velocity = 0.9385623391
p11 = 0.6153900119
p12 = 0.2443095291
p22 = 0.224491015

$ time-series solve centred_moving_average "x=[12, 15, 14, 18, 20, 19, 23, 25]" k=4
cma = [15.75,17.25,18.875,20.875]
first_index = 3
first = 15.75

$ time-series solve augmented_dickey_fuller_test "x=[10.2, 10.8, 10.5, 11.3, 11, 11.9, 11.6, 12.4, 12, 12.9, 12.5, 13.4]" p=2 regression=c
tau = -1.186531751
gamma = -0.03905259823
se = 0.0329132349
nobs = 9

$ time-series solve compound_average_growth_rate "x=[100, 110, 125, 133, 150]"
rate = 0.1066819197
rate_percent = 10.66819197
total_growth = 0.5
periods = 4

$ time-series solve exponential_trend_forecast "x=[100, 112, 124, 140, 155]" h=1
a = 89.64949872
b = 0.1099653413
growth_rate = 0.1162393824
forecast = 173.4170967

$ time-series solve ar_least_squares "x=[4, 6, 5, 8, 7, 9, 8, 10, 9, 12]" p=1
const = 3.5625
phi = [0.6354166666666665]
phi1 = 0.6354166667
sigma2 = 2.515046296
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/time-series-ai999x/blob/main/docs/time_series_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/time-series-ai999x/blob/main/docs/time_series_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/time-series-ai999x/blob/main/docs/time_series_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/time-series-ai999x/blob/main/docs/time_series_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/time-series-ai999x/blob/main/docs/time_series_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/time-series-ai999x/blob/main/docs/time_series_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/time-series-ai999x/blob/main/docs/time_series_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/time-series-ai999x/blob/main/docs/time_series_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/time-series-ai999x/blob/main/docs/time_series_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Time Series on ai999x.com](https://www.ai999x.com/models/time-series-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.6 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/time-series-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.6`; the [GitHub release](https://github.com/AI999X/time-series-ai999x/releases/tag/v1.6.6) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `time-series` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/time-series`) and its data (`rust/models/time_series/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p time-series-ai999x
./target/release/time-series --help
cargo test -p time-series-ai999x
```
