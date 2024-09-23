#
#   this script generates two vectors of data 
#   with a specified mean, stdev, and Pearson 
#   correlation between them
#

import numpy as np
import matplotlib.pyplot as plt
import scipy.stats as stats
import statistics as st


def transform_sd(xs, sd):
    '''
    transforms a vector with a stdev of 1 to
    a newly specified stdev.
    '''
    
    x_bar = st.mean(xs)
    z_scores = [(x-x_bar) for x in xs]
    new_xs = [((z*sd) + x_bar) for z in z_scores]

    return new_xs


target_r = -0.5
n = 1000

cov = [
    [1.0, target_r],
    [target_r, 1.0]
]

mu_x, sd_x = 70, 5
mu_y, sd_y = 150, 10

mu = [mu_x, mu_y]

x, y = np.random.multivariate_normal(mu, cov, n).T

x = transform_sd(x, sd_x)
y = transform_sd(y, sd_y)

print('** GOALS **')
print(f'X: N~({mu_x}, {sd_x})')
print(f'Y: N~({mu_y}, {sd_y})')
print(f'corr(X,Y): {target_r}')
print()
print('** ACTUALS **')
print(f'X: N~({st.mean(x)}, {st.stdev(x)})')
print(f'Y: N~({st.mean(y)}, {st.stdev(y)})')
print(f'corr(X,Y): {stats.pearsonr(x, y)[0]}')