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
## S379 — **la pluie**, factice (ADR-202 D3) : `PLUIE=<mm/h>` dans l'environnement, ou la touche P (0, 2, 10, 50 mm/h) ;
## les rides de `pluie.gdshaderinc`, au taux de `pluie.gd`. `-- --controle-pluie` : le taux de naissance des anneaux compté
## sur des images de contrôle (critère 2 de S379), puis quitte. `-- --cout-pluie` : le temps GPU de l'image, sans pluie et
## sous la pluie, trois vues (médiane de 240 images). S380 — `-- --controle-gouttes` : le nombre et les tailles des
## gouttes dans l'air, comptés (critère 2 de S380). S381 — `-- --controle-ciel` : le ciel couvert mesuré (critères 2 à 4
## de S381).
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
var monde_env: Environment
const Pluie = preload("res://pluie.gd")
var pluie_mm_h := 0.0
## S380 — la pluie dans l'air (`pluie_air.gd`, ADR-205 pièce 1).
var pluie_air: Node3D
## S383 — les gerbes (`gerbes.gd`, ADR-205 pièce 4).
var gerbes: Node3D
## S381 — le ciel de pluie (ADR-205, pièce 3) : tous les matériaux qui incluent `ciel.gdshaderinc`, et `COUVERT=` (0 à 1)
## qui force la couverture ; sinon, la pluie couvre le ciel.
var materiaux_ciel: Array = []
## S382 — **les occultants** (ADR-206) : chaque boîte de la scène, sauf le sol, masque le ciel des autres surfaces
## (`occultation.gdshaderinc`). `OCCULTATION=0` n'en déclare aucun : l'image d'avant, au bit.
var occultants_min := PackedVector3Array()
var occultants_max := PackedVector3Array()
## Les cotes de la construction, pour les points d'essai du contrôle de S382.
var geo := {}
## S382 P6c — les surfaces qui reçoivent le ciel ([instance, matériau, normale forcée vers le haut]) et la passe qui cuit
## leur part du ciel vue (`cuisson_ciel.gdshader`). `OCCULTATION=0` : ni occultant, ni subdivision, ni cuisson — la
## géométrie et l'image d'avant.
var occultation_active := OS.get_environment("OCCULTATION") != "0"
var recepteurs := []
var cuisson: SubViewport
const CUISSON_LARGEUR := 1024


func couvert_voulu() -> float:
	if OS.get_environment("COUVERT") != "":
		return clampf(float(OS.get_environment("COUVERT")), 0.0, 1.0)
	return 1.0 if pluie_mm_h > 0.0 else 0.0
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
	## S375 : au ras du bord sud, vers l'impact du jet — les reflets du ciel révèlent les pentes de quelques millièmes.
	"rasante": [Vector3(-0.6, 1.62, 2.3), Vector3(-2.8, 1.40, -0.4)],
	## S379 : de près, en oblique, comme les photographies de pluie (à 1,8 m de l'eau, 30° sous l'horizontale) ; et d'aplomb.
	"pluie_proche": [Vector3(-3.0, 2.3, 2.0), Vector3(-3.0, 1.4, 0.45)],
	"pluie_aplomb": [Vector3(-1.5, 3.2, 0.3), Vector3(-1.5, 1.4, 0.0)],
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
	if OS.get_environment("PLUIE") != "":
		pluie_mm_h = float(OS.get_environment("PLUIE"))
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
	elif "--controle-pluie" in args:
		controle_pluie()
	elif "--cout-pluie" in args:
		cout_pluie()
	elif "--controle-gouttes" in args:
		controle_gouttes()
	elif "--controle-ciel" in args:
		controle_ciel()
	elif "--controle-occultation" in args:
		controle_occultation()
	elif "--controle-ombre" in args:
		controle_ombre()
	elif "--controle-gerbes" in args:
		controle_gerbes()
	elif "--captures" in args:
		captures()


## Le ciel de la mer (`ciel.gdshader`, une seule source), le soleil de la scène, la tonalité AgX.
func environnement() -> void:
	var env := Environment.new()
	var ciel := Sky.new()
	var m := ShaderMaterial.new()
	m.shader = load("res://ciel.gdshader")
	ciel.sky_material = m
	materiaux_ciel.append(m)
	env.background_mode = Environment.BG_SKY
	env.sky = ciel
	env.ambient_light_source = Environment.AMBIENT_SOURCE_SKY
	env.tonemap_mode = Environment.TONE_MAPPER_AGX
	env.glow_enabled = true
	var monde := WorldEnvironment.new()
	monde.environment = env
	monde_env = env
	add_child(monde)


func vue(nom: String) -> void:
	var v: Array = VUES[nom]
	camera.position = v[0]
	camera.look_at(v[1], Vector3.UP)


## Une boîte entre deux coins (Godot), d'une paroi : `albedo`, joints de carrelage tous les `joint` m (0 : aucun).
## S382 : `occultant`, elle masque le ciel des autres surfaces ; ses faces subdivisées tous les `PAS_SOMMETS` mètres au
## plus, la part du ciel vue étant calculée aux sommets.
func boite(a: Vector3, b: Vector3, albedo: Color, joint := 0.0, occultant := true) -> MeshInstance3D:
	var mi := MeshInstance3D.new()
	var taille := (b - a).abs()
	if occultation_active and taille.x < 1000.0 and taille.z < 1000.0:
		mi.mesh = maillage_boite(taille)
	else:
		var bm := BoxMesh.new()
		bm.size = taille
		mi.mesh = bm
	mi.position = 0.5 * (a + b)
	var m := ShaderMaterial.new()
	m.shader = load("res://paroi.gdshader")
	m.set_shader_parameter("albedo", Vector3(albedo.r, albedo.g, albedo.b))
	m.set_shader_parameter("joint", joint)
	mi.material_override = m
	materiaux_ciel.append(m)
	add_child(mi)
	recepteurs.append([mi, m, false])
	if occultant:
		occultants_min.append(Vector3(minf(a.x, b.x), minf(a.y, b.y), minf(a.z, b.z)))
		occultants_max.append(Vector3(maxf(a.x, b.x), maxf(a.y, b.y), maxf(a.z, b.z)))
	return mi


## S382 P6d — **une boîte à faces graduées** : près de chaque arête, là où une autre surface peut toucher (pied de mur,
## dessous d'un débord), la part du ciel vue varie vite ; l'interpolation entre sommets espacés de 10 cm la manquait (0,36
## pour 0,45 à 5 cm sous la margelle, P6b). Coordonnées de chaque axe : 1 cm au bord, ×1,3 par pas, 10 cm au plus.
## Faces orientées comme Godot les attend (sens horaire vu de devant : u × v = −n).
func maillage_boite(taille: Vector3) -> ArrayMesh:
	var axes := [coordonnees_bords(taille.x), coordonnees_bords(taille.y), coordonnees_bords(taille.z)]
	var sommets := PackedVector3Array()
	var normales := PackedVector3Array()
	var indices := PackedInt32Array()
	# (axe de la normale, signe, axe u, axe v), u × v = −n.
	for f in [[0, 1, 2, 1], [0, -1, 1, 2], [1, 1, 0, 2], [1, -1, 2, 0], [2, 1, 1, 0], [2, -1, 0, 1]]:
		var n := Vector3.ZERO
		n[f[0]] = f[1]
		var cu: PackedFloat32Array = axes[f[2]]
		var cv: PackedFloat32Array = axes[f[3]]
		var base := sommets.size()
		for j in cv.size():
			for i in cu.size():
				var s := Vector3.ZERO
				s[f[0]] = 0.5 * taille[f[0]] * f[1]
				s[f[2]] = cu[i]
				s[f[3]] = cv[j]
				sommets.append(s)
				normales.append(n)
		var nu := cu.size()
		for j in cv.size() - 1:
			for i in nu - 1:
				var k := base + j * nu + i
				indices.append_array([k, k + 1, k + nu, k + 1, k + nu + 1, k + nu])
	var tableaux := []
	tableaux.resize(Mesh.ARRAY_MAX)
	tableaux[Mesh.ARRAY_VERTEX] = sommets
	tableaux[Mesh.ARRAY_NORMAL] = normales
	tableaux[Mesh.ARRAY_INDEX] = indices
	var am := ArrayMesh.new()
	am.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, tableaux)
	return am


## Les coordonnées d'un axe de longueur `l`, centrées : 1 cm aux deux bords, ×1,3 par pas, `PAS_SOMMETS` au plus.
static func coordonnees_bords(l: float) -> PackedFloat32Array:
	var demi := PackedFloat32Array([0.0])
	var x := 0.0
	var h := 0.01
	while x + h < 0.5 * l - 0.5 * h:
		x += h
		demi.append(x)
		h = minf(h * 1.3, PAS_SOMMETS)
	var c := PackedFloat32Array()
	for v in demi:
		c.append(-0.5 * l + v)
	for i in range(demi.size() - 1, -1, -1):
		c.append(0.5 * l - demi[i])
	return c


## S382 — le nombre de subdivisions d'une longueur pour que les sommets soient à `PAS_SOMMETS` au plus ; aucune au-delà
## de 50 m (le sol lointain, où la part vue vaut 1).
const PAS_SOMMETS := 0.1
func subdivisions(longueur: float) -> int:
	if not occultation_active or longueur > 50.0:
		return 0
	return maxi(int(ceil(longueur / PAS_SOMMETS)) - 1, 0)


## S382 — **le sol autour de la piscine**, un maillage gradué : pas de `PAS_SOMMETS` sur l'emprise élargie de 3 m (`a`,
## `b`, coins opposés en x et z), puis croissant de 15 % par pas jusqu'à `rayon` du centre — assez loin pour que la part du
## ciel vue y vaille 1 exactement (la coupure d'`occultation.gdshaderinc` est à ≈ 180 m pour 2,85 m de haut). Au-delà, le
## sol lointain, quatre boîtes sans subdivision.
func sol_gradue(y: float, a: Vector2, b: Vector2, rayon: float, albedo: Color) -> void:
	if not occultation_active:
		boite(Vector3(-3000, y - 0.2, -3000), Vector3(3000, y, 3000), albedo, 0.0, false)
		return
	var xs := coordonnees_graduees(a.x - 3.0, b.x + 3.0, rayon)
	var zs := coordonnees_graduees(a.y - 3.0, b.y + 3.0, rayon)
	var sommets := PackedVector3Array()
	var normales := PackedVector3Array()
	var indices := PackedInt32Array()
	for z in zs:
		for x in xs:
			sommets.append(Vector3(x, y, z))
			normales.append(Vector3.UP)
	var nx := xs.size()
	for j in zs.size() - 1:
		for i in nx - 1:
			var k := j * nx + i
			indices.append_array([k, k + 1, k + nx, k + 1, k + nx + 1, k + nx])
	var tableaux := []
	tableaux.resize(Mesh.ARRAY_MAX)
	tableaux[Mesh.ARRAY_VERTEX] = sommets
	tableaux[Mesh.ARRAY_NORMAL] = normales
	tableaux[Mesh.ARRAY_INDEX] = indices
	var am := ArrayMesh.new()
	am.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, tableaux)
	var mi := MeshInstance3D.new()
	mi.mesh = am
	var m := ShaderMaterial.new()
	m.shader = load("res://paroi.gdshader")
	m.set_shader_parameter("albedo", Vector3(albedo.r, albedo.g, albedo.b))
	mi.material_override = m
	materiaux_ciel.append(m)
	add_child(mi)
	recepteurs.append([mi, m, false])
	var r := rayon
	boite(Vector3(-3000, y - 0.2, -3000), Vector3(-r, y, 3000), albedo, 0.0, false)
	boite(Vector3(r, y - 0.2, -3000), Vector3(3000, y, 3000), albedo, 0.0, false)
	boite(Vector3(-r, y - 0.2, -3000), Vector3(r, y, -r), albedo, 0.0, false)
	boite(Vector3(-r, y - 0.2, r), Vector3(r, y, 3000), albedo, 0.0, false)


## Les coordonnées d'un axe : pas de `PAS_SOMMETS` de `a` à `b`, puis croissant de 15 % par pas jusqu'à ±`rayon`.
static func coordonnees_graduees(a: float, b: float, rayon: float) -> PackedFloat32Array:
	var c := PackedFloat32Array()
	var n := int(ceil((b - a) / PAS_SOMMETS))
	for i in n + 1:
		c.append(a + (b - a) * i / n)
	var gauche := PackedFloat32Array()
	var x := a
	var h := PAS_SOMMETS
	while x > -rayon:
		h *= 1.15
		x = maxf(x - h, -rayon)
		gauche.append(x)
	gauche.reverse()
	gauche.append_array(c)
	x = b
	h = PAS_SOMMETS
	while x < rayon:
		h *= 1.15
		x = minf(x + h, rayon)
		gauche.append(x)
	return gauche


## S382 — les occultants déclarés à tous les matériaux qui incluent `occultation.gdshaderinc` (parois, bassin).
func poser_occultants() -> void:
	var n := occultants_min.size() if occultation_active else 0
	for m in materiaux_ciel:
		m.set_shader_parameter("occultants_n", n)
		m.set_shader_parameter("occultants_min", occultants_min)
		m.set_shader_parameter("occultants_max", occultants_max)
	if occultation_active:
		cuire_ciel_vu()


## S382 P6c — **la cuisson** : tous les sommets des récepteurs, en coordonnées du monde, deviennent les points d'un maillage
## rendu une fois dans une `SubViewport` flottante à monde propre ; chaque point écrit sa part du ciel vue au texel de son
## rang. Chaque récepteur reçoit la texture et le rang de son premier sommet. À refaire quand un occultant change.
func cuire_ciel_vu() -> void:
	var points := PackedVector3Array()
	var normales := PackedVector3Array()
	for r in recepteurs:
		var mi: MeshInstance3D = r[0]
		var tableaux := mi.mesh.surface_get_arrays(0)
		var v: PackedVector3Array = tableaux[Mesh.ARRAY_VERTEX]
		var nn = tableaux[Mesh.ARRAY_NORMAL]
		var xf := mi.global_transform
		(r[1] as ShaderMaterial).set_shader_parameter("vu_decalage", points.size())
		for i in v.size():
			points.append(xf * v[i])
			normales.append(Vector3.UP if r[2] or nn == null else (xf.basis * (nn as PackedVector3Array)[i]).normalized())
	var hauteur := int(ceil(float(points.size()) / CUISSON_LARGEUR))
	if cuisson == null:
		cuisson = SubViewport.new()
		cuisson.own_world_3d = true
		cuisson.use_hdr_2d = true
		cuisson.transparent_bg = true
		add_child(cuisson)
		var cam := Camera3D.new()
		cuisson.add_child(cam)
		cam.current = true
	cuisson.size = Vector2i(CUISSON_LARGEUR, hauteur)
	for enfant in cuisson.get_children():
		if enfant is MeshInstance3D:
			enfant.queue_free()
	var tableaux := []
	tableaux.resize(Mesh.ARRAY_MAX)
	tableaux[Mesh.ARRAY_VERTEX] = points
	tableaux[Mesh.ARRAY_NORMAL] = normales
	var am := ArrayMesh.new()
	am.add_surface_from_arrays(Mesh.PRIMITIVE_POINTS, tableaux)
	var mi := MeshInstance3D.new()
	mi.mesh = am
	# Jamais écarté par le cadrage de la caméra : les points se placent eux-mêmes (`POSITION`).
	mi.custom_aabb = AABB(Vector3(-1, -1, -3), Vector3(2, 2, 2))
	var m := ShaderMaterial.new()
	m.shader = load("res://cuisson_ciel.gdshader")
	m.set_shader_parameter("largeur", CUISSON_LARGEUR)
	m.set_shader_parameter("hauteur", hauteur)
	m.set_shader_parameter("occultants_n", occultants_min.size())
	m.set_shader_parameter("occultants_min", occultants_min)
	m.set_shader_parameter("occultants_max", occultants_max)
	mi.material_override = m
	cuisson.add_child(mi)
	cuisson.render_target_update_mode = SubViewport.UPDATE_ONCE
	var texture := cuisson.get_texture()
	for r in recepteurs:
		(r[1] as ShaderMaterial).set_shader_parameter("vu_cuite", texture)
	print("CUISSON_CIEL_S382 sommets=%d texture=%dx%d" % [points.size(), CUISSON_LARGEUR, hauteur])


## Une surface d'eau horizontale couvrant l'intérieur d'un bac (`fond_m`, `taille_m` de B).
func eau(bac: Dictionary) -> MeshInstance3D:
	var mi := MeshInstance3D.new()
	var pm := PlaneMesh.new()
	var taille: Array = bac["taille_m"]
	pm.size = Vector2(float(taille[0]), float(taille[1]))
	pm.subdivide_width = subdivisions(pm.size.x)
	pm.subdivide_depth = subdivisions(pm.size.y)
	mi.mesh = pm
	var c := godot(bac["fond_m"])
	mi.position = Vector3(c.x, c.y, c.z)
	var m := ShaderMaterial.new()
	m.shader = load("res://bassin.gdshader")
	mi.material_override = m
	materiaux_ciel.append(m)
	add_child(mi)
	recepteurs.append([mi, m, true])
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
	geo = {"lx": lx, "lz": lz, "e": e, "sol_y": sol_y, "fond_y": fb.y + c, "haut": haut}
	# Le sol, au niveau du fond du bac tampon, jusqu'à l'horizon ; S382 : gradué autour de la piscine (voir `sol_gradue`,
	# appelé en fin de construction, quand l'emprise est connue).
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
	sol_gradue(sol_y, Vector2(-lx - e - 0.1, -lz - e - 0.1), Vector2(maxf(x1 + et, lx + e + 0.1), lz + e + 0.1), 200.0,
		Color(0.20, 0.19, 0.17))
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
	# S380 : les gouttes s'arrêtent au sol et à l'eau des deux bacs.
	pluie_air = load("res://pluie_air.gd").new()
	add_child(pluie_air)
	pluie_air.plancher = sol_y
	pluie_air.nappes = [Vector4(fb.x - lx, fb.z - lz, fb.x + lx, fb.z + lz),
		Vector4(x0, ft.z - tz, x1, ft.z + tz)]
	gerbes = load("res://gerbes.gd").new()
	add_child(gerbes)
	gerbes.nappes = pluie_air.nappes
	gerbes.fenetre = Vector4(minf(fb.x - lx, x0), minf(fb.z - lz, ft.z - tz), maxf(fb.x + lx, x1), maxf(fb.z + lz, ft.z + tz))
	poser_occultants()


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
	# `EXAGERE=k` : témoin de débogage seulement (S375) — les hauteurs de δ multipliées par k, pour éprouver la chaîne de
	# rendu (déplacement, pentes) ; jamais une image de revue.
	var echelle := 1e-4 * (float(OS.get_environment("EXAGERE")) if OS.get_environment("EXAGERE") != "" else 1.0)
	for c in n:
		valeurs[c] = float(champ_octets.decode_s16(base + 2 * c)) * echelle
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
		m.set_shader_parameter("pluie", Pluie.uniformes(pluie_mm_h))
	pluie_air.niveaux = [eau_bassin.global_position.y, eau_tampon.global_position.y]
	var couvert := couvert_voulu()
	for m in materiaux_ciel:
		m.set_shader_parameter("couvert", couvert)
	pluie_air.couvert = couvert
	pluie_air.configurer(pluie_mm_h)
	pluie_air.suivre(camera, s)
	gerbes.niveaux = pluie_air.niveaux
	gerbes.couvert = couvert
	gerbes.configurer(pluie_mm_h)
	gerbes.suivre(camera, s)
	# S380 : l'extinction par les gouttes (ADR-205, pièce 2) — la brume de Godot, `1 − e^(−β·d)`, couleur du ciel ; par
	# temps sec, pas de brume, comme avant.
	var beta := Pluie.extinction(pluie_mm_h)
	monde_env.fog_enabled = beta > 0.0
	if beta > 0.0:
		monde_env.fog_density = beta
		monde_env.fog_aerial_perspective = 1.0
		monde_env.fog_sky_affect = 0.0
	var q_dev := float(l[5])
	var q_pompe := float(l[6])
	# L'agitation d'habillage : une ride de fond, plus là où l'eau tombe.
	materiau_bassin.set_shader_parameter("agitation", 0.012)
	materiau_tampon.set_shader_parameter("agitation", 0.012 + 0.006 * q_dev)
	var seuil := float(donnees["deversoir"]["seuil_m"][2])
	texte.text = ("Pluie %s mm/h (touche P) — " % mm_h(pluie_mm_h) if pluie_mm_h > 0.0 else "") + "Piscine (V et δ 3D, S375) — rejeu du cœur, t = %.1f s\nBassin : surface %+.1f mm par rapport au seuil\nBac tampon : %.3f m d'eau\nDéversoir : %.2f l/s\nPompe : %s, %.2f l/s" % [
		s, (float(l[1]) - seuil) * 1000.0, float(l[2]) - float(donnees["tampon"]["fond_m"][2]), q_dev,
		"en marche" if float(l[7]) > 0.5 else "arrêtée", q_pompe]


func _process(delta: float) -> void:
	var args := OS.get_cmdline_user_args()
	if pas.is_empty() or "--captures" in args or "--controle-piscine" in args or "--controle-pluie" in args or "--cout-pluie" in args or "--controle-gouttes" in args or "--controle-ciel" in args or "--controle-occultation" in args or "--controle-ombre" in args or "--controle-gerbes" in args:
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
			KEY_4:
				vue("rasante")
			KEY_SPACE:
				en_pause = not en_pause
			KEY_P:
				var suite := {0.0: 2.0, 2.0: 10.0, 10.0: 50.0}
				pluie_mm_h = suite.get(pluie_mm_h, 0.0)
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
			var suffixe := "_pluie%s" % mm_h(pluie_mm_h) if pluie_mm_h > 0.0 else ""
			var chemin := ProjectSettings.globalize_path("res://captures/piscine_%s%s_%03ds.png" % [nom, suffixe, int(s)])
			get_viewport().get_texture().get_image().save_png(chemin)
			print("CAPTURE_PISCINE_S374 vue=%s t=%.1f fichier=%s" % [nom, s, chemin])
	get_tree().quit()


## **Critère 3 de S374** : la cote rendue de chaque surface — la position de son maillage, relue — contre celle que V
## publie, à des instants qui tombent sur un pas et entre deux pas ; au dixième de millimètre.
func controle() -> void:
	var pire := 0.0
	# Le critère 3 de S374 lit la cote du maillage plan : avec la surface de δ (S375), le maillage est au repos et porte
	# ses hauteurs dans le nuanceur — seul le bac tampon est alors plan. `DELTA=0` rend le contrôle de S374 entier.
	var bacs := [1, 2] if champ.is_empty() else [2]
	for s in [0.0, 37.3, 100.0, 199.95, 250.05, 329.9]:
		t = s
		appliquer(t)
		await RenderingServer.frame_post_draw
		var i: int = int(floor(float(s) / dt))
		var u: float = float(s) / dt - float(i)
		for k in bacs:
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


## **Critère 2 de S379** : le taux de naissance des anneaux, compté. Vue orthographique d'aplomb sur le bassin, le nuanceur
## en mode contrôle (rouge pur : un cœur d'anneau — rayon 1 cm — de moins de 30 ms ; noir ailleurs), tonalité linéaire et
## sans halo ; à chaque instant, les taches rouges sont relevées (4-connexité) et comparées à `taux × aire × 0,03 s`. Deux
## cœurs qui se touchent font une seule tache (à 50 mm/h, 58 cœurs jeunes par m² : une fusion pour huit) : chaque tache
## compte pour `max(1, arrondi(aire / aire médiane))` cœurs, et le compte brut est donné à côté. Les instants sont
## espacés de plus d'une vie d'anneau : les comptes sont indépendants. Intensités : `PLUIE` ou 2, 10, 50.
func controle_pluie() -> void:
	monde_env.tonemap_mode = Environment.TONE_MAPPER_LINEAR
	monde_env.glow_enabled = false
	texte.visible = false
	# Les anneaux seuls : ni gouttes dans l'air ni brume (S380) sur les images de contrôle.
	pluie_air.configurer(0.0)
	monde_env.fog_enabled = false
	var b: Dictionary = donnees["bassin"]
	var tb: Array = b["taille_m"]
	var aire := float(tb[0]) * float(tb[1])
	var centre := godot(b["fond_m"])
	camera.projection = Camera3D.PROJECTION_ORTHOGONAL
	camera.size = float(tb[1]) + 0.6
	camera.position = Vector3(centre.x, 20.0, centre.z)
	camera.look_at(Vector3(centre.x, 0.0, centre.z), Vector3(0, 0, -1))
	materiau_tampon.set_shader_parameter("pluie", Vector4.ZERO)
	var intensites := [pluie_mm_h] if pluie_mm_h > 0.0 else [2.0, 10.0, 50.0]
	for r in intensites:
		var u: Vector4 = Pluie.uniformes(r)
		var aires: Array[int] = []
		var instants := 20
		for i in instants:
			var s := 3.1 + 1.37 * float(i)
			materiau_bassin.set_shader_parameter("temps", s)
			materiau_bassin.set_shader_parameter("pluie", u)
			materiau_bassin.set_shader_parameter("controle_pluie", true)
			for _k in 3:
				await RenderingServer.frame_post_draw
			var image := get_viewport().get_texture().get_image()
			aires.append_array(taches_rouges(image))
		var triees := aires.duplicate()
		triees.sort()
		var mediane := float(triees[triees.size() / 2]) if not triees.is_empty() else 1.0
		var total := 0
		for a in aires:
			total += maxi(1, roundi(float(a) / mediane))
		var attendu := u.x * aire * 0.03 * instants
		var ecart := (float(total) - attendu) / attendu
		var sigma := 1.0 / sqrt(attendu)
		print("CONTROLE_PLUIE_S379 pluie_mm_h=%s taux=%.1f aire_m2=%.1f instants=%d taches=%d aire_mediane_px=%d coeurs=%d attendus=%.1f ecart=%+.2f%% (poisson 1 sigma %.2f%%) %s" % [
			mm_h(r), u.x, aire, instants, aires.size(), int(mediane), total, attendu, 100.0 * ecart, 100.0 * sigma, "tenu" if absf(ecart) <= 0.10 else "manque"])
	# S380, critère 3 : l'extinction posée dans la scène contre celle de la loi ; par temps sec, pas de brume.
	for r in [0.0, 2.0, 10.0, 50.0]:
		pluie_mm_h = r
		appliquer(3.1)
		var loi := Pluie.extinction(r)
		print("CONTROLE_PLUIE_S380 pluie_mm_h=%s extinction_loi=%s brume_active=%s brume_scene=%s visibilite_m=%s" % [
			mm_h(r), String.num_scientific(loi), str(monde_env.fog_enabled),
			String.num_scientific(monde_env.fog_density if monde_env.fog_enabled else 0.0),
			"%.0f" % (3.912 / loi) if loi > 0.0 else "infinie"])
	get_tree().quit()


## Une intensité de pluie pour l'affichage et les noms de fichiers : `10`, `2.5`.
static func mm_h(r: float) -> String:
	return str(int(r)) if r == floorf(r) else str(r)


## Les aires (pixels) des taches rouges d'une image de contrôle (rouge > 0,5, vert et bleu < 0,2), en 4-connexité.
func taches_rouges(image: Image) -> Array[int]:
	image.convert(Image.FORMAT_RGB8)
	var w := image.get_width()
	var h := image.get_height()
	var d := image.get_data()
	var vu := PackedByteArray()
	vu.resize(w * h)
	var aires: Array[int] = []
	for i in w * h:
		if vu[i] != 0 or d[3 * i] < 128 or d[3 * i + 1] > 51 or d[3 * i + 2] > 51:
			continue
		var n := 0
		var pile := [i]
		vu[i] = 1
		while not pile.is_empty():
			var c: int = pile.pop_back()
			n += 1
			var x := c % w
			var voisins := []
			if x > 0: voisins.append(c - 1)
			if x < w - 1: voisins.append(c + 1)
			if c >= w: voisins.append(c - w)
			if c < w * (h - 1): voisins.append(c + w)
			for v in voisins:
				if vu[v] == 0 and d[3 * v] >= 128 and d[3 * v + 1] <= 51 and d[3 * v + 2] <= 51:
					vu[v] = 1
					pile.append(v)
		aires.append(n)
	return aires


## Le coût de la pluie : temps GPU de l'image entière (médiane de 240 images après 30 de mise en route), à 0, 2, 10 et
## 50 mm/h, pour trois vues au temps 200 s.
func cout_pluie() -> void:
	var vp := get_viewport().get_viewport_rid()
	RenderingServer.viewport_set_measure_render_time(vp, true)
	texte.visible = false
	for nom in ["pluie_proche", "pluie_aplomb", "rasante"]:
		vue(nom)
		var ligne_cout := "COUT_PLUIE_S379 vue=%s" % nom
		for r in [0.0, 2.0, 10.0, 50.0]:
			pluie_mm_h = r
			appliquer(200.0)
			for _i in 30:
				await RenderingServer.frame_post_draw
			var t := []
			for _i in 240:
				await RenderingServer.frame_post_draw
				t.append(RenderingServer.viewport_get_measured_render_time_gpu(vp))
			t.sort()
			ligne_cout += " gpu_ms_%s=%.3f" % [mm_h(r), float(t[120])]
		print(ligne_cout)
	get_tree().quit()


## **Critère 2 de S380** : les gouttes dans l'air, comptées. Vue orthographique d'aplomb, 7,9 m de haut d'image ; seules les
## gouttes d'une tranche de 0,5 m (16 à 16,5 m, dans les deux boîtes), chacune un carré de deux pixels, d'une couleur pure
## par classe de diamètre — rouge [1 ; 1,5), vert [1,5 ; 2), bleu [2 ; 3), blanc [3 ; 6] mm. Dix instants espacés de 0,53 s
## (une goutte traverse la tranche en moins de 0,1 s : comptes indépendants). Attendu par classe : densité de Marshall et
## Palmer × aire de la boîte vue × 0,5 m ; les taches qui se touchent sont rendues à leur nombre par l'aire médiane.
func controle_gouttes() -> void:
	monde_env.tonemap_mode = Environment.TONE_MAPPER_LINEAR
	monde_env.glow_enabled = false
	texte.visible = false
	var r := pluie_mm_h if pluie_mm_h > 0.0 else 10.0
	pluie_mm_h = r
	# Les gouttes seules : décor caché, fond noir, sans anticrénelage (il mêle les bords des points au fond).
	for enfant in get_children():
		if enfant is MeshInstance3D:
			enfant.visible = false
	monde_env.background_mode = Environment.BG_COLOR
	monde_env.background_color = Color(0, 0, 0)
	get_viewport().msaa_3d = Viewport.MSAA_DISABLED
	get_viewport().screen_space_aa = Viewport.SCREEN_SPACE_AA_DISABLED
	get_viewport().use_taa = false
	camera.projection = Camera3D.PROJECTION_ORTHOGONAL
	camera.size = 7.9
	camera.position = Vector3(0.0, 20.0, 0.0)
	camera.look_at(Vector3(0.0, 0.0, 0.0), Vector3(0, 0, -1))
	var vp := get_viewport().get_visible_rect().size
	var cote_z := camera.size
	var cote_x := camera.size * vp.x / vp.y
	pluie_air.configurer(r)
	monde_env.fog_enabled = false
	pluie_air.controle = true
	pluie_air.tranche = Vector2(16.0, 16.5)
	pluie_air.taille_controle = 2.0 * camera.size / vp.y
	var bornes := [[1.0, 1.5], [1.5, 2.0], [2.0, 3.0], [3.0, 6.0]]
	var aires_par_classe := [[], [], [], []]
	var instants := 10
	for i in instants:
		pluie_air.suivre(camera, 3.1 + 0.53 * float(i))
		for _k in 3:
			await RenderingServer.frame_post_draw
		var image_controle := get_viewport().get_texture().get_image()
		if i == 0:
			image_controle.save_png(ProjectSettings.globalize_path("res://captures/controle_gouttes_s380.png"))
		var par_classe := taches_colorees(image_controle)
		for c in 4:
			aires_par_classe[c].append_array(par_classe[c])
	var pire := 0.0
	for c in 4:
		var d0: float = bornes[c][0]
		var d1: float = bornes[c][1]
		# La boîte de la classe : ±4 m (diamètres sous 2 mm) ou ±8 m ; l'aire vue en est l'intersection avec l'image.
		var demi := 4.0 if d1 <= 2.0 else 8.0
		var aire := minf(2.0 * demi, cote_x) * minf(2.0 * demi, cote_z)
		var attendu := Pluie.densite_gouttes(r, d0, d1) * aire * 0.5 * instants
		var aires: Array = aires_par_classe[c]
		var triees := aires.duplicate()
		triees.sort()
		var mediane := float(triees[triees.size() / 2]) if not triees.is_empty() else 1.0
		var total := 0
		for a in aires:
			total += maxi(1, roundi(float(a) / mediane))
		var ecart := (float(total) - attendu) / attendu
		# Le critère écrit (±5 %) oubliait le bruit de Poisson : une classe n'y est jugée que si son écart-type relatif
		# est sous 2,5 % ; sinon, à deux écarts-types.
		var sigma := 1.0 / sqrt(attendu)
		if sigma <= 0.025:
			pire = maxf(pire, absf(ecart))
		elif absf(ecart) > 2.0 * sigma:
			pire = maxf(pire, 1.0)
		print("CONTROLE_GOUTTES_S380 pluie_mm_h=%s classe=[%s;%s) aire_m2=%.2f taches=%d mediane_px=%d max_px=%d gouttes=%d attendues=%.1f ecart=%+.2f%% (poisson 1 sigma %.2f%%)" % [
			mm_h(r), str(d0), str(d1), aire, aires.size(), int(mediane), int(triees[-1]) if not triees.is_empty() else 0, total, attendu, 100.0 * ecart, 100.0 / sqrt(attendu)])
	print("CONTROLE_GOUTTES_S380 critere=2 pire_mesurable=%.2f%% %s" % [100.0 * pire, "tenu" if pire <= 0.05 else "manque"])
	get_tree().quit()


## Les aires (pixels) des taches de chaque couleur pure — rouge, vert, bleu, blanc —, en 4-connexité.
func taches_colorees(image: Image) -> Array:
	image.convert(Image.FORMAT_RGB8)
	var w := image.get_width()
	var h := image.get_height()
	var d := image.get_data()
	var classe := PackedByteArray()
	classe.resize(w * h)
	for i in w * h:
		var r := d[3 * i]
		var g := d[3 * i + 1]
		var b := d[3 * i + 2]
		if r > 204 and g > 204 and b > 204:
			classe[i] = 4
		elif r >= 128 and g <= 51 and b <= 51:
			classe[i] = 1
		elif g >= 128 and r <= 51 and b <= 51:
			classe[i] = 2
		elif b >= 128 and r <= 51 and g <= 51:
			classe[i] = 3
	var resultat := [[], [], [], []]
	for i in w * h:
		var k := classe[i]
		if k == 0:
			continue
		var n := 0
		var pile := [i]
		classe[i] = 0
		while not pile.is_empty():
			var c: int = pile.pop_back()
			n += 1
			var x := c % w
			for v in [c - 1 if x > 0 else -1, c + 1 if x < w - 1 else -1, c - w, c + w]:
				if v >= 0 and v < w * h and classe[v] == k:
					classe[v] = 0
					pile.append(v)
		resultat[k - 1].append(n)
	return resultat


## **Critères 2 à 4 de S381** — le ciel de pluie, relu dans un tampon flottant, tonalité linéaire, sans brume ni halo.
## (2) la radiance du ciel couvert au centre de l'image, visée à 1, 15, 30, 60 et 89,5° d'élévation, dos au soleil,
## contre `Lz·(1 + 2·sin h)/3` ; ses trois canaux ; (3) visée vers le soleil (58° d'élévation) : la radiance du ciel clair
## (disque compris) et celle du ciel couvert, contre la CIE ; (4) le sol, face horizontale mate, vu d'aplomb sous le ciel
## clair et sous le ciel couvert : même radiance ; une face verticale, le rapport attendu.
func controle_ciel() -> void:
	get_viewport().use_hdr_2d = true
	monde_env.tonemap_mode = Environment.TONE_MAPPER_LINEAR
	monde_env.glow_enabled = false
	monde_env.fog_enabled = false
	texte.visible = false
	pluie_air.configurer(0.0)
	var soleil_b := Vector3(-0.4, 0.3, 0.8).normalized()
	var h_soleil := asin(soleil_b.z)
	var lz := (9.0 / 7.0) * 2.0 * (0.6 + 0.4 * soleil_b.z)
	var az_soleil := atan2(soleil_b.x, soleil_b.y)
	var pire := 0.0
	var ecart_canaux := 0.0
	for m in materiaux_ciel:
		m.set_shader_parameter("couvert", 1.0)
	for h_deg in [1.0, 15.0, 30.0, 60.0, 89.5]:
		var c := await radiance_centre(az_soleil + PI, deg_to_rad(h_deg))
		var attendu := lz * (1.0 + 2.0 * sin(deg_to_rad(h_deg))) / 3.0
		var e := (c.g - attendu) / attendu
		pire = maxf(pire, absf(e))
		ecart_canaux = maxf(ecart_canaux, (maxf(c.r, maxf(c.g, c.b)) - minf(c.r, minf(c.g, c.b))) / c.g)
		print("CONTROLE_CIEL_S381 critere=2 h=%.1f rendu=(%.4f, %.4f, %.4f) cie=%.4f ecart=%+.3f%%" % [h_deg, c.r, c.g, c.b, attendu, 100.0 * e])
	print("CONTROLE_CIEL_S381 critere=2 pire=%.3f%% canaux=%.3f%% %s" % [100.0 * pire, 100.0 * ecart_canaux, "tenu" if pire <= 0.01 and ecart_canaux <= 0.01 else "manque"])
	var couvert_soleil := await radiance_centre(az_soleil, h_soleil)
	for m in materiaux_ciel:
		m.set_shader_parameter("couvert", 0.0)
	var clair_soleil := await radiance_centre(az_soleil, h_soleil)
	var attendu_s := lz * (1.0 + 2.0 * sin(h_soleil)) / 3.0
	var e_s := (couvert_soleil.g - attendu_s) / attendu_s
	print("CONTROLE_CIEL_S381 critere=3 vers_le_soleil clair=%.4f couvert=%.4f cie=%.4f ecart=%+.3f%% %s" % [clair_soleil.g, couvert_soleil.g, attendu_s, 100.0 * e_s, "tenu" if absf(e_s) <= 0.01 else "manque"])
	# (4) Le sol vu d'aplomb, loin du bassin ; une face verticale : le mur ouest du bloc, vu de l'ouest.
	var resultats := []
	for couvert in [0.0, 1.0]:
		for m in materiaux_ciel:
			m.set_shader_parameter("couvert", couvert)
		camera.projection = Camera3D.PROJECTION_PERSPECTIVE
		camera.position = Vector3(-12.0, 6.0, 0.0)
		camera.look_at(Vector3(-12.0, 0.0, -0.01), Vector3.UP)
		for _k in 4:
			await RenderingServer.frame_post_draw
		var sol := moyenne_centre(get_viewport().get_texture().get_image())
		camera.position = Vector3(-10.0, 1.0, 0.0)
		camera.look_at(Vector3(0.0, 1.0, 0.0), Vector3.UP)
		for _k in 4:
			await RenderingServer.frame_post_draw
		var mur := moyenne_centre(get_viewport().get_texture().get_image())
		resultats.append([sol, mur])
	var r_sol: float = resultats[1][0].g / resultats[0][0].g
	var r_mur: float = resultats[1][1].g / resultats[0][1].g
	# Le mur ouest : normale (−1, 0, 0) dans Godot, (−1, 0, 0) dans B ; sous le ciel clair `0,6 + 0,4·max(n·s, 0)`.
	var clair_mur := 0.6 + 0.4 * maxf(-soleil_b.x, 0.0)
	# S382 : la verticale exacte du ciel couvert de la CIE, (π/2 + 4/3)/(7π/3) = 0,39618.
	var attendu_mur := (0.6 + 0.4 * soleil_b.z) * ((PI / 2.0 + 4.0 / 3.0) / (7.0 * PI / 3.0) + 0.2 * 0.5) / clair_mur
	print("CONTROLE_CIEL_S381 critere=4 sol clair=%.4f couvert=%.4f rapport=%.5f %s ; mur_ouest rapport=%.4f attendu=%.4f" % [
		resultats[0][0].g, resultats[1][0].g, r_sol, "tenu" if absf(r_sol - 1.0) <= 0.01 else "manque", r_mur, attendu_mur])
	get_tree().quit()


## S382 — **le critère 2** (ADR-206) : la part du ciel vue, rendue directement par les surfaces (`controle_occultation` :
## rouge = ciel uniforme, vert = ciel couvert de la CIE), lue au centre de l'image en des points d'essai — caméra à 0,6 m le
## long de la normale, visant le point. Les occultants et les points sont imprimés pour `outils/occultation_ciel.py`, qui
## refait l'intégrale indépendamment (rayons 3D contre boîtes, grille fine). Puis la lumière elle-même, au sol et au mur :
## radiance rendue avec et sans occultants sous le ciel couvert.
func controle_occultation() -> void:
	get_viewport().use_hdr_2d = true
	monde_env.tonemap_mode = Environment.TONE_MAPPER_LINEAR
	monde_env.glow_enabled = false
	monde_env.fog_enabled = false
	texte.visible = false
	pluie_air.configurer(0.0)
	var lx: float = geo["lx"]
	var lz: float = geo["lz"]
	var e: float = geo["e"]
	var sol_y: float = geo["sol_y"]
	var haut: float = geo["haut"]
	var y_eau := eau_bassin.global_position.y
	var points := []
	for d in [0.05, 0.25, 0.5, 1.0, 2.0, 4.0]:
		points.append(["sol_ouest_%.2f" % d, Vector3(-lx - e - d, sol_y, 0.0), Vector3.UP])
	points.append(["sol_sud_10", Vector3(0.0, sol_y, lz + e + 10.0), Vector3.UP])
	points.append(["sol_loin_150", Vector3(0.0, sol_y, lz + e + 150.0), Vector3.UP])
	points.append(["mur_ouest_mi", Vector3(-lx - e, 0.5 * (sol_y + haut), 0.0), Vector3.LEFT])
	points.append(["mur_ouest_haut", Vector3(-lx - e, haut - 0.05, 0.0), Vector3.LEFT])
	points.append(["fond_coin_no", Vector3(-lx + 0.1, geo["fond_y"], -lz + 0.1), Vector3.UP])
	points.append(["fond_centre", Vector3(0.0, geo["fond_y"], 0.0), Vector3.UP])
	points.append(["carrelage_nord_mi", Vector3(0.0, 0.5 * (geo["fond_y"] + haut), -lz + 0.01), Vector3.BACK])
	points.append(["eau_pres_nord", Vector3(0.0, y_eau, -lz + 0.1), Vector3.UP])
	points.append(["margelle_ouest", Vector3(-lx - 0.5 * e, haut + 0.05, 0.0), Vector3.UP])
	var boites := []
	for i in occultants_min.size():
		var a := occultants_min[i]
		var b := occultants_max[i]
		boites.append([a.x, a.y, a.z, b.x, b.y, b.z])
	print("CONTROLE_OCCULTATION_S382 occultants=%s" % JSON.stringify(boites))
	for m in materiaux_ciel:
		m.set_shader_parameter("controle_occultation", true)
	for pt in points:
		var p: Vector3 = pt[1]
		var n: Vector3 = pt[2]
		camera.position = p + 0.6 * n
		camera.look_at(p, Vector3(0, 0, -1) if absf(n.y) > 0.99 else Vector3.UP)
		for _k in 4:
			await RenderingServer.frame_post_draw
		var v := moyenne_centre(get_viewport().get_texture().get_image())
		print("CONTROLE_OCCULTATION_S382 point=%s p=[%.5f,%.5f,%.5f] n=[%.0f,%.0f,%.0f] vu_uniforme=%.5f vu_cie=%.5f" % [
			pt[0], p.x, p.y, p.z, n.x, n.y, n.z, v.r, v.g])
	for m in materiaux_ciel:
		m.set_shader_parameter("controle_occultation", false)
		m.set_shader_parameter("couvert", 1.0)
	# La lumière : radiance du sol à 0,25 m et du mur ouest à mi-hauteur, avec et sans occultants (ciel couvert).
	var decalages := []
	for rc in recepteurs:
		decalages.append((rc[1] as ShaderMaterial).get_shader_parameter("vu_decalage"))
	for pt in [points[1], points[8]]:
		var r := []
		for avec in [true, false]:
			for k in recepteurs.size():
				(recepteurs[k][1] as ShaderMaterial).set_shader_parameter("vu_decalage", decalages[k] if avec else -1)
			var p: Vector3 = pt[1]
			var n: Vector3 = pt[2]
			camera.position = p + 0.6 * n
			camera.look_at(p, Vector3(0, 0, -1) if absf(n.y) > 0.99 else Vector3.UP)
			for _k in 4:
				await RenderingServer.frame_post_draw
			r.append(moyenne_centre(get_viewport().get_texture().get_image()).g)
		print("CONTROLE_OCCULTATION_S382 lumiere point=%s avec=%.5f sans=%.5f rapport=%.5f" % [pt[0], r[0], r[1], r[0] / r[1]])
	get_tree().quit()


## S383 — **les critères 3 à 5 des gerbes**. (3, 4) Vue orthographique d'aplomb d'une partie du bassin, 2 mm par pixel :
## les cœurs d'anneaux de moins de 30 ms en rouge (contrôle de S379, disques d'1 cm de rayon) et, par-dessus, un carré vert d'1
## cm au centre de chaque gerbe de moins de 30 ms ; vingt instants, images écrites dans `captures/controle_gerbes_*.png`,
## lues par `outils/controle_gerbes.py` (chaque vert au centre d'un rouge ; nombre contre `taux × 0,03 s × aire`). (5) La
## gerbe de référence (s = 1) seule, vue de profil sur fond noir, 0,05 mm par pixel, à 1, 3, 7, 12, 18, 41 et 52 ms : la
## hauteur de son sommet (pixel le plus haut dont la radiance dépasse 2 % de celle du dessin), contre les relevés de P2.
func controle_gerbes() -> void:
	monde_env.tonemap_mode = Environment.TONE_MAPPER_LINEAR
	monde_env.glow_enabled = false
	monde_env.fog_enabled = false
	texte.visible = false
	pluie_air.configurer(0.0)
	var r := pluie_mm_h if pluie_mm_h > 0.0 else 10.0
	pluie_mm_h = r
	var u: Vector4 = Pluie.uniformes(r)
	var centre := godot(donnees["bassin"]["fond_m"])
	var y_eau := eau_bassin.global_position.y
	camera.projection = Camera3D.PROJECTION_ORTHOGONAL
	camera.size = 1.44
	camera.position = Vector3(centre.x - 1.0, y_eau + 5.0, centre.z)
	camera.look_at(Vector3(centre.x - 1.0, y_eau, centre.z), Vector3(0, 0, -1))
	materiau_tampon.set_shader_parameter("pluie", Vector4.ZERO)
	gerbes.niveaux = [y_eau, eau_tampon.global_position.y]
	gerbes.configurer(r)
	gerbes.controle = true
	gerbes.taille_controle = 0.01
	gerbes.vie_controle = 0.03
	var largeur := 1.44 * 1280.0 / 720.0
	print("CONTROLE_GERBES_S383 pluie_mm_h=%s taux=%.2f aire_m2=%.4f m_par_px=%.6f" % [mm_h(r), u.x, largeur * 1.44, 1.44 / 720.0])
	for i in 20:
		var s := 3.1 + 1.37 * float(i)
		materiau_bassin.set_shader_parameter("temps", s)
		materiau_bassin.set_shader_parameter("pluie", u)
		materiau_bassin.set_shader_parameter("controle_pluie", true)
		gerbes.suivre(camera, s)
		for _k in 3:
			await RenderingServer.frame_post_draw
		var chemin := ProjectSettings.globalize_path("res://captures/controle_gerbes_%02d.png" % i)
		get_viewport().get_texture().get_image().save_png(chemin)
	print("CONTROLE_GERBES_S383 images=20 fichiers=captures/controle_gerbes_*.png")
	# (5) La gerbe seule, de profil.
	materiau_bassin.set_shader_parameter("controle_pluie", false)
	gerbes.controle = false
	gerbes.seule = true
	for enfant in get_children():
		if enfant is MeshInstance3D:
			enfant.visible = false
	monde_env.background_mode = Environment.BG_COLOR
	monde_env.background_color = Color(0, 0, 0)
	get_viewport().use_hdr_2d = true
	var base := Vector3(0.0, 0.0, 0.0)
	camera.size = 0.036
	camera.position = base + Vector3(0.0, 0.016, 1.0)
	camera.look_at(base + Vector3(0.0, 0.016, 0.0), Vector3.UP)
	for t_ms in [1.0, 3.0, 7.0, 12.0, 18.0, 41.0, 52.0]:
		# L'âge rendu est la fin de la pose : la silhouette y est la moyenne des 16,7 ms d'avant ; `outils/controle_gerbes.py`
		# compare donc le sommet au plus haut des relevés sur les quatre instants de cette pose.
		gerbes.gerbe_seule = Vector4(base.x, base.y, base.z, t_ms / 1000.0)
		gerbes.suivre(camera, 0.0)
		for _k in 3:
			await RenderingServer.frame_post_draw
		var im := get_viewport().get_texture().get_image()
		im.save_png(ProjectSettings.globalize_path("res://captures/controle_gerbe_seule_%02dms.png" % int(t_ms)))
		im.convert(Image.FORMAT_RGBF)
		var w := im.get_width()
		var hh := im.get_height()
		var ref := 0.0
		for j in hh:
			for i in range(w / 2 - 40, w / 2 + 40):
				ref = maxf(ref, im.get_pixel(i, j).g)
		var haut := -1
		for j in hh:
			for i in range(0, w, 2):
				if im.get_pixel(i, j).g > 0.02 * ref:
					haut = j
					break
			if haut >= 0:
				break
		# La rangée j est à y = 0,016 + (hh/2 − j − 0,5)·(0,036/hh) au-dessus de l'eau.
		var y_haut := 0.016 + (0.5 * hh - float(haut) - 0.5) * (0.036 / hh) if haut >= 0 else 0.0
		print("CONTROLE_GERBES_S383 critere=5 t_ms=%.0f sommet_mm=%.2f" % [t_ms, 1000.0 * y_haut])
	get_tree().quit()


## S382 — **le critère 4** : l'ombre de l'arête haute et extérieure de la margelle sud (hauteur `H` au-dessus du sol) tombe
## sur le sol à `z = z_arête + H·(−s_z)/s_y` (s, direction du soleil dans Godot), sur la ligne x = 0. Vue orthographique
## d'aplomb, **5 mm par pixel** (720 pixels pour 3,6 m) ; la part du soleil rendue directement (bleu du contrôle), moyennée
## sur les 9 colonnes du centre ; le bord est où elle passe ½ (interpolation linéaire). Puis la même vue éclairée, ombres
## allumées et éteintes : les pixels où la part du soleil vaut 1 doivent être identiques.
func controle_ombre() -> void:
	get_viewport().use_hdr_2d = true
	monde_env.tonemap_mode = Environment.TONE_MAPPER_LINEAR
	monde_env.glow_enabled = false
	monde_env.fog_enabled = false
	texte.visible = false
	pluie_air.configurer(0.0)
	var s := Vector3(-0.4, 0.3, 0.8).normalized()
	var s_monde := Vector3(s.x, s.z, -s.y)
	var z_arete: float = float(geo["lz"]) + float(geo["e"]) + 0.1
	var h: float = float(geo["haut"]) + 0.05 - float(geo["sol_y"])
	var z_attendu := z_arete + h * (-s_monde.z) / s_monde.y
	camera.projection = Camera3D.PROJECTION_ORTHOGONAL
	camera.size = 3.6
	camera.position = Vector3(0.0, float(geo["sol_y"]) + 10.0, z_attendu)
	camera.look_at(Vector3(0.0, float(geo["sol_y"]), z_attendu), Vector3(0, 0, -1))
	for m in materiaux_ciel:
		m.set_shader_parameter("couvert", 0.0)
		m.set_shader_parameter("controle_occultation", true)
	for _k in 4:
		await RenderingServer.frame_post_draw
	var im := get_viewport().get_texture().get_image()
	im.convert(Image.FORMAT_RGBF)
	var w := im.get_width()
	var hh := im.get_height()
	var metres_par_pixel := 3.6 / hh
	# L'image : le haut est vers −z (le nord) ; la ligne j est à z = z_attendu + (j + 0,5 − hh/2)·mpp.
	var profil := []
	for j in hh:
		var somme := 0.0
		for i in range(w / 2 - 4, w / 2 + 5):
			somme += im.get_pixel(i, j).b
		profil.append(somme / 9.0)
	var z_mesure := NAN
	for j in hh - 1:
		# Le sol seulement : au-delà de l'arête (au nord, le bloc et l'eau).
		if z_attendu + (j + 0.5 - 0.5 * hh) * metres_par_pixel < z_arete + 0.05:
			continue
		if profil[j] >= 0.5 and profil[j + 1] < 0.5 or profil[j] < 0.5 and profil[j + 1] >= 0.5:
			var f: float = (0.5 - profil[j]) / (profil[j + 1] - profil[j])
			z_mesure = z_attendu + (j + 0.5 + f - 0.5 * hh) * metres_par_pixel
			break
	var ecart_px := (z_mesure - z_attendu) / 0.005
	print("CONTROLE_OMBRE_S382 critere=4 z_attendu=%.4f z_mesure=%.4f ecart=%.2f mm (%.2f pixel de 5 mm) %s" % [
		z_attendu, z_mesure, 1000.0 * (z_mesure - z_attendu), ecart_px, "tenu" if absf(ecart_px) <= 1.0 else "manque"])
	var largeur := 0
	for j in hh:
		var z := z_attendu + (j + 0.5 - 0.5 * hh) * metres_par_pixel
		if z > z_arete + 0.05 and profil[j] > 0.02 and profil[j] < 0.98:
			largeur += 1
	# La pénombre attendue : le bord pour chaque direction du bord du disque solaire (demi-angle 0,266°).
	var t1 := s_monde.cross(Vector3.UP).normalized()
	var t2 := s_monde.cross(t1)
	var zmin := INF
	var zmax := -INF
	for k in 360:
		var a := deg_to_rad(float(k))
		var sd := (s_monde + 0.00465 * (cos(a) * t1 + sin(a) * t2)).normalized()
		var z := z_arete + h * (-sd.z) / sd.y
		zmin = minf(zmin, z)
		zmax = maxf(zmax, z)
	print("CONTROLE_OMBRE_S382 penombre=%.1f mm (%d pixels entre 2 et 98 %%) ; disque entier %.1f mm" % [
		largeur * metres_par_pixel * 1000.0, largeur, (zmax - zmin) * 1000.0])
	# Hors de l'ombre, rien ne change : la vue éclairée, ombres allumées puis éteintes.
	var masque_soleil := im
	for m in materiaux_ciel:
		m.set_shader_parameter("controle_occultation", false)
	var images := []
	for ombres in [true, false]:
		for m in materiaux_ciel:
			m.set_shader_parameter("ombres_soleil", ombres)
		for _k in 4:
			await RenderingServer.frame_post_draw
		var e := get_viewport().get_texture().get_image()
		e.convert(Image.FORMAT_RGBF)
		images.append(e)
	var eclaires := 0
	var differents := 0
	for j in hh:
		for i in w:
			if masque_soleil.get_pixel(i, j).b >= 1.0:
				eclaires += 1
				if images[0].get_pixel(i, j) != images[1].get_pixel(i, j):
					differents += 1
	print("CONTROLE_OMBRE_S382 hors_ombre pixels=%d differents=%d %s" % [eclaires, differents, "tenu" if differents == 0 else "manque"])
	get_tree().quit()


## La radiance (linéaire) au centre de l'image, caméra au-dessus du bassin visant l'azimut `az` (B : 0 au nord, sens
## horaire vers l'est) et l'élévation `h`.
func radiance_centre(az: float, h: float) -> Color:
	camera.projection = Camera3D.PROJECTION_PERSPECTIVE
	camera.position = Vector3(0.0, 3.0, 0.0)
	var d_b := Vector3(sin(az) * cos(h), cos(az) * cos(h), sin(h))
	var d := Vector3(d_b.x, d_b.z, -d_b.y)
	var haut := Vector3.UP if absf(d.y) < 0.99 else Vector3(0, 0, -1)
	camera.look_at(camera.position + d, haut)
	for _k in 4:
		await RenderingServer.frame_post_draw
	return moyenne_centre(get_viewport().get_texture().get_image())


## La moyenne linéaire d'un carré de 9 × 9 pixels au centre d'une image flottante.
func moyenne_centre(image: Image) -> Color:
	image.convert(Image.FORMAT_RGBF)
	var cx := image.get_width() / 2
	var cy := image.get_height() / 2
	var somme := Color(0, 0, 0)
	for j in range(-4, 5):
		for i in range(-4, 5):
			somme += image.get_pixel(cx + i, cy + j)
	return somme / 81.0
