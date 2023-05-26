extends TextureButton

signal done

@onready var clicks: Label = $Clicks


func _pressed():
	Globals.clicks += 1
	clicks.text = str(Globals.clicks)
	if Globals.clicks >= Globals.goal:
		Globals.times += 1
		done.emit()
