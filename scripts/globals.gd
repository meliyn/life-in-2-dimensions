extends Node

signal set_goal(goal: int)
signal set_times(times: int)
signal set_hp(hp: float)
signal set_second_dimension(yes: bool)
signal set_clicks(clicks: int)

var goal: int = 50
var clicks: int = 0
var times: int = 0
var second_dimension: bool = false
var hp: float = 100


func _init():
	set_clicks.connect(func(v): clicks = v)
	set_goal.connect(func(v): goal = v)
	set_hp.connect(func(v): hp = v)
	set_second_dimension.connect(func(v): second_dimension = v)
	set_times.connect(func(v): times = v)


func _ready():
	Audio.play_loop(preload("res://audios/music/Audio Export/music.wav"), 0)


func _process(delta):
	if clicks >= goal:
		set_goal.emit((times + 1) * 50 + (randi() % (times + 1) * 50 + 0))
	if times >= 2:
		set_hp.emit(hp - clamp(delta * times, 0, 10))
	if hp < 0:
		var node = get_node("/root/Game")
		if node != null:
			get_tree().get_root().remove_child(node)
			get_tree().get_root().add_child(preload("res://scenes/game_over.tscn").instantiate())


func _unhandled_input(event):
	if event.is_action_pressed("warp"):
		set_second_dimension.emit(not second_dimension)
