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
	## S359 : plongeante, pour voir le fond de la scène côtière à travers la surface.
	"plongeante": [Vector3(0.0, 12.0, 30.0), -0.75],
}
## S359 — la scène côtière (`--cote`) : un fond de sable sous la mer, pour voir l'eau selon la profondeur. Étendue
## et pas de la grille du fond, m.
const FOND_X := 1200.0
const FOND_Y := [-200.0, 900.0]
const FOND_PAS := 4.0
## L'optique de la colonne (S359), bandes 650, 550 et 450 nm : absorption de l'eau pure `a` (Pope & Fry 1997) et
## rétrodiffusion `b_b` (Morel 1974), celles d'ADR-177 ; cosinus moyen de la lumière descendante `μ̄_d` = 0,8, *à
## calibrer* (Kirk 1994 : 0,7 à 0,9 selon le soleil). `Kd = (a + b_b)/μ̄_d`.
const ABSORPTION := Vector3(0.340, 0.0565, 0.00922)
const RETRODIFFUSION := Vector3(0.00071, 0.00145, 0.00344)
const MU_D := 0.8

var donnees: Dictionary
var materiau: ShaderMaterial
var camera: Camera3D
var mer: MeshInstance3D
var t0 := 12.0
var temps := 12.0
var anime := true
## S360 : la surface fine par FFT (`detail.gd`), si l'export la porte.
var detail: Node


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
	# S359 : `SANS_EAU=1` masque la mer, pour voir le fond seul.
	mer.visible = OS.get_environment("SANS_EAU") != "1"
	if "--cote" in args or "--controle-fond" in args:
		add_child(fond())
	uniformes_fixes()
	phases(temps)
	if donnees.has("detail"):
		detail = load("res://detail.gd").new()
		add_child(detail)
		if detail.charger(donnees["detail"]) and OS.get_environment("DETAIL") != "0":
			detail.calculer(temps)
			# S360 : l'eau lit les deux cascades ; `DETAIL=0` garde la queue de 60 composantes, en témoin.
			materiau.set_shader_parameter("detail_a0", detail.textures[0][0])
			materiau.set_shader_parameter("detail_b0", detail.textures[0][1])
			materiau.set_shader_parameter("detail_a1", detail.textures[1][0])
			materiau.set_shader_parameter("detail_b1", detail.textures[1][1])
			materiau.set_shader_parameter("detail_cotes", Vector2(float(detail.cotes[0]), float(detail.cotes[1])))
			materiau.set_shader_parameter("detail_actif", true)
		elif not "--controle-fft" in args:
			detail = null
	if "--controle-fft" in args:
		anime = false
		controle_fft()
		return
	if "--captures" in args:
		anime = false
		captures()
	if "--controle-fond" in args:
		anime = false
		controle_fond()
	if "--controle-ecume" in args:
		anime = false
		controle_ecume()


## Le ciel : depuis S359, celui de l'afficheur lui-même (`ciel.gdshaderinc` — dégradé du « ciel clair » relevé sur la
## photographie de référence de l'utilisateur, S261 et R14, nuages, soleil), que l'eau reflète ; S357 en approchait les
## couleurs par le ciel procédural de Godot. Le soleil à la direction de la scène ; tonalité AgX, halo, perspective
## aérienne. S357 P3 : le ciel physique par défaut de Godot rendait un ciel gris de crépuscule.
func environnement() -> void:
	var env := Environment.new()
	var ciel := Sky.new()
	# S359 P6 : le ciel clair de l'afficheur (`ciel.gdshader`), celui que l'eau reflète — une seule source.
	var materiau_ciel := ShaderMaterial.new()
	materiau_ciel.shader = load("res://ciel.gdshader")
	ciel.sky_material = materiau_ciel
	env.background_mode = Environment.BG_SKY
	env.sky = ciel
	env.ambient_light_source = Environment.AMBIENT_SOURCE_SKY
	env.reflected_light_source = Environment.REFLECTION_SOURCE_SKY
	env.tonemap_mode = Environment.TONE_MAPPER_AGX
	# S359 : `TONALITE=lineaire` — sans courbe ni halo, comme l'afficheur de `--meilleur` : les rapports de luminance
	# mesurés sur l'image sont alors ceux des radiances rendues.
	var lineaire := OS.get_environment("TONALITE") == "lineaire"
	if lineaire:
		env.tonemap_mode = Environment.TONE_MAPPER_LINEAR
	# S357 P3 : sans reflets à l'écran — en rasant, leurs rayons retombent sur l'eau elle-même et remplacent le ciel
	# clair de l'horizon par sa propre couleur sombre. `REFLETS_ECRAN=1` les rallume, pour comparer.
	env.ssr_enabled = OS.get_environment("REFLETS_ECRAN") == "1"
	env.glow_enabled = not lineaire
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


## La profondeur du fond sous le plan moyen, m, au point `(x, y)` de B : 6 m sous la caméra, 40 m à 400 m devant,
## puis le large (300 m) ; bancs de sable d'un mètre. Jamais sous 5 m, deux fois Hs : le fond ne touche pas les vagues
## — mais **B ne le voit pas** (liste 2.7) : ni réfraction, ni levée, ni déferlement. Démonstration optique seulement.
static func profondeur(x: float, y: float) -> float:
	var d := 6.0 + 0.085 * clampf(y + 30.0, 0.0, 400.0)
	if y > 370.0:
		d = minf(40.0 + 0.8 * (y - 370.0), 300.0)
	d += 0.8 * sin(0.021 * x + 0.3) * sin(0.017 * y) + 0.4 * sin(0.047 * x - 0.031 * y)
	return maxf(d, 5.0)


## Le fond, une grille régulière en coordonnées du monde — il ne suit pas la caméra.
func fond() -> MeshInstance3D:
	var nx := int(2.0 * FOND_X / FOND_PAS) + 1
	var ny := int((float(FOND_Y[1]) - float(FOND_Y[0])) / FOND_PAS) + 1
	var sommets := PackedVector3Array()
	var normales := PackedVector3Array()
	sommets.resize(nx * ny)
	normales.resize(nx * ny)
	for j in ny:
		var y := float(FOND_Y[0]) + FOND_PAS * j
		for i in nx:
			var x := -FOND_X + FOND_PAS * i
			sommets[j * nx + i] = Vector3(x, -profondeur(x, y), -y)
			var dx := (profondeur(x + 0.5, y) - profondeur(x - 0.5, y))
			var dy := (profondeur(x, y + 0.5) - profondeur(x, y - 0.5))
			# Hauteur du fond −d : pente (−∂d/∂x, −∂d/∂y) en B, soit la normale (∂d/∂x, 1, −∂d/∂y) dans Godot.
			normales[j * nx + i] = Vector3(dx, 1.0, -dy).normalized()
	var indices := PackedInt32Array()
	indices.resize(6 * (nx - 1) * (ny - 1))
	var n := 0
	for j in ny - 1:
		for i in nx - 1:
			var a := j * nx + i
			indices[n] = a
			indices[n + 1] = a + nx
			indices[n + 2] = a + 1
			indices[n + 3] = a + 1
			indices[n + 4] = a + nx
			indices[n + 5] = a + nx + 1
			n += 6
	var tableaux := []
	tableaux.resize(Mesh.ARRAY_MAX)
	tableaux[Mesh.ARRAY_VERTEX] = sommets
	tableaux[Mesh.ARRAY_NORMAL] = normales
	tableaux[Mesh.ARRAY_INDEX] = indices
	var m := ArrayMesh.new()
	m.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, tableaux)
	var sol := ShaderMaterial.new()
	sol.shader = load("res://sol.gdshader")
	var instance := MeshInstance3D.new()
	instance.mesh = m
	instance.material_override = sol
	return instance


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
	materiau.set_shader_parameter("ecume_seuils_deferlement", PackedFloat32Array(donnees["ecume_seuils_deferlement"]))
	materiau.set_shader_parameter("ecume_empreinte_min", float(donnees["ecume_empreinte_min_m"]))
	materiau.set_shader_parameter("kd", (ABSORPTION + RETRODIFFUSION) / MU_D)
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
		if detail != null:
			detail.calculer(temps)


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
			KEY_5:
				pose("plongeante")
			KEY_ESCAPE:
				get_tree().quit()


## Les quatre poses à 12 s, figées, capturées par Godot lui-même.
func captures() -> void:
	DirAccess.make_dir_recursive_absolute(ProjectSettings.globalize_path("res://captures"))
	for nom in ["proche", "rasante", "reference", "haute", "plongeante"]:
		pose(nom)
		mer.global_position = Vector3(camera.global_position.x, 0.0, camera.global_position.z)
		for _i in 12:
			await RenderingServer.frame_post_draw
		var image := get_viewport().get_texture().get_image()
		var suffixe := "_cote" if "--cote" in OS.get_cmdline_user_args() else ""
		var chemin := ProjectSettings.globalize_path("res://captures/godot_%s%s_12s.png" % [nom, suffixe])
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


## S359 — **le contrôle de la colonne d'eau**, mer plate, émission seule : lumières, ambiance, reflets, brume, halo
## éteints, tonalité linéaire. Mode 1 : la profondeur verticale du fond que le nuanceur reconstruit depuis le tampon de
## profondeur, relue en cinq pixels contre `profondeur()` au point où le rayon du pixel touche le fond.
func controle_fond() -> void:
	materiau.set_shader_parameter("n_bande", 0)
	materiau.set_shader_parameter("n_queue", 0)
	materiau.set_shader_parameter("detail_actif", false)
	var env: Environment = (get_children().filter(func(c): return c is WorldEnvironment)[0] as WorldEnvironment).environment
	env.tonemap_mode = Environment.TONE_MAPPER_LINEAR
	env.glow_enabled = false
	env.fog_enabled = false
	env.ambient_light_source = Environment.AMBIENT_SOURCE_DISABLED
	env.reflected_light_source = Environment.REFLECTION_SOURCE_DISABLED
	for c in get_children():
		if c is DirectionalLight3D:
			c.light_energy = 0.0
	var echelle := 50.0
	materiau.set_shader_parameter("controle", 1)
	materiau.set_shader_parameter("echelle_controle", echelle)
	camera.position = Vector3(0.0, 15.0, 20.0)
	camera.rotation = Vector3(-0.9, 0.0, 0.0)
	mer.global_position = Vector3(camera.global_position.x, 0.0, camera.global_position.z)
	for _i in 12:
		await RenderingServer.frame_post_draw
	var image := get_viewport().get_texture().get_image()
	var taille := image.get_size()
	var pire := 0.0
	for f in [Vector2(0.5, 0.5), Vector2(0.25, 0.28), Vector2(0.75, 0.28), Vector2(0.25, 0.83), Vector2(0.75, 0.83)]:
		var px := Vector2i(int(f.x * taille.x), int(f.y * taille.y))
		var rendu := image.get_pixel(px.x, px.y).srgb_to_linear().r * echelle
		var attendu := profondeur_au_pixel(Vector2(px))
		var tolere := 0.02 * attendu + 0.05
		pire = maxf(pire, absf(rendu - attendu) / tolere)
		print("CONTROLE_FOND_S359 mode=profondeur pixel=%s rendu_m=%.3f attendu_m=%.3f ecart_m=%.3f tolere_m=%.3f" % [px, rendu, attendu, rendu - attendu, tolere])
	print("CONTROLE_FOND_S359 mode=profondeur pire_sur_tolere=%.3f critere=%s" % [pire, "tenu" if pire <= 1.0 else "manque"])
	# Mode 2 : au nadir, trois profondeurs ; la transmission rendue contre exp(−2·Kd·H), H au centre de l'image.
	materiau.set_shader_parameter("controle", 2)
	var kd := (ABSORPTION + RETRODIFFUSION) / MU_D
	var pire_t := 0.0
	for y_b in [-30.0, 76.0, 252.0]:
		camera.position = Vector3(0.0, 10.0, -y_b)
		camera.rotation = Vector3(-PI / 2.0, 0.0, 0.0)
		mer.global_position = Vector3(camera.global_position.x, 0.0, camera.global_position.z)
		for _i in 12:
			await RenderingServer.frame_post_draw
		image = get_viewport().get_texture().get_image()
		var centre := Vector2i(taille.x / 2, taille.y / 2)
		var c := image.get_pixel(centre.x, centre.y).srgb_to_linear()
		var hc := profondeur_au_pixel(Vector2(centre))
		var attendu := Vector3(exp(-2.0 * kd.x * hc), exp(-2.0 * kd.y * hc), exp(-2.0 * kd.z * hc))
		var ecart := Vector3(c.r, c.g, c.b) - attendu
		pire_t = maxf(pire_t, maxf(absf(ecart.x), maxf(absf(ecart.y), absf(ecart.z))))
		print("CONTROLE_FOND_S359 mode=transmission H_m=%.3f rendu=(%.4f, %.4f, %.4f) attendu=(%.4f, %.4f, %.4f)" % [hc, c.r, c.g, c.b, attendu.x, attendu.y, attendu.z])
	print("CONTROLE_FOND_S359 mode=transmission pire=%.4f critere=%s" % [pire_t, "tenu" if pire_t <= 0.01 else "manque"])
	get_tree().quit()


## La profondeur du fond sous la surface plate, le long du rayon du pixel : où il entre dans l'eau, puis où il touche
## le fond `y = −profondeur(x, −z)`, par pas de 5 cm puis dichotomie.
func profondeur_au_pixel(px: Vector2) -> float:
	var o := camera.project_ray_origin(px)
	var d := camera.project_ray_normal(px)
	var t := -o.y / d.y
	var sous := func(u: float) -> bool:
		var p: Vector3 = o + d * u
		return p.y < -profondeur(p.x, -p.z)
	while not sous.call(t):
		t += 0.05
	var a := t - 0.05
	var b := t
	for _i in 40:
		var m := 0.5 * (a + b)
		if sous.call(m):
			b = m
		else:
			a = m
	var p := o + d * b
	return -p.y


## S360 — **le contrôle de l'écume** : la couverture en sortie directe (mode 3), tonalité linéaire, sans brume ni halo,
## au nadir à 12 et 40 m au-dessus de neuf positions espacées de 200 m, à 12 s. Les masques vont dans `captures/`, que
## `outils/ecume_taches.py` relit : couverture, taches, diamètres.
func controle_ecume() -> void:
	var env: Environment = (get_children().filter(func(c): return c is WorldEnvironment)[0] as WorldEnvironment).environment
	env.tonemap_mode = Environment.TONE_MAPPER_LINEAR
	env.glow_enabled = false
	env.fog_enabled = false
	materiau.set_shader_parameter("controle", 3)
	DirAccess.make_dir_recursive_absolute(ProjectSettings.globalize_path("res://captures"))
	for hauteur in [12.0, 40.0]:
		for i in 9:
			var x := 200.0 * float(i % 3 - 1)
			var y := 200.0 * float(i / 3 - 1)
			camera.position = Vector3(x, hauteur, -y)
			camera.rotation = Vector3(-PI / 2.0, 0.0, 0.0)
			mer.global_position = Vector3(camera.global_position.x, 0.0, camera.global_position.z)
			for _k in 12:
				await RenderingServer.frame_post_draw
			var chemin := ProjectSettings.globalize_path("res://captures/ecume_%d_%d.png" % [int(hauteur), i])
			get_viewport().get_texture().get_image().save_png(chemin)
	print("CONTROLE_ECUME_S360 masques=18 hauteurs=12,40")
	get_tree().quit()


## S360 — **le contrôle de la FFT** : à `t₀ + 3,7 s`, le champ des deux cascades contre la somme directe de leurs
## composantes, et la pente quadratique moyenne contre l'export (`detail.gd`).
func controle_fft() -> void:
	if detail == null:
		push_error("pas de détail exporté")
		get_tree().quit(1)
		return
	var t := t0 + 3.7
	detail.calculer(t)
	for _k in 6:
		await RenderingServer.frame_post_draw
	detail.controler(t, func(tout: bool) -> void: get_tree().quit(0 if tout else 1))
