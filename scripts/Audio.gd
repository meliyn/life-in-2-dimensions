extends Node


func play(stream: AudioStream, volume: float):
	var audio := AudioStreamPlayer.new()
	add_child(audio)
	audio.volume_db = volume
	audio.stream = stream
	audio.play()
	audio.finished.connect(func(): remove_child(audio))


func play_loop(stream: AudioStream, volume: float):
	var audio := AudioStreamPlayer.new()
	add_child(audio)
	audio.volume_db = volume
	audio.stream = stream
	audio.play()
	audio.finished.connect(func(): audio.play())
