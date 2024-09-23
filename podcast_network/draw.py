# coding: utf-8
import json
import os
import sys
import networkx as nx
import matplotlib.pyplot as plt
from bokeh.models import Circle, MultiLine, Plot, Range1d
from bokeh.plotting import figure, from_networkx, show
from collections import Counter
from sklearn.preprocessing import LabelEncoder

# show everyone? or just people with podcasts
everyone = True

def load_data(path):
    if os.path.exists(path):
        return json.loads(open(path, 'r').read())
    else:
        return {}

shows = load_data('data/shows.json')
episodes = load_data('data/episodes.json')
show2person = load_data('data/show2person.json')
person2show = load_data('data/person2show.json')

enc = LabelEncoder()

persons = list(person2show.keys())
all_guests = [v['guests'] for v in episodes.values() if v['guests']]
all_guests = [x for y in all_guests for x in y]
all_people = list(set(persons).union(set(all_guests)))

enc.fit(all_people)

G = nx.Graph()
G.add_nodes_from([(enc.transform([p])[0], {'name': p}) for p in all_people])
edges = []
for k,v in person2show.items():
    for show_id in v:
        guests = [v['guests'] for v in episodes.values() if v['show_id'] == show_id and v['guests']]
        guests = [x for y in guests for x in y]
        for g in guests:
            if enc.transform([g])[0] in G.nodes or everyone:
                a = enc.transform([k])[0]
                b = enc.transform([g])[0]
                if a < b:
                    edges.append((a, b))
                else:
                    edges.append((b, a))

edges = Counter(edges)
edges = [(*e, {'weight': c}) for e, c in edges.items()]
G.add_edges_from(edges)

plot = figure(height=600, width=900, tooltips="@name")

graph_renderer = from_networkx(G, nx.spring_layout, scale=1, center=(0, 0))
graph_renderer.node_renderer.glyph = Circle(size=16, fill_color="lightblue")
graph_renderer.edge_renderer.glyph = MultiLine(line_color="black", line_alpha=0.8, line_width=1.5)
plot.renderers.append(graph_renderer)

show(plot)