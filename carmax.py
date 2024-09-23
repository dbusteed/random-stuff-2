import argparse
import pandas as pd
import re
import time
import plotly.express as px
from bs4 import BeautifulSoup
from tqdm import tqdm
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.chrome.options import Options

parser = argparse.ArgumentParser()
parser.add_argument("-v", "--viz-only", dest="viz_only", action="store_true")
parser.add_argument("-p", "--pages", dest="pages", default=5, type=int)
parser.set_defaults(viz_only=False)
args = parser.parse_args()

if not args.viz_only:
    chrome_options = Options()
    # chrome_options.add_argument("--headless")

    b = webdriver.Chrome(options=chrome_options)
    # b.get('https://www.carmax.com/cars/toyota')
    b.get('https://www.carmax.com/cars/toyota/sienna')

    try:
        for _ in tqdm(range(args.pages)):
            b.find_element(By.XPATH, '//*[@id="see-more"]/div/hzn-button').click()
            time.sleep(20)
    except:
        pass

    page = BeautifulSoup(b.page_source, 'html.parser')

    price = [int(re.sub(r'[\$,\*]', '', e.text)) for e in page.find_all(class_='sc--price-miles-info--price')]
    miles = [int(re.sub(r'K mi', '', e.text)) * 1000 for e in page.find_all(class_='sc--price-miles-info--miles')]
    year = [int(e.text.split()[0]) for e in page.find_all(class_='sc--make-model-info--year-make')]
    make = [e.text.split()[1] for e in page.find_all(class_='sc--make-model-info--year-make')]
    model = [e.text.split()[0] for e in page.find_all(class_='sc--make-model-info--model-trim')]
    trim = [e.text.split()[1] for e in page.find_all(class_='sc--make-model-info--model-trim')]

    df = pd.DataFrame({
        'price': price,
        'miles': miles,
        'year': year,
        'make': make,
        'model': model,
        'trim': trim,
    })
    
    b.quit()

else:
    df = pd.read_csv('_carmax.csv')

df.to_csv('_carmax.csv', index=False)
df.iloc[-1] = [34995, 29790, 2021, '', 'Bingo', '']
#print(df)

fig = px.scatter(
    data_frame=df,
    x='miles',
    y='price',
    color='model',
    custom_data=['year'],
    trendline="ols", 
    #  trendline_scope="overall"
)
# fig = px.scatter_3d(df, x='miles', y='price', z='year', color='model')
fig.show()

