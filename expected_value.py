#
#   visualization of an example of Expected Values
#   found in the book "Naked Statistics"
#   
#   if given the opportunity to play a game in which
#   you roll a 6-sided die, and get paid $1 if you 
#   roll a one, $2 if you roll a two, etc, and each 
#   attempt at the game costs $3, is it worth it?
#   YES! because the expected value of the game is 
#   (.17 * 1) + (.17 * 2) + ... + (.17 * 6) ~= 3.5.
#   Therefore, overtime, you will make money by playing the
#   game. This program simulates this experiment and
#   makes a nifty little chart
#

from random import randint
import matplotlib.pyplot as plt

cost = 3
ev = 3.5
ex_gain = ev - cost
money = 0
games = 1000
money_history = []

for i in range(games):
    money -= cost
    roll = randint(1,6)
    money += roll
    money_history.append(money)

plt.plot(money_history)
plt.plot( [i*ex_gain for i in range(games)] )
plt.show()
