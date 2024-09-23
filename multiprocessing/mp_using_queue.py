#
#   messing around with `multiprocessing.Queue`
#

import multiprocessing
import logging as log
import time
from functools import partial
from random import randint
import math

log.basicConfig(level=log.INFO,
                format='%(asctime)s  %(levelname)-6s %(message)s',
                handlers=[
                    log.StreamHandler()
                ],
                datefmt='%H:%M:%S')

def run_job(jobs, queue):
    nums = []
    for j in jobs:
        log.info(f'running job: {j}')
        time.sleep(1)
        nums.append(randint(1, 100))
    queue.put((1, nums))

def run_jobs(jobs):

    queue = multiprocessing.Queue()
    chunk = int(math.ceil(len(jobs) / float(5)))
    procs = []

    for i in range(5):
        p = multiprocessing.Process(
            target=run_job,
            args=(jobs[chunk*i:chunk*(i+1)], queue)
        )
        procs.append(p)
        p.start()
    
    nums = []
    for i in range(5):
        i, n = queue.get()
        nums.extend(n)

    for p in procs:
        p.join()

    return sum(nums)

jobs = [f'job {i}' for i in range(20)]

start_time = time.time()
total = run_jobs(jobs)
duration = time.time() - start_time
log.info(f"Duration {round(duration, 2)} seconds, total: {total}")
