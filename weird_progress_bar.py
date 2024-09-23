#
#   poorly made custom progress bar, but kinda
#   useful for displaying the progress of different
#   steps within an iterative process
#

from sys import stdout
from time import sleep

for i in range(5):
    print(f'({i+1}/5) [ ', end='')
    stdout.flush()
    sleep(.5)

    print('step 1, ', end='')
    stdout.flush()
    sleep(.5)

    print('step 2, ', end='')
    stdout.flush()
    sleep(.5)

    print('step 3 ', end='')
    stdout.flush()
    sleep(.5)

    print(']')
