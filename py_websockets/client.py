import asyncio
import websockets

async def hello():
    uri = 'ws://localhost:7890'
    async with websockets.connect(uri) as sock:
        name = input('Name?: ')

        await sock.send(name)
        print(f'> {name}')

        greeting = await sock.recv()
        print(f'< {greeting}')

asyncio.get_event_loop().run_until_complete(hello())
