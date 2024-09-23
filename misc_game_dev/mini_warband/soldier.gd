class_name Soldier 

var level: int

func _init(lvl: int = 0):
	level = lvl
	
func get_level():
	print(level)

func attack():
	return randi_range(1, 20) + (level * 2)

func _to_string():
	return 'Soldier ' + str(level)
