#
#   select a class given a custom, discrte
#   probability distribution
#

from random import random
import matplotlib.pyplot as plt

prob = [.1, .3, .2, .4]

dist = {str(x):0 for x in prob}

for _ in range(10000):
    r = random()
    for p in prob:
        r -= p
        if r <= 0:
            dist[str(p)] += 1
            break

x = list(dist.keys())
y = list(dist.values())

plt.bar(x, y)
plt.show()