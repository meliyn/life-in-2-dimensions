extends Node

var goal: int = 50
var clicks: int = 0
var times: int = 0
var in_second_dimension: bool = false
var hp: float = 100


func _process(_delta):
	match Globals.times:
		_:
			goal = (Globals.times + 1) * 50
	if Input.is_action_just_pressed("warp"):
		in_second_dimension = not in_second_dimension
		print(in_second_dimension)
	if times >= 2:
		hp -= 0.1
