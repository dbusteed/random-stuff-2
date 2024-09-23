extends Node2D

var upnp
@onready var text = $TextEdit

func discover():
	var res = upnp.discover(500)
	text.text += str(res) + '\n'
	var addr = upnp.query_external_address()
	text.text += addr + '\n'

func _ready():
	text.text += 'READY\n'
	upnp = UPNP.new()
	#var thread = Thread.new()
	#thread.start(discover)

func _process(_delta):
	pass

func _on_button_pressed():
	discover()
