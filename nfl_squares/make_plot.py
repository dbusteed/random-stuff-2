#
#   make heatmaps based for the different square scores
#

import matplotlib.pyplot as plt
import pandas as pd
import seaborn as sns

df = pd.read_csv("data.csv")

df['points1'] = df['score1'].apply(lambda s: int(str(s)[-1]))
df['points2'] = df['score2'].apply(lambda s: int(str(s)[-1]))

blank_squares = pd.DataFrame(columns=list(range(0, 10)), index=list(range(0, 10)))
blank_squares = blank_squares.fillna(0)

qtrs = [1,2,3,4]
squares = {}

for qtr in qtrs:
    squares[qtr] = blank_squares.copy()
    qtr_df = df[df['qtr'] == qtr]

    for row in qtr_df.itertuples():
        squares[qtr].loc[row.points1, row.points2] += 1

    total = len(qtr_df)
    for i in range(0, 10):
        for j in range(0, 10):
            squares[qtr].loc[i, j] /= total

fig, axs = plt.subplots(ncols=2, nrows=2)

fig.suptitle("Distribution of \"Squares\" Points (from all NFL games 1990-2020)", fontsize=16)
sns.heatmap(squares[1], ax=axs[0, 0], cbar=False, cmap="Blues", linewidths=.5, annot=True, fmt=".3f")
sns.heatmap(squares[2], ax=axs[0, 1], cbar=False, cmap="Blues", linewidths=.5, annot=True, fmt=".3f")
sns.heatmap(squares[3], ax=axs[1, 0], cbar=False, cmap="Blues", linewidths=.5, annot=True, fmt=".3f")
sns.heatmap(squares[4], ax=axs[1, 1], cbar=False, cmap="Blues", linewidths=.5, annot=True, fmt=".3f")

axs[0, 0].set(title="First Quarter", xlabel="Team 1", ylabel="Team 2")
axs[0, 1].set(title="Second Quarter", xlabel="Team 1", ylabel="Team 2")
axs[1, 0].set(title="Third Quarter", xlabel="Team 1", ylabel="Team 2")
axs[1, 1].set(title="Fourth Quarter", xlabel="Team 1", ylabel="Team 2")

plt.show()
