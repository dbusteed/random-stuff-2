#
#   scrape NFL scores data (by quarter)
#

import pandas as pd
import requests
from bs4 import BeautifulSoup as bs
from concurrent.futures import ThreadPoolExecutor
from tqdm import tqdm

first_year = 1990
last_year = 2022

base_url = "https://www.footballdb.com"

headers = {
    'User-Agent': 'Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:89.0) Gecko/20100101 Firefox/89.0'
}

output = {
    "qtr": [],
    "score1": [],
    "score2": [],
}


def cummulate_scores(scores):
    t = [scores[0]]
    for x in scores[1:]:
        t.append(t[-1]+x)
    return t


def grab_score_data(link):
    raw_html = requests.get(link, headers=headers).text
    html = bs(raw_html, 'html.parser')

    scores_tbl = html.find(class_="statistics")
    scores_rows = scores_tbl.find_all("tr", class_="row0")

    scores = [[int(t.text) for t in r.find_all("td")[1:-1]] for r in scores_rows]
    scores = list(map(cummulate_scores, scores))

    output = {
        'qtr': list(range(1, len(scores[0])+1)),
        'score1': scores[0],
        'score2': scores[1],
    }

    return output


game_links = []

pbar = tqdm(range(first_year, last_year+1))
for year in pbar:
    pbar.set_description(f'Finding games for {year}')

    url = f"{base_url}/games/index.html?lg=NFL&yr={year}"

    raw_html = requests.get(url, headers=headers).text
    html = bs(raw_html, 'html.parser')

    page_links = html.find_all("a")
    game_links.extend([base_url + a["href"] for a in page_links if a["href"].startswith("/games/boxscore")])

with ThreadPoolExecutor(max_workers=20) as ex:
    data = list(tqdm(ex.map(grab_score_data, game_links), total=len(game_links), desc='Grabbing game data'))

df = pd.DataFrame(columns=['qtr', 'score1', 'score2'])
for d in tqdm(data, desc='Building final dataset'):
    df = df.append(pd.DataFrame(d))

df.to_csv("data.csv", index=False)
