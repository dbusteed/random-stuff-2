extends RigidBody2D

@export var id: String
@export var color: Color
@export var player_id: String
@export var speed: float
var state = READY
var eps = 20
var item = ""
var sprite_a = 1

signal despawn

enum {
	READY,
	MOVING,
	COOLDOWN
}

@rpc("call_local")
func init(n, color_, p):
#	print('init! ', n, ' ', color)
	add_to_group(str(n))
	position = p
	player_id = n
	id = player_id + '_' + name
	color = color_
	$Sprite2D.modulate = color_
	$Sprite2D2.modulate = color_


func cooldown():
	state = COOLDOWN
	$Sprite2D.modulate.a = 0.5
	$Timer.start(.05)


func _physics_process(_delta):
	speed = linear_velocity.abs().length()
	if speed > 0:
		state = MOVING
	if speed < eps and state == MOVING:
		linear_velocity = Vector2.ZERO
		cooldown()
		
	linear_velocity = linear_velocity.lerp(Vector2.ZERO, 0.0075)

	
func launch(force):
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


func _on_body_entered(body):
	if not is_multiplayer_authority(): return
	if not body.is_in_group('ball'): return
	if player_id == body.player_id: return
			
	if player_id < body.player_id:
		if state != 1:
			get_tree().get_root().get_node("World").test.rpc_id(int(player_id), name, global_position)
			game.remove_ball.rpc_id(1, name)
		elif body.state != 1:
			get_tree().get_root().get_node("World").test.rpc_id(int(body.player_id), body.name, body.global_position)
			game.remove_ball.rpc_id(1, body.name)


func zoom_camera(dir):
	var step = 0.10 * dir
	$Camera2D.zoom += Vector2(step, step)


func _on_area_2d_2_area_entered(area):
	if not is_multiplayer_authority(): return
	game.add_item.rpc_id(1, name, player_id, area.get_parent())


func use_item():
	match item:
		"INVISIBILITY":
			pass
	item = ""	
	# tell the server?
	# who manages the time?


@rpc("call_local")
func set_item(i):
	item = i
	get_tree().get_root().get_node("World").update_hud()
	
