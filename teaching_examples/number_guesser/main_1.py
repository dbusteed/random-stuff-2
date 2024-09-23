# import function for making random numbers
from random import randint

# make random number between 1-100
num = randint(1, 101)

# give `guess` a starting value, so
# that we can enter the `while` loop
guess = -1

# starting value for the count
count = 1

# continue to ask the user for guesses
# until they guess the correct number
while guess != num:

    # get input guess from user
    guess = input("guess my number: ")
    
    # `input` returns a string, so
    # we need to convert it to an number
    guess = int(guess)

    # notify the user if their
    # guess is wrong
    if guess > num:
        print('lower!')
        count = count + 1

    # also, increase their count
    # if their guess is wrong
    elif guess < num:
        print('higher!')
        count = count + 1

    else:
        print('you win!')
        print('(' + str(count) + ' guesses)')