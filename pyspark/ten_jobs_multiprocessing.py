import math
from multiprocessing import Queue, Process
from time import time, sleep

tic = time()

nums = list(range(1, 101))

def do_work(queue, numbers):
    summ = 0
    for n in numbers:
        sleep(1)
        summ += (n**2)
    queue.put(summ)

queue = Queue()
procs = []
nproc = 4

chunk = int(math.ceil(len(nums) / float(nproc)))

for i in range(nproc):
    p = Process(
        target=do_work,
        args=(
            queue,
            nums[chunk*i:chunk*(i+1)]
        )
    )
    procs.append(p)
    p.start()


for p in procs:
    p.join()

sum_of_results = 0
for i in range(nproc):
    sum_of_results += queue.get()

toc = time()

print(sum_of_results)
print(toc-tic, 'seconds')