#
#   illustrating the Monty Hall problem
#

from random import randint
import matplotlib.pyplot as plt

N_ROUNDS = 5000

stay_wins = 0
switch_wins = 0
stay_record = []
switch_record = []

for _n in range(N_ROUNDS):
    prize = randint(1, 3)
    choice = randint(1, 3)

    # simulate Monty opening the door by selecting the door that
    # is not the prize, and not the one selected by the player
    open_door = [x for x in [1,2,3] if x not in (prize, choice)][0]

    # which means, the alternate door (the door the player can switch
    # too) is the door that is not their own, and not the one that has been opened
    alt_door = [x for x in [1,2,3] if x not in (choice, open_door)][0]

    # evaluate success using the "stay strategy"
    if choice == prize:
        stay_wins += 1
    stay_record.append(stay_wins)

    # evaluate success using the "switch strategy"
    if alt_door == prize:
        switch_wins += 1
    switch_record.append(switch_wins)


stay_pct = round(stay_wins / N_ROUNDS, 2)
switch_pct = round(switch_wins / N_ROUNDS, 2)

plt.scatter(range(N_ROUNDS), stay_record, c='red', label=f'STAY')
plt.scatter(range(N_ROUNDS), switch_record, c='blue', label='SWITCH')
plt.xlabel('number of games')
plt.ylabel('number of wins')
plt.title(f'STAY: {stay_pct}%; SWITCH: {switch_pct}%')
plt.legend()
plt.show()

