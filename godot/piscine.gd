extends Node3D
## S374 — **la piscine de V** (demande de l'utilisateur) : un bassin, un déversoir, un bac tampon, une pompe. V calcule
## (`code/water-core/examples/piscine_v.rs`) et publie chaque pas dans `res://donnees/piscine_v.json` — surfaces des bacs,
## débits du déversoir et de la pompe, commande ; ce script le **rejoue**, interpolé à l'image, et ne recalcule rien de V
## (I-01). Premier pas : pas encore de commande en direct (elle demande V dans le processus, par godot-rust, ou un lien).
##
## Lancer : `Godot --path godot res://piscine.tscn` — la vue animée ; touches 1 à 3 : les vues ; espace : pause ; Échap :
## quitter. `-- --captures` : les images de revue (`captures/piscine_<vue>_<t>s.png`), puis quitte. `-- --controle-piscine` :
## la surface rendue de chaque bac contre celle que V publie (critère 3 de S374), puis quitte.
##
## Axes : ceux de B dans l'export (x à l'est, y au nord, z en haut) ; dans Godot, un point (x, y, z) de B est (x, z, −y).

var donnees: Dictionary
var pas: Array
var dt := 0.1
var t := 0.0
var en_pause := false
var camera: Camera3D
var eau_bassin: MeshInstance3D
var eau_tampon: MeshInstance3D
var materiau_bassin: ShaderMaterial
var materiau_tampon: ShaderMaterial
var texte: Label
## S375 — la surface de δ 3D du bassin (ADR-200), si l'export existe (`examples/piscine_delta.rs`) : l'en-tête, les images
## (entiers de 16 bits, dixièmes de millimètre autour du repos), les deux textures lues par `bassin.gdshader`.
var champ: Dictionary
var champ_octets: PackedByteArray
var champ_images := 0
var champ_image_s := 0.05
var champ_nx := 0
var champ_ny := 0
var champ_index := [-1, -1]
var hauteurs: Array[Image] = []
var textures: Array[ImageTexture] = []

## Les vues : position de la caméra et point visé (Godot).
const VUES := {
	"ensemble": [Vector3(-9.5, 6.0, 10.0), Vector3(1.0, 0.3, 0.0)],
	"deversoir": [Vector3(8.0, 3.0, 5.0), Vector3(4.6, -0.3, 0.0)],
	"buse": [Vector3(-1.2, 3.2, 4.6), Vector3(-3.2, 1.3, 0.0)],
}


static func godot(p: Array) -> Vector3:
	return Vector3(float(p[0]), float(p[2]), -float(p[1]))


func _ready() -> void:
	var f := FileAccess.open("res://donnees/piscine_v.json", FileAccess.READ)
	if f == null:
		push_error("donnees/piscine_v.json absent : cargo run --manifest-path code/Cargo.toml --release --offline -p water-core --example piscine_v")
		get_tree().quit(1)
		return
	donnees = JSON.parse_string(f.get_as_text())
	pas = donnees["pas"]
	dt = float(donnees["pas_s"])
	var fc := FileAccess.open("res://donnees/piscine_delta.json", FileAccess.READ)
	if fc != null and OS.get_environment("DELTA") != "0":
		champ = JSON.parse_string(fc.get_as_text())
		champ_octets = FileAccess.get_file_as_bytes("res://donnees/piscine_delta.bin")
		champ_nx = int(champ["nx"])
		champ_ny = int(champ["ny"])
		champ_images = int(champ["images"])
		champ_image_s = float(champ["image_s"])
		if champ_octets.size() != champ_images * champ_nx * champ_ny * 2:
			push_error("piscine_delta.bin : taille inattendue")
			champ = {}
	environnement()
	camera = Camera3D.new()
	camera.fov = 50.0
	camera.near = 0.05
	camera.far = 2000.0
	add_child(camera)
	camera.current = true
	vue("ensemble")
	construire()
	texte = Label.new()
	texte.position = Vector2(16, 12)
	texte.add_theme_font_size_override("font_size", 18)
	texte.add_theme_color_override("font_color", Color(1, 1, 1))
	texte.add_theme_color_override("font_outline_color", Color(0, 0, 0))
	texte.add_theme_constant_override("outline_size", 4)
	var calque := CanvasLayer.new()
	add_child(calque)
	calque.add_child(texte)
	appliquer(t)
	var args := OS.get_cmdline_user_args()
	if "--controle-piscine" in args:
		controle()
	elif "--captures" in args:
		captures()


## Le ciel de la mer (`ciel.gdshader`, une seule source), le soleil de la scène, la tonalité AgX.
func environnement() -> void:
	var env := Environment.new()
	var ciel := Sky.new()
	var m := ShaderMaterial.new()
	m.shader = load("res://ciel.gdshader")
	ciel.sky_material = m
	env.background_mode = Environment.BG_SKY
	env.sky = ciel
	env.ambient_light_source = Environment.AMBIENT_SOURCE_SKY
	env.tonemap_mode = Environment.TONE_MAPPER_AGX
	env.glow_enabled = true
	var monde := WorldEnvironment.new()
	monde.environment = env
	add_child(monde)


func vue(nom: String) -> void:
	var v: Array = VUES[nom]
	camera.position = v[0]
	camera.look_at(v[1], Vector3.UP)


## Une boîte entre deux coins (Godot), d'une paroi : `albedo`, joints de carrelage tous les `joint` m (0 : aucun).
func boite(a: Vector3, b: Vector3, albedo: Color, joint := 0.0) -> MeshInstance3D:
	var mi := MeshInstance3D.new()
	var bm := BoxMesh.new()
	bm.size = (b - a).abs()
	mi.mesh = bm
	mi.position = 0.5 * (a + b)
	var m := ShaderMaterial.new()
	m.shader = load("res://paroi.gdshader")
	m.set_shader_parameter("albedo", Vector3(albedo.r, albedo.g, albedo.b))
	m.set_shader_parameter("joint", joint)
	mi.material_override = m
	add_child(mi)
	return mi


## Une surface d'eau horizontale couvrant l'intérieur d'un bac (`fond_m`, `taille_m` de B).
func eau(bac: Dictionary) -> MeshInstance3D:
	var mi := MeshInstance3D.new()
	var pm := PlaneMesh.new()
	var taille: Array = bac["taille_m"]
	pm.size = Vector2(float(taille[0]), float(taille[1]))
	mi.mesh = pm
	var c := godot(bac["fond_m"])
	mi.position = Vector3(c.x, c.y, c.z)
	var m := ShaderMaterial.new()
	m.shader = load("res://bassin.gdshader")
	mi.material_override = m
	add_child(mi)
	return mi


## La piscine : le sol, le bloc du bassin (fond et murs carrelés, le mur est arasé au seuil du déversoir), le bac tampon en
## béton contre la face extérieure du mur est.
func construire() -> void:
	var b: Dictionary = donnees["bassin"]
	var tp: Dictionary = donnees["tampon"]
	var fb := godot(b["fond_m"])
	var tb: Array = b["taille_m"]
	var lx := 0.5 * float(tb[0])
	var lz := 0.5 * float(tb[1])
	var haut := fb.y + float(tb[2])
	var seuil := float(donnees["deversoir"]["seuil_m"][2])
	var e := 0.2
	var sol_y := float(tp["fond_m"][2])
	var beton := Color(0.42, 0.41, 0.39)
	var carrelage := Color(0.52, 0.72, 0.80)
	var c := 0.01
	# Le sol, au niveau du fond du bac tampon, jusqu'à l'horizon.
	boite(Vector3(-3000, sol_y - 0.2, -3000), Vector3(3000, sol_y, 3000), Color(0.20, 0.19, 0.17))
	# Le bloc du bassin, en béton : dalle jusqu'au fond, murs ouest, nord, sud à 1,5 m, est au seuil.
	boite(Vector3(-lx - e, sol_y, -lz - e), Vector3(lx + e, fb.y, lz + e), beton)
	boite(Vector3(-lx - e, fb.y, -lz - e), Vector3(-lx, haut, lz + e), beton)
	boite(Vector3(lx, fb.y, -lz - e), Vector3(lx + e, seuil, lz + e), beton)
	boite(Vector3(-lx, fb.y, lz), Vector3(lx, haut, lz + e), beton)
	boite(Vector3(-lx, fb.y, -lz - e), Vector3(lx, haut, -lz), beton)
	# Le carrelage, un centimètre sur les faces intérieures : le fond, et les murs (l'est jusqu'au seuil).
	boite(Vector3(-lx, fb.y, -lz), Vector3(lx, fb.y + c, lz), carrelage, 0.25)
	boite(Vector3(-lx, fb.y, -lz), Vector3(-lx + c, haut, lz), carrelage, 0.25)
	boite(Vector3(lx - c, fb.y, -lz), Vector3(lx, seuil, lz), carrelage, 0.25)
	boite(Vector3(-lx, fb.y, lz - c), Vector3(lx, haut, lz), carrelage, 0.25)
	boite(Vector3(-lx, fb.y, -lz), Vector3(lx, haut, -lz + c), carrelage, 0.25)
	# Les margelles, un peu plus larges, sur les trois murs hauts.
	boite(Vector3(-lx - e - 0.1, haut, -lz - e - 0.1), Vector3(-lx, haut + 0.05, lz + e + 0.1), beton)
	boite(Vector3(-lx, haut, lz), Vector3(lx + e, haut + 0.05, lz + e + 0.1), beton)
	boite(Vector3(-lx, haut, -lz - e - 0.1), Vector3(lx + e, haut + 0.05, -lz), beton)
	# Le bac tampon : sa face ouest est le mur est du bassin ; murs nord, sud, est.
	var ft := godot(tp["fond_m"])
	var tt: Array = tp["taille_m"]
	var x0 := ft.x - 0.5 * float(tt[0])
	var x1 := ft.x + 0.5 * float(tt[0])
	var tz := 0.5 * float(tt[1])
	var th := ft.y + float(tt[2])
	var et := 0.15
	boite(Vector3(x1, sol_y, -tz - et), Vector3(x1 + et, th, tz + et), beton)
	boite(Vector3(x0, sol_y, tz), Vector3(x1, th, tz + et), beton)
	boite(Vector3(x0, sol_y, -tz - et), Vector3(x1, th, -tz), beton)
	eau_bassin = eau(b)
	eau_tampon = eau(tp)
	if not champ.is_empty():
		eau_bassin.mesh = maillage_champ()
		eau_bassin.position.y = float(champ["repos_m"])
		for i in 2:
			var im := Image.create_empty(champ_nx, champ_ny, false, Image.FORMAT_RF)
			hauteurs.append(im)
			textures.append(ImageTexture.create_from_image(im))
		var m: ShaderMaterial = eau_bassin.material_override
		m.set_shader_parameter("champ_actif", true)
		m.set_shader_parameter("hauteur_a", textures[0])
		m.set_shader_parameter("hauteur_b", textures[1])
		m.set_shader_parameter("champ_texel", Vector2(1.0 / champ_nx, 1.0 / champ_ny))
		m.set_shader_parameter("champ_dx", float(champ["dx_m"]))
	materiau_bassin = eau_bassin.material_override
	materiau_tampon = eau_tampon.material_override


## S375 — le maillage de la surface de δ : un sommet au centre de chaque colonne, et un anneau sur les murs qui prend la
## hauteur de la colonne voisine ; UV = centre de la colonne dans les textures (lecture au plus proche, exacte).
func maillage_champ() -> ArrayMesh:
	var dxm := float(champ["dx_m"])
	var o: Array = champ["origine_b_m"]
	var xs := [float(o[0])]
	for i in champ_nx:
		xs.append(float(o[0]) + (i + 0.5) * dxm)
	xs.append(float(o[0]) + champ_nx * dxm)
	var ys := [float(o[1])]
	for j in champ_ny:
		ys.append(float(o[1]) + (j + 0.5) * dxm)
	ys.append(float(o[1]) + champ_ny * dxm)
	var sommets := PackedVector3Array()
	var uv := PackedVector2Array()
	var normales := PackedVector3Array()
	for jj in ys.size():
		for ii in xs.size():
			sommets.append(Vector3(xs[ii], 0.0, -float(ys[jj])))
			normales.append(Vector3.UP)
			var ci := clampi(ii - 1, 0, champ_nx - 1)
			var cj := clampi(jj - 1, 0, champ_ny - 1)
			uv.append(Vector2((ci + 0.5) / champ_nx, (cj + 0.5) / champ_ny))
	var indices := PackedInt32Array()
	var w := xs.size()
	for jj in ys.size() - 1:
		for ii in w - 1:
			var a := jj * w + ii
			indices.append_array([a, a + 1, a + w, a + 1, a + w + 1, a + w])
	var tableaux := []
	tableaux.resize(Mesh.ARRAY_MAX)
	tableaux[Mesh.ARRAY_VERTEX] = sommets
	tableaux[Mesh.ARRAY_NORMAL] = normales
	tableaux[Mesh.ARRAY_TEX_UV] = uv
	tableaux[Mesh.ARRAY_INDEX] = indices
	var m := ArrayMesh.new()
	m.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, tableaux)
	m.custom_aabb = AABB(Vector3(-5, -1, -3), Vector3(10, 2, 6))
	return m


## L'image `k` de δ dans la texture `emplacement` (0 : avant, 1 : après) ; hauteurs en mètres au-dessus du repos.
func charger_image(k: int, emplacement: int) -> void:
	if champ_index[emplacement] == k:
		return
	var n := champ_nx * champ_ny
	var valeurs := PackedFloat32Array()
	valeurs.resize(n)
	var base := k * n * 2
	for c in n:
		valeurs[c] = float(champ_octets.decode_s16(base + 2 * c)) * 1e-4
	hauteurs[emplacement].set_data(champ_nx, champ_ny, false, Image.FORMAT_RF, valeurs.to_byte_array())
	textures[emplacement].update(hauteurs[emplacement])
	champ_index[emplacement] = k


## Les deux images de δ qui encadrent le temps `s` (l'image `k` est celle de `(k + 1)·image_s`), et leur mélange.
func poser_champ(s: float) -> void:
	var x := clampf(s / champ_image_s - 1.0, 0.0, float(champ_images - 1))
	var k := mini(int(floor(x)), champ_images - 2)
	charger_image(k, 0)
	charger_image(k + 1, 1)
	materiau_bassin.set_shader_parameter("melange", x - float(k))


## La ligne interpolée du rejeu au temps `s` : les colonnes de l'export, linéaires entre deux pas de V.
func ligne(s: float) -> Array:
	var x := clampf(s / dt, 0.0, float(pas.size() - 1))
	var i := mini(int(floor(x)), pas.size() - 2)
	var u := x - float(i)
	var a: Array = pas[i]
	var c: Array = pas[i + 1]
	var r := []
	for k in a.size():
		r.append(lerpf(float(a[k]), float(c[k]), u))
	return r


## Pose l'état du temps `s` : cotes des surfaces (celles de V, interpolées), agitation, indications.
func appliquer(s: float) -> void:
	var l := ligne(s)
	if champ.is_empty():
		eau_bassin.position.y = float(l[1])
	else:
		poser_champ(s)
	eau_tampon.position.y = float(l[2])
	for m in [materiau_bassin, materiau_tampon]:
		m.set_shader_parameter("temps", s)
	var q_dev := float(l[5])
	var q_pompe := float(l[6])
	# L'agitation d'habillage : une ride de fond, plus là où l'eau tombe.
	materiau_bassin.set_shader_parameter("agitation", 0.012)
	materiau_tampon.set_shader_parameter("agitation", 0.012 + 0.006 * q_dev)
	var seuil := float(donnees["deversoir"]["seuil_m"][2])
	texte.text = "Piscine de V (S374) — rejeu du cœur, t = %.1f s\nBassin : surface %+.1f mm par rapport au seuil\nBac tampon : %.3f m d'eau\nDéversoir : %.2f l/s\nPompe : %s, %.2f l/s" % [
		s, (float(l[1]) - seuil) * 1000.0, float(l[2]) - float(donnees["tampon"]["fond_m"][2]), q_dev,
		"en marche" if float(l[7]) > 0.5 else "arrêtée", q_pompe]


func _process(delta: float) -> void:
	if pas.is_empty() or "--captures" in OS.get_cmdline_user_args() or "--controle-piscine" in OS.get_cmdline_user_args():
		return
	if not en_pause:
		t = fmod(t + delta, float(pas.size() - 1) * dt)
	appliquer(t)


func _unhandled_input(event: InputEvent) -> void:
	if event is InputEventKey and event.pressed:
		match event.keycode:
			KEY_1:
				vue("ensemble")
			KEY_2:
				vue("deversoir")
			KEY_3:
				vue("buse")
			KEY_SPACE:
				en_pause = not en_pause
			KEY_ESCAPE:
				get_tree().quit()


## Les images de revue : trois vues à quatre instants — pompe arrêtée, débordement qui s'installe, régime établi, pompe
## arrêtée depuis 20 s.
func captures() -> void:
	DirAccess.make_dir_recursive_absolute(ProjectSettings.globalize_path("res://captures"))
	var instants := [2.0, 5.6, 6.5, 40.0, 200.0, 260.0]
	if OS.get_environment("INSTANTS") != "":
		instants = Array(OS.get_environment("INSTANTS").split(",")).map(func(x): return float(x))
	var vues := ["ensemble", "deversoir", "buse"]
	if OS.get_environment("VUES") != "":
		vues = Array(OS.get_environment("VUES").split(","))
	for nom in vues:
		vue(nom)
		for s in instants:
			t = s
			appliquer(t)
			for _i in 8:
				await RenderingServer.frame_post_draw
			var chemin := ProjectSettings.globalize_path("res://captures/piscine_%s_%03ds.png" % [nom, int(s)])
			get_viewport().get_texture().get_image().save_png(chemin)
			print("CAPTURE_PISCINE_S374 vue=%s t=%.1f fichier=%s" % [nom, s, chemin])
	get_tree().quit()


## **Critère 3 de S374** : la cote rendue de chaque surface — la position de son maillage, relue — contre celle que V
## publie, à des instants qui tombent sur un pas et entre deux pas ; au dixième de millimètre.
func controle() -> void:
	var pire := 0.0
	for s in [0.0, 37.3, 100.0, 199.95, 250.05, 329.9]:
		t = s
		appliquer(t)
		await RenderingServer.frame_post_draw
		var i: int = int(floor(float(s) / dt))
		var u: float = float(s) / dt - float(i)
		for k in [1, 2]:
			var attendu := lerpf(float(pas[i][k]), float(pas[mini(i + 1, pas.size() - 1)][k]), u)
			var rendu := (eau_bassin if k == 1 else eau_tampon).global_position.y
			var e := absf(rendu - attendu)
			pire = maxf(pire, e)
			print("CONTROLE_PISCINE_S374 t=%.2f bac=%s rendu_m=%.7f v_m=%.7f ecart_m=%s" % [s, "bassin" if k == 1 else "tampon", rendu, attendu, String.num_scientific(e)])
	print("CONTROLE_PISCINE_S374 critere=3 pire_m=%s %s" % [String.num_scientific(pire), "tenu" if pire <= 1e-4 else "manque"])
	# S375, critère 4 : les hauteurs de δ chargées dans la texture sont celles du fichier, à l'arrondi f32 près (le nuanceur
	# les lit au plus proche, aux centres des colonnes).
	if not champ.is_empty():
		var ecarts := 0
		var lus := 0
		for k in [0, 100, 1000, champ_images - 1]:
			charger_image(k, 0)
			for c in range(0, champ_nx * champ_ny, 7):
				var attendu := float(champ_octets.decode_s16((k * champ_nx * champ_ny + c) * 2)) * 1e-4
				var lu := hauteurs[0].get_pixel(c % champ_nx, c / champ_nx).r
				lus += 1
				# La texture est en f32 ; GDScript calcule en f64 : égal à l'arrondi f32 près (demi-ulp relatif, 6·10⁻⁸).
				if absf(lu - attendu) > 6e-8 * absf(attendu):
					ecarts += 1
		print("CONTROLE_PISCINE_S375 critere=4 hauteurs_lues=%d ecarts=%d %s" % [lus, ecarts, "tenu" if ecarts == 0 else "manque"])
	get_tree().quit()
