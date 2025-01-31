extends KinematicBody2D

const PlayerHurtSound = preload("res://Player/PlayerHurtSound.tscn")

export var ACCELERATION = 500
export var MAX_SPEED = 100
export var FRICTION = 500
export var ROLL_SPEED = 100

enum {
	MOVE,
	ROLL,
	ATTACK
}

var state = MOVE
var velocity = Vector2.ZERO
var roll_vector = Vector2.DOWN
var stats = PlayerStats

onready var animation_player = $AnimationPlayer
onready var animation_tree = $AnimationTree
onready var animation_state = animation_tree.get("parameters/playback")
onready var sword_hitbox = $HitBoxPivot/SwordHitBox
onready var hurtbox = $HurtBox
onready var blink_animation = $BlinkAnimationPlayer

func _ready():
	randomize()
	stats.connect("no_health", self, "queue_free")
	animation_tree.active = true
	sword_hitbox.knockback_vector = roll_vector

func _physics_process(delta):
	match state:
		MOVE:
			move_state(delta)
		
		ROLL:
			roll_state(delta)
		
		ATTACK:
			attack_state(delta)

func move_state(delta):
	var input_vector = Vector2.ZERO
	input_vector.x = Input.get_action_strength("ui_right") - Input.get_action_strength("ui_left")
	input_vector.y = Input.get_action_strength("ui_down") - Input.get_action_strength("ui_up")
	input_vector = input_vector.normalized()
	
	if input_vector != Vector2.ZERO:
		roll_vector = input_vector
		sword_hitbox.knockback_vector = input_vector
		animation_tree.set("parameters/Idle/blend_position", input_vector)
		animation_tree.set("parameters/Run/blend_position", input_vector)
		animation_tree.set("parameters/Attack/blend_position", input_vector)
		animation_tree.set("parameters/Roll/blend_position", input_vector)
		animation_state.travel("Run")
		velocity = velocity.move_toward(input_vector * MAX_SPEED, ACCELERATION * delta)
	else:
		animation_state.travel("Idle")
		velocity = velocity.move_toward(Vector2.ZERO, FRICTION * delta)
		
	move()
	
	if Input.is_action_just_pressed("roll"):
		state = ROLL
	
	if Input.is_action_just_pressed("attack"):
		state = ATTACK

func move():
	velocity = move_and_slide(velocity)

func roll_state(delta):
	velocity = roll_vector * ROLL_SPEED
	animation_state.travel("Roll")
	move()
	
func attack_state(delta):
	velocity = Vector2.ZERO
	animation_state.travel("Attack")

func roll_animation_finished():
	velocity *= 0.9
	state = MOVE

func attack_animation_finished():
	state = MOVE

func _on_HurtBox_area_entered(area):
	stats.health -= area.damage
	hurtbox.start_invicibility(0.6)
	hurtbox.create_hit_effect()
	var player_hurt_sound = PlayerHurtSound.instance()
	get_tree().current_scene.add_child(player_hurt_sound)

func _on_HurtBox_invincibility_started():
	blink_animation.play("Start")

func _on_HurtBox_invincibility_ended():
	blink_animation.play("Stop")
