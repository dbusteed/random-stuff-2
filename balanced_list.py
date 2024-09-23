#
#  looking into different ways for "balancing" a list
#

import math
import statistics as st
from random import randint

n_lists = 3

def score_balance(lists):
    sums = [sum(l) for l in lists]
    return st.stdev(sums)


naive = []
sort = []


# NAIVE APPROACH -- we hope that because
# the list is random, we'll get a balanced list
# if we split into N groups by taking the the first
# N items, then the next N items, etc

for _ in range(100):
    nums = [randint(1, 1000) for _ in range(100)]
    lists = []
    chunk = int(math.ceil(len(nums)) / float(n_lists))

    for i in range(n_lists):
        sublist = nums[chunk*i:chunk*(i+1)]
        lists.append(sublist)

    naive.append(score_balance(lists))


# SORTED APPROACH

for _ in range(100):
    nums = [randint(1, 1000) for _ in range(100)]
    lists = [[] for _ in range(n_lists)]
    nums = sorted(nums)

    for i in range(0, len(nums), n_lists):
        for j in range(n_lists):
            try:
                lists[j].append(nums[i+j])
            except IndexError:
                pass

    sort.append(score_balance(lists))


print(st.mean(naive))
print(st.mean(sort))