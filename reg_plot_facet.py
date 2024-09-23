import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import seaborn as sns
from scipy.stats import pearsonr

df = pd.read_csv('../data/mtcars_reg.csv')
target = 'mpg'

df2 = df.select_dtypes(include=np.number)
nplots = len(df2.columns) - 1

plot_size = 4

ncol = 3
nrow = int(np.ceil(nplots / ncol))

fig, axes = plt.subplots(nrow, ncol, figsize=(ncol*plot_size, nrow*plot_size))
fig.tight_layout(pad=3.0)

[ax.set(xticklabels=[]) for ax in axes.ravel()]
[ax.set(yticklabels=[]) for ax in axes.ravel()]

for i, col in enumerate(df2.columns[1:]):
    x, y = i // ncol, i % ncol
    sns.regplot(y=target, 
                x=col,
                data=df,
                line_kws={'color': 'red'},
                ax=axes[x, y])
    # axes[x, y].set_title('asdf')

    r = pearsonr(df2[target], df2[col])
    r = round(r[0], 2)
    axes[x, y].text(0.05,
                    0.9,
                    f"r = {r}",
                    fontsize=12,
                    bbox=dict(visible=False, alpha=0.25),
                    transform=axes[x, y].transAxes)

plt.show()
