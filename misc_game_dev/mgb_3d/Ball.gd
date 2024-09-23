extends RigidBody3D


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass


func _on_input_event(camera, event, position, normal, shape_idx):
	var mouse_click = event as InputEventMouseButton
	if mouse_click and mouse_click.button_index == 1 and mouse_click.pressed:
		print("clicked")
		var cam: Camera3D = get_tree().get_root().get_node("World/Camera3D")		
		cam.look_at_from_position(global_position + Vector3(-2, 1, 0), global_position)
