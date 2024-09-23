extends Node2D

# The URL we will connect to
@export 
var websocket_url = "ws://localhost:9001"

# Our WebSocketClient instance
var socket = WebSocketPeer.new()
var connected = false
var socket_id = ''

#
# create game
# join game
# 

func _ready():
	socket.connect_to_url(websocket_url)

func _process(_delta):
	socket.poll()
	
	match socket.get_ready_state():
		socket.STATE_OPEN:			
			if socket.get_available_packet_count():
				var data = JSON.parse_string(socket.get_packet().get_string_from_utf8())
				print(data)
				var type = data["type"]
				var value = data["value"]
				
				match type:
					"WHOAMI":
						socket_id = value
					
					"GAME_LIST":
						$VBoxContainer/ItemList.clear()
						for game in value:
							$VBoxContainer/ItemList.add_item(game)
					
					"UPDATE_LOBBY":
						$VBoxContainer/ItemList2.clear()
						for peer in value:
							$VBoxContainer/ItemList2.add_item(peer)
							
					"GAME_UPDATE":
						print(value)
				


func _on_button_pressed():
	var data = {"type": "CREATE", "value": ""}
	socket.send_text(JSON.stringify(data))
	
	# TODO go to a lobby screen
	# then someone starts the game


func _on_button_2_pressed():
	var idx = $VBoxContainer/ItemList.get_selected_items()[0]
	var data = {"type": "JOIN", "value": $VBoxContainer/ItemList.get_item_text(idx)}
	socket.send_text(JSON.stringify(data))


func _on_button_3_pressed():
	socket.send_text(JSON.stringify({
		"type": "START_GAME",
		"value": ""
	}))
	
	$VBoxContainer.hide()
	var level = load("res://level.tscn").instantiate()
	get_tree().get_root().get_node("Main").add_child(level)
	
