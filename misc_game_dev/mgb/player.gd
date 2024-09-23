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
	
	var i = 0
	for bl in $Balls.get_children():
		bl.position = spawn
		bl.init(name, color, i < 2)
		bl.ball_lost.connect(add_to_spawn_queue)
		i += 1
	
	ball = $Balls.get_node(selected_ball)
	ball.activate_cam()
	

func change_ball(dir):
	balls = []
	for bl in $Balls.get_children():
		balls.append(str(bl.name))
	
	while true:
		idx = balls.find(selected_ball)
		idx = (idx + dir) % len(balls)
		selected_ball = str(balls[idx])
		ball = $Balls.get_node(selected_ball)
		if ball.state != ball.INACTIVE:
			break
	
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
		
		# the ball is killed, still sits there for a bit, once it moves position, it's removed from that hole?
		
		if Input.is_action_just_released("lmb"):
			for dot in line.get_children():
				dot.visible = false
			var mpos = get_global_mouse_position()
			var force = ball.global_position - mpos
			force *= 2
			ball.launch(force)
			
	if Input.is_action_just_pressed("debug"):
		print()
		print(game.players)
		print(game.holes)
		for h in get_tree().get_root().get_node('World/Holes').get_children():
			print()
			print(h.name)
			print(h.moving)
			print(h.stopped)
			print(h.contested)
			


func add_to_spawn_queue(ball_name, pos):
	print('adding to spawn queue!')
	if any_active_balls() and ball_name == selected_ball:
		change_ball(1)
	else:
		pass # focus a camera somewhere else?
	spawn_queue.push_back(pos)


func _process(_delta):
	if len(spawn_queue) and $Timer.is_stopped():
		print('starting timer')
		$Timer.start(4)
	

func _on_timer_timeout():
	var pos = spawn_queue[0]
	
	if len(game.players[str(name).to_int()].holes) > 0:
		
		var holes = get_tree().get_root().get_node('World/Holes')
		var min_dist = INF
		var spawn_pos = null
		
		for hole in game.players[str(name).to_int()].holes:
			var hpos = game.players[str(name).to_int()].holes[hole]
			if pos.distance_to(hpos) < min_dist:
				if holes.get_node(hole).contested: continue
				min_dist = pos.distance_to(hpos)
				spawn_pos = hpos
				
		if spawn_pos != null:
			spawn_queue.pop_front()
			$Timer.stop()
			for bl in $Balls.get_children():
				if bl.state == bl.INACTIVE:
					bl.activate(spawn_pos)
					break
						
	elif not any_active_balls():
			print('you lose!')


func any_active_balls():
	for bl in $Balls.get_children():
		if bl.state != bl.INACTIVE:
			return true
	return false
	
