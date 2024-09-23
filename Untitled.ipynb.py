#!/usr/bin/env python
# coding: utf-8

# In[2]:


import matplotlib.pyplot as plt
import pandas as pd
import plotly.express as px
from sklearn.decomposition import PCA


# In[11]:


df = pd.read_csv('../data/mammography.csv')


# In[12]:


df = df[['x1', 'x2', 'x3', 'y']]
df['y'] = df['y'].astype(str)


# In[13]:


fig = px.scatter_3d(df, x='x1', y='x2', z='x3', color='y')
fig.update_layout(autosize=False, width=600, height=500)
fig.show()


# In[18]:


df['y'].value_counts()


# In[10]:


from sklearn.linear_model import LogisticRegression
from sklearn.metrics import confusion_matrix
from sklearn.model_selection import train_test_split


# In[20]:


X = df.drop(columns='y')
y = df['y']

X_train, X_test, y_train, y_test = train_test_split(X, y, stratify=y)


# In[23]:


lr = LogisticRegression()
lr.fit(X_train, y_train)
y_pred = lr.predict(X_test)
History
confusion_matrix(y_test, y_pred)


# In[ ]:


cm = confusion_metrix()

