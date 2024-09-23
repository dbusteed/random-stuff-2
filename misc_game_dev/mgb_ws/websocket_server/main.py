import asyncio
import json
from uuid import uuid4
from websockets.server import serve

sockets = {}
games = {}

class Peer:
    def __init__(self, ws, peer_id, peer_name, game):
        self.ws = ws
        self.peer_id = peer_id
        self.peer_name = peer_name
        self.game = game

class Game:
    def __init__(self, game_id, players):
        self.game_id = game_id
        self.players = players

    async def broadcast(self, msg):
        for player in self.players:
            await player.ws.send(json.dumps(msg))

    async def update_lobby(self):
        msg = {
            'type': 'UPDATE_LOBBY',
            'value': [p.peer_id for p in self.players]
        }
        await self.broadcast(msg)

async def update_game_list():
    for sock in sockets.values():
        await sock.ws.send(json.dumps({
            'type': 'GAME_LIST',
            'value': list(games.keys())
        }))

async def echo(websocket):
    peer_id = str(websocket.id)
    print(f'peer connected: {peer_id}')
    sockets[peer_id] = Peer(websocket, peer_id, '', None)
    await update_game_list()

    async for message in websocket:
        kind, value = [*json.loads(message).values()]
        match kind:
            case 'CREATE':
                game_id = str(uuid4())
                games[game_id] = Game(game_id, [sockets[peer_id]])
                sockets[peer_id].game = game_id
                await update_game_list()
                await games[game_id].update_lobby()

            case 'JOIN':
                games[value].players.append(Peer(websocket, peer_id, '', None))
                sockets[peer_id].game = value
                await games[value].update_lobby()

            case _:
                break
    
async def main():
    async with serve(echo, "localhost", 9001):
        await asyncio.Future()  # run forever

asyncio.run(main())