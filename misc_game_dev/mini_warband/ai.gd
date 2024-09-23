extends Node2D

@export var ai_name: String

var army = []

func _ready():
	$Menu.hide()
	$Label.text = ai_name
	$Menu/M/M/V/H/Label.text = ai_name
	army.append(Soldier.new())
	army.append(Soldier.new())
	army.append(Soldier.new())

func show_menu():
	$Menu.show()
