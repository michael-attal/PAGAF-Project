extends Node2D

@export var boid_entity : PackedScene
@export var count : int

@export var separation_radius : float = 1
@export var heading_radius : float = 1
@export var cohesion_radius : float = 1

@export var separation_factor : float = 1
@export var heading_factor : float = 1
@export var cohesion_factor : float = 1

var birbs : Array

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	for i in range(count):
		var newBirb = boid_entity.instantiate()
		(newBirb as Node2D).position += Vector2(randf() * 500, randf() * 500)
		add_child(newBirb)
		birbs.append(newBirb)

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	for i in birbs:
		var birb := i as SimpleBoid
		
		var v1 : Vector2
		var v2 : Vector2
		var v3 : Vector2
		for j in birbs:
			if i == j:
				continue
			
			var otherBirb := j as SimpleBoid
			var dist = birb.position.distance_to(otherBirb.position)
			if dist < separation_radius:
				v1 += (otherBirb.position - birb.position) * separation_factor
			elif dist < heading_radius:
				v2 += otherBirb.direction * heading_factor
			elif dist < cohesion_radius:
				v3 += (birb.position - otherBirb.position) * cohesion_factor
		
		v1 /= birbs.size()
		v2 /= birbs.size()
		v3 /= birbs.size()
		
		birb.direction += v1 + v2 + v3
	pass
