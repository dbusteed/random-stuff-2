#
#   this visualizes the 'randomness' of `random.choice`
#

from random import choice
import matplotlib.pyplot as plt

groups = ["this", "that", "other", "extra"]

dist = {str(x):0 for x in groups}

for _ in range(10000):
    dist[choice(groups)] += 1

x = list(dist.keys())
y = list(dist.values())

plt.bar(x, y)
plt.show()