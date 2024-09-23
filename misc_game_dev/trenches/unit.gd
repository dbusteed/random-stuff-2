extends CharacterBody2D

@onready var movement_tween: Node = $MovementTween
@export var grid_size: int = 16

var direction: Vector2 = Vector2.ZERO
var can_move: bool = true
var can_shoot: bool = true
var targets: Dictionary = {}
var target = null
var shift = false

signal unit_moved

func _process(delta: float) -> void:

	if Input.is_action_pressed("up") || Input.is_action_pressed("down"):
		direction = Vector2(0, -int(Input.is_action_pressed("up")) + int(Input.is_action_pressed("down")))
	elif Input.is_action_pressed("left") || Input.is_action_pressed("right"):
		direction = Vector2(-int(Input.is_action_pressed("left")) + int(Input.is_action_pressed("right")), 0)
	else:
		direction = Vector2.ZERO

	if Input.is_key_pressed(KEY_SHIFT):
		shift = true
	else:
		shift = false

	if target and can_shoot:
		can_shoot = false
		if randf() > 0.7:
			print('shot and hit!')
			target.health -= 1
		else:
			print('shot and missed!')
		if target.health <= 0:
			target.queue_free()
		$Timer.start(1.0)

func _physics_process(delta: float) -> void:
	if can_move and direction != Vector2.ZERO:
		
		var map = get_tree().root.get_node("World/TileMap")
		var end = global_position + direction * grid_size
		var til = map.get_cell_atlas_coords(0, map.local_to_map(end))
		#if til == Vector2i(0, 0):
		can_move = false
		movement_tween.run(self, global_position + direction * grid_size, 0.3)
		movement_tween.tween.finished.connect(on_movement_tween_finished)
		#if shift:
			#if til == Vector2i(1, 0):
				#can_move = false
				#movement_tween.run(self, global_position + direction * grid_size, 0.9)
				#movement_tween.tween.finished.connect(on_movement_tween_finished)
			

func on_movement_tween_finished() -> void:
	can_move = true
	var map = get_tree().root.get_node("World/TileMap")
	var pos2 = map.local_to_map(global_position)
	var sight = {}
	for t in Helpers.find_surrounding_tiles(pos2, 6):
		sight[t] = null
	unit_moved.emit(name, sight)


func _on_timer_timeout():
	can_shoot = true


func _on_area_2d_body_entered(body):
	target = body

func _on_area_2d_body_exited(body):
	target = null
