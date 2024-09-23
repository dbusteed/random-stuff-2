#
#  
#

import math
import matplotlib
import matplotlib.pyplot as plt
import pandas as pd
import statistics as st
from random import randint, shuffle

n_lists = 4

sort = []

# "sorted" approach to balancing the lists
nums = [randint(1,1000) for _ in range(1000)]
lists = [[] for _ in range(n_lists)]
nums = sorted(nums)

for i in range(0, len(nums), n_lists):
    for j in range(n_lists):
        try:
            lists[j].append(nums[i+j])
        except IndexError:
            pass

_df = {s:lists[i] for i,s in enumerate('abcd')}
df = pd.DataFrame(_df)
df["index"] = df.index
df["total"] = df["a"] + df["b"] + df["c"] + df["d"]

for l in lists:
    shuffle(l)

_df2 = {s:lists[i] for i,s in enumerate('abcd')}
df2 = pd.DataFrame(_df2)
df2["index"] = df2.index
df2["total"] = df2["a"] + df2["b"] + df2["c"] + df2["d"]


plt.subplot(1, 2, 1)
plt.bar(df["index"], df["total"])

plt.subplot(1, 2, 2)
plt.bar(df2["index"], df2["total"])

plt.show()
