#!/usr/bin/python3.8

#
#   Little POC CLI tool for breathing exercies.
#   Improvements would be cmd line args for different
#   intervals, etc

from time import sleep
from os import system
from tqdm import tqdm

clear = lambda: system("cls")

ROUTINES = {
    "basic": [("inhale", 6), ("exhale", 6)],
    "box_4": [("inhale", 4), ("  hold", 4), ("exhale", 4), ("  hold", 4)]
}

class Breather:
    def __init__(self, cycles, stages):
        self.cycles = cycles
        self.stages = stages
        self.bars = [self._make_pbar(s[0], s[1]) for s in stages]
        clear()

    def start(self):
        print('starting in 3 seconds...')
        sleep(3)

        for c in range(self.cycles):
            self._print_header(c+1)
            
            for b in self.bars:
                b.reset()

            for b in self.bars:
                for _ in range(b.total):
                    sleep(.1)
                    b.update(1)
            
            sleep(.1)            
            clear()

        for b in self.bars:
            b.close()
        clear()

    def _make_pbar(self, desc, seconds):
        pbar = tqdm(total=(seconds*10), desc=f'{desc} ({seconds})', bar_format='{desc} |{bar:20}| ')
        return pbar

    def _print_header(self, count):
        print(f"\tRound: {count} / {self.cycles}\n")


try:
    b = Breather(25, ROUTINES["basic"])
    b.start()

except KeyboardInterrupt:
    pass
