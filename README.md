# Time Series (time-series-ai999x)

[![tests](https://github.com/AI999X/time-series-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/time-series-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/time-series-ai999x)](https://pypi.org/project/time-series-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/time-series-ai999x)

Time Series covers moving averages, exponential smoothing, autocorrelation, ARIMA basics, stationarity tests, change points, forecasting accuracy and Kalman filtering: 78 calculations in 14 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Time Series desktop app: h-step forecast error variance of an ARMA model, with its worked example and live scene](https://www.ai999x.com/?ax_shot=time-series-ai999x/hero.webp)](https://www.ai999x.com/models/time-series-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `time-series` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/time-series-ai999x/#try) |
| **Python** 3.10+ | `pip install time-series-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

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

## Screenshots

[![The Time Series desktop app: h-step forecast error variance of an ARMA model, with its worked example and live scene](https://www.ai999x.com/?ax_shot=time-series-ai999x/hero-dark.webp)](https://www.ai999x.com/models/time-series-ai999x/)

[![Forecasts of an AR(p) model: the live scene in the Time Series desktop app](https://www.ai999x.com/?ax_shot=time-series-ai999x/see-1.webp)](https://www.ai999x.com/models/time-series-ai999x/)

*Forecasts of an AR(p) model*

[![Exponential smoothing forecast update: the live scene in the Time Series desktop app](https://www.ai999x.com/?ax_shot=time-series-ai999x/see-2.webp)](https://www.ai999x.com/models/time-series-ai999x/)

*Exponential smoothing forecast update*

[![Mean absolute error: the live scene in the Time Series desktop app](https://www.ai999x.com/?ax_shot=time-series-ai999x/see-3.webp)](https://www.ai999x.com/models/time-series-ai999x/)

*Mean absolute error*

Every calculation, with its formula, a worked example and its figures: **[Time Series on ai999x.com](https://www.ai999x.com/models/time-series-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/time-series-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/time-series-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `time-series` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/time-series`) and its data (`rust/models/time_series/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p time-series-ai999x
./target/release/time-series --help
cargo test -p time-series-ai999x
```
