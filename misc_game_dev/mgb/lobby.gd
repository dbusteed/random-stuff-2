extends Control

func _ready():
	# Called every time the node is added to the scene.
	game.connection_failed.connect(self._on_connection_failed)
	game.connection_succeeded.connect(self._on_connection_success)
	game.player_list_changed.connect(self.refresh_lobby)
	game.game_ended.connect(self._on_game_ended)
	game.game_error.connect(self._on_game_error)
	# Set the player name according to the system username. Fallback to the path.
	if OS.has_environment("USERNAME"):
		$Connect/Name.text = OS.get_environment("USERNAME")
	else:
		var desktop_path = OS.get_system_dir(0).replace("\\", "/").split("/")
		$Connect/Name.text = desktop_path[desktop_path.size() - 2]


func _on_host_pressed():
	if $Connect/Name.text == "":
		$Connect/ErrorLabel.text = "Invalid name!"
		return

	$Connect.hide()
	$Players.show()
	$Connect/ErrorLabel.text = ""

	var player_name = $Connect/Name.text
	game.host_game(player_name)
	refresh_lobby()


func _on_join_pressed():
	if $Connect/Name.text == "":
		$Connect/ErrorLabel.text = "Invalid name!"
		return

	var ip = $Connect/IPAddress.text
	if not ip.is_valid_ip_address():
		$Connect/ErrorLabel.text = "Invalid IP address!"
		return

	$Connect/ErrorLabel.text = ""
	$Connect/Host.disabled = true
	$Connect/Join.disabled = true

	var player_name = $Connect/Name.text
	game.join_game(ip, player_name)


func _on_connection_success():
	$Connect.hide()
	$Players.show()


func _on_connection_failed():
	$Connect/Host.disabled = false
	$Connect/Join.disabled = false
	$Connect/ErrorLabel.set_text("Connection failed.")


func _on_game_ended():
	show()
	$Connect.show()
	$Players.hide()
	$Connect/Host.disabled = false
	$Connect/Join.disabled = false


func _on_game_error(errtxt):
	$ErrorDialog.dialog_text = errtxt
	$ErrorDialog.popup_centered()
	$Connect/Host.disabled = false
	$Connect/Join.disabled = false


func refresh_lobby():
	var players = game.get_player_list()
	players.sort()
	$Players/List.clear()
#	$Players/List.add_item(game.get_player_name() + " (You)")
	for p in players:
		$Players/List.add_item(p)

	$Players/Start.disabled = not multiplayer.is_server()


func _on_start_pressed():
	game.begin_game()


func _on_find_public_ip_pressed():
	OS.shell_open("https://icanhazip.com/")


#
#func _ready():
#	# Called every time the node is added to the scene.
#	game.connection_failed.connect(self._on_connection_failed)
#	game.connection_succeeded.connect(self._on_connection_success)
#	game.player_list_changed.connect(self.refresh_lobby)
#	game.game_ended.connect(self._on_game_ended)
#	game.game_error.connect(self._on_game_error)
#	# Set the player name according to the system username. Fallback to the path.
#	if OS.has_environment("USERNAME"):
#		$Connect/Name.text = OS.get_environment("USERNAME")
#	else:
#		var desktop_path = OS.get_system_dir(0).replace("\\", "/").split("/")
#		$Connect/Name.text = desktop_path[desktop_path.size() - 2]
#
#
#func _on_host_pressed():
#	if $Connect/Name.text == "":
#		$Connect/ErrorLabel.text = "Invalid name!"
#		return
#
#	$Connect.hide()
#	$Players.show()
#	$Connect/ErrorLabel.text = ""
#
#	var player_name = $Connect/Name.text
#	game.host_game(player_name)
#	refresh_lobby()
#
#
#func _on_join_pressed():
#	if $Connect/Name.text == "":
#		$Connect/ErrorLabel.text = "Invalid name!"
#		return
#
#	var ip = $Connect/IPAddress.text
#	if not ip.is_valid_ip_address():
#		$Connect/ErrorLabel.text = "Invalid IP address!"
#		return
#
#	$Connect/ErrorLabel.text = ""
#	$Connect/Host.disabled = true
#	$Connect/Join.disabled = true
#
#	var player_name = $Connect/Name.text
#	game.join_game(ip, player_name)
#
#
#func _on_connection_success():
#	$Connect.hide()
#	$Players.show()
#
#
#func _on_connection_failed():
#	$Connect/Host.disabled = false
#	$Connect/Join.disabled = false
#	$Connect/ErrorLabel.set_text("Connection failed.")
#
#
#func _on_game_ended():
#	show()
#	$Connect.show()
#	$Players.hide()
#	$Connect/Host.disabled = false
#	$Connect/Join.disabled = false
#
#
#func _on_game_error(errtxt):
#	$ErrorDialog.dialog_text = errtxt
#	$ErrorDialog.popup_centered()
#	$Connect/Host.disabled = false
#	$Connect/Join.disabled = false
#
#
#func refresh_lobby():
#	var players = game.get_player_list()
#	players.sort()
#	$Players/List.clear()
#	$Players/List.add_item(game.get_player_name() + " (You)")
#	for p in players:
#		$Players/List.add_item(p)
#
#	$Players/Start.disabled = not multiplayer.is_server()
#
#
#func _on_start_pressed():
#	game.begin_game()
#
#
#func _on_find_public_ip_pressed():
#	OS.shell_open("https://icanhazip.com/")
