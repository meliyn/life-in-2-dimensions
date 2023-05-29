extends Control


func _ready():
	Globals.set_second_dimension.connect(func(second_dimension): visible = second_dimension)
