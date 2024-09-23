#
#   having fun with web automation. Ztype is an online
#   typing game, where you type fast to destroy spaceships
#   or something. this script (unintelligently) does the
#   typing for you!
#

from time import sleep
from selenium import webdriver
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.common.action_chains import ActionChains
from string import ascii_lowercase

#b = webdriver.Chrome()
b = webdriver.Firefox()
b.get('https://zty.pe/')

actions = ActionChains(b)

input('Click START GAME in the browser, then press any key to begin...:')

# actions.key_down(Keys.ENTER).perform()

while True:
    for letter in ascii_lowercase:
        actions.key_down(letter).perform()
