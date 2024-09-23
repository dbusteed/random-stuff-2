from os import system
from random import randint
from sys import exit
from time import sleep

TARGET = 50

def roll_die():
    return randint(1, 7)

def clear_screen():
    system('clear')

def print_scores():
    clear_screen()
    if player_turn:
        print(f'* Player: {player_score}  ({temp_score})')
        print(f'  Devil:  {devil_score}')
    else:
        print(f'  Player: {player_score}')
        print(f'* Devil:  {devil_score}  ({temp_score})')


player_score = 0    
devil_score = 0
temp_score = 0
player_turn = True

while player_score < TARGET and devil_score < TARGET:
    
    print_scores()

    if player_turn:
        print('\n[r] roll die\n[p] pass die\n[q] quit\n')
        choice = input('?: ').lower()
        print()
        
        if choice == 'r':
            die = roll_die()
            print('You rolled a', die)
            if die == 1:
                print('You lost all of your temp points and lost your turn!')
                temp_score = 0
                player_turn = False
            else:
                print('You gained', die, 'points!')
                temp_score = temp_score + die
            sleep(1)

        elif choice == 'p':
            player_score = player_score + temp_score
            temp_score = 0
            player_turn = False

        elif choice == 'q':
            exit(0)

    # devil's turn
    else:

        print()

        # simple strategy
        if temp_score < 20:
            die = roll_die()
            print('The Devil rolled a', die)
            if die == 1:
                print('The Devil lost its temp points and lost its turn!')
                temp_score = 0
                player_turn = True
            else:
                print('The Devil gained', die, 'points!')
                temp_score = temp_score + die
        
        else:
            print('The Devil passes the dice')
            devil_score = devil_score + temp_score
            temp_score = 0
            player_turn = True
            
        sleep(1)

if player_score >= TARGET:
    print('Player won!')
else:
    print('Devil won!')
