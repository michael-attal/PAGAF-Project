extends Sprite2D
class_name SimpleBoid

@export var speed : float = 1

var direction : Vector2

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	direction = Vector2.RIGHT
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	position += direction * speed * delta;
	pass
