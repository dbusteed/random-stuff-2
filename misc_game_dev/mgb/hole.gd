extends Node2D


var eps = 3
var moving = {}
var stopped = {}
var players = {}
var contested = false

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

func _process(_delta):
	if name == 'Hole':
		pass
#		print()
		print(multiplayer.get_unique_id())
		print(moving)
		print(stopped)
		print(contested)
	
	for ball in moving.values():
		ball.slowdown()
		if ball.state != ball.INACTIVE and ball.linear_velocity.abs().length() <= eps:
			ball.linear_velocity = Vector2.ZERO
			ball.cooldown()
			stopped[ball.id] = {'player_id': ball.id.split('_')[0], 'ball': ball}
			moving.erase(ball.id)
			
	players = {}
	for ball_id in stopped.keys():
		players[ball_id.split('_')[0]] = 0

	if players.size() == 0:
		contested = false
	
	elif players.size() == 1:
		var player_id = int(players.keys()[0])
		if game.holes[name]:
			game.players[game.holes[name]].holes.erase(name)
		game.holes[name] = player_id
		game.players[player_id].holes[name] = position
		$Sprite2D.modulate = game.players[player_id]['color']
		$Sprite2D2.modulate = Color(game.players[player_id]['color'], .2)
		contested = false
		
	else:
		contested = true


func _on_area_2d_area_entered(area):
	var body = area.get_parent()
	if body.state != body.INACTIVE:
		moving[body.id] = body

func _on_area_2d_area_exited(area):
	var body = area.get_parent()
	if body.state != body.INACTIVE:
		moving.erase(body.id)
		stopped.erase(body.id)
