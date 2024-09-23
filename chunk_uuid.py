#
# POC for splitting data into somewhat
# even groups using a GUID
#

import matplotlib.pyplot as plt
from uuid import uuid4


def hasher(hex_char):
    dec_num = int(hex_char, 16)
    group = dec_num // 2
    return group


n_ids = 100_000
n_groups = 8

ids = [str(uuid4()) for _ in range(n_ids)]
groups = [[] for _ in range(n_groups)]

for idd in ids:
    group = hasher(idd[0])
    groups[group].append(idd)

x = list(range(n_groups))
y = [len(g) for g in groups]
plt.bar(x, y)
plt.show()
