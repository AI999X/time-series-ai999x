# time-series-ai999x — offline, verified time series solver with step-by-step answers

[![tests](https://github.com/AI999X/time-series-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/time-series-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/time-series-ai999x)](https://pypi.org/project/time-series-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/time-series-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/time-series-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/time-series-ai999x/)** · **[Guides](https://huggingface.co/AI999X/time-series-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/time-series-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.1.0

Field: **Mathematics & Statistics** · Tasks: Question Answering, Token Classification, Text Classification, Time Series Forecasting, Sentence Similarity, Feature Extraction

time-series-ai999x is an offline, deterministic Python solver that answers time series questions with verified results from 77 certified capabilities, gives 403 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**time-series-ai999x** is a compact, deterministic time series engine for Python. It reads a question in plain English (or a typed call), picks a certified time series capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** time-series-ai999x comes as the AI999X desktop app and as the command-line program `time-series` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Time Series desktop app** 1.4.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-time-series-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-time-series-1.4.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.4.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz) |
| **Command line** (`time-series`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/time-series-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install time-series-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Time Series** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Time Series among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-time-series-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-time-series-1.4.0-linux-x86_64.tar.gz
./ai999x-time-series-1.4.0-linux-x86_64/ai999x-time-series
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
time-series --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
time-series --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words** — Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ time-series ask "Find the 3-period simple moving average of [10, 12, 14, 13, 15]."
Read as: Simple moving average
sma = [12.0,13.0,14.0]
last = 14
count = 3
  by: Simple moving average  (mean of each window of k consecutive values)
  assumes: 1 <= k <= n
```

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ time-series info simple_moving_average
Simple moving average
  mean of each window of k consecutive values
  in   x        the series, oldest first (vector) [1]
  in   k        the window length (number of periods) (integer) [1]
  out  sma      the moving averages, one per complete window (the first covers x_1..x_k) [1]
  out  last     the latest moving average (forecast of the next period) [1]
  out  count    number of moving averages (n - k + 1) [1]
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ time-series solve simple_moving_average "x=[10, 12, 14, 13, 15]" k=3
sma = [12.0,13.0,14.0]
last = 14
count = 3
  by: Simple moving average  (mean of each window of k consecutive values)
  assumes: 1 <= k <= n
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | time-series batch

`cases.jsonl`:

```json
{"key": "simple_moving_average", "inputs": {"x": "[10, 12, 14, 13, 15]", "k": "3"}}
{"key": "holt_linear_trend", "inputs": {"x": "[10, 12, 15, 16, 19, 21]", "alpha": 0.5, "beta": 0.3}}
```

```console
$ time-series batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":14.0,"unit":"","solved_for":"last","values":[["sma",[12.0,13.0,14.0],""],["last",14.0,""],["count",3,""]],"target":"last","phrase":"the moving averages, one per complete window (the first covers x_1..x_k)","assumptions":["1 <= k <= n"],"name":"Simple moving average","equation":"mean of each window of k consecutive values"}
{"outcome":"SOLVED","message":"","value":22.653285906249998,"unit":"","solved_for":"forecast","si":[["alpha",0.5],["beta",0.3]],"values":[["forecast",22.653285906249998,""],["level",20.6553396875,""],["trend",1.9979462187499992,""],["sse",7.492761855456635,""]],"target":"forecast","phrase":"the h-step-ahead forecast","assumptions":["0 < alpha, beta <= 1","l_0 = x_1, b_0 = x_2 - x_1 unless stated","trend form: beta multiplies (l_t - l_{t-1}) (the error-correction beta* of many texts)"],"name":"Holt's linear trend method","equation":"l_t = alpha x_t + (1-alpha)(l_{t-1}+b_{t-1}); b_t = beta (l_t - l_{t-1}) + (1-beta) b_{t-1}"}
```

### In your browser

Open [Time Series on ai999x.com](https://www.ai999x.com/models/time-series-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `time_series_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install time-series-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### AR(1) forecast towards the mean

The h-step forecast mu + phi^h (x_n - mu) of a stationary AR(1) with mean mu, and (when the innovation variance is given) its forecast error variance and 95 % prediction interval.

Command line:

```console
$ time-series solve ar1_forecast x=58 phi=0.7 mu=50 h=3
forecast = 52.744
  by: AR(1) forecast towards the mean  (closed forms of the AR(1) recursion)
  x = 58
  phi = 0.7
  mu = 50
  assumes: |phi| < 1; known parameters; normal innovations for the interval
```

Python:

```python
import time_series_ai999x as m

a = m.solve("ar1_forecast", x="58", phi="0.7", mu="50", h="3")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/time-series-ai999x/#cap-ar1_forecast) and press **Compute in your browser**.

### AR(p) estimation by conditional least squares

Fit x_t = c + phi_1 x_{t-1} + ... + phi_p x_{t-p} + e_t by ordinary least squares on t = p+1..n; the innovation variance is the residual sum of squares divided by n - p.

Command line:

```console
$ time-series solve ar_least_squares "x=[4, 6, 5, 8, 7, 9, 8, 10, 9, 12]" p=1
const = 3.5625
phi = [0.6354166666666665]
phi1 = 0.6354166667
sigma2 = 2.515046296
  by: AR(p) estimation by conditional least squares  (least squares (Householder QR) of x_t on (1, x_{t-1}, ..., x_{t-p}), t = p+1..n)
  assumes: n - p > p + 2; regressors not collinear; the first p values are conditioned on
```

Python:

```python
import time_series_ai999x as m

a = m.solve("ar_least_squares", x=[4, 6, 5, 8, 7, 9, 8, 10, 9, 12], p="1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/time-series-ai999x/#cap-ar_least_squares) and press **Compute in your browser**.

_What this means:_ This procedure fits an autoregressive model of order p by minimizing the sum of squared errors for observations after the initial p values. The resulting constant term is 3.5625, while the lag coefficients are given in [0.635417]. The innovation variance, calculated as the residual sum of squares divided by the degrees of freedom, is 2.51505. This approach conditions on the first p data points, treating them as fixed starting values rather than estimating their distribution.

### AR(p) estimation by Yule-Walker

Estimate the coefficients phi_1..phi_p and the innovation variance of an AR(p) model from data by the Yule-Walker equations (sample autocovariances divided by n, mean removed).

Command line:

```console
$ time-series solve ar_yule_walker "x=[4, 6, 5, 8, 7, 9, 8, 10, 9, 12]" p=2
phi = [0.21521751734534134,0.40165819531144314]
phi1 = 0.2152175173
sigma2 = 3.767657229
  by: AR(p) estimation by Yule-Walker  (solve the Toeplitz system R phi = r (Durbin-Levinson); sigma^2 = c_0 prod(1 - phi_kk^2))
  assumes: the series is not constant; 1 <= p < n - 1; denominator n
```

Python:

```python
import time_series_ai999x as m

a = m.solve("ar_yule_walker", x=[4, 6, 5, 8, 7, 9, 8, 10, 9, 12], p="2")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/time-series-ai999x/#cap-ar_yule_walker) and press **Compute in your browser**.

_What this means:_ This calculation estimates the autoregressive parameters for order 2 using the Yule-Walker equations. It solves a system of linear equations based on sample autocovariances to determine the coefficients [0.215218, 0.401658], specifically identifying phi one as 0.215218. The resulting innovation variance, which measures the unpredictability of the series given past values, is computed as 3.76766. These values describe how much current observations depend on previous ones in the stationary process.

### Forecasts of an AR(p) model

h-step forecasts of x_t = c + phi_1 x_{t-1} + ... + phi_p x_{t-p} + e_t from the last p observations (future shocks set to 0).

Command line:

```console
$ time-series solve ar_forecast "phi=[0.5, 0.2]" "x=[10, 11, 12]" c=1 h=3
forecasts = [9.2,8.0,6.84]
forecast = 6.84
  by: Forecasts of an AR(p) model  (recursion xhat_{n+j} = c + sum_i phi_i xhat_{n+j-i} (observed values where available))
  c = 1
  assumes: phi_1 multiplies the latest observation
```

Python:

```python
import time_series_ai999x as m

a = m.solve("ar_forecast", phi=[0.5, 0.2], x=[10, 11, 12], c="1", h="3")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/time-series-ai999x/#cap-ar_forecast) and press **Compute in your browser**.

_What this means:_ This calculation produces forecasts for an Autoregressive model of order p using the recursion formula. It combines the constant term with weighted past values, where future shocks are set to zero. The result shows the predicted value at horizon 3, which is 6.84, based on the series ending in [10, 11, 12] and coefficients [0.5, 0.2].

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/time-series-ai999x/blob/main/docs/time_series_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/time-series-ai999x/blob/main/docs/time_series_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/time-series-ai999x/blob/main/docs/time_series_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/time-series-ai999x/blob/main/docs/time_series_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/time-series-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified time series capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 403 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `time-series` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** moving averages, exponential smoothing and Holt-Winters, autocorrelation, ARIMA basics, stationarity tests, change points, forecast accuracy and Kalman filtering.

## Model overview

| Property | Value |
|---|---|
| Package | `time-series-ai999x` 1.1.0 (import `time_series_ai999x`) |
| Type | Deterministic neuro-symbolic time series engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 77 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 403 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `time-series-ai999x "<question>"`; for the native program use `time-series` (above). The outputs above were produced by running this exact code on time-series-ai999x 1.1.0.

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

## Python worked examples, with their steps

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
@software{time_series_ai999x_1_1_0,
  title  = {time-series-ai999x: an offline, verified time series solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.1.0},
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
