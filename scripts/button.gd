extends TextureButton

@onready var clicks: Label = $Clicks


func _ready():
	button_down.connect(_down)
	button_up.connect(_up)
	Globals.set_second_dimension.connect(func(second_dimension): visible = not second_dimension)


func _down():
	Audio.play(preload("res://audios/button1.wav"), -5)
	if randi() % 100 + 1 < 10:
		Audio.play(preload("res://audios/jim.ogg"), 0)


func _up():
	Audio.play(preload("res://audios/button2.wav"), -5)
	Globals.clicks += 1
	clicks.text = str(Globals.clicks)
	if Globals.clicks >= Globals.goal:
		Globals.set_hp.emit(Globals.hp + 50)
		Globals.set_times.emit(Globals.times + 1)
