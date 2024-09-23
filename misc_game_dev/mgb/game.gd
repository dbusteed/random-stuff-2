extends Node

const DEFAULT_PORT = 10567
const MAX_PEERS = 12

var peer = null
var player_name = "The Warrior"

var world = null

var players = {}
var holes = {}

signal player_list_changed()
signal connection_failed()
signal connection_succeeded()
signal game_ended()
signal game_error(what)

var colors = [
	Color.BLUE,
	Color.RED,
	Color.PURPLE,
	Color.YELLOW,
	Color.ORANGE,
	Color.FUCHSIA
]


func _player_connected(id):
	# Registration of a client beings here, tell the connected player that we are here.
	register_player.rpc_id(id, player_name)


# Callback from SceneTree.
func _player_disconnected(id):
	if has_node("/root/World"): # Game is in progress.
		if multiplayer.is_server():
			game_error.emit("Player " + players[id] + " disconnected")
			end_game()
	else: # Game is not in progress.
		# Unregister this player.
		unregister_player(id)


# Callback from SceneTree, only for clients (not server).
func _connected_ok():
	# We just connected to a server
	connection_succeeded.emit()


# Callback from SceneTree, only for clients (not server).
func _server_disconnected():
	game_error.emit("Server disconnected")
	end_game()


# Callback from SceneTree, only for clients (not server).
func _connected_fail():
	multiplayer.set_network_peer(null) # Remove peer
	connection_failed.emit()


# Lobby management functions.
@rpc("any_peer")
func register_player(new_player_name):
	var id = multiplayer.get_remote_sender_id()
	players[id] = new_player_name
	player_list_changed.emit()


func unregister_player(id):
	players.erase(id)
	player_list_changed.emit()


@rpc("call_local")
func update_holes(h):
	holes = h

@rpc("call_local")
func update_players(new_one):
	players = new_one

@rpc("call_local")
func load_world():
	var w = load("res://world.tscn").instantiate()
	get_tree().get_root().add_child(w)
	get_tree().get_root().get_node("Lobby").hide()

	# Set up score.
#	world.get_node("Score").add_player(multiplayer.get_unique_id(), player_name)
#	for pn in players:
#		world.get_node("Score").add_player(pn, players[pn])
	get_tree().set_pause(false) # Unpause and unleash the game!


func host_game(new_player_name):
	player_name = new_player_name
	peer = ENetMultiplayerPeer.new()
	peer.create_server(DEFAULT_PORT, MAX_PEERS)
	multiplayer.set_multiplayer_peer(peer)
	players[multiplayer.get_unique_id()] = player_name


func join_game(ip, new_player_name):
	player_name = new_player_name
	peer = ENetMultiplayerPeer.new()
	peer.create_client(ip, DEFAULT_PORT)
	multiplayer.set_multiplayer_peer(peer)
	players[multiplayer.get_unique_id()] = player_name


func get_player_list():
	return players.values()


func get_player_name():
	return player_name


func begin_game():
	assert(multiplayer.is_server())
	load_world.rpc()

	world = get_tree().get_root().get_node("World")
	var player_scene = load("res://player.tscn")

	var tmp_holes = {}
	var hooles = world.get_node("Holes").get_children()
	for h in hooles:
		tmp_holes[h.name] = null
	update_holes.rpc(tmp_holes)
	
	hooles.shuffle()
	
	var players2 = {}
	var i = 0
	for p in players:
		players2[p] = {
			'name': players[p],
			'color': colors[randi() % colors.size()],
			'spawn': hooles[i].position,
			'holes': {}
		}
		i += 1
	update_players.rpc(players2)

	for p_id in players:
#		var spawn_pos = world.get_node("SpawnPoints/" + str(spawn_points[p_id])).position
		var player = player_scene.instantiate()
#		player.synced_position = spawn_pos
		player.name = str(p_id)
#		player.color = players[p_id]['color']
#		player.set_player_name(player_name if p_id == multiplayer.get_unique_id() else players[p_id])
		world.get_node("Players").add_child(player)


func end_game():
	if has_node("/root/World"):
		get_node("/root/World").queue_free()

	game_ended.emit()
	players.clear()


func _ready():
	randomize()
	multiplayer.peer_connected.connect(self._player_connected)
	multiplayer.peer_disconnected.connect(self._player_disconnected)
	multiplayer.connected_to_server.connect(self._connected_ok)
	multiplayer.connection_failed.connect(self._connected_fail)
	multiplayer.server_disconnected.connect(self._server_disconnected)


func _process(_delta):
	pass
#	print('holes ', holes)
#	print('players ', players)
	var score = {'Unclaimed': 0}
	for h in holes:
		if holes[h] == null:
			score['Unclaimed'] += 1
		elif score.has(holes[h]):
			score[holes[h]] += 1
		else:
			score[holes[h]] = 1
	
#	print(players)
#	if world != null:
#		var s = ''
#		for p in players:
#			s += players[p].name + "\n"
#		print(s)
#		world.get_node("CanvasLayer/Label").text = s
