extends Node3D
## S380 — **la pluie dans l'air** (ADR-205, pièce 1), pour la mer et la piscine : deux classes de gouttes, chacune un
## `GPUParticles3D` dont le nuanceur (`pluie_air.gdshader`) place chaque goutte par hachage de son indice, et dont le dessin
## (`goutte.gdshader`) est la traînée de Garg et Nayar. Le **nombre** de gouttes est celui de Marshall et Palmer dans le
## volume de la boîte de la classe (`pluie.gd`, `densite_gouttes`) ; les grosses gouttes, plus rares et visibles de plus
## loin, ont une boîte plus grande. Sous 1 mm, une goutte ne laisse pas de traînée visible à plus d'un mètre ou deux : elle
## n'est que dans l'extinction (pièce 2). Temps sec : aucune particule, l'image d'avant au bit.

const Pluie = preload("res://pluie.gd")
## Les classes : diamètres [d0, d1) (mm), demi-côté horizontal de la boîte (m), hauteur (m).
const CLASSES := [[1.0, 2.0, 4.0, 6.0], [2.0, 6.0, 8.0, 10.0]]

var systemes: Array[GPUParticles3D] = []
var materiaux: Array[ShaderMaterial] = []
var dessins: Array[ShaderMaterial] = []
var classes_actives: Array = []
var r_mm_h := 0.0
## Le sol le plus bas (m, Godot) ; les eaux des bacs : rectangles (x0, z0, x1, z1) et niveaux.
var plancher := 0.0
var nappes: Array = []
var niveaux: Array = []
var controle := false
var tranche := Vector2(0.0, 0.5)
var taille_controle := 0.02
## S381 — le ciel couvert (`ciel.gdshaderinc`), pour la radiance des gouttes.
var couvert := 0.0


## Le nombre de gouttes d'une classe à l'intensité `r` : densité × volume de sa boîte.
static func nombre(r: float, c: Array) -> int:
	return int(round(Pluie.densite_gouttes(r, c[0], c[1]) * (2.0 * c[2]) * (2.0 * c[2]) * c[3]))


## Refait les systèmes pour l'intensité `r` (le nombre d'une particule ne change pas en vol : on les recrée).
func configurer(r: float) -> void:
	if is_equal_approx(r, r_mm_h) and (r <= 0.0 or not systemes.is_empty()):
		return
	r_mm_h = r
	for s in systemes:
		s.free()
	systemes.clear()
	materiaux.clear()
	dessins.clear()
	classes_actives.clear()
	if r <= 0.0:
		return
	for c in CLASSES:
		var n := nombre(r, c)
		if n <= 0:
			continue
		var gp := GPUParticles3D.new()
		gp.amount = n
		gp.lifetime = 100000.0
		gp.explosiveness = 1.0
		gp.local_coords = false
		gp.visibility_aabb = AABB(Vector3(-200, -200, -200), Vector3(400, 400, 400))
		gp.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
		var m := ShaderMaterial.new()
		m.shader = load("res://pluie_air.gdshader")
		m.set_shader_parameter("boite", Vector3(c[2], c[3], c[2]))
		m.set_shader_parameter("lambda_mp", Pluie.lambda_mp(r))
		m.set_shader_parameter("d0", c[0])
		m.set_shader_parameter("d1", c[1])
		gp.process_material = m
		var q := QuadMesh.new()
		q.size = Vector2(1.0, 1.0)
		var dm := ShaderMaterial.new()
		dm.shader = load("res://goutte.gdshader")
		q.material = dm
		gp.draw_pass_1 = q
		add_child(gp)
		systemes.append(gp)
		materiaux.append(m)
		dessins.append(dm)
		classes_actives.append(c)


## Suit la caméra au temps `t` (s) : la boîte de chaque classe devant elle (les trois quarts de son demi-côté), assez basse
## pour toucher le sol ; les niveaux d'eau du moment.
func suivre(camera: Camera3D, t: float) -> void:
	var cam := camera.global_position
	var avant := -camera.global_transform.basis.z
	var horizontal := Vector3(avant.x, 0.0, avant.z)
	horizontal = horizontal.normalized() if horizontal.length() > 1e-4 else Vector3.ZERO
	var angle := deg_to_rad(camera.fov) / float(camera.get_viewport().get_visible_rect().size.y)
	for k in systemes.size():
		var c: Array = classes_actives[k]
		var centre := cam + horizontal * 0.75 * float(c[2])
		centre.y = maxf(plancher + 0.5 * float(c[3]), cam.y - 0.25 * float(c[3]))
		systemes[k].global_position = centre
		var m := materiaux[k]
		m.set_shader_parameter("centre", centre)
		m.set_shader_parameter("camera", cam)
		m.set_shader_parameter("temps", t)
		m.set_shader_parameter("angle_pixel", angle)
		m.set_shader_parameter("plancher", plancher)
		m.set_shader_parameter("n_nappes", nappes.size())
		var rects := []
		var hauts := []
		for i in 4:
			rects.append(nappes[i] if i < nappes.size() else Vector4.ZERO)
			hauts.append(niveaux[i] if i < niveaux.size() else 0.0)
		m.set_shader_parameter("nappes", rects)
		m.set_shader_parameter("niveaux", hauts)
		m.set_shader_parameter("controle", controle)
		m.set_shader_parameter("tranche", tranche)
		m.set_shader_parameter("taille_controle", taille_controle)
		dessins[k].set_shader_parameter("controle", controle)
		dessins[k].set_shader_parameter("couvert", couvert)
