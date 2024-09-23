extends Camera3D

@onready var ball = $"../Ball"

# Called when the node enters the scene tree for the first time.
func _ready():
	pass


# Called every frame. 'delta' is the elapsed time since the previous frame.

func _process(delta):
	if Input.is_action_just_pressed("lmb"):
		var mpos = get_viewport().get_mouse_position()
		var start = project_ray_origin(get_viewport().get_mouse_position())
		var end = start + project_ray_normal(mpos) * 2000
		var world = get_world_3d().direct_space_state
		var query = PhysicsRayQueryParameters3D.create(start, end)
		var result = world.intersect_ray(query)
		if result:
			var force = ball.global_position - result.position
			force.y = 0
#			force *= 125.5
			ball.launch(force)

	if Input.is_action_pressed("rmb"):
		ball.stop()
