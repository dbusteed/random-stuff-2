extends RigidBody2D

@export var id: String
@export var color: Color
@export var player_id: String
@export var speed: float
@export var state = READY
var eps = 20

signal ball_lost

enum {
	READY,
	LAUNCH,
	MOVING,
	COOLDOWN,
	INACTIVE,
}


func init(n, color_, active):
	set_multiplayer_authority(str(n).to_int())
	player_id = n
	id = player_id + '_' + name
	color = color_
	$Sprite2D.modulate = color_
	$Sprite2D2.modulate = color_
	if not active:
		deactivate()


func cooldown():
	state = COOLDOWN
	$Sprite2D.modulate.a = 0.5
	$Timer.start(.05)


func _physics_process(_delta):
	if state == INACTIVE: return
	
	speed = linear_velocity.abs().length()
	if speed > 0:
		state = MOVING
	if speed < eps and state == MOVING:
		linear_velocity = Vector2.ZERO
		cooldown()
		
	linear_velocity = linear_velocity.lerp(Vector2.ZERO, 0.0075)

	
func launch(force):
#	state = LAUNCH
	apply_central_impulse(force)


func slowdown():
	linear_velocity = linear_velocity.lerp(Vector2.ZERO, 0.05)
	
	
func activate_cam():
	$Camera2D.make_current()


func _on_timer_timeout():
	$Sprite2D2.scale += Vector2(.01, .01)
	if $Sprite2D2.scale >= Vector2(.25, .25):
		state = READY
		$Timer.stop()
		$Sprite2D.modulate.a = 1
		$Sprite2D2.scale = Vector2.ZERO


func activate(spawn_pos):
	state = READY
	position = spawn_pos
	$Sprite2D.visible = true
	$Area2D.set_deferred("monitoring", true)
	$Area2D.set_deferred("monitorable", true)
	$Area2D2.set_deferred("monitoring", true)	
	$Area2D2.set_deferred("monitorable", true)
	$Area2D2/CollisionShape2D.set_deferred("disabled", false)


func deactivate():
	state = INACTIVE
	$Sprite2D.visible = false
	$Area2D.set_deferred("monitoring", false)
	$Area2D.set_deferred("monitorable", false)
	$Area2D2.set_deferred("monitoring", false)
	$Area2D2.set_deferred("monitorable", false)
	$Area2D2/CollisionShape2D.set_deferred("disabled", true)
	

func zoom_camera(dir):
	var step = 0.10 * dir
	$Camera2D.zoom += Vector2(step, step)


func _on_area_2d_2_body_entered(body):
	if not is_multiplayer_authority(): return
	if player_id == body.player_id: return
	
	var holes = get_tree().get_root().get_node("World/Holes")
	
	if linear_velocity == Vector2.ZERO:
		deactivate()
		ball_lost.emit(name, global_position)
		for hole in holes.get_children():
			if hole.stopped.has(id):
				print('removing!')
				hole.stopped.erase(id)
		
	elif body.linear_velocity == Vector2.ZERO:
#		body.deactivate()
		for hole in holes.get_children():
			if hole.stopped.has(id):
				hole.stopped.erase(id)
