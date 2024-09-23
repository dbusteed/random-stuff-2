#
#   finding the expected value of a slot machine
#   (note: i'm not too sure how real slot machines
#    work, I made this one up)
#

import matplotlib.pyplot as plt
import numpy as np
from collections import Counter
from random import choice

t = np.array([1,2,3,4,5])
multiplier = np.interp(t, (t.min(), t.max()), (1, 1.5))
tokens = {k:v for k,v in zip(t, multiplier)}

N = 100_000

TPL_WIN = 10
DBL_WIN = 5
COST = 4

values = []

for _ in range(N):
    res = choice(t), choice(t), choice(t)
    counts = Counter(res)
    try:
        counts = [(k,v) for k,v in counts.items() if v > 1][0]
        if counts[1] == 2:
            values.append(int(DBL_WIN * tokens[counts[0]]) - COST)
        elif counts[1] == 1:
            values.append(int(TPL_WIN * tokens[counts[0]]) - COST)
        else:
            values.append(-COST)
    except:
        pass

print(np.mean(values))
