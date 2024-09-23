const WebSocket = require('ws')
const { v4: uuidv4 } = require('uuid')

const port = 9001
const wss = new WebSocket.Server({ port: port })

const sockets = {}
const games = {}

console.log(`running on ws://127.0.0.1:${port}`)

wss.on('connection', (ws, req) => {    
    const peerId = req.headers['sec-websocket-key']
    console.log(`${peerId} connected`)

    // add the peer
    sockets[peerId] = { ws: ws, game: null }
    ws.send(JSON.stringify({
        type: 'WHOAMI',
        value: peerId,
    }))

    // tell the peer about all games
    // TODO add more details to this later, i.e. if 
    // game is active, number of players
    updateGameList()

    ws.on('message', message => {
        let { type, value } = JSON.parse(message)

        switch (type) {
            case 'CREATE':
                var gameId = uuidv4()
                games[gameId] = {
                    players: {peerId: {}},
                }
                sockets[peerId].game = gameId
                updateGameList()
                break

            case 'JOIN':
                games[value].players[peerId] = {}
                sockets[peerId].game = value
                break

            case 'START_GAME':
                var gameId = sockets[peerId].game
                games[gameId].status = 'playing'
                games[gameId].players.forEach(peerId => {
                    // make one ball
                    let id = uuidv4()
                    games[gameId].balls[id] = {
                        player: peerId,
                        ballId: id,
                        status: 'idle'
                    }
                })
                games[gameId].players.forEach(peerId => {
                    var j = JSON.stringify({
                        type: 'GAME_UPDATE',
                        value: games[gameId],
                    })
                    console.log(j)
                    sockets[peerId].ws.send(j)
                })
                break

            default:
                console.log('Error, unknown message type!', type, value)
                break
        }
    })

    ws.on('close', (code, reason) => {
        console.log(`${peerId} disconnected`)

        // TODO handle this, remove them from games, remove
        // games with no players
    })
})

// setInterval(() => {
//     console.log('\n\n')
//     console.log(games)
// }, 5000)

const updateGameList = () => {
    Object.values(sockets).forEach((sock) => {
        sock.ws.send(JSON.stringify({
            type: 'GAME_LIST',
            value: Object.keys(games)
        }))
    })
}
