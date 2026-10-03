extends Node3D
## S461 — C11 : **la scène `--v1` rejouée dans Godot** (décision de l'utilisateur, S460 ; C10-SCENES-S454 §10). L'afficheur calcule
## et enregistre (`water-viewer --v1-banc` avec `EXPORT_GODOT=godot/donnees` : `saut.json`, `saut.bin`) ; ce script **rejoue**,
## comme la piscine (S374–S375), et ne recalcule rien (I-01). L'eau du domaine : un rayon par pixel dans le champ `φ`
## (`saut_eau.gdshader`) ; la mer de B au-delà (`saut_mer.gdshader`) ; le joueur, une capsule debout (S466 ; une sphère avant) ;
## le ciel de la scène (`ciel.gdshader`), la tonalité AgX et le halo de Godot.
##
## Lancer : `Godot --path godot res://saut.tscn` — glisser : orbite, molette : distance, Espace : pause, Échap : quitter ;
## S465 : P, la pluie (0, 2, 10, 50 mm/h ; `PLUIE=<mm/h>`).
## `-- --captures` : les images aux instants de R38 (`captures/saut_t<t>.png`), puis quitte. `-- --cout` : 600 images sans
## synchronisation verticale, la cadence imprimée, puis quitte.
##
## S464 — **le direct** : `-- --direct` se connecte à `water-viewer --v1-direct` (127.0.0.1:47011, `V1_PORT`), qui calcule la scène
## au temps réel et pousse chaque image ; ce script affiche la dernière reçue. Avec `--captures` : trois images du direct
## (`captures/saut_direct_<n>.png`) ; avec `--cout` : la cadence et les images reçues par seconde.
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
## S462 : les caustiques enregistrées (`saut_caustiques.bin`), une carte par image.
var octets_c: PackedByteArray
var texture_c: ImageTexture
## S463 : la surface fine (`detail.gd`, les cascades FFT de S360, depuis `donnees/mer_b.json`).
var detail: Node
var mat_eau: ShaderMaterial
var mat_mer: ShaderMaterial
var joueur: MeshInstance3D
## S467 — l'albédo du joueur, le même au-dessus de l'eau (`joueur.gdshader`) et vu à travers elle (`couleur_corps`).
const ALBEDO_JOUEUR := Vector3(0.22, 0.22, 0.24)
var mat_joueur: ShaderMaterial
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
## S464 — le direct : la connexion, ce qui est reçu et pas encore lu, la taille d'une image du lien, les images reçues.
var direct := false
var lien: StreamPeerTCP
var tampon := PackedByteArray()
var taille_lien := 0
var recues := 0
## S465 — la pluie : l'intensité (mm/h ; `PLUIE=`, touche P), son horloge, les gouttes dans l'air, les gerbes, le ciel.
const Pluie = preload("res://pluie.gd")
var pluie_mm_h := 0.0
var t_pluie := 0.0
var pluie_air: Node3D
var gerbes: Node3D
var materiau_ciel: ShaderMaterial
var monde_env: Environment
## S469 — la scène montée : en direct, `_ready` attend l'en-tête, et `_process` tourne déjà — sans les matériaux (S465 : l'horloge
## de la pluie les appelait nuls).
var prete := false


static func b_vers_godot(p: Vector3) -> Vector3:
	return Vector3(p.x, p.z, -p.y)


func _ready() -> void:
	direct = "--direct" in OS.get_cmdline_user_args()
	if direct:
		if not await connecter():
			get_tree().quit(1)
			return
	else:
		var fichier := FileAccess.open("res://donnees/saut.json", FileAccess.READ)
		if fichier == null:
			push_error("donnees/saut.json absent : `water-viewer --v1-banc` avec EXPORT_GODOT=godot/donnees")
			get_tree().quit(1)
			return
		entete = JSON.parse_string(fichier.get_as_text())
		octets = FileAccess.get_file_as_bytes("res://donnees/saut.bin")
		if FileAccess.file_exists("res://donnees/saut_caustiques.bin"):
			octets_c = FileAccess.get_file_as_bytes("res://donnees/saut_caustiques.bin")
	nx = int(entete["nx"])
	ny = int(entete["ny"])
	nfen = int(entete["k1"]) - int(entete["k0"])
	taille = nx * ny * nfen
	taille_lien = 20 + taille + nx * ny
	if not direct:
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
		m.set_shader_parameter("domaine", Vector2(nx * dx, ny * dx))
		# `CAUSTIQUES=0` : sans elles (l'image de S461).
		m.set_shader_parameter("caustiques", (direct or octets_c.size() == nx * ny * images.size()) and OS.get_environment("CAUSTIQUES") != "0")
	# Le joueur : S466, une capsule debout (`demi_longueur` de l'en-tête ; absente ou nulle : la sphère).
	joueur = MeshInstance3D.new()
	var demi := float(entete.get("demi_longueur", 0.0))
	if demi > 0.0:
		var cm := CapsuleMesh.new()
		cm.radius = float(entete["rayon"])
		cm.height = 2.0 * (cm.radius + demi)
		joueur.mesh = cm
	else:
		var sm := SphereMesh.new()
		sm.radius = float(entete["rayon"])
		sm.height = 2.0 * sm.radius
		joueur.mesh = sm
	for m in [mat_eau, mat_mer]:
		m.set_shader_parameter("corps_demi_longueur", demi)
		m.set_shader_parameter("corps_albedo", ALBEDO_JOUEUR)
	# S467 : éclairé par notre nuanceur, dans les unités de l'eau (comme les parois de la piscine, S374), et non par Godot.
	mat_joueur = ShaderMaterial.new()
	mat_joueur.shader = load("res://joueur.gdshader")
	mat_joueur.set_shader_parameter("albedo", ALBEDO_JOUEUR)
	joueur.material_override = mat_joueur
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
	# S463 — la surface fine : les cascades de la mer de R14 (`mer_b.json`), échelonnées (`FORCE_DETAIL`, 0,5) ; `DETAIL=0` : sans.
	var fm := FileAccess.open("res://donnees/mer_b.json", FileAccess.READ)
	if fm != null and OS.get_environment("DETAIL") != "0":
		var mer_b: Dictionary = JSON.parse_string(fm.get_as_text())
		if mer_b.has("detail"):
			detail = load("res://detail.gd").new()
			add_child(detail)
			if detail.charger(mer_b["detail"]):
				detail.calculer(0.0)
				var force := float(OS.get_environment("FORCE_DETAIL")) if OS.get_environment("FORCE_DETAIL") != "" else 0.5
				for m in [mat_eau, mat_mer]:
					m.set_shader_parameter("detail_a0", detail.textures[0][0])
					m.set_shader_parameter("detail_a1", detail.textures[1][0])
					m.set_shader_parameter("detail_cotes", Vector2(float(detail.cotes[0]), float(detail.cotes[1])))
					m.set_shader_parameter("detail_texels", float(detail.N))
					m.set_shader_parameter("force_detail", force)
					m.set_shader_parameter("detail_actif", true)
				print("SAUT_GODOT_S463 detail mss_cascades=%s force=%.2f mss_ajoutee=%.4f" % [str(detail.mss_realisee), force,
						force * force * (float(detail.mss_realisee[0]) + float(detail.mss_realisee[1]))])
			else:
				detail = null
	# S465 — la pluie : les gouttes s'arrêtent à la mer (une nappe de 400 m au niveau moyen), les gerbes autour de la scène.
	pluie_air = load("res://pluie_air.gd").new()
	add_child(pluie_air)
	pluie_air.plancher = 0.0
	pluie_air.nappes = [Vector4(-200.0, -200.0, 200.0, 200.0)]
	pluie_air.niveaux = [niveau]
	gerbes = load("res://gerbes.gd").new()
	add_child(gerbes)
	gerbes.nappes = pluie_air.nappes
	gerbes.niveaux = pluie_air.niveaux
	gerbes.fenetre = Vector4(-20.0, -20.0, 20.0, 20.0)
	if OS.get_environment("PLUIE") != "":
		pluie_mm_h = float(OS.get_environment("PLUIE"))
	regler_pluie()
	placer_camera()
	if not direct:
		charger(0)
	prete = true
	var args := OS.get_cmdline_user_args()
	if "--captures" in args:
		if direct:
			captures_direct()
		else:
			captures()
	elif "--cout" in args:
		cout()


## S465 — la pluie à `pluie_mm_h` : les uniformes des rides (`pluie.gd`), le ciel couvert, les gouttes, les gerbes, l'extinction.
func regler_pluie() -> void:
	var couvert := 1.0 if pluie_mm_h > 0.0 else 0.0
	if OS.get_environment("COUVERT") != "":
		couvert = float(OS.get_environment("COUVERT"))
	for m in [mat_eau, mat_mer, materiau_ciel]:
		m.set_shader_parameter("pluie", Pluie.uniformes(pluie_mm_h))
		m.set_shader_parameter("couvert", couvert)
	mat_joueur.set_shader_parameter("couvert", couvert)
	pluie_air.couvert = couvert
	pluie_air.configurer(pluie_mm_h)
	gerbes.couvert = couvert
	gerbes.configurer(pluie_mm_h)
	var beta := Pluie.extinction(pluie_mm_h)
	monde_env.fog_enabled = beta > 0.0
	if beta > 0.0:
		monde_env.fog_density = beta
		monde_env.fog_aerial_perspective = 1.0
		monde_env.fog_sky_affect = 0.0
	print("SAUT_GODOT_S465 pluie mm_h=%.1f taux_anneaux_m2_s=%.1f couvert=%.1f extinction_m=%.5f" % [pluie_mm_h,
			Pluie.taux_anneaux(pluie_mm_h), couvert, beta])


## S464 — la connexion au direct : l'en-tête (`FST1`, sa longueur, le JSON).
func connecter() -> bool:
	var port := int(OS.get_environment("V1_PORT")) if OS.get_environment("V1_PORT") != "" else 47011
	lien = StreamPeerTCP.new()
	if lien.connect_to_host("127.0.0.1", port) != OK:
		push_error("direct : connexion impossible à 127.0.0.1:%d" % port)
		return false
	var attente := Time.get_ticks_msec()
	while Time.get_ticks_msec() - attente < 20000:
		lien.poll()
		if lien.get_status() == StreamPeerTCP.STATUS_CONNECTED and lien.get_available_bytes() > 0:
			tampon.append_array(lien.get_data(lien.get_available_bytes())[1])
		if tampon.size() >= 8:
			var n := tampon.decode_u32(4)
			if tampon.slice(0, 4).get_string_from_ascii() != "FST1":
				push_error("direct : en-tête inattendu")
				return false
			if tampon.size() >= 8 + n:
				entete = JSON.parse_string(tampon.slice(8, 8 + n).get_string_from_utf8())
				tampon = tampon.slice(8 + n)
				print("SAUT_GODOT_S464 direct connecté à 127.0.0.1:%d" % port)
				return true
		await get_tree().process_frame
	push_error("direct : pas d'en-tête de `water-viewer --v1-direct` sur 127.0.0.1:%d" % port)
	return false


## S464 — ce qui est arrivé : la dernière image complète appliquée, les plus anciennes sautées.
func recevoir() -> void:
	lien.poll()
	# S469 : l'afficheur parti (`DUREE`), le lien fermé — la dernière image reste, sans erreur à chaque image.
	if lien.get_status() != StreamPeerTCP.STATUS_CONNECTED:
		return
	var n := lien.get_available_bytes()
	if n > 0:
		tampon.append_array(lien.get_data(n)[1])
	var completes := tampon.size() / taille_lien
	if completes == 0:
		return
	var debut := (completes - 1) * taille_lien
	if tampon.slice(debut, debut + 4).get_string_from_ascii() != "IMG1":
		push_error("direct : image désalignée")
		tampon = PackedByteArray()
		return
	var instant := tampon.decode_float(debut + 4)
	var centre := Vector3(tampon.decode_float(debut + 8), tampon.decode_float(debut + 12), tampon.decode_float(debut + 16))
	appliquer(tampon.slice(debut + 20, debut + 20 + taille), tampon.slice(debut + 20 + taille, debut + taille_lien), instant, centre)
	recues += completes
	tampon = tampon.slice(completes * taille_lien)


func environnement() -> void:
	var env := Environment.new()
	var ciel := Sky.new()
	materiau_ciel = ShaderMaterial.new()
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
	monde_env = env
	# Le soleil de la scène (`SOLEIL_B`).
	var vers_soleil := b_vers_godot(Vector3(-0.4, 0.3, 0.8)).normalized()
	var soleil := DirectionalLight3D.new()
	add_child(soleil)
	soleil.look_at_from_position(Vector3.ZERO, -vers_soleil, Vector3.UP)


func placer_camera() -> void:
	var r := Vector3(sin(azimut) * cos(elevation), sin(elevation), cos(azimut) * cos(elevation)) * distance
	camera.look_at_from_position(cible + r, cible, Vector3.UP)
	var pixel := 2.0 * tan(deg_to_rad(camera.fov) * 0.5) / float(get_viewport().get_visible_rect().size.y)
	for m in [mat_eau, mat_mer]:
		if m != null:
			m.set_shader_parameter("angle_pixel", pixel)


## L'image `i` de l'enregistrement : la texture de `φ`, l'instant, le corps.
func charger(i: int) -> void:
	if i == courante:
		return
	courante = i
	var carte := PackedByteArray()
	if octets_c.size() == nx * ny * images.size():
		carte = octets_c.slice(i * nx * ny, (i + 1) * nx * ny)
	var e: Array = images[i]
	appliquer(octets.slice(i * taille, (i + 1) * taille), carte, float(e[0]), Vector3(float(e[1]), float(e[2]), float(e[3])))


## Une image (enregistrée ou reçue) : `φ` sur 8 bits, la carte des caustiques (vide : aucune), l'instant, le centre du corps (B).
func appliquer(phi_octets: PackedByteArray, carte_octets: PackedByteArray, instant: float, centre: Vector3) -> void:
	var couches: Array[Image] = []
	for k in nfen:
		couches.append(Image.create_from_data(nx, ny, false, Image.FORMAT_R8, phi_octets.slice(k * nx * ny, (k + 1) * nx * ny)))
	if texture_phi == null:
		texture_phi = ImageTexture3D.new()
		texture_phi.create(Image.FORMAT_R8, nx, ny, nfen, false, couches)
		mat_eau.set_shader_parameter("phi_tex", texture_phi)
	else:
		texture_phi.update(couches)
	if carte_octets.size() == nx * ny:
		var carte := Image.create_from_data(nx, ny, false, Image.FORMAT_R8, carte_octets)
		if texture_c == null:
			texture_c = ImageTexture.create_from_image(carte)
			for m in [mat_eau, mat_mer]:
				m.set_shader_parameter("caustiques_tex", texture_c)
		else:
			texture_c.update(carte)
	joueur.position = b_vers_godot(centre)
	if detail != null:
		detail.calculer(instant)
	for m in [mat_eau, mat_mer]:
		m.set_shader_parameter("temps", instant)
		m.set_shader_parameter("corps", Vector4(centre.x, centre.y, centre.z, float(entete["rayon"])))


## L'image la plus proche de l'instant `s`.
func image_a(s: float) -> int:
	var meilleure := 0
	for i in images.size():
		if absf(float(images[i][0]) - s) < absf(float(images[meilleure][0]) - s):
			meilleure = i
	return meilleure


func _process(delta: float) -> void:
	if not prete:
		return
	# S465 : la pluie a sa propre horloge (l'enregistrement boucle, la pluie non).
	t_pluie += delta
	for m in [mat_eau, mat_mer]:
		m.set_shader_parameter("temps_pluie", t_pluie)
	if pluie_mm_h > 0.0:
		pluie_air.suivre(camera, t_pluie)
		gerbes.suivre(camera, t_pluie)
	if direct:
		if lien != null and not en_pause:
			recevoir()
		return
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
		elif event.keycode == KEY_P:
			pluie_mm_h = {0.0: 2.0, 2.0: 10.0, 10.0: 50.0}.get(pluie_mm_h, 0.0)
			regler_pluie()
		elif event.keycode == KEY_ESCAPE:
			get_tree().quit()


func captures() -> void:
	DirAccess.make_dir_recursive_absolute(ProjectSettings.globalize_path("res://captures"))
	for s in [0.30, 0.55, 0.85, 1.60, 5.80, 6.30]:
		charger(image_a(s))
		for _i in 8:
			await RenderingServer.frame_post_draw
		var image := get_viewport().get_texture().get_image()
		var suffixe := "_pluie%d" % int(pluie_mm_h) if pluie_mm_h > 0.0 else ""
		var chemin := ProjectSettings.globalize_path("res://captures/saut_t%.2f%s.png" % [s, suffixe])
		image.save_png(chemin)
		print("SAUT_GODOT_S461 capture t=%.2f image=%d instant=%.3f %s" % [s, courante, float(images[courante][0]), chemin])
	get_tree().quit()


## S464 : trois images du direct, à 1,5 s d'intervalle après 2 s.
func captures_direct() -> void:
	DirAccess.make_dir_recursive_absolute(ProjectSettings.globalize_path("res://captures"))
	var depart := Time.get_ticks_msec()
	for n in 3:
		while Time.get_ticks_msec() - depart < 2000 + 1500 * n:
			await get_tree().process_frame
		var image := get_viewport().get_texture().get_image()
		var chemin := ProjectSettings.globalize_path("res://captures/saut_direct_%d.png" % n)
		image.save_png(chemin)
		print("SAUT_GODOT_S464 capture direct %d images_recues=%d %s" % [n, recues, chemin])
	get_tree().quit()


func cout() -> void:
	DisplayServer.window_set_vsync_mode(DisplayServer.VSYNC_DISABLED)
	var durees: Array[float] = []
	var recues_avant := recues
	var debut := Time.get_ticks_usec()
	var avant := Time.get_ticks_usec()
	for _i in 600:
		await RenderingServer.frame_post_draw
		var maintenant := Time.get_ticks_usec()
		durees.append((maintenant - avant) * 1e-3)
		avant = maintenant
	durees.sort()
	print("SAUT_GODOT_S461 cout images=600 image_ms_mediane=%.2f image_ms_p99=%.2f images_par_s=%.0f" % [
			durees[300], durees[594], 1000.0 / durees[300]])
	if direct:
		var secondes := (Time.get_ticks_usec() - debut) * 1e-6
		print("SAUT_GODOT_S464 direct images_recues=%d en %.2f s, soit %.1f par seconde" % [recues - recues_avant, secondes,
				float(recues - recues_avant) / secondes])
	get_tree().quit()
