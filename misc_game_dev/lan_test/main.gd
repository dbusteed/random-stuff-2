extends Control

var udp_broadcaster: PacketPeerUDP
var broadcast_port: int = 9111
var broadcast_address: String = '192.168.0.255'
@onready var broadcast_timer: Timer = $BroadcastTimer

var udp_listener: PacketPeerUDP
var listener_port: int = 9112


func _ready():
	udp_listener = PacketPeerUDP.new()
	var ok = udp_listener.bind(listener_port)
	
	if ok == OK:
		print('listen success')
		$Label.text = 'listening!'
	else:
		print('listen fail')
		$Label.text = 'not listening!'


func _process(_delta):
	if udp_listener.get_available_packet_count() > 0:
		print('we have a packet!')


func _on_host_pressed():
	udp_broadcaster = PacketPeerUDP.new()
	udp_broadcaster.set_broadcast_enabled(true)
	udp_broadcaster.set_dest_address(broadcast_address, listener_port)
	
	var ok = udp_broadcaster.bind(broadcast_port)
	
	if ok == OK:
		print("bound to broadcast")
	else:
		print("Failed to bind to broadcast port!")
		
	$BroadcastTimer.start()


func _on_broadcast_timer_timeout():
	print('send data!')
	var data = JSON.stringify({'name': 'hello!'})
	var packet = data.to_utf8_buffer()
	udp_broadcaster.put_packet(packet)
