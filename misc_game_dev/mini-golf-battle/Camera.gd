extends Camera3D

@onready var ball = $"../Ball"
@onready var dots = $"../Dots"
@export var force_adj: float = 50

const SPEED = 10

# Called when the node enters the scene tree for the first time.
func _ready():
#	look_at(ball.global_position)
	pass


func _physics_process(delta):
	if Input.is_action_pressed("up"):
		global_position.z += -delta * SPEED
	elif Input.is_action_pressed("down"):
		global_position.z += delta * SPEED
	elif Input.is_action_pressed("left"):
		global_position.x += -delta * SPEED
	elif Input.is_action_pressed("right"):
		global_position.x += delta * SPEED
	
	if Input.is_action_just_pressed("rotate_left"):
		rotate_y(0.5)
	elif Input.is_action_just_pressed("rotate_right"):
		rotate_y(0.5)
	
	if Input.is_action_pressed("lmb"):
		var mpos = get_viewport().get_mouse_position()
		var start = project_ray_origin(get_viewport().get_mouse_position())
		var end = start + project_ray_normal(mpos) * 2000
		var world = get_world_3d().direct_space_state
		var query = PhysicsRayQueryParameters3D.create(start, end)
		var result = world.intersect_ray(query)
	
		var a = Vector2(ball.global_position.x, ball.global_position.z)
		var b = Vector2(result.position.x, result.position.z)
		var angle = a.angle_to_point(b)
		var dist = clamp(a.distance_to(b), 0, 15)
		var seg_x = (cos(angle) * dist) / 4
		var seg_y = (sin(angle) * dist) / 4

		var prev_pos = ball.global_position
		for dot in dots.get_children():
			dot.global_position = prev_pos + Vector3(seg_x, 0, seg_y)
			prev_pos = dot.global_position
			dot.visible = true
	
	elif Input.is_action_just_released("lmb"):
		var mpos = get_viewport().get_mouse_position()
		var start = project_ray_origin(get_viewport().get_mouse_position())
		var end = start + project_ray_normal(mpos) * 2000
		var world = get_world_3d().direct_space_state
		var query = PhysicsRayQueryParameters3D.create(start, end)
		var result = world.intersect_ray(query)
		
		for dot in dots.get_children():
			dot.visible = false
		var force = ball.global_position - result.position
		force.y = 0
		force *= 2
		ball.launch(force)

	if Input.is_action_just_pressed("ball_1"):
		print('ball')
		
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):	
	pass
