extends Node3D
## S383 — **les gerbes de la pluie** (ADR-205, pièce 4) : un `GPUParticles3D` dont chaque particule est une couche, une
## période et une maille des rides (`gerbe.gdshader`, le tirage de `pluie.gdshaderinc`) sur une fenêtre qui couvre les eaux,
## et dont le dessin est la gerbe mesurée (`gerbe_dessin.gdshader`). Le nombre de particules : couches × 2 périodes ×
## mailles de la fenêtre ; il ne change qu'avec l'intensité. `GERBES=0` : aucune (l'image de la pluie de S382, au bit).
## Temps sec : aucune particule.

const Pluie = preload("res://pluie.gd")
const MAILLE := 0.38
const VIE := 0.6
const COUCHES_MAX := 256

var systeme: GPUParticles3D
var materiau: ShaderMaterial
var dessin: ShaderMaterial
var r_mm_h := -1.0
## La fenêtre (x0, z0, x1, z1, monde de Godot) ; les eaux : rectangles (x0, z0, x1, z1) et niveaux.
var fenetre := Vector4.ZERO
var nappes: Array = []
var niveaux: Array = []
var couvert := 0.0
var controle := false
var taille_controle := 0.02
var vie_controle := 0.05
var actif := OS.get_environment("GERBES") != "0"


## Le nombre de particules à l'intensité `r`.
func nombre(r: float) -> int:
	var couches := Pluie.taux_anneaux(r) * MAILLE * MAILLE * VIE
	var nc := mini(int(ceil(couches)), COUCHES_MAX)
	return nc * 2 * mailles().x * mailles().y


func mailles() -> Vector2i:
	return Vector2i(int(ceil((fenetre.z - fenetre.x) / MAILLE)) + 1, int(ceil((fenetre.w - fenetre.y) / MAILLE)) + 1)


## Refait le système pour l'intensité `r` (le nombre de particules en dépend).
func configurer(r: float) -> void:
	if is_equal_approx(r, r_mm_h):
		return
	r_mm_h = r
	if systeme != null:
		systeme.free()
		systeme = null
	if r <= 0.0 or not actif:
		return
	systeme = GPUParticles3D.new()
	systeme.amount = nombre(r)
	systeme.lifetime = 100000.0
	systeme.explosiveness = 1.0
	systeme.local_coords = false
	systeme.visibility_aabb = AABB(Vector3(-500, -500, -500), Vector3(1000, 1000, 1000))
	systeme.cast_shadow = GeometryInstance3D.SHADOW_CASTING_SETTING_OFF
	materiau = ShaderMaterial.new()
	materiau.shader = load("res://gerbe.gdshader")
	materiau.set_shader_parameter("fenetre", fenetre)
	materiau.set_shader_parameter("n_mailles", mailles())
	systeme.process_material = materiau
	var q := QuadMesh.new()
	q.size = Vector2(1.0, 1.0)
	dessin = ShaderMaterial.new()
	dessin.shader = load("res://gerbe_dessin.gdshader")
	# Après l'eau des bacs, qui lit l'écran et le recouvrirait (tri des objets transparents par la distance de leur boîte).
	dessin.render_priority = 1
	q.material = dessin
	systeme.draw_pass_1 = q
	add_child(systeme)


## L'état du temps `t` (s), vu de `camera`.
func suivre(camera: Camera3D, t: float) -> void:
	if systeme == null:
		return
	materiau.set_shader_parameter("pluie", Pluie.uniformes(r_mm_h))
	materiau.set_shader_parameter("temps", t)
	materiau.set_shader_parameter("camera", camera.global_position)
	materiau.set_shader_parameter("angle_pixel", deg_to_rad(camera.fov) / float(camera.get_viewport().get_visible_rect().size.y))
	var rects := []
	var hauts := []
	for i in 4:
		rects.append(nappes[i] if i < nappes.size() else Vector4.ZERO)
		hauts.append(niveaux[i] if i < niveaux.size() else 0.0)
	materiau.set_shader_parameter("nappes", rects)
	materiau.set_shader_parameter("niveaux", hauts)
	materiau.set_shader_parameter("n_nappes", nappes.size())
	materiau.set_shader_parameter("controle", controle)
	materiau.set_shader_parameter("taille_controle", taille_controle)
	materiau.set_shader_parameter("vie_controle", vie_controle)
	dessin.set_shader_parameter("controle", controle)
	dessin.set_shader_parameter("couvert", couvert)
