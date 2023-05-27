extends Label


func _ready():
	Globals.set_hp.connect(func(hp): text = "HP: %s" % round(hp))
