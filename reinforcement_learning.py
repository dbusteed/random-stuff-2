import matplotlib.pyplot as plt
import numpy as np
from enum import Enum, auto
from pprint import pprint
from random import shuffle, choice
from tqdm import tqdm


class Gamestate(Enum):
    ONGOING = auto()
    WON = auto()
    LOST = auto()
    DRAW = auto()


class Action(Enum):
    STAND = 0
    HIT = 1


def get_deck():
    deck = ['2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K', 'A']
    deck *= 4
    shuffle(deck)
    return deck


def get_hand_value(hand):
    val = 0
    for card in [c for c in hand if c != 'A']:
        if card in ('J', 'Q', 'K'):
            val += 10
        else:
            val += int(card)
    for card in [c for c in hand if c == 'A']:
        if val <= 10:
            val += 11
        else:
            val += 1
    return val


def get_hand_key(hand):
    hand_key = str(get_hand_value(hand))
    if 'A' in hand:
        hand_key += '*'
    return hand_key


def get_max_val(obj):
    idx = np.argmax(list(obj.values()), axis=0)
    return list(obj.keys())[idx[0]]


def eval_blackjack(deck, state, hand=None, dealer=None, action=None):
    if hand and dealer and (action != None):
        if action == Action.STAND:
            while get_hand_value(dealer) < 18:
                dealer.append(deck.pop())
            h_val = get_hand_value(hand)
            d_val = get_hand_value(dealer)
            
            if h_val > 21:
                state = Gamestate.LOST
            elif d_val > 21:
                state = Gamestate.WON
            elif h_val > d_val:
                state = Gamestate.WON
            elif h_val < d_val:
                state = Gamestate.LOST
            else:
                state = Gamestate.DRAW
        
        elif action == Action.HIT:
            hand.append(deck.pop())
            if get_hand_value(hand) > 21:
                state = Gamestate.LOST
                hand.pop()
    else:
        hand = [deck.pop(), deck.pop()]
        dealer = [deck.pop(), deck.pop()]

    return (deck, state, hand, dealer, action)


def play_blackjack(states, eps):
    deck = get_deck()
    state = Gamestate.ONGOING
    hand = None
    dealer = None
    action = None

    while state == Gamestate.ONGOING:
        deck, state, hand, dealer, action = eval_blackjack(deck, state, hand, dealer, action)

        key = get_hand_key(hand)
        if state == Gamestate.ONGOING:
            if key not in states:
                states[key] = {Action.STAND: [0, 0], Action.HIT: [0, 0]}
                action = choice([Action.STAND, Action.HIT])
            else:
                if np.random.rand() < eps:
                    action = choice([Action.STAND, Action.HIT])
                else:
                    action = get_max_val(states[key])

            states[key][action][1] += 1
        
        elif state == Gamestate.DRAW:
            result = 0  # ignore DRAW cause noone wins

        else:
            result = 1 if state == Gamestate.WON else 0

            # update Q-value for the given state-action combination
            states[key][action][0] += ((1 / states[key][action][1]) * (result - states[key][action][0]))

            hand.pop()
            action = Action.HIT

            # update Q-values for the state-action combos that led 
            # to this point. for example, if the hand was [4, 4, 5, K],
            # we should remember that it was a good idea to HIT when 
            # starting with [4, 4]
            while len(hand) > 1:
                key = get_hand_key(hand)
                states[key][action][0] += ((1 / states[key][action][1]) * (result - states[key][action][0]))
                hand.pop()

    return states, result


epsilons = [0.01, 0.05, 0.50, .00001]
n_games = 10_000

results = {}
report_interval = n_games / 20

for eps in epsilons:
    wins = 0
    win_pct = []
    states = {}
    for n in tqdm(range(n_games)):
        states, result = play_blackjack(states, eps)
        wins += result
    
        if n % report_interval == 0 and n != 0:
            win_pct.append(wins / (n+1))

    results[eps] = win_pct


# states = [(k,v) for k,v in states.items()]
# states = sorted(states, key=lambda x: x[0], reverse=True)
# pprint(states)

xs = [str(n) for n in range(n_games) if n % report_interval == 0 and n != 0]

for e, arr in results.items():
    plt.plot(xs, arr, label=f'{str(e)} -- {round(arr[-1], 5)}')

plt.legend(loc='lower left', facecolor='white')
plt.show()
