extends Node2D

var army = []

func _ready():
	army.append(Soldier.new(3))
	army.append(Soldier.new(4))
	army.append(Soldier.new(5))

func _process(delta):
	if Input.is_action_just_pressed("up"):
		global_position = global_position + Vector2(0, -24)
	elif Input.is_action_just_pressed("down"):
		global_position = global_position + Vector2(0, 24)
	elif Input.is_action_just_pressed("left"):
		global_position = global_position + Vector2(-24, 0)
	elif Input.is_action_just_pressed("right"):
		global_position = global_position + Vector2(24, 0)

	if Input.is_action_just_pressed("select"):
		for a in $Area2D.get_overlapping_areas():
			a.get_parent().show_menu()
			
	if Input.is_action_just_pressed("ui_right"):
		print(len(army))
		
	if Input.is_action_just_pressed("ui_left"):
		Global.battle(army, [Soldier.new(), Soldier.new()])
			
	#print('army: ', len(army))
