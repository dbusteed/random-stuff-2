# run this with `ipython` rather than `python`

import matplotlib.pyplot as plt
import pandas as pd
from IPython import get_ipython
from random import randint
from tqdm import tqdm

ipy = get_ipython()

rows = [10**i for i in range(1, 8)]
mean_times = []
median_times = []

for nrow in tqdm(rows):
    df = pd.DataFrame({'x1': [randint(0, 1000) for _ in range(nrow)]})
    
    res = ipy.run_line_magic('timeit', '-oq df.mean()')
    mean_times.append(res.average)

    res = ipy.run_line_magic('timeit', '-oq df.median()')
    median_times.append(res.average)

df = pd.DataFrame({
    'Rows': rows,
    'Mean': mean_times,
    'Median': median_times
})

df.plot(x='Rows', kind='bar', stacked=False)
plt.xticks(rotation=90)
plt.show()
