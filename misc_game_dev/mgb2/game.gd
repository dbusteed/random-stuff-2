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

enum {
	MENU,
	LOBBY,
	GAME
}

var state = MENU

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
	get_tree().set_pause(false)


func host_game(new_player_name):
	player_name = new_player_name
	peer = WebSocketMultiplayerPeer.new()
	peer.create_server(DEFAULT_PORT)
	multiplayer.set_multiplayer_peer(peer)
	players[multiplayer.get_unique_id()] = player_name


func join_game(ip, new_player_name):
	player_name = new_player_name
	peer = WebSocketMultiplayerPeer.new()
	peer.create_client("ws://" + ip + ":" + str(DEFAULT_PORT))
	multiplayer.set_multiplayer_peer(peer)
	players[multiplayer.get_unique_id()] = player_name


func get_player_list():
	return players.values()


func get_player_name():
	return player_name


@rpc("call_local")
func set_game_state(s):
	state = s


func begin_game():
	assert(multiplayer.is_server())
	load_world.rpc()

	world = get_tree().get_root().get_node("World")
	var ball_scene = load("res://ball.tscn")
	
	var item_scene = load("res://item.tscn")
	var item = item_scene.instantiate()
	item.global_position = Vector2(200, 200)
	world.get_node('Items').add_child(item, true)

	var tmp_holes = {}
	var hooles = world.get_node("Holes").get_children()
	for h in hooles:
		tmp_holes[h.name] = null
	update_holes.rpc(tmp_holes)
	
	hooles.shuffle()
	
	var spawn_holes = get_tree().get_nodes_in_group("spawn")
	
	var players2 = {}
	var i = 0
	for p in players:
		var c = colors[randi() % colors.size()]
		var s = spawn_holes.pick_random()
		players2[p] = {
			'name': players[p],
			'color': c,
			'spawn': s.position,
			'holes': {}
		}
		i += 1
		colors.pop_at(colors.find(c))
		spawn_holes.pop_at(spawn_holes.find(s))
	update_players.rpc(players2)

	for p_id in players:
		for n in range(2):			
			var ball = ball_scene.instantiate()
			world.get_node("Balls").add_child(ball, true)
			ball.init.rpc(str(p_id), players[p_id].color, players[p_id].spawn)
	
	world.find_ball.rpc()
	update_score()
	set_game_state(GAME)


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


@rpc("any_peer", "call_local")
func launch_ball(ball_name, force):	
	get_tree().get_root().get_node("World/Balls").get_node(str(ball_name)).launch(force)


@rpc("any_peer", "call_local")
func spawn(pid, color, pos):
	world = get_tree().get_root().get_node("World")
	var nballs = len(get_tree().get_nodes_in_group(pid))
	var ball_scene = load("res://ball.tscn")
	var ball = ball_scene.instantiate()
	world.get_node("Balls").add_child(ball, true)
	ball.init.rpc(pid, color, pos)
	if nballs == 0:
		world.find_ball.rpc_id(str(pid).to_int())


@rpc("call_local")
func remove_ball(ball_name):
	var ball = get_tree().get_root().get_node("World/Balls").get_node(str(ball_name))
	ball.call_deferred("queue_free")


@rpc("call_local")
func add_item(ball_name, pid, _item):
	var ball = get_tree().get_root().get_node("World/Balls").get_node(str(ball_name))
	ball.set_item.rpc_id(str(pid).to_int(), _item.item)
	_item.call_deferred("queue_free")


@rpc("call_local")
func update_ui(s):
	get_tree().get_root().get_node("World/CanvasLayer/VBoxContainer/Label").text = s


@rpc("any_peer", "call_local")
func update_score():
	var score = {'Unclaimed': 0}
	for p in players:
		score[p] = 0
	for h in holes:
		if holes[h] == null:
			score['Unclaimed'] += 1
		elif score.has(holes[h]):
			score[holes[h]] += 1
		else:
			score[holes[h]] = 1
	var s = ''
	for p in players:
		s += players[p].name + ": " + str(score[p]) + "\n"		
	update_ui.rpc(s)
