extends Node

func find_surrounding_tiles(start: Vector2i, dist: float) -> Array:
	var dist2 = dist ** 2
	var tiles = [start]
	var open = {}
	var closed = {}
	var neighbors = [
		Vector2i(1, 0),
		Vector2i(0, 1),
		Vector2i(-1, 0), 
		Vector2i(0, -1)
	]
	
	for t in tiles:
		open[t] = null
		
	while len(open) > 0:
		var t = open.keys()[0]		
		for n in neighbors:
			var tt = n + t
			if !open.has(tt) and !closed.has(tt):
				if (tt - start).abs().length_squared() < dist2:
					tiles.append(tt)
					open[tt] = null
				else:
					closed[tt] = null
		open.erase(t)
		closed[t] = null
		
	return tiles
