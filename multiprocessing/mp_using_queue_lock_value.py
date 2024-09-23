#
#   messing around with `multiprocessing.Queue`
#

from multiprocessing import Queue, Process, Lock, Value
import logging as log
import time
from functools import partial
from random import randint, random
import math

log.basicConfig(level=log.INFO,
                format='%(asctime)s  %(levelname)-6s %(message)s',
                handlers=[
                    log.StreamHandler()
                ],
                datefmt='%H:%M:%S')

total = 0

def run_job(jobs, queue, lock, v):
    nums = []
    for i, j in enumerate(jobs):
        if i == 1:
            with lock:
                v.value += 1
                print('total completed:', v.value)
                print(type(v.value))

        log.info(f'running job: {j}')
        time.sleep(random() + 1)
        nums.append(randint(1, 100))
    queue.put((1, nums))

def run_jobs(jobs):

    queue = Queue()
    lock = Lock()
    chunk = int(math.ceil(len(jobs) / float(5)))
    procs = []
    v = Value('i', 0)

    for i in range(5):
        p = Process(
            target=run_job,
            args=(jobs[chunk*i:chunk*(i+1)], queue, lock, v)
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
