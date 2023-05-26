extends Label

const SPEED: float = 10
var elasped: float = 0


func _process(_delta):
	match Globals.times:
		0:
			text = "This is your life in 2 dimensions...\n(Your job is to click the button)\n(Click to 50 times to proceed)"
		1:
			Globals.goal = 100
			text = "You do not need to eat or drink in this dimension\n(Click to 100 times to proceed)"
		2:
			Globals.goal = 150
			text = "Press G to go to the other dimension"
		_:
			Globals.goal = Globals.times ** 3
			text = "Goal: %s" % Globals.goal
