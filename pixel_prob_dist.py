import matplotlib.pyplot as plt
import numpy as np
from PIL import Image
from random import random

img = Image.open('data/bmo_gray.png')
pix = np.array(img)

dist = np.array([x[0] for y in pix for x in y]).reshape(16, 16)
dist = (255 - dist) / 255
dist = dist / sum(dist.reshape(-1))

def run_experiment(n):
    x = [0 for _ in range(256)]
    for _ in range(n):
        r = random()
        for i, p in enumerate(dist.reshape(-1)):
            r -= p
            if r <= 0:
                x[i] += 1
                break
    x = np.array(x).reshape(16, 16)
    return x


ns = [
    500,
    1_000,
    5_000,
    10_000,
    100_000,
]

fig, ax = plt.subplots(1, len(ns))

for i, n in enumerate(ns):
    print(f'running experiment (n={n})')
    x = run_experiment(n)
    ax[i].set_title(f'N = {n}')
    ax[i].tick_params(axis='both', labelsize=0, length=0)
    ax[i].imshow(x, cmap='gray_r')
    ax[i].grid(False)

plt.show()
