import matplotlib.pyplot as plt
import numpy as np
from requests import get
from sys import exit

USERNAME = 'dbusteed'
BASE_URL = f'https://api.github.com'

# adjust `per_page` or add pagination
# if the user has > 200 repos
repo_url = f'{BASE_URL}/users/{USERNAME}/repos?page=1&per_page=200'

resp = get(repo_url)
if not resp.ok:
    print(resp.reason)
    exit(1)
repo_names = [repo['name'] for repo in resp.json()]

exclude = [
    'HTML', 'CSS', 'Jupyter Notebook'
]

lang_stats = {}
for repo in repo_names:
    lang_url = f'{BASE_URL}/repos/{USERNAME}/{repo}/languages'
    resp = get(lang_url)
    if not resp.ok:
        print(resp.reason)
        exit(1)
    data = resp.json()
    data = {k:v for k, v in data.items() if k not in exclude}
    lang_sum = sum([v for v in data.values()])
    
    for k, v in data.items():
        if k in lang_stats:
            lang_stats[k].append(v / lang_sum)
        else:
            lang_stats[k] = [v / lang_sum]
    
total = sum([x for y in lang_stats.values() for x in y])
for k, v in lang_stats.items():
    lang_stats[k] = sum(v) / total

y = np.arange(len(lang_stats))

fix, ax = plt.subplots()
ax.barh(y, lang_stats.values())
ax.set_yticks(y)
ax.set_yticklabels(lang_stats.keys())

plt.show()