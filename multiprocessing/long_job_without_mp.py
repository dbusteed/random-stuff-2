#
#   messing around with `multiprocessing`
#

import time

def run_job(job):
    print(f'running job: {job}')
    time.sleep(1)

def run_jobs(jobs):
    for job in jobs:
        run_job(job)

jobs = [f'job {i}' for i in range(10)]

start_time = time.time()
run_jobs(jobs)
duration = time.time() - start_time
print(f"Duration {duration} seconds")
