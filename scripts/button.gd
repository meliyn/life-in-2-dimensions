extends TextureButton

@onready var clicks: Label = $Clicks


func _process(_delta):
	visible = not Globals.in_second_dimension


func _pressed():
	Globals.clicks += 1
	clicks.text = str(Globals.clicks)
	if Globals.clicks >= Globals.goal:
		Globals.times += 1
