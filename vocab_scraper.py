#
#   web scraper used for grabbing Japanese vocab
#   words from a handy study site. the weird formatting
#   (<SEP>) allowed me to upload this data to Quizlet
#   and make flash cards
#

from bs4 import BeautifulSoup as bs
from selenium import webdriver

b = webdriver.Chrome("/home/davis/opt/chromedriver")

b.get('https://www.jlptstudy.net/N2/?vocab-list')

html = b.page_source

ppage = bs(html, 'html.parser')

main_div = ppage.find(class_='main-window')

tr = main_div.find_all('tr')

rows = []
for t in tr:
    a = []
    for x in t.find_all('td'):
        a.append(x.text.strip())
    rows.append(a)
    
def fformat(arr):
    if arr[2]:
        return f"{arr[2]} ({arr[1]})<SEP>{arr[4]}"
    else:
        return f"{arr[1]}<SEP>{arr[4]}"

with open('upload.txt', 'w') as f:
    for r in rows:
        f.write(fformat(r))
        f.write('\n')