import numpy as np
from scipy.stats import binom, poisson, geom, norm, expon, uniform

# BINOMIALVERTEILUNG

# Parameter (ändern!)
n = 50        # Anzahl Versuche
p = 0.3       # Erfolgswahrscheinlichkeit
k = 12        # betrachteter Wert

# Wahrscheinlichkeiten
P_eq = binom.pmf(k, n, p)      # P(X = k)
P_le = binom.cdf(k, n, p)      # P(X ≤ k)
P_gt = binom.sf(k, n, p)       # P(X > k)
P_ge = binom.sf(k-1, n, p)     # P(X ≥ k)

# Kennzahlen
EX = binom.mean(n, p)
Var = binom.var(n, p)

print("Binomial:")
print(P_eq, P_le, P_gt, P_ge, EX, Var)

# POISSON-VERTEILUNG

# Parameter (ändern!)
lam = 4.5     # Erwartungswert λ
k = 3

P_eq = poisson.pmf(k, lam)
P_le = poisson.cdf(k, lam)
P_gt = poisson.sf(k, lam)

EX = poisson.mean(lam)
Var = poisson.var(lam)

print("\nPoisson:")
print(P_eq, P_le, P_gt, EX, Var)

# GEOMETRISCHE VERTEILUNG

# Parameter (ändern!)
p = 0.2
k = 8

P_eq = geom.pmf(k, p)        # P(X = k)
P_le = geom.cdf(k, p)        # P(X ≤ k)
P_gt = geom.sf(k, p)         # P(X > k) => 1 - P(X ≤ k)

# Direktformel
P_gt_formula = (1 - p)**k

EX = geom.mean(p)
Var = geom.var(p)

print("\nGeometrisch:")
print(P_eq, P_le, P_gt, P_gt_formula, EX, Var)

# NORMALVERTEILUNG

# Parameter (ändern!)
mu = 10
sigma = 2
x = 12

# Wahrscheinlichkeiten
P_le = norm.cdf(x, mu, sigma)     # P(X ≤ x)
P_gt = norm.sf(x, mu, sigma)      # P(X > x)

# Intervallwahrscheinlichkeit
a, b = 8, 13
P_interval = norm.cdf(b, mu, sigma) - norm.cdf(a, mu, sigma)

print("\nNormalverteilung:")
print(P_le, P_gt, P_interval)

# EXPONENTIALVERTEILUNG

# Parameter (ändern!)
lam = 0.4
x = 5

P_le = expon.cdf(x, scale=1/lam)
P_gt = expon.sf(x, scale=1/lam)

EX = expon.mean(scale=1/lam)
Var = expon.var(scale=1/lam)

print("\nExponential:")
print(P_le, P_gt, EX, Var)

# SIMULATION

samples = binom.rvs(n=30, p=0.4, size=100_000)
print("\nSimulation:")
print(np.mean(samples), np.var(samples))