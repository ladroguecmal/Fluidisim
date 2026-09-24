extends Node3D
## S357, ADR-192 D2 — la mer de B dans Godot : le prototype.
## Données : `res://donnees/mer_b.json`, exportées par l'afficheur (`--meilleur --export-godot`), dérivées, non versionnées.
## Lancer : `Godot --path godot` — vue animée ; touches 1 à 4 : poses de R14 (référence, proche, rasante, haute) ;
## Échap : quitter. `Godot --path godot -- --captures` : les quatre poses à 12 s, en PNG dans `captures/`, puis quitte.
## `Godot --headless --path godot -- --controle` : la hauteur de la bande recalculée ici contre le cœur, puis quitte.

## La grille polaire autour de la caméra : dense près de l'œil, lâche au loin, sans couture.
const ANGLES := 720
const RAYONS := 360
const R_MIN := 0.25
const R_MAX := 12000.0
## Les poses de R14 : position dans Godot (x, hauteur, −y de B) et tangage (rad).
const POSES := {
	"reference": [Vector3(0.0, 7.0, 18.0), -0.13135],
	"proche": [Vector3(0.0, 4.0, 7.0), -0.18],
	"rasante": [Vector3(0.0, 2.0, 18.0), -0.05],
	"haute": [Vector3(0.0, 22.0, 34.0), -0.42],
}

var donnees: Dictionary
var materiau: ShaderMaterial
var camera: Camera3D
var mer: MeshInstance3D
var t0 := 12.0
var temps := 12.0
var anime := true


func _ready() -> void:
	var fichier := FileAccess.open("res://donnees/mer_b.json", FileAccess.READ)
	if fichier == null:
		push_error("donnees/mer_b.json absent : lancer l'afficheur avec --meilleur --export-godot")
		get_tree().quit(1)
		return
	donnees = JSON.parse_string(fichier.get_as_text())
	t0 = float(donnees["instant_s"])
	temps = t0
	var args := OS.get_cmdline_user_args()
	if "--controle" in args:
		controle()
		get_tree().quit()
		return
	environnement()
	camera = Camera3D.new()
	camera.fov = 50.0
	camera.near = 0.1
	camera.far = 20000.0
	add_child(camera)
	camera.current = true
	pose("proche")
	materiau = ShaderMaterial.new()
	materiau.shader = load("res://eau.gdshader")
	mer = MeshInstance3D.new()
	mer.mesh = grille_polaire()
	mer.material_override = materiau
	mer.custom_aabb = AABB(Vector3(-R_MAX, -100.0, -R_MAX), Vector3(2.0 * R_MAX, 200.0, 2.0 * R_MAX))
	add_child(mer)
	uniformes_fixes()
	phases(temps)
	if "--captures" in args:
		anime = false
		captures()


## Le ciel de Godot aux couleurs du « ciel clair » de l'afficheur, relevées sur la photographie de référence de
## l'utilisateur (S261, R14) — horizon et zénith linéaires `(0,694 ; 0,838 ; 0,930)` et `(0,015 ; 0,150 ; 0,600)`,
## donnés ici en sRGB comme Godot les attend ; le soleil à la direction de la scène ; tonalité AgX, reflets à l'écran,
## halo, perspective aérienne. S357 P3 : le ciel physique par défaut de Godot rendait un ciel gris de crépuscule.
func environnement() -> void:
	var env := Environment.new()
	var ciel := Sky.new()
	var procedural := ProceduralSkyMaterial.new()
	procedural.sky_horizon_color = Color(0.694, 0.838, 0.930).linear_to_srgb()
	procedural.sky_top_color = Color(0.015, 0.150, 0.600).linear_to_srgb()
	# Le demi-ciel du bas n'est jamais vu : il ne sert qu'aux lobes rugueux des reflets rasants, qui voient en réalité
	# l'horizon — pris à sa couleur.
	procedural.ground_horizon_color = procedural.sky_horizon_color
	procedural.ground_bottom_color = procedural.sky_horizon_color
	ciel.sky_material = procedural
	env.background_mode = Environment.BG_SKY
	env.sky = ciel
	env.ambient_light_source = Environment.AMBIENT_SOURCE_SKY
	env.reflected_light_source = Environment.REFLECTION_SOURCE_SKY
	env.tonemap_mode = Environment.TONE_MAPPER_AGX
	# S357 P3 : sans reflets à l'écran — en rasant, leurs rayons retombent sur l'eau elle-même et remplacent le ciel
	# clair de l'horizon par sa propre couleur sombre. `REFLETS_ECRAN=1` les rallume, pour comparer.
	env.ssr_enabled = OS.get_environment("REFLETS_ECRAN") == "1"
	env.glow_enabled = true
	env.fog_enabled = true
	env.fog_density = 0.00012
	env.fog_aerial_perspective = 1.0
	env.fog_sky_affect = 0.0
	var monde := WorldEnvironment.new()
	monde.environment = env
	add_child(monde)
	var d: Array = donnees["soleil"]
	var vers_soleil := Vector3(float(d[0]), float(d[2]), -float(d[1])).normalized()
	var soleil := DirectionalLight3D.new()
	add_child(soleil)
	soleil.look_at_from_position(Vector3.ZERO, -vers_soleil, Vector3.UP)
	soleil.light_energy = 1.0


func pose(nom: String) -> void:
	var p: Array = POSES[nom]
	camera.position = p[0]
	camera.rotation = Vector3(float(p[1]), 0.0, 0.0)


func grille_polaire() -> ArrayMesh:
	var sommets := PackedVector3Array()
	sommets.resize(1 + ANGLES * RAYONS)
	var q := log(R_MAX / R_MIN) / float(RAYONS - 1)
	for j in RAYONS:
		var r := R_MIN * exp(q * j)
		for i in ANGLES:
			var a := TAU * float(i) / float(ANGLES)
			sommets[1 + j * ANGLES + i] = Vector3(r * cos(a), 0.0, r * sin(a))
	var indices := PackedInt32Array()
	indices.resize(3 * ANGLES + 6 * ANGLES * (RAYONS - 1))
	var n := 0
	for i in ANGLES:
		indices[n] = 0
		indices[n + 1] = 1 + (i + 1) % ANGLES
		indices[n + 2] = 1 + i
		n += 3
	for j in RAYONS - 1:
		for i in ANGLES:
			var a := 1 + j * ANGLES + i
			var b := 1 + j * ANGLES + (i + 1) % ANGLES
			indices[n] = a
			indices[n + 1] = b
			indices[n + 2] = a + ANGLES
			indices[n + 3] = b
			indices[n + 4] = b + ANGLES
			indices[n + 5] = a + ANGLES
			n += 6
	var tableaux := []
	tableaux.resize(Mesh.ARRAY_MAX)
	tableaux[Mesh.ARRAY_VERTEX] = sommets
	tableaux[Mesh.ARRAY_INDEX] = indices
	var m := ArrayMesh.new()
	m.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, tableaux)
	return m


func uniformes_fixes() -> void:
	materiau.set_shader_parameter("n_bande", donnees["bande"].size())
	materiau.set_shader_parameter("n_queue", donnees["queue"].size())
	materiau.set_shader_parameter("modulation_M", float(donnees["modulation_M"]))
	materiau.set_shader_parameter("retard_tours", float(donnees["retard_tours"]))
	materiau.set_shader_parameter("split", int(donnees["split"]))
	if donnees["asymetries"]:
		var k: Array = donnees["k_moyens"]
		materiau.set_shader_parameter("k_moyens", Vector2(float(k[0]), float(k[1])))
	materiau.set_shader_parameter("ecume_seuils", PackedFloat32Array(donnees["ecume_seuils"]))
	var hauteur := get_viewport().get_visible_rect().size.y
	materiau.set_shader_parameter("angle_pixel", 2.0 * tan(deg_to_rad(camera.fov) / 2.0) / hauteur)
	materiau.set_shader_parameter("pas_radial", log(R_MAX / R_MIN) / float(RAYONS - 1))


## Les lignes `[a, kx, ky, φ(t)]` : la phase avance de `−ω·(t − t₀)`, repliée ici en double précision (I-08).
func lignes(nom: String, t: float) -> PackedVector4Array:
	var entree: Array = donnees[nom]
	var sortie := PackedVector4Array()
	sortie.resize(entree.size())
	for i in entree.size():
		var r: Array = entree[i]
		var phase := fposmod(float(r[3]) - float(r[4]) * (t - t0) + PI, TAU) - PI
		sortie[i] = Vector4(float(r[0]), float(r[1]), float(r[2]), phase)
	return sortie


func phases(t: float) -> void:
	materiau.set_shader_parameter("bande", lignes("bande", t))
	materiau.set_shader_parameter("queue", lignes("queue", t))


func _process(delta: float) -> void:
	if camera == null:
		return
	mer.global_position = Vector3(camera.global_position.x, 0.0, camera.global_position.z)
	if anime:
		temps += delta
		phases(temps)


func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventKey and event.pressed:
		match event.keycode:
			KEY_1:
				pose("reference")
			KEY_2:
				pose("proche")
			KEY_3:
				pose("rasante")
			KEY_4:
				pose("haute")
			KEY_ESCAPE:
				get_tree().quit()


## Les quatre poses à 12 s, figées, capturées par Godot lui-même.
func captures() -> void:
	DirAccess.make_dir_recursive_absolute(ProjectSettings.globalize_path("res://captures"))
	for nom in ["proche", "rasante", "reference", "haute"]:
		pose(nom)
		mer.global_position = Vector3(camera.global_position.x, 0.0, camera.global_position.z)
		for _i in 12:
			await RenderingServer.frame_post_draw
		var image := get_viewport().get_texture().get_image()
		var chemin := ProjectSettings.globalize_path("res://captures/godot_%s_12s.png" % nom)
		image.save_png(chemin)
		print("CAPTURE_GODOT_S357 pose=%s fichier=%s %dx%d" % [nom, chemin, image.get_width(), image.get_height()])
	get_tree().quit()


## Le contrôle : `η` linéaire de la bande à `t₀ + 3 s`, recalculé ici depuis les lignes exportées, contre le cœur.
func controle() -> void:
	var dt := float(donnees["controle_dt_s"])
	var pire := 0.0
	for point in donnees["controle"]:
		var x := float(point[0])
		var y := float(point[1])
		var eta := 0.0
		for r in donnees["bande"]:
			eta += float(r[0]) * sin(float(r[1]) * x + float(r[2]) * y + float(r[3]) - float(r[4]) * dt)
		var ecart: float = absf(eta - float(point[2]))
		pire = maxf(pire, ecart)
		print("CONTROLE_GODOT_S357 x=%.1f y=%.1f eta_godot=%.9f eta_coeur=%.9f ecart=%s" % [x, y, eta, float(point[2]), String.num_scientific(ecart)])
	print("CONTROLE_GODOT_S357 ecart_max=%s" % String.num_scientific(pire))
