# Time Series: the desktop app, user guide

**Time Series**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Time Series covers moving averages, exponential smoothing, autocorrelation, ARIMA basics, stationarity tests, change points, forecasting accuracy and Kalman filtering. Its desktop app holds all 78 calculations in 14 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-time-series-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-time-series-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-time-series-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-time-series.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-time-series-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-time-series-1.6.5-linux-x86_64.tar.gz
./ai999x-time-series-1.6.5-linux-x86_64/ai999x-time-series
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Time Series app, opened on Simple moving average with its worked example.](https://www.ai999x.com/?ax_shot=time-series-ai999x/hero.webp)

*The Time Series app, opened on Simple moving average with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Moving averages › Simple moving average (or type part of its name in the search box). It opens with a worked example filled in: “Find the 3-period simple moving average of [10, 12, 14, 13, 15].” The answer: sma = [12, 13, 14], last = 14, count = 3.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Simple moving average: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-1.webp)

*Simple moving average: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Find the 3-period simple moving average of [10, 12, 14, 13, 15].” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Simple moving average
sma = [12.0,13.0,14.0]
last = 14
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Holt's linear trend method](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-2.webp)

*Holt's linear trend method*

![Ljung-Box test](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-3.webp)

*Ljung-Box test*

![Mean absolute percentage error](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-4.webp)

*Mean absolute percentage error*

![Kalman filter for a random-walk (local level) model](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-5.webp)

*Kalman filter for a random-walk (local level) model*

![AR(1) forecast towards the mean](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-6.webp)

*AR(1) forecast towards the mean*

![Mean, variance and autocorrelation of an AR(1) process](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-7.webp)

*Mean, variance and autocorrelation of an AR(1) process*

![Single change point in the mean (least squares)](https://www.ai999x.com/?ax_shot=time-series-ai999x/scene-8.webp)

*Single change point in the mean (least squares)*

![Simple moving average: a chart, enlarged.](https://www.ai999x.com/?ax_shot=time-series-ai999x/chart-1.webp)

*Simple moving average: a chart, enlarged.*

![Holt's linear trend method: a chart, enlarged.](https://www.ai999x.com/?ax_shot=time-series-ai999x/chart-2.webp)

*Holt's linear trend method: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Time Series app in the dark theme.](https://www.ai999x.com/?ax_shot=time-series-ai999x/hero-dark.webp)

*The Time Series app in the dark theme.*

## 7. Every calculation

78 calculations in 14 topics; each has its own entry on the model's page (https://www.ai999x.com/models/time-series-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| ARIMA models | AR(1) forecast towards the mean, AR(p) estimation by conditional least squares, AR(p) estimation by Yule-Walker, Forecasts of an AR(p) model, Forecasts of an ARIMA(p,1,0) model, Forecasts of an ARMA, ARIMA or seasonal ARIMA model with MA terms, Partial autocorrelations from autocorrelations, Yule-Walker equations from autocorrelations |
| ARMA theory | h-step forecast error variance of an ARMA model, Invertibility of an MA(2) process, Mean, variance and autocorrelation of an AR(1) process, Pi weights (AR-infinity representation) of an ARMA process, Psi weights (MA-infinity representation) of an ARMA process, Stationarity of an AR(2) process, Theoretical autocorrelations of an AR(2) process, Theoretical autocorrelations of an ARMA(1,1) process, Theoretical autocorrelations of an ARMA(p,q) process, Theoretical autocorrelations of an MA(q) process, Theoretical partial autocorrelations of an ARMA(p,q) process, Theoretical partial autocorrelations of an MA(1) process, Variance and lag-1 autocovariance of an ARMA(p,q) process |
| Autocorrelation | Bartlett standard error of a sample autocorrelation, Box-Pierce test, Ljung-Box test, Sample autocorrelation at lag k, Sample autocovariance at lag k, Sample partial autocorrelation at lag k |
| Change points | CUSUM estimate of a change point, Multiple change points with a penalty (PELT), Single change point in the mean (least squares), Tabular CUSUM control chart, Two change points in the mean (optimal segmentation) |
| Classical decomposition | Seasonal indices (additive decomposition), Seasonal indices (multiplicative decomposition), Seasonally adjusted series (additive), Seasonally adjusted series (multiplicative) |
| Cross-correlation | Lag of the strongest cross-correlation, Sample cross-correlation at lag k |
| Exponential smoothing | Best smoothing constant among candidates, Damped-trend Holt method, Exponential smoothing forecast update, Holt's linear trend method, Holt-Winters additive method, Holt-Winters multiplicative method, One update of Holt's method, Simple exponential smoothing |
| Forecast accuracy | Mean absolute error, Mean absolute percentage error, Mean absolute scaled error, Mean forecast error (bias), Mean squared error, Root mean squared error, Symmetric mean absolute percentage error |
| Interpolation of missing values | Exponential (log-linear) interpolation, Linear interpolation of a missing value |
| Kalman filtering | Kalman filter for a constant-velocity model, Kalman filter for a random-walk (local level) model, Kalman measurement update (scalar), One Kalman filter step (random-walk model), Rauch-Tung-Striebel smoother (random-walk model), Steady-state Kalman gain (random-walk model), Steady-state Kalman gains of a constant-velocity model |
| Moving averages | Centred moving average, Exponential moving average, Linearly weighted moving average, Simple moving average |
| Stationarity tests | Augmented Dickey-Fuller statistic, Dickey-Fuller unit-root statistic, KPSS stationarity statistic |
| Transformations and growth | Compound average growth rate of a series, Regular and seasonal differencing |
| Trend fitting | Exponential trend fit and forecast, Linear trend fit and forecast, Linear trend with seasonal dummy variables, Mann-Kendall trend test, Quadratic trend fit and forecast, Random-walk-with-drift (drift method) forecast, Sen's slope (Theil-Sen trend) |

## 8. Other ways to use it

The same engine runs on the command line (`time-series`, its own guide), in Python (`pip install time-series-ai999x`, `import time_series_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/time-series-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/time-series-ai999x/
