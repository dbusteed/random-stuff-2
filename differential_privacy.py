#
#   an example of differntial privacy.
#
#   suppose you wanted to calculate the average
#   income of N of your friends while still 
#   respecting their privacy. 
#
#   this script simulates using a simple solution
#   to this issue using differntial privacy
#

import numpy as np
np.random.seed(2112)

N_FRIENDS = 1000
ACTUAL_INCOME_MEAN = 70000
ACTUAL_INCOME_SD = 30000
OBSCURE_MEAN = 30000
OBSCURE_SD = 30000

true_incomes = []
for _ in range(N_FRIENDS):
    income = int(np.random.normal(ACTUAL_INCOME_MEAN, ACTUAL_INCOME_SD))
    true_incomes.append(income)

# this is the value we'd like to calculate, but are unable
# to due to privacy concerns
true_mean = np.mean(true_incomes)


estimated_incomes = []
for inc in true_incomes:
    _inc = inc + int(np.random.normal(OBSCURE_MEAN, OBSCURE_SD))
    estimated_incomes.append(_inc)

estimated_mean = np.mean(estimated_incomes) - OBSCURE_MEAN

print(f'   actual mean: {true_mean}')
print(f'estimated mean: {estimated_mean}')
