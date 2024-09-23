import asyncio
import websockets

async def hello(sock, path):
    name = await sock.recv()
    print(f'< {name}')

    greeting = f'Hello {name}'

    await sock.send(greeting)
    print(f'> {greeting}')

start_server = websockets.serve(hello, 'localhost', 7890)

asyncio.get_event_loop().run_until_complete(start_server)
asyncio.get_event_loop().run_forever()
