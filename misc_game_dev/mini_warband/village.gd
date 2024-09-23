extends Node2D

@export var village_name: String

var available_recruits: int = 5

func _ready():
	$VillageMenu/M/M/V/Button.pressed.connect(recruit_volunteers)
	$VillageMenu/M/M/V/Button3.pressed.connect(leave)
	
	$VillageMenu.hide()
	$Label.text = village_name
	$VillageMenu/M/M/V/H/Label .text = village_name


func show_menu():
	$VillageMenu.show()


func recruit_volunteers():
	$VillageMenu/M/M/V/Button.disabled = true
	$VillageMenu/M/M/V/RichTextLabel.text = "You recruited 1 volunteer."
	get_tree().get_root().get_node('World/Player').army.append(Soldier.new())


func leave():
	$VillageMenu.hide()
