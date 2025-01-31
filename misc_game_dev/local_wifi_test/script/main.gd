extends Control

const DEF_PORT = 8080
const PROTO_NAME = "ludus"

@onready var _host_btn = $Panel/VBoxContainer/HBoxContainer2/HBoxContainer/Host
@onready var _connect_btn = $Panel/VBoxContainer/HBoxContainer2/HBoxContainer/Connect
@onready var _disconnect_btn = $Panel/VBoxContainer/HBoxContainer2/HBoxContainer/Disconnect
@onready var _name_edit = $Panel/VBoxContainer/HBoxContainer/NameEdit
@onready var _host_edit = $Panel/VBoxContainer/HBoxContainer2/Hostname
@onready var _game = $Panel/VBoxContainer/Game

#var peer = WebSocketMultiplayerPeer.new()
var peer = ENetMultiplayerPeer.new()
var upnp
var socket 

func _init():
	pass
	#peer.supported_protocols = ["ludus"]


func _ready():
	socket = PacketPeerUDP.new()
	#udp_server.listen(8092)
	print(IP.get_local_interfaces())
	multiplayer.peer_connected.connect(self._peer_connected)
	multiplayer.peer_disconnected.connect(self._peer_disconnected)
	multiplayer.server_disconnected.connect(self._close_network)
	multiplayer.connection_failed.connect(self._close_network)
	multiplayer.connected_to_server.connect(self._connected)

	$AcceptDialog.get_label().horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	$AcceptDialog.get_label().vertical_alignment = VERTICAL_ALIGNMENT_CENTER
	# Set the player name according to the system username. Fallback to the path.
	if OS.has_environment("USERNAME"):
		_name_edit.text = OS.get_environment("USERNAME")
	else:
		var desktop_path = OS.get_system_dir(0).replace("\\", "/").split("/")
		_name_edit.text = desktop_path[desktop_path.size() - 2]


func start_game():
	_host_btn.disabled = true
	_name_edit.editable = false
	_host_edit.editable = false
	_connect_btn.hide()
	_disconnect_btn.show()
	_game.start()


func stop_game():
	_host_btn.disabled = false
	_name_edit.editable = true
	_host_edit.editable = true
	_disconnect_btn.hide()
	_connect_btn.show()
	_game.stop()


func _close_network():
	stop_game()
	$AcceptDialog.popup_centered()
	$AcceptDialog.get_ok_button().grab_focus()
	multiplayer.multiplayer_peer = null
	peer.close()


func _connected():
	_game.set_player_name.rpc(_name_edit.text)


func _peer_connected(id):
	_game.on_peer_add(id)


func _peer_disconnected(id):
	print("Disconnected %d" % id)
	_game.on_peer_del(id)


func _on_Host_pressed():
	multiplayer.multiplayer_peer = null
	peer.create_server(DEF_PORT)
	multiplayer.multiplayer_peer = peer
	_game.add_player(1, _name_edit.text)
	start_game()


func _on_Disconnect_pressed():
	_close_network()


func _on_Connect_pressed():
	multiplayer.multiplayer_peer = null
	#peer.create_client("ws://" + _host_edit.text + ":" + str(DEF_PORT))
	peer.create_client(_host_edit.text, DEF_PORT)
	multiplayer.multiplayer_peer = peer
	start_game()


func _on_button_pressed():
	socket.set_broadcast_enabled(true)
	socket.set_dest_address("192.168.255.255", 9999)
	print('yas')


func _on_button_2_pressed():
	#socket.bind(9999, "127.0.0.1")
	socket.join_multicast_group("192.168.255.255", "wlan0")
	print('yoos')
