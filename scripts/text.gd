extends Label

const SPEED: float = 10
var elasped: float = 0
var tips: String = ""


func _ready():
	Globals.set_goal.connect(_on_goal_change)


func _on_goal_change():
	print(Globals.times)
	match Globals.times:
		0:
			tips = "This is your life in 2 dimensions...Your job is to click the button"
		1:
			tips = "You do not need to eat or drink in this dimension"
		2:
			tips = "Press G to go to the other dimension"
		_:
			tips = ""


func _process(_delta):
	text = tips + "\nGoal: %s" % Globals.goal
