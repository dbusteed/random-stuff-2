extends Control

var server


func _ready():
	server = get_tree().root.get_node("Server")


func _process(delta):
	pass


func _on_button_pressed():
	var game = load("res://game.tscn").instantiate()
	get_tree().root.add_child.call_deferred(game)
	get_tree().root.get_node("Main").queue_free()	
	
	var proto = $VBoxContainer/HBoxContainer/OptionButton.get_item_text($VBoxContainer/HBoxContainer/OptionButton.selected)
	var hostname = $VBoxContainer/HBoxContainer/Hostname.text
	var port = $VBoxContainer/HBoxContainer/Port.text
	server.peer.create_client(proto + hostname + ":" + port)
	multiplayer.set_multiplayer_peer(server.peer)
