extends Node3D
## S461 — C11 : **la scène `--v1` rejouée dans Godot** (décision de l'utilisateur, S460 ; C10-SCENES-S454 §10). L'afficheur calcule
## et enregistre (`water-viewer --v1-banc` avec `EXPORT_GODOT=godot/donnees` : `saut.json`, `saut.bin`) ; ce script **rejoue**,
## comme la piscine (S374–S375), et ne recalcule rien (I-01). L'eau du domaine : un rayon par pixel dans le champ `φ`
## (`saut_eau.gdshader`) ; la mer de B au-delà (`saut_mer.gdshader`) ; le joueur, une sphère ; le ciel de la scène (`ciel.gdshader`),
## la tonalité AgX et le halo de Godot.
##
## Lancer : `Godot --path godot res://saut.tscn` — glisser : orbite, molette : distance, Espace : pause, Échap : quitter.
## `-- --captures` : les images aux instants de R38 (`captures/saut_t<t>.png`), puis quitte. `-- --cout` : 600 images sans
## synchronisation verticale, la cadence imprimée, puis quitte.
##
## Axes : ceux de B dans l'export (x à l'est, y au nord, z en haut) ; dans Godot, un point (x, y, z) de B est (x, z, −y).

var entete: Dictionary
var octets: PackedByteArray
var images: Array
var nx := 0
var ny := 0
var nfen := 0
var taille := 0
var texture_phi: ImageTexture3D
var mat_eau: ShaderMaterial
var mat_mer: ShaderMaterial
var joueur: MeshInstance3D
var camera: Camera3D
var t := 0.0
var duree := 0.0
var courante := -1
var en_pause := false
var azimut := 0.0
var elevation := 0.0
var distance := 3.0
var cible := Vector3.ZERO
var glisse := false


static func b_vers_godot(p: Vector3) -> Vector3:
	return Vector3(p.x, p.z, -p.y)


func _ready() -> void:
	var fichier := FileAccess.open("res://donnees/saut.json", FileAccess.READ)
	if fichier == null:
		push_error("donnees/saut.json absent : `water-viewer --v1-banc` avec EXPORT_GODOT=godot/donnees")
		get_tree().quit(1)
		return
	entete = JSON.parse_string(fichier.get_as_text())
	octets = FileAccess.get_file_as_bytes("res://donnees/saut.bin")
	nx = int(entete["nx"])
	ny = int(entete["ny"])
	nfen = int(entete["k1"]) - int(entete["k0"])
	taille = nx * ny * nfen
	images = entete["images"]
	if octets.size() != taille * images.size():
		push_error("saut.bin : taille inattendue")
		get_tree().quit(1)
		return
	duree = float(images[images.size() - 1][0])
	var dx := float(entete["dx"])
	var niveau := float(entete["niveau"])
	environnement()
	# L'eau du domaine : une boîte sur la fenêtre de `φ`, ses faces arrière rendues.
	var boite := MeshInstance3D.new()
	var bm := BoxMesh.new()
	var k0 := float(entete["k0"])
	var k1 := float(entete["k1"])
	bm.size = Vector3(nx * dx, (k1 - k0) * dx, ny * dx)
	boite.mesh = bm
	boite.position = Vector3(0.5 * nx * dx, 0.5 * (k0 + k1) * dx, -0.5 * ny * dx)
	mat_eau = ShaderMaterial.new()
	mat_eau.shader = load("res://saut_eau.gdshader")
	mat_eau.set_shader_parameter("dims", Vector3(nx, ny, int(entete["nz"])))
	mat_eau.set_shader_parameter("fenetre", Vector2(k0, k1))
	mat_eau.set_shader_parameter("dx", dx)
	boite.material_override = mat_eau
	add_child(boite)
	# La mer de B au-delà : un grand plan au niveau moyen.
	var mer := MeshInstance3D.new()
	var pm := PlaneMesh.new()
	pm.size = Vector2(4000.0, 4000.0)
	mer.mesh = pm
	mer.position = Vector3(0.5 * nx * dx, niveau, -0.5 * ny * dx)
	mat_mer = ShaderMaterial.new()
	mat_mer.shader = load("res://saut_mer.gdshader")
	mat_mer.set_shader_parameter("debut", 0.5 * dx)
	mer.material_override = mat_mer
	add_child(mer)
	var h: Array = entete["houle"]
	for m in [mat_eau, mat_mer]:
		m.set_shader_parameter("houle", Vector4(float(h[0]), float(h[1]), float(h[2]), float(h[3])))
		m.set_shader_parameter("niveau", niveau)
		m.set_shader_parameter("etendue", Vector2((nx - 0.5) * dx, (ny - 0.5) * dx))
	# Le joueur.
	joueur = MeshInstance3D.new()
	var sm := SphereMesh.new()
	sm.radius = float(entete["rayon"])
	sm.height = 2.0 * sm.radius
	joueur.mesh = sm
	var mj := StandardMaterial3D.new()
	mj.albedo_color = Color(0.42, 0.42, 0.45)
	mj.roughness = 0.6
	joueur.material_override = mj
	add_child(joueur)
	# La caméra de R38 : de côté et d'au-dessus, vers le point d'entrée.
	camera = Camera3D.new()
	camera.fov = 40.0
	camera.far = 4000.0
	add_child(camera)
	var centre_b := Vector3(0.5 * nx * dx, 0.5 * ny * dx, niveau - 0.15)
	cible = b_vers_godot(centre_b)
	var oeil := b_vers_godot(centre_b + Vector3(1.7, -2.1, 1.25))
	var r := oeil - cible
	distance = r.length()
	azimut = atan2(r.x, r.z)
	elevation = asin(r.y / distance)
	placer_camera()
	charger(0)
	var args := OS.get_cmdline_user_args()
	if "--captures" in args:
		captures()
	elif "--cout" in args:
		cout()


func environnement() -> void:
	var env := Environment.new()
	var ciel := Sky.new()
	var materiau_ciel := ShaderMaterial.new()
	materiau_ciel.shader = load("res://ciel.gdshader")
	ciel.sky_material = materiau_ciel
	env.background_mode = Environment.BG_SKY
	env.sky = ciel
	env.ambient_light_source = Environment.AMBIENT_SOURCE_SKY
	env.reflected_light_source = Environment.REFLECTION_SOURCE_SKY
	env.tonemap_mode = Environment.TONE_MAPPER_AGX
	env.glow_enabled = true
	var monde := WorldEnvironment.new()
	monde.environment = env
	add_child(monde)
	# Le soleil de la scène (`SOLEIL_B`).
	var vers_soleil := b_vers_godot(Vector3(-0.4, 0.3, 0.8)).normalized()
	var soleil := DirectionalLight3D.new()
	add_child(soleil)
	soleil.look_at_from_position(Vector3.ZERO, -vers_soleil, Vector3.UP)


func placer_camera() -> void:
	var r := Vector3(sin(azimut) * cos(elevation), sin(elevation), cos(azimut) * cos(elevation)) * distance
	camera.look_at_from_position(cible + r, cible, Vector3.UP)
	var pixel := 2.0 * tan(deg_to_rad(camera.fov) * 0.5) / float(get_viewport().get_visible_rect().size.y)
	if mat_mer != null:
		mat_mer.set_shader_parameter("angle_pixel", pixel)


## L'image `i` de l'enregistrement : la texture de `φ`, l'instant, le corps.
func charger(i: int) -> void:
	if i == courante:
		return
	courante = i
	var couches: Array[Image] = []
	var base := i * taille
	for k in nfen:
		var debut := base + k * nx * ny
		couches.append(Image.create_from_data(nx, ny, false, Image.FORMAT_R8, octets.slice(debut, debut + nx * ny)))
	if texture_phi == null:
		texture_phi = ImageTexture3D.new()
		texture_phi.create(Image.FORMAT_R8, nx, ny, nfen, false, couches)
		mat_eau.set_shader_parameter("phi_tex", texture_phi)
	else:
		texture_phi.update(couches)
	var e: Array = images[i]
	var centre := Vector3(float(e[1]), float(e[2]), float(e[3]))
	joueur.position = b_vers_godot(centre)
	for m in [mat_eau, mat_mer]:
		m.set_shader_parameter("temps", float(e[0]))
		m.set_shader_parameter("corps", Vector4(centre.x, centre.y, centre.z, float(entete["rayon"])))


## L'image la plus proche de l'instant `s`.
func image_a(s: float) -> int:
	var meilleure := 0
	for i in images.size():
		if absf(float(images[i][0]) - s) < absf(float(images[meilleure][0]) - s):
			meilleure = i
	return meilleure


func _process(delta: float) -> void:
	if en_pause or "--captures" in OS.get_cmdline_user_args():
		return
	t += delta
	if t > duree:
		t = 0.0
	# La dernière image dont l'instant est passé (l'enregistrement est à 30 images/s, en ordre).
	var i := 0 if t < float(images[maxi(courante, 0)][0]) else maxi(courante, 0)
	while i + 1 < images.size() and float(images[i + 1][0]) <= t:
		i += 1
	charger(i)


func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT:
			glisse = event.pressed
		elif event.button_index == MOUSE_BUTTON_WHEEL_UP and event.pressed:
			distance = maxf(distance * 0.9, 0.3)
			placer_camera()
		elif event.button_index == MOUSE_BUTTON_WHEEL_DOWN and event.pressed:
			distance = minf(distance * 1.1, 50.0)
			placer_camera()
	elif event is InputEventMouseMotion and glisse:
		azimut -= event.relative.x * 0.005
		elevation = clampf(elevation + event.relative.y * 0.005, -1.4, 1.5)
		placer_camera()
	elif event is InputEventKey and event.pressed and not event.echo:
		if event.keycode == KEY_SPACE:
			en_pause = not en_pause
		elif event.keycode == KEY_ESCAPE:
			get_tree().quit()


func captures() -> void:
	DirAccess.make_dir_recursive_absolute(ProjectSettings.globalize_path("res://captures"))
	for s in [0.30, 0.55, 0.85, 1.60, 5.80, 6.30]:
		charger(image_a(s))
		for _i in 8:
			await RenderingServer.frame_post_draw
		var image := get_viewport().get_texture().get_image()
		var chemin := ProjectSettings.globalize_path("res://captures/saut_t%.2f.png" % s)
		image.save_png(chemin)
		print("SAUT_GODOT_S461 capture t=%.2f image=%d instant=%.3f %s" % [s, courante, float(images[courante][0]), chemin])
	get_tree().quit()


func cout() -> void:
	DisplayServer.window_set_vsync_mode(DisplayServer.VSYNC_DISABLED)
	var durees: Array[float] = []
	var avant := Time.get_ticks_usec()
	for _i in 600:
		await RenderingServer.frame_post_draw
		var maintenant := Time.get_ticks_usec()
		durees.append((maintenant - avant) * 1e-3)
		avant = maintenant
	durees.sort()
	print("SAUT_GODOT_S461 cout images=600 image_ms_mediane=%.2f image_ms_p99=%.2f images_par_s=%.0f" % [
			durees[300], durees[594], 1000.0 / durees[300]])
	get_tree().quit()
