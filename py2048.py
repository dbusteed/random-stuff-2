#!/usr/bin/python3

import curses
import numpy as np
from enum import Enum
from random import choice
from time import sleep

# TODO make the code more flexible to use
# this value, there's still some hardcoded parts
N = 4

# OTHER TODO:
# - display score
# - check for game over
# - add colors

#
# the first character will be the 'blank'
# tile, so ideally this will be an empty
# string or something low key like '.'
#
CHARS = ' ABCDEFGHI'


class Shift(Enum):
    UP = 1
    DOWN = 2
    LEFT = 3
    RIGHT = 4


def clearwin(win) -> None:
    y, x = win.getmaxyx()
    s = ' ' * (x - 2)   
    for i in range(1, y-1):
        win.addstr(i, 1, s)
    win.refresh()


def stop_curses():
    curses.curs_set(1)
    curses.nocbreak()
    stdscr.keypad(False)
    curses.echo()
    curses.endwin()


def print_board(board):
    for i in range(len(board)):
        arr_str = ' '.join(board[i])
        gamewin.addstr(i+1, 2, arr_str)


def shift_board(board, shift):
    if shift in (Shift.UP, Shift.DOWN):
        def get_row(i): return board[:, i]
        def set_row(i, row): board[:, i] = row
    else:
        def get_row(i): return board[i, :]
        def set_row(i, row): board[i, :] = row

    # whether we insert at the end or the front
    insert_at = (N-1) if shift in (Shift.UP, Shift.LEFT) else 0
    
    shifted = False
    for i in range(4):
        # get the row and clear zeros
        row = get_row(i)
        _row = row.copy()
        row = [r for r in row if r != CHARS[0]]

        # check for merges
        for j in range(0, len(row)-1):
            if row[j] == row[j+1]:
                
                # set the tile to the merged value, which will be
                # the next character defined in the `CHARS` string
                row[j] = CHARS[(CHARS.index(row[j]) + 1) % len(CHARS)]
                
                # set the other tile to blank, which should
                # be the first character in `CHARS`
                row[j+1] = CHARS[0]

        # clear blanks and insert them
        row = [r for r in row if r != CHARS[0]]
        while len(row) < 4:
            row.insert(insert_at, CHARS[0])
        
        if list(_row) != row:
            shifted = True
        set_row(i, row)
    
    # add a new tile if the board changed
    if shifted:
        x, y = choice(np.argwhere(board == CHARS[0]))
        board[x, y] = choice(CHARS[1:3])


#
# init curses
#
stdscr = curses.initscr()
curses.noecho()
curses.cbreak()
curses.curs_set(0)
stdscr.keypad(True)

#
# make the game window
#
gamewin = curses.newwin(N*3, N*3, 1, 1)
gamewin.refresh()

#
# make the board with some starting tiles
#
board = np.array([CHARS[0]] * N**2).reshape(N, -1)
for _ in range(N):
    x, y = choice(np.argwhere(board == CHARS[0]))
    board[x, y] = choice(CHARS[1:3])

#
# draw the game border
#
top_border = '╭' + ((len(board)*2+1) * '─') + '╮'
bot_border = '╰' + ((len(board)*2+1) * '─') + '╯'
gamewin.addstr(0, 0, top_border)
gamewin.addstr(5, 0, bot_border)
for i in range(1, 5):
    gamewin.addstr(i, 0, '│')
    gamewin.addstr(i, 10, '│')

#
# keymap for the shift function
#
keymap = {
    'w': Shift.UP,
    'a': Shift.LEFT,
    'd': Shift.RIGHT,
    's': Shift.DOWN
}

#
# main loop, yeehaw
#
while True:
    try:
        print_board(board)
        key = gamewin.getkey()

        if key == 'q':
            break
        
        if key in 'wasd':
            shift_board(board, keymap[key])

        gamewin.refresh()
        sleep(.1)
    
    except KeyboardInterrupt:
        stop_curses()
        import sys; sys.exit(0)

    except Exception as e:
        stop_curses()
        raise e

stop_curses()