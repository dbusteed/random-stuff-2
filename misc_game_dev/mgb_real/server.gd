extends Node2D

var peer
var tiles

var colors = [
	Color.BLUE,
	Color.RED,
	Color.PURPLE,
	Color.YELLOW,
	Color.ORANGE,
	Color.FUCHSIA
]

func _ready():
	randomize()
	peer = WebSocketMultiplayerPeer.new()
	
	if "--server" in OS.get_cmdline_user_args():
		get_window().title = 'SERVER'

		peer.create_server(5000)
		multiplayer.set_multiplayer_peer(peer)
		
		multiplayer.peer_connected.connect(self._player_connected)
		multiplayer.peer_disconnected.connect(self._player_disconnected)
		
		var game = load("res://game.tscn").instantiate()
		get_tree().root.add_child.call_deferred(game)
				
		var open = {}
		var closed = {}
		
		tiles = [
			Vector2i(0, 0),
			Vector2i(1, 0),
			Vector2i(0, 1),
			Vector2i(-1, 0), 
			Vector2i(0, -1)
		]

		var neighbors = [
			Vector2i(1, 0),
			Vector2i(0, 1),
			Vector2i(-1, 0), 
			Vector2i(0, -1)
		]
		
		for t in tiles:
			open[t] = null
		
		var count = 0
		while len(open) > 0 and count < 20:
			var t = open.keys()[0]
			for n in neighbors:
				var tt = n + t
				if !open.has(tt) and !closed.has(tt):
					if randf() > 0.25:
						tiles.append(tt)
						open[tt] = null
					else:
						closed[tt] = null
			open.erase(t)
			count += 1

		var min_x
		var max_x
		var min_y
		var max_y
		for t in tiles:
			pass	

		#var hole_scene = load("res://hole.tscn")
		#var hole = hole_scene.instantiate()
		#hole.global_position = Vector2(0, 0)
		#add_child(hole)
		
	else:
		var main = load("res://main.tscn").instantiate()
		get_tree().root.add_child.call_deferred(main)


func _player_connected(id):
	print(id, ' joined')
	var ball = load("res://ball.tscn").instantiate()
	var c = colors[randi() % colors.size()]
	get_tree().get_root().get_node("Game/Balls").add_child(ball, true)
	
	ball.init.rpc(str(id), c, Vector2(0, 0))
	get_tree().get_root().get_node("Game").find_ball.rpc_id(id)
	get_tree().get_root().get_node("Game").build_map.rpc(tiles)


func _player_disconnected(id):
	print(id, ' left')
	for ball in get_tree().get_nodes_in_group(str(id)):
		ball.queue_free()


@rpc("any_peer")
func launch_ball(ball_name, force):	
	get_tree().get_root().get_node("Game/Balls").get_node(str(ball_name)).launch(force)
