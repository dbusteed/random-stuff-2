"""

keep track of open games

endpoints
- GET list of games
- POST end game (called by game server)
- POST create a new game (called by client)

"""

from typing import Annotated

from fastapi import FastAPI, Body

app = FastAPI()
games = {k: {} for k in range(5000, 5011)}


@app.get("/games")
async def list_games():
    return games


@app.post("/create-game")
async def create_game(name: Annotated[str, Body()]):
    for port, game in games.items():
        if game == {}:
            games[port] = {"name": name, "port": port, "status": "lobby", "players": 0}
            # TODO actually spawn the game server here!
            break
    return games


