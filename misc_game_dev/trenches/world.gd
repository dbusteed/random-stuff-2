extends Node2D

@onready var fog = $Fog
@onready var map = $TileMap

var full_vision = {}
var half_vision = {}

var sight = {}
var known = {}

func _ready():
	var unit = $Unit
	unit.unit_moved.connect(update_fog)
	
	var tiles = Helpers.find_surrounding_tiles(Vector2i(2, 2), 9)
	sight[name] = tiles
	for t in tiles:
		fog.set_cell(0, t, 0)
		known[t] = null

func update_fog(n, s):
	sight[n] = s.keys()
	known.merge(s)
	
	for t in known.keys():
		fog.set_cell(0, t, 0, Vector2(2, 0))	
	
	var all_sight = []
	for unit in sight.keys():
		all_sight += sight[unit]
		for t in sight[unit]:
			fog.set_cell(0, t, 0)

	for e in get_tree().get_nodes_in_group("enemy"):
		var pos = map.local_to_map(e.global_position)
		if pos in all_sight:
			e.visible = true
		else:
			e.visible = false
