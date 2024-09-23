extends Node2D

@onready var line = $DottedLine
@onready var ball_scene = preload("res://ball.tscn")

const SPEED = 100
const MIN_BALL = 2
var seg = 5

var color = Color(1, 1, 1)
var spawn
var angle
var dist
var seg_x
var seg_y
var balls
var idx
var b
var spawn_queue = []

var ball = null
var selected_ball = 'Ball'


func _enter_tree():
	set_multiplayer_authority(str(name).to_int())


func _ready():
	if not is_multiplayer_authority(): return
	
	color = game.players[str(name).to_int()]['color']
	spawn = game.players[str(name).to_int()]['spawn']
	
	var new_ball = ball_scene.instantiate()
	new_ball.position = spawn
	new_ball.init(name, color)
	new_ball.despawn.connect(add_to_spawn_queue)
#	game.add_ball.rpc(str(name).to_int(), new_ball)
	
#	new_ball = ball_scene.instantiate()
#	new_ball.position = spawn
#	new_ball.init(name, color)
#	new_ball.despawn.connect(add_to_spawn_queue)
	print(name, ' adding own ball')
	get_tree().get_root().get_node("World/Balls").add_child(new_ball, true)
	
#	for bl in $Balls.get_children():
#		bl.position = spawn
#		bl.init(name, color)
#		bl.despawn.connect(add_to_spawn_queue)
	
#	ball = $Balls.get_node(selected_ball)
#	ball.activate_cam()
	

func change_ball(dir):
	balls = []
	for bl in $Balls.get_children():
		balls.append(str(bl.name))
	
	idx = balls.find(selected_ball)
	idx = (idx + dir) % len(balls)
	
	selected_ball = str(balls[idx])
	ball = $Balls.get_node(selected_ball)
	ball.activate_cam()		
	

func _unhandled_input(_event):
	if not is_multiplayer_authority() or not ball: return

	if Input.is_action_just_pressed("next_ball"):
		change_ball(1)	
	elif Input.is_action_just_pressed("prev_ball"):
		change_ball(-1)
	
	if Input.is_action_just_pressed("cam_zoom_in"):
		ball.zoom_camera(1)
	elif Input.is_action_just_pressed("cam_zoom_out"):
		ball.zoom_camera(-1)
		
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
			var force = ball.global_position - mpos
			force *= 2
			ball.launch(force)
			
	if Input.is_action_just_pressed("debug"):
#		print(game.players)
#		print(game.holes)
		print(get_tree().get_root().get_node("/World/Players"))


func add_to_spawn_queue(ball_name, pos):
	print('adding to spawn queue!')
	if ball_name == selected_ball:
		selected_ball = $Balls.get_children()[0].name
		ball = $Balls.get_node(str(selected_ball))
		ball.activate_cam()		
	spawn_queue.push_back(pos)
	
	# eliminated when all balls eliminated AND no holes controlled
	# otherwise, balls will spawn in the holes, or remaining balls
	# can take other holes
	
	# if holes == 0 and balls == [], then LOSE!

func _process(_delta):
	if len(spawn_queue) and $Timer.is_stopped():
		print('starting timer')
		$Timer.start(3)
	

func _on_timer_timeout():
	var pos = spawn_queue.pop_front()
	print('got em ', pos)
	$Timer.stop()
	
	balls = $Balls.get_child_count()
	if len(game.players[str(name).to_int()].holes) == 0 and ball == 0:
		print('you lose!')
		ball = null
		
	var new_ball = ball_scene.instantiate()
	new_ball.position = pos
	new_ball.init(name, color)
	new_ball.despawn.connect(add_to_spawn_queue)
	$Balls.add_child(new_ball)
