from pyspark import SparkContext
from time import time, sleep

tic = time()

sc = SparkContext("local[*]", "TenJobsExample")

nums = sc.parallelize(list(range(1, 101)))

def do_work(x):
    sleep(1)
    return x**2

results = nums.map(do_work)
sum_of_results = results.sum()

toc = time()

print(sum_of_results)
print(toc-tic, 'seconds')
