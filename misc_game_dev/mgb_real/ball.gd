extends RigidBody2D

@export var id: String
@export var color: Color
@export var player_id: String


@rpc("call_local")
func init(name_, color_, position_):
	add_to_group(str(name_))
	position = position_
	player_id = name_
	id = player_id + '_' + name
	color = color_
	$Sprite2D.modulate = color_


func activate_camera():
	$Camera2D.make_current()


func launch(force):
	apply_central_impulse(force)
