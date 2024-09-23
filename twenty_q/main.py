#!/usr/bin/python3.8

import pandas as pd
import numpy as np
from sklearn.tree import DecisionTreeClassifier

df1 = pd.read_csv("data.csv")
df2 = df1.iloc[:, 2:]

df1_A = df1[["answer", "count"]]
df1_B = df1.drop(columns=["answer", "count"])
df1_B = df1_B.reindex(sorted(df1_B.columns), axis=1)
df1 = pd.merge(df1_A, df1_B, left_index=True, right_index=True)

X = df1.drop(columns=["answer", "count"])
y = df1[["answer"]]

clf = DecisionTreeClassifier()
clf.fit(X, y)

def calc_entropy(col):
    return sum([abs(c-0.5) for c in col]) / len(col)

def encode_answer(ans):
    if ans == "y":
        return 1.0
    elif ans == "n":
        return 0.0
    else:
        return 0.5

print("\nAnswer each question with [y]es, [n]o, or [s]ometimes\n")

responses = {q:0.5 for q in df2.columns}

q = 1
limit = 10

while len(df2.columns) > 0 and q <= limit:
    entries = len(df2)
    col_question = pd.Series.idxmax(df2.apply(calc_entropy))
    df2 = df2.drop(columns=[col_question])
    
    question = col_question.replace("_", " ")

    ans = input(f"Q{q}: {question}? ").lower()
    responses[col_question] = encode_answer(ans[0]) 
    q += 1

response_values = [responses[k] for k in sorted(responses)]
answer = clf.predict([response_values])[0]

print(f"\n{answer} ??")

ans = input("\nWas I correct [y/n]? ").lower()
if ans[0] == 'n':
    answer = input("What was the correct answer? ")
responses["answer"] = answer

if df1[df1["answer"] == answer].empty:
    responses["count"] = 1
    df1 = df1.append(responses, ignore_index=True)
else:
    tmp = df1[df1["answer"] != answer]
    row = df1[df1["answer"] == answer]
    count = int(row["count"])
    responses["count"] = (count + 1)
    for item in row.iloc[:, 2:].items():
        responses[item[0]] = round(((responses[item[0]] + (count * float(item[1]))) / (count + 1)), 4)
    df1 = tmp.append(responses, ignore_index=True)

ans = input(f"\nWould you like to add additional info about \"{answer}\" [y/n]? ").lower()
if ans == "y":
    print("\nThanks!")
    print(f"* Add simple questions that describe \"{answer}\", along with the answer ('y', 'n', 's')")
    print("* Separate the question and the answer with two asterisks (**)")
    print("* Also, don't add question marks! The program takes care of that")
    print("* Some examples:")
    print("    Is it smaller than a truck**y")
    print("    Can it be found in a kitcken**n")
    print("    Is it blue**s")
    print("\nEnter 'q' or hit Ctrl+c at any time to quit")

    q = ""
    while q != "q":
        q = input(">>> ")
        
        try:
            q,a = q.split("**")
            q = q.replace(" ", "_")
            if a.lower() not in ("y", "n"):
                a = "s"
            a = encode_answer(a.lower())
            df1[q] = 0.5
            df1.loc[df1["answer"] == answer, q] = a

        except Exception as e:
            q = "q"

df1.to_csv("data.csv", index=False)