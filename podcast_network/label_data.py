import json
import os
import sys
import spacy
import spotipy
from random import choice
from spotipy.oauth2 import SpotifyClientCredentials

nlp = spacy.load('en_core_web_sm')
sp = spotipy.Spotify(
    auth_manager=SpotifyClientCredentials(
        client_id="f112fb2de5af498795c8f71a25ef859a",
        client_secret="5e3e98de0ad74c528eb365a12c2f6514"
    )
)

def load_data(path):
    if os.path.exists(path):
        return json.loads(open(path, 'r').read())
    else:
        return {}

def get_episodes(show_id):
    offset = 0
    episodes = {}
    while True:
        print('.', end='')
        sys.stdout.flush()
        
        res = sp.show_episodes(show_id, limit=50, offset=offset, market='US')
        eps = {
            r['id']: {
                'show_id': show_id,
                'ep_id': r['id'],
                'ep_name': r['name'],
                'ep_desc': r['description'],
                'labeled': False,
                'guests': []
            } for r in res['items']}
        episodes = {**episodes, **eps}
        if not res['next']:
            break
        offset += 50
    return episodes

shows = load_data('data/shows.json')
episodes = load_data('data/episodes.json')
show2person = load_data('data/show2person.json')
person2show = load_data('data/person2show.json')

while True:
    cmd = input('>>> ')
    
    if cmd == 'h':
        print('''
[a] add show
[e] get episodes
[l] label episodes
[s] data summary
[h] help
[q] quit
        ''')
    
    elif cmd == 'a':
        print()
        show_query = input('Search for a show: ')
        res = sp.search(show_query, limit=5, type='show', market='US')
        res = [(r['id'], r['name'], r['total_episodes']) for r in res['shows']['items']]
        for i, r in enumerate(res):
            print(f'  {i+1}. {r[1]}')
        sel = int(input('Select show: ')) - 1
        hosts = input('Host names: ')
        hosts = [host.strip() for host in hosts.split(',')]
        shows[res[sel][0]] = {
            'name': res[sel][1],
            'total_episodes': res[sel][2]
        }
        show2person[res[sel][0]] = hosts
        for h in hosts: 
            if h in person2show:
                person2show[h].append(res[sel][0])
            else:
                person2show[h] = [res[sel][0]]
        print('Show added!\n')
    
    elif cmd == 'e':
        print('\nFetch episodes for which show?')
        print('  0. RETURN TO MENU')
        for i, (show_id, show) in enumerate(shows.items()):
            print(f"  {i+1}. {show['name']}")
        sel = int(input('Select a show: '))
        if sel:
            show_id = list(shows.keys())[sel - 1]
            new_eps = get_episodes(show_id)
            episodes = {**episodes, **new_eps}
        print('\n')

    elif cmd == 'l':
        print('\nLabel episodes for which show?')
        print('  0. RETURN TO MENU')
        for i, (show_id, show) in enumerate(shows.items()):
            print(f"  {i+1}. {show['name']}")
        sel = int(input('Select a show: '))
        if sel:
            show_id = list(shows.keys())[sel - 1]
            eps = {k:v for k,v in episodes.items() if v['show_id'] == show_id}
            print('\nblank lines for no guests, q to return, s to skip, a to accept suggestion')
            while True and len(eps):
                eps = {k:v for k,v in eps.items() if not v['labeled']}
                if not len(eps):
                    print('all done!')
                    break
                e = choice(list(eps.values()))
                print()
                print(e['ep_name'])
                print(e['ep_desc'])
                suggest = [ent.text for ent in nlp(e['ep_name']).ents if ent.label_ == 'PERSON']
                suggest = ','.join(suggest)
                print(f'Suggested: {suggest}')
                inp = input('Guests: ')
                if inp == 'q':
                    break
                elif inp == 's':
                    continue
                elif inp == 'a':
                    guests = [g.strip() for g in suggest.split(',')]
                elif inp == '':
                    guests = []
                else:
                    guests = [g.strip() for g in inp.split(',')]
                episodes[e['ep_id']]['guests'] = guests
                episodes[e['ep_id']]['labeled'] = True

    elif cmd == 's':
        print('\nData Summary:')
        for show_id, show in shows.items():
            eps = {k:v for k,v in episodes.items() if v['show_id'] == show_id}
            eps2 = {k:v for k,v in eps.items() if v['labeled']}
            ratio = (len(eps2) / len(eps)) if len(eps) else 0
            print(f"  {show['name'][:20]:<20} {len(eps2):>4} {len(eps):>4} {ratio:.3f}")
        print()

    elif cmd == 'q':
        with open('data/shows.json', 'w') as f:
            f.write(json.dumps(shows, indent=2))
        with open('data/episodes.json', 'w') as f:
            f.write(json.dumps(episodes, indent=2))
        with open('data/person2show.json', 'w') as f:
            f.write(json.dumps(person2show, indent=2))
        with open('data/show2person.json', 'w') as f:
            f.write(json.dumps(show2person, indent=2))
        break


# Key: BKQYKTZ6KP6CGEAYQPJA
# Secret: mv$^DGsFPdwrcgrTg4fVmqAskPMPcDk#nHYqsjt9


# results = sp.search(q='weezer', limit=20)
# for idx, track in enumerate(results['tracks']['items']):
#     print(idx, track['name'])
    
# sp.artists()
# sp.artist()
# sp.artist(4)
# sp.artist('4')
# dir(sp)
# sp.search("A", limit=10, type='show', market='GB')
# sp.search("A", limit=10, type='show', market='US')
# sp.search("A", limit=1, type='show', market='US')
# sp.search("o", limit=1, type='show', market='US')
# sp.search("l", limit=1, type='show', market='US')
# sp.search("p", limit=1, type='show', market='US')
# dir(sp.search)
# help(sp.search)
# sp.search("T", limit=1, type='show', market='US')
# sp.search("J", limit=1, type='show', market='US')
# sp.search("The J", limit=1, type='show', market='US')
# sp.search("The Joe", limit=1, type='show', market='US')
# sp.search("The Joe", limit=3, type='show', market='US')
# help(sp.search)
# sp.search("The Joe", limit=3, type='episode', market='US')
# sp.search("The Joe", limit=3, type='episode', market='US')
# sp
# sp.show()
# sp.show('7wIDnmJ41exqZZ5GnsBGDS')
# sp.show('')
# sp.show('spotify:episode:7MDxyrrhD7gC7XMRwB0ulv')
# sp.show('7MDxyrrhD7gC7XMRwB0ulv')
# sp.search("The Joe", limit=3, type='show', market='US')
# sp.show('7wIDnmJ41exqZZ5GnsBGDS')
# sp.show('spotify:show:7wIDnmJ41exqZZ5GnsBGDS')
# help(sp.show)
# sp.show('spotify:show:7wIDnmJ41exqZZ5GnsBGDS', market='US')
# sp.show_episodes('spotify:show:7wIDnmJ41exqZZ5GnsBGDS', market='US')
# sp.show('spotify:show:7wIDnmJ41exqZZ5GnsBGDS', market='US')
# sp.show_episodes('spotify:show:7wIDnmJ41exqZZ5GnsBGDS', market='US')
# help(sp.show_episodes)
# get_ipython().run_line_magic('save', 'spot.py 1-100')
