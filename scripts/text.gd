extends Label

const SPEED: float = 10
var elasped: float = 0


func _process(_delta):
	match Globals.times:
		0:
			text = (
				"This is your life in 2 dimensions...\n(Your job is to click the button)\n(Click to %s times to proceed)"
				% Globals.goal
			)
		1:
			text = (
				"You do not need to eat or drink in this dimension\n(Click to %s times to proceed)"
				% Globals.goal
			)
		2:
			text = "Press G to go to the other dimension\nGoal: %s" % Globals.goal
		_:
			text = "Goal: %s" % Globals.goal
