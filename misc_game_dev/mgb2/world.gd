extends Node2D

@onready var line = $DottedLine
@onready var ball_scene = preload("res://ball.tscn")

var seg = 5
var ball = null
var ball_name = null
var spawn_queue = []
var cam_zoom = 1.5

var balls
var angle
var dist
var seg_x
var seg_y
var idx

func _unhandled_input(_event):
	if not ball: return

	if Input.is_action_just_pressed("next_ball"):
		change_ball(1)	
	elif Input.is_action_just_pressed("prev_ball"):
		change_ball(-1)

#	if Input.is_action_just_pressed("cam_zoom_in"):
#		ball.zoom_camera(1)
#	elif Input.is_action_just_pressed("cam_zoom_out"):
#		ball.zoom_camera(-1)
	
	if Input.is_action_just_pressed("use_item"):
		if ball.item != "":
			ball.use_item()
		
	
	if ball.state == ball.READY:
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
			angle = ball.global_position.angle_to_point(mpos)
			dist = clamp(ball.global_position.distance_to(get_global_mouse_position()), 0.0, 200.0)
			var force = -1 * Vector2(dist * cos(angle), dist * sin(angle))
			force *= 2
			game.launch_ball.rpc_id(1, ball.name, force)
			
	if Input.is_action_just_pressed("debug"):
		print(game.players)
		print(game.holes)


func change_ball(dir):
	balls = []
	for b in get_tree().get_nodes_in_group(str(multiplayer.get_unique_id())):
		balls.append(str(b.name))
	
	idx = balls.find(ball_name)
	idx = (idx + dir) % len(balls)
	
	ball_name = str(balls[idx])
	ball = $Balls.get_node(ball_name)
	ball.activate_cam()
	update_hud()


func update_hud():
	$CanvasLayer/VBoxContainer/Label2.set_text(ball.item)


@rpc("call_local")
func find_ball():
	balls = get_tree().get_nodes_in_group(str(multiplayer.get_unique_id()))
	ball = balls[0]
	ball_name = ball.name
	ball.activate_cam()


func _process(_delta):
	if len(spawn_queue) and $Timer.is_stopped():
		$Timer.start(3)


func _on_timer_timeout():
	$Timer.stop()
	var p_id = multiplayer.get_unique_id()
	
	balls = get_tree().get_nodes_in_group(str(multiplayer.get_unique_id()))
	var my_holes = game.players[str(p_id).to_int()].holes
	var holes = get_tree().get_root().get_node('World/Holes')
	
	var avail_holes = {}
	for hole in my_holes:
		if not holes.get_node(hole).contested:
			avail_holes[hole] = my_holes[hole]
			
	if len(avail_holes) == 0:
		print('no holes available!')
	else:
		var pos = spawn_queue.pop_front()
		var min_dist = INF
		var spawn_pos = null
		
		for hole in avail_holes:
			var hpos = avail_holes[hole]
			if pos.distance_to(hpos) < min_dist:
				min_dist = pos.distance_to(hpos)
				spawn_pos = hpos
		
		game.spawn.rpc_id(1, str(p_id), game.players[p_id].color, spawn_pos)
		

@rpc("call_local")
func test(_ball_name, pos):
	if _ball_name == ball_name:
		balls = get_tree().get_nodes_in_group(str(multiplayer.get_unique_id()))
		var i = 0
		for b in balls:
			if b.name == ball_name:
				balls.remove_at(i)
				break
			i += 1
		if len(balls):
			ball = balls[0]
			ball_name = ball.name
			ball.activate_cam()
	spawn_queue.push_back(pos)

