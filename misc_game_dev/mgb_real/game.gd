extends Node2D

@onready var line = $DottedLine
@onready var tilemap = $TileMap

var angle
var dist
var seg_x
var seg_y
var balls
var ball
var ball_name
var server
var seg = 5

func _ready():
	server = get_tree().root.get_node("Server")
	
	# so basically, some algo to decide the tiles/shape of the map,
	# then run that thru this to update autotiles, the send that same list to 
	# clients to run it themselves
	#
	# the shape selection might happen elsewhere, so that both server and client
	# can run the same code here to build the level
	
	
	# list of "tiles", this is what will get passed to clients
	# start with (0, 0)
	

func _unhandled_input(_event):
	
	if Input.is_action_pressed("lmb"):
		angle = ball.global_position.angle_to_point(get_global_mouse_position())
		dist = clamp(ball.global_position.distance_to(get_global_mouse_position()), 0.0, 200.0)		
		seg_x = (cos(angle) * dist) / seg
		seg_y = (sin(angle) * dist) / seg
		var prev_pos = ball.global_position
		for dot in line.get_children():
			dot.global_position = prev_pos + Vector2(seg_x, seg_y)
			prev_pos = dot.global_position
			dot.visible = true
		
	if Input.is_action_just_released("lmb"):
		for dot in line.get_children():
			dot.visible = false
		var mpos = get_global_mouse_position()
		var force = ball.global_position - mpos
		force *= 2
		server.launch_ball.rpc_id(1, ball.name, force)


func _on_button_pressed():
	print("\nDEBUG!")
	print(server.peer.get_unique_id())
	

@rpc("call_local")
func find_ball():
	balls = get_tree().get_nodes_in_group(str(multiplayer.get_unique_id()))
	ball = balls[0]
	ball_name = ball.name
	ball.activate_camera()


@rpc("call_local")
func build_map(tiles):
	tilemap.set_cells_terrain_connect(0, tiles, 0, 0)
