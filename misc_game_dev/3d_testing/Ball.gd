extends RigidBody3D


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func launch(force):
	apply_central_impulse(force)

func stop():
	linear_velocity = linear_velocity.lerp(Vector3.ZERO, 0.05)
		
