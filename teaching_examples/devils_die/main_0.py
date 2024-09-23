from random import randint
from time import sleep

TARGET = 50

def roll_die():
    return randint(1, 7)

player_score = 0
devil_score = 0
temp_score = 0
player_turn = True

while player_score < TARGET and devil_score < TARGET:
    
    if player_turn:
        print('\nPlayer Score:', player_score, 'Temp Score:', temp_score)
        print('[r] roll die  [p] pass die')
        choice = input('?: ').lower()
        
        if choice == 'r':
            die = roll_die()
            print('You rolled a', die)
            if die == 1:
                print('You lost all of your temp points and lost your turn!')
                temp_score = 0
                player_turn = False
                sleep(1)
            else:
                print('You gained', die, 'points!')
                temp_score = temp_score + die
        
        elif choice == 'p':
            player_score = player_score + temp_score
            temp_score = 0
            player_turn = False

    # devil's turn
    else:
        print('\nDevil Score:', devil_score, 'Temp Score:', temp_score)
        
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
