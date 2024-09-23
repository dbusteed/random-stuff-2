import wget
from concurrent.futures import ThreadPoolExecutor
from time import sleep
from tqdm import tqdm

def download(u):
    wget.download(u, bar=None)
    sleep(1)

urls = [
    "https://raw.githubusercontent.com/dbusteed/spark-structured-streaming/master/README.md",
    "https://raw.githubusercontent.com/dbusteed/spark-structured-streaming/master/iot_devices.py",
    "https://raw.githubusercontent.com/dbusteed/spark-structured-streaming/master/.gitignore",
    "https://raw.githubusercontent.com/dbusteed/spark-structured-streaming/master/StreamHandler/src/main/scala/StreamHandler.scala",
    "https://raw.githubusercontent.com/dbusteed/spark-structured-streaming/master/StreamHandler/build.sbt",
    "https://raw.githubusercontent.com/dbusteed/spark-structured-streaming/master/README.md",
    "https://raw.githubusercontent.com/dbusteed/spark-structured-streaming/master/iot_devices.py",
    "https://raw.githubusercontent.com/dbusteed/spark-structured-streaming/master/.gitignore",
    "https://raw.githubusercontent.com/dbusteed/spark-structured-streaming/master/StreamHandler/src/main/scala/StreamHandler.scala",
    "https://raw.githubusercontent.com/dbusteed/spark-structured-streaming/master/StreamHandler/build.sbt"
]

with ThreadPoolExecutor(max_workers=5) as ex:
    res = list(tqdm(ex.map(download, urls), total=len(urls)))
