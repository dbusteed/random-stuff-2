#!/usr/bin/python3

import os
os.environ['PYGAME_HIDE_SUPPORT_PROMPT'] = "hide"

import pygame
from random import randint

pygame.init()

WIDTH = 320
HEIGHT = 320
CELL_SIZE = 32

FALL_RATE = 300
NEW_CONE_RATE = 400
NEW_CONE_AMT = 3

cell_width = int(WIDTH / CELL_SIZE)
cell_height = int(HEIGHT / CELL_SIZE)

win = pygame.display.set_mode((WIDTH, HEIGHT))

cone = pygame.image.load('sprites/cone.png')
box = pygame.image.load('sprites/box.png')
font = pygame.font.SysFont("impact", 32)

app_running = True
clock = pygame.time.Clock()

position = 0
cones = [[randint(0, cell_width), 0]]

last_fall = 0
last_new_cone = 0
score = 0

while app_running:
    clock.tick(60)
    win.fill((0, 0, 0))

    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            app_running = False

        if event.type == pygame.KEYDOWN:
            if event.key == pygame.K_LEFT:
                position = (position - 1) % cell_width
            if event.key == pygame.K_RIGHT:
                position = (position + 1) % cell_width
    
    if pygame.time.get_ticks() - last_fall > FALL_RATE:
        if last_fall:
            for i, c in enumerate(cones[:]):
                c[1] += 1
                if c[1] == cell_height - 1 and c[0] == position:
                    app_running = False
                    print(f'Final Score: {score}')
                elif c[1] > cell_height:
                    del cones[i]
                    score += 1
        last_fall = pygame.time.get_ticks()
        # FALL_RATE = max(FALL_RATE - 1, 100)

    if pygame.time.get_ticks() - last_new_cone > NEW_CONE_RATE:
        for _ in range(NEW_CONE_AMT):
            cones.append([randint(0, 10), 0])
        last_new_cone = pygame.time.get_ticks()
        NEW_CONE_RATE = max(NEW_CONE_RATE - 1, 100)

    for cx, cy in cones:
        win.blit(cone, (cx * CELL_SIZE, cy * CELL_SIZE))

    win.blit(box, (position * CELL_SIZE, HEIGHT - 32))
    
    score_ui = font.render(str(score), 1, (255, 255, 255))
    win.blit(score_ui, (0, 0))

    pygame.display.update()

pygame.quit()
