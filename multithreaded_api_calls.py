import requests
from concurrent.futures import ThreadPoolExecutor
from tqdm import tqdm

# example of api with page param => https://api.api.com?page=10

url = "https://api.api.com"

def make_api_call(page):
    try:
        response = requests.get(f'{url}?page={page}')
        return response.json()['data']
    except:
        return []
    
# if we have 100 api calls we need to do,
# we make a list of [1,2,...100]. each item
# in the list will get passed to `make_api_call()`
total_pages = 100
pages = list(range(1, total_pages+1))


with ThreadPoolExecutor(max_workers=20) as ex:
    # use the "executor" and map the list of parameters (`pages`)
    # to the function `make_api_call()`
    results = ex.map(make_api_call, pages)
    
    # same as above but with a progress bar
    # results = list(tqdm(ex.map(download, urls), total=len(urls)))

# then do something with `results`