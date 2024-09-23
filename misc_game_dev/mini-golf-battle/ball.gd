extends CharacterBody3D

@export var friction: float = 0.01
var collision

func _physics_process(delta):
	velocity = lerp(velocity, Vector3.ZERO, friction)	
	collision = move_and_collide(velocity)
	if collision:
		var norm = collision.get_normal()
		velocity = velocity.bounce(norm)
		velocity *= Vector3(0.8, 0, 0.8)
#		velocity.y = 0

func launch(force):
	velocity = force
	rotate(Vector3(0, 1, 0), 0.5)
