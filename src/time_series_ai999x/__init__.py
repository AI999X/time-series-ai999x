"""time-series-ai999x — offline time series answers with their formula, steps and verification.

    import time_series_ai999x as m
    m.ask("Find the 3-period simple moving average of [10, 12, 14, 13, 15].").show()
    m.ask("Apply Holt's linear trend method with alpha = 0.5 and beta = 0.3 to [10, 12, 15, 16, 19, 21].").show()
    m.solve("ljung_box_test", x="4, 6, 5, 8, 7, 9, 8, 10, 9, 12, 11, 13", h=3)
    m.capabilities()                   # every certified capability

77 certified capabilities: moving averages (simple, linearly weighted, exponential, centred 2 x m); exponential
smoothing (simple, Holt, damped Holt, additive and multiplicative Holt-Winters with stated initialization, single
updates, best smoothing constant); classical decomposition (seasonal indices, seasonal adjustment); differencing and
compound growth; sample autocorrelation, autocovariance, partial autocorrelation, Bartlett standard errors,
Ljung-Box and Box-Pierce tests; AR(p) estimation (Yule-Walker, least squares) and forecasting, ARIMA(p,1,0)
forecasts; ARMA theory (autocorrelations, partial autocorrelations, variance, psi and pi weights, forecast error
variance, AR(2) stationarity, MA(2) invertibility); cross-correlation; trend fitting (linear, quadratic,
exponential, seasonal dummies, drift, Sen's slope, Mann-Kendall); forecast accuracy (MAE, MSE, RMSE, MAPE, sMAPE,
MASE, bias); interpolation; change points (least squares, two change points, PELT, CUSUM, tabular CUSUM); Dickey-
Fuller, augmented Dickey-Fuller and KPSS statistics; Kalman filtering (update, step, local-level filter, RTS
smoother, constant-velocity filter, steady-state gains). Series are written oldest first in [brackets];
autocovariances divide by n; conventions are stated with every capability. Built by the AI999X model machine from
the same engine as every domain; a question that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.1"
_P = Product("time_series", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
