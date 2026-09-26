extends Node3D
## S357, ADR-192 D2 — la mer de B dans Godot : le prototype.
## Données : `res://donnees/mer_b.json`, exportées par l'afficheur (`--meilleur --export-godot`), dérivées, non versionnées.
## Lancer : `Godot --path godot` — vue animée ; touches 1 à 4 : poses de R14 (référence, proche, rasante, haute) ;
## Échap : quitter. `Godot --path godot -- --captures` : les quatre poses à 12 s, en PNG dans `captures/`, puis quitte.
## `Godot --headless --path godot -- --controle` : la hauteur de la bande recalculée ici contre le cœur, puis quitte.
## S379 — la pluie, factice (ADR-202 D3) : `PLUIE=<mm/h>`, ou la touche P (0, 2, 10, 50 mm/h) ; `pluie.gd`,
## `pluie.gdshaderinc`, comme la piscine. Sans pluie, l'image d'avant au bit. `-- --cout-pluie` : son temps GPU.

## La grille polaire autour de la caméra : dense près de l'œil, lâche au loin, sans couture.
const ANGLES := 720
const RAYONS := 360
const R_MIN := 0.25
const R_MAX := 12000.0
## S371 — des anneaux **sous R_MIN**, au même pas logarithmique (3 % du rayon), jusqu'à 1,2 cm : vu de 4 cm, l'éventail
## central de 0,25 m montrait ses facettes (pentes interpolées sur un seul triangle). Les anneaux de R_MIN à R_MAX sont
## inchangés ; les nouveaux ne sont vus que caméra au ras de l'eau.
const ANNEAUX_INTERIEURS := 100
## Les poses de R14 : position dans Godot (x, hauteur, −y de B) et tangage (rad).
const POSES := {
	"reference": [Vector3(0.0, 7.0, 18.0), -0.13135],
	"proche": [Vector3(0.0, 4.0, 7.0), -0.18],
	"rasante": [Vector3(0.0, 2.0, 18.0), -0.05],
	"haute": [Vector3(0.0, 22.0, 34.0), -0.42],
	## S359 : plongeante, pour voir le fond de la scène côtière à travers la surface.
	"plongeante": [Vector3(0.0, 12.0, 30.0), -0.75],
	## S365 — sous la surface (liste 8.6) : au zénith, à 5 m, pour la fenêtre de Snell ; à 3 m dans la scène côtière,
	## visée horizontale, puis vers le fond (6,85 m de fond sous la caméra).
	"sous_eau_zenith": [Vector3(0.0, -5.0, 0.0), PI / 2.0],
	"sous_eau": [Vector3(0.0, -3.0, 20.0), 0.0],
	"sous_eau_fond": [Vector3(0.0, -3.0, 20.0), -0.6],
	## S366 : en contre-plongée à 4 m, le cadrage de la photographie de référence (Hanifaru, Maldives) : la surface en haut,
	## l'eau en bas.
	"sous_eau_oblique": [Vector3(0.0, -4.0, 0.0), 0.52],
	## S366 : à l'horizontale, face au soleil (son azimut, (−0,4 ; 0,3) dans B) et dos à lui — le lobe avant.
	"sous_eau_vers_soleil": [Vector3(0.0, -4.0, 0.0), 0.0, 0.9273],
	"sous_eau_dos_soleil": [Vector3(0.0, -4.0, 0.0), 0.0, 0.9273 + PI],
	## S371 — **à demi immergée** (ADR-019 §6) : la hauteur est **relative à la surface exacte** au centre du plan proche
	## (`pose()`), la ligne d'eau passe donc au centre de l'image à 0. Visée horizontale vers le large ; face au soleil ;
	## un peu au-dessus, plongeante ; un peu en dessous, en contre-plongée.
	"demi": [Vector3(0.0, 0.0, 20.0), 0.0],
	"demi_soleil": [Vector3(0.0, 0.0, 20.0), 0.0, 0.9273],
	"demi_dessus": [Vector3(0.0, 0.04, 20.0), -0.12],
	"demi_dessous": [Vector3(0.0, -0.04, 20.0), 0.12],
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
## S366 — le lobe avant de la lumière de l'eau (β, K, g), ajusté sur la radiance mesurée par Tyler (1960), lac Pend
## Oreille, 4,2 m (`outils/tyler_radiance.py`) : résidu logarithmique 0,12. Eau de lac : lobe emprunté par notre eau pure.
const LOBE_TYLER := Vector3(1.0031, 213.349, 0.855)

var donnees: Dictionary
var materiau: ShaderMaterial
var camera: Camera3D
var mer: MeshInstance3D
var t0 := 12.0
var temps := 12.0
var anime := true
## S360 : la surface fine par FFT (`detail.gd`), si l'export la porte.
var detail: Node
## S361 : le matériau du fond, et la carte de caustiques — vue orthographique hors écran, grille dense de la surface.
var materiau_sol: ShaderMaterial
## S365 : le matériau du ciel, qui devient le fond de l'eau quand la caméra y est.
var materiau_ciel: ShaderMaterial
## S366 : l'environnement, dont la brume — un phénomène de l'air — s'éteint quand la caméra est dans l'eau.
var environnement_scene: Environment
## S368 — le champ d'écume sur la carte (`ecume.gd`, production de la référence de S367). **Éteinte par défaut depuis le
## 2026-09-26** (décision de l'utilisateur : l'écume attend des photographies qui en renseignent la forme, la couleur et
## la place sur la vague) : `ECUME=champ` rend celle de S368, `ECUME=ancienne` celle de S360. Centre courant du champ
## (axes de B), et 60 s de passé simulé à chaque recentrage.
var ecume: Node
var ecume_centre := Vector2(INF, INF)
const ECUME_PASSE_S := 60.0
const ECUME_PAS_S := 0.1
## κ du seuil de déferlement `κ·σ_a` : 0 = la relation de S367, `2,5 + ln(W/0,0226)/(−3,52)` ; sinon la valeur recalée.
## **S368, recalé sur la carte** (`--controle-ecume-couverture`, 40 s au régime) : la relation de S367 donne κ = 2,978 et
## 0,62 % pour 0,42 % visés — la bande de l'afficheur n'est pas celle du cœur ; κ = 3,09, prédit par la pente de S367,
## donne 0,405 % (rapport 0,96) ; 3,00 → 1,36, 3,20 → 0,62.
const KAPPA_ECUME := 3.09
## S371 — la caméra à demi immergée (ADR-019 §6) : le plan proche coupe-t-il la surface ? Alors le milieu se décide par
## pixel (`surface_b.gdshaderinc`). Bornes de la bande : |η| (`Σ|a|` et le second ordre de Tayfun, `½·k̄·(Σ|a|)²` par
## système), la pente lagrangienne (`Σ a·k` et `2·k̄·Σ|a|·Σ a·k`), le gradient du déplacement (`G = Σ a·k`).
var marge_vagues := 0.0
var borne_pente := 0.0
var borne_g := 0.0
var demi_actif := false
## La transition de la ligne d'eau, en fraction de la hauteur d'image (S371 P2 : ≈ 1 % sur les photographies de référence).
const TRANSITION_LIGNE := 0.006
## S371 P6 — le ménisque sur le hublot : bande claire et trait sombre, en fraction de la hauteur d'image (photographie A
## de S371 P2 : 6 et 1,5 pixels sur 670).
const MENISQUE_BANDE := 0.009
const MENISQUE_TRAIT := 0.0025
var vue_caustiques: SubViewport
var materiau_caustiques: ShaderMaterial
var maillage_caustiques: MeshInstance3D
## La carte : 64 m de côté, 1 024 texels (6,25 cm) ; la surface source, 100 m, 1 024 × 1 024 sommets (9,8 cm, cinq par
## plus courte longueur d'onde de la cascade de 32 m). Écrite divisée par `CARTE_ECHELLE`.
const CARTE_COTE := 64.0
const CARTE_TEXELS := 1024
const SOURCE_COTE := 100.0
const SOURCE_DIVISIONS := 1023
const CARTE_ECHELLE := 32.0


const Pluie = preload("res://pluie.gd")
var pluie_mm_h := 0.0
## La densité de la brume par temps sec (m⁻¹) ; S380 : la pluie y ajoute son extinction (ADR-205, pièce 2).
const BRUME_SECHE := 0.00012
## S380 — la pluie dans l'air (`pluie_air.gd`, ADR-205 pièce 1) ; les gouttes s'arrêtent au niveau moyen de la mer.
var pluie_air: Node3D
## S383 — les gerbes (`gerbes.gd`, ADR-205 pièce 4), posées sur la surface déplacée.
var gerbes: Node3D


## S381 — le ciel de pluie (ADR-205, pièce 3) : `COUVERT=` (0 à 1) force la couverture ; sinon, la pluie couvre le ciel.
func couvert_voulu() -> float:
	if OS.get_environment("COUVERT") != "":
		return clampf(float(OS.get_environment("COUVERT")), 0.0, 1.0)
	return 1.0 if pluie_mm_h > 0.0 else 0.0


func _ready() -> void:
	if OS.get_environment("PLUIE") != "":
		pluie_mm_h = float(OS.get_environment("PLUIE"))
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
	# S365 : `FOV=120` — le champ vertical, en degrés (la fenêtre de Snell demande de voir au-delà de 48°).
	if OS.get_environment("FOV") != "":
		camera.fov = float(OS.get_environment("FOV"))
	camera.near = 0.1
	camera.far = 20000.0
	add_child(camera)
	camera.current = true
	pose("proche")
	materiau = ShaderMaterial.new()
	materiau.shader = nuanceur_eau
	mer = MeshInstance3D.new()
	mer.mesh = grille_polaire()
	mer.material_override = materiau
	mer.custom_aabb = AABB(Vector3(-R_MAX, -100.0, -R_MAX), Vector3(2.0 * R_MAX, 200.0, 2.0 * R_MAX))
	add_child(mer)
	# S359 : `SANS_EAU=1` masque la mer, pour voir le fond seul.
	mer.visible = OS.get_environment("SANS_EAU") != "1"
	if "--cote" in args or "--controle-fond" in args or "--controle-caustiques" in args or "--controle-caustiques-scene" in args or "--controle-sous-eau" in args or "--controle-ligne-eau" in args:
		add_child(fond())
		# S361 : les caustiques du fond, carte directe ; `CAUSTIQUES=0` les éteint.
		if OS.get_environment("CAUSTIQUES") != "0":
			carte_caustiques()
	uniformes_fixes()
	materiau.set_shader_parameter("ciel_mesure", OS.get_environment("CIEL") != "clair")
	phases(temps)
	if donnees.has("detail"):
		detail = load("res://detail.gd").new()
		add_child(detail)
		if detail.charger(donnees["detail"]) and OS.get_environment("DETAIL") != "0" and OS.get_environment("MER_PLATE") != "1":
			detail.calculer(temps)
			# S360 : l'eau lit les deux cascades ; `DETAIL=0` garde la queue de 60 composantes, en témoin.
			materiau.set_shader_parameter("detail_a0", detail.textures[0][0])
			materiau.set_shader_parameter("detail_b0", detail.textures[0][1])
			materiau.set_shader_parameter("detail_a1", detail.textures[1][0])
			materiau.set_shader_parameter("detail_b1", detail.textures[1][1])
			materiau.set_shader_parameter("detail_cotes", Vector2(float(detail.cotes[0]), float(detail.cotes[1])))
			materiau.set_shader_parameter("detail_texels", float(detail.N))
			materiau.set_shader_parameter("detail_actif", true)
			# S361 : la cascade de 32 m entre dans la carte de caustiques.
			if materiau_caustiques != null:
				materiau_caustiques.set_shader_parameter("detail_a0", detail.textures[0][0])
				materiau_caustiques.set_shader_parameter("detail_c0", detail.textures[0][2])
				materiau_caustiques.set_shader_parameter("detail_cote0", float(detail.cotes[0]))
				materiau_caustiques.set_shader_parameter("caustiques_detail", true)
		elif not "--controle-fft" in args:
			detail = null
	materiau.set_shader_parameter("ecume_visible", OS.get_environment("ECUME") in ["champ", "ancienne"])
	if OS.get_environment("ECUME") == "champ" or "--controle-ecume-champ" in args or "--controle-ecume-couverture" in args:
		ecume = load("res://ecume.gd").new()
		add_child(ecume)
		var pulsations := PackedFloat32Array()
		for r in donnees["bande"]:
			pulsations.append(float(r[4]))
		ecume.initialiser(pulsations, Vector2.ZERO, 9.81)
		ecume.seuil = ecume_seuil()
		materiau.set_shader_parameter("ecume_champ", ecume.texture)
		materiau.set_shader_parameter("ecume_cote", ecume.N * ecume.PAS)
		materiau.set_shader_parameter("ecume_champ_actif", true)
		# La direction dominante des vagues de la bande, pondérée par l'énergie : Σ a²·k̂.
		var d := Vector2.ZERO
		for r in donnees["bande"]:
			var k := Vector2(float(r[1]), float(r[2]))
			if k.length() > 0.0:
				d += float(r[0]) * float(r[0]) * k.normalized()
		materiau.set_shader_parameter("ecume_direction", d.normalized())
	if "--controle-ecume-champ" in args:
		anime = false
		controle_ecume_champ()
		return
	if "--controle-ecume-couverture" in args:
		anime = false
		controle_ecume_couverture()
		return
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
	if "--controle-caustiques" in args:
		anime = false
		controle_caustiques()
	if "--controle-caustiques-scene" in args:
		anime = false
		controle_caustiques_scene()
	if "--controle-sous-eau" in args:
		anime = false
		controle_sous_eau()
	if "--controle-ligne-eau" in args:
		anime = false
		controle_ligne_eau()
	if "--cout-pluie" in args:
		cout_pluie()
	if "--cout-demi" in args:
		anime = false
		cout_demi()


## Le ciel : depuis S359, celui de l'afficheur lui-même (`ciel.gdshaderinc` — dégradé du « ciel clair » relevé sur la
## photographie de référence de l'utilisateur, S261 et R14, nuages, soleil), que l'eau reflète ; S357 en approchait les
## couleurs par le ciel procédural de Godot. Le soleil à la direction de la scène ; tonalité AgX, halo, perspective
## aérienne. S357 P3 : le ciel physique par défaut de Godot rendait un ciel gris de crépuscule.
func environnement() -> void:
	var env := Environment.new()
	var ciel := Sky.new()
	# S359 P6 : le ciel clair de l'afficheur (`ciel.gdshader`), celui que l'eau reflète — une seule source.
	materiau_ciel = ShaderMaterial.new()
	materiau_ciel.shader = load("res://ciel.gdshader")
	# S363 : le ciel calé sur la photographie ; `CIEL=clair` rend le ciel clair d'avant, pour le ciel et ses reflets.
	materiau_ciel.set_shader_parameter("ciel_mesure", OS.get_environment("CIEL") != "clair")
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
	# S363 : les autres courbes de Godot, pour les mesurer contre la photographie (`outils/tonalite_godot.py`) —
	# `TONALITE=reinhard|filmic|aces|agx`, `EXPOSITION`, `BLANC` (AgX l'ignore en 4.4) ; `HALO=0|1` force le halo.
	var courbes := {"reinhard": Environment.TONE_MAPPER_REINHARDT, "filmic": Environment.TONE_MAPPER_FILMIC,
			"aces": Environment.TONE_MAPPER_ACES, "agx": Environment.TONE_MAPPER_AGX}
	if courbes.has(OS.get_environment("TONALITE")):
		env.tonemap_mode = courbes[OS.get_environment("TONALITE")]
	# S363 P4 bis — **`TONALITE=photo`**, une option, pas le défaut : la courbe calée sur la photographie de référence
	# (S308) par `outils/tonalite_godot.py compromis` — les quatre grandeurs de luminance et la teinte des creux
	# ensemble, pose proche : ACES, blanc 4 (rien d'écrêté), saturation 0,5 (ACES seul rend les creux 3,3 fois trop bleus).
	if OS.get_environment("TONALITE") == "photo":
		env.tonemap_mode = Environment.TONE_MAPPER_ACES
		env.tonemap_exposure = 0.983
		env.tonemap_white = 4.0
		env.adjustment_enabled = true
		env.adjustment_saturation = 0.5
	if OS.get_environment("EXPOSITION") != "":
		env.tonemap_exposure = float(OS.get_environment("EXPOSITION"))
	if OS.get_environment("BLANC") != "":
		env.tonemap_white = float(OS.get_environment("BLANC"))
	# Et les ajustements, appliqués par Godot après le passage en sRGB : `CONTRASTE`, `SATURATION`.
	if OS.get_environment("CONTRASTE") != "" or OS.get_environment("SATURATION") != "":
		env.adjustment_enabled = true
		env.adjustment_contrast = float(OS.get_environment("CONTRASTE")) if OS.get_environment("CONTRASTE") != "" else 1.0
		env.adjustment_saturation = float(OS.get_environment("SATURATION")) if OS.get_environment("SATURATION") != "" else 1.0
	# S357 P3 : sans reflets à l'écran — en rasant, leurs rayons retombent sur l'eau elle-même et remplacent le ciel
	# clair de l'horizon par sa propre couleur sombre. `REFLETS_ECRAN=1` les rallume, pour comparer.
	env.ssr_enabled = OS.get_environment("REFLETS_ECRAN") == "1"
	env.glow_enabled = not lineaire
	if OS.get_environment("HALO") != "":
		env.glow_enabled = OS.get_environment("HALO") == "1"
	env.fog_enabled = true
	env.fog_density = BRUME_SECHE
	env.fog_aerial_perspective = 1.0
	env.fog_sky_affect = 0.0
	environnement_scene = env
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
	sol.shader = nuanceur_sol
	materiau_sol = sol
	var instance := MeshInstance3D.new()
	instance.mesh = m
	instance.material_override = sol
	return instance


## S361 — la carte de caustiques : une vue orthographique hors écran, son propre monde, fond noir, tonalité linéaire,
## tampon HDR ; une grille dense de la surface que `caustiques.gdshader` projette sur le fond ; la bathymétrie en
## texture, tirée de `profondeur()` aux sommets mêmes de la grille du fond — une seule source.
func carte_caustiques() -> void:
	vue_caustiques = SubViewport.new()
	vue_caustiques.size = Vector2i(CARTE_TEXELS, CARTE_TEXELS)
	vue_caustiques.own_world_3d = true
	vue_caustiques.use_hdr_2d = true
	vue_caustiques.msaa_3d = Viewport.MSAA_DISABLED
	vue_caustiques.render_target_update_mode = SubViewport.UPDATE_ALWAYS
	add_child(vue_caustiques)
	var noir := Environment.new()
	noir.background_mode = Environment.BG_COLOR
	noir.background_color = Color.BLACK
	noir.tonemap_mode = Environment.TONE_MAPPER_LINEAR
	var oeil := Camera3D.new()
	oeil.environment = noir
	oeil.projection = Camera3D.PROJECTION_ORTHOGONAL
	oeil.size = 200.0
	oeil.far = 2000.0
	oeil.position = Vector3(0.0, 500.0, 0.0)
	oeil.rotation = Vector3(-PI / 2.0, 0.0, 0.0)
	vue_caustiques.add_child(oeil)
	oeil.current = true
	var grille := PlaneMesh.new()
	grille.size = Vector2(SOURCE_COTE, SOURCE_COTE)
	grille.subdivide_width = SOURCE_DIVISIONS
	grille.subdivide_depth = SOURCE_DIVISIONS
	maillage_caustiques = MeshInstance3D.new()
	maillage_caustiques.mesh = grille
	materiau_caustiques = ShaderMaterial.new()
	materiau_caustiques.shader = load("res://caustiques.gdshader")
	maillage_caustiques.material_override = materiau_caustiques
	maillage_caustiques.custom_aabb = AABB(Vector3(-1e5, -1e4, -1e5), Vector3(2e5, 2e4, 2e5))
	vue_caustiques.add_child(maillage_caustiques)
	var nx := int(2.0 * FOND_X / FOND_PAS) + 1
	var ny := int((float(FOND_Y[1]) - float(FOND_Y[0])) / FOND_PAS) + 1
	var bathy := Image.create_empty(nx, ny, false, Image.FORMAT_RF)
	for j in ny:
		for i in nx:
			bathy.set_pixel(i, j, Color(profondeur(-FOND_X + FOND_PAS * i, float(FOND_Y[0]) + FOND_PAS * j), 0.0, 0.0))
	materiau_caustiques.set_shader_parameter("bathymetrie", ImageTexture.create_from_image(bathy))
	# Centres des texels sur les sommets de la grille du fond.
	materiau_caustiques.set_shader_parameter("fond_origine", Vector2(-FOND_X - 0.5 * FOND_PAS, float(FOND_Y[0]) - 0.5 * FOND_PAS))
	materiau_caustiques.set_shader_parameter("fond_taille", Vector2(nx * FOND_PAS, ny * FOND_PAS))
	materiau_caustiques.set_shader_parameter("n_bande", donnees["bande"].size())
	materiau_caustiques.set_shader_parameter("caustiques_detail", false)
	materiau_caustiques.set_shader_parameter("cote_carte", CARTE_COTE)
	materiau_caustiques.set_shader_parameter("texels", float(CARTE_TEXELS))
	materiau_caustiques.set_shader_parameter("carte_echelle", CARTE_ECHELLE)
	materiau_sol.set_shader_parameter("carte_caustiques", vue_caustiques.get_texture())
	materiau_sol.set_shader_parameter("carte_cote", CARTE_COTE)
	materiau_sol.set_shader_parameter("carte_texels", float(CARTE_TEXELS))
	materiau_sol.set_shader_parameter("carte_echelle", CARTE_ECHELLE)
	materiau_sol.set_shader_parameter("carte_retournee", OS.get_environment("CARTE_RETOURNEE") == "1")
	materiau_sol.set_shader_parameter("caustiques", true)


## La carte suit la caméra : centrée 25 m devant elle au sol, calée sur ses texels ; la surface source, décalée du
## trajet réfracté moyen (12 m de fond), calée sur les pas de sa grille — pas de scintillement à l'échantillonnage.
func suivre_carte(centre_force = null) -> void:
	if materiau_caustiques == null or camera == null:
		return
	var devant := -camera.global_transform.basis.z
	devant.y = 0.0
	var centre: Vector2
	if centre_force != null:
		centre = centre_force
	else:
		var sol := camera.global_position + 25.0 * (devant.normalized() if devant.length() > 1e-3 else Vector3.ZERO)
		centre = Vector2(sol.x, -sol.z)
	var texel := CARTE_COTE / CARTE_TEXELS
	centre = (centre / texel).round() * texel
	var p0 := Vector2(0.333, -0.250)
	var source := centre - 12.0 * p0
	var pas := SOURCE_COTE / (SOURCE_DIVISIONS + 1)
	source = (source / pas).round() * pas
	maillage_caustiques.position = Vector3(source.x, 0.0, -source.y)
	materiau_caustiques.set_shader_parameter("origine", centre)
	materiau_sol.set_shader_parameter("carte_origine", centre)


func pose(nom: String) -> void:
	var p: Array = POSES[nom]
	camera.position = p[0]
	camera.rotation = Vector3(float(p[1]), float(p[2]) if p.size() > 2 else 0.0, 0.0)
	if nom.begins_with("demi"):
		# S371 : hauteur relative à la surface exacte à l'objectif — le centre du plan proche, dont la position horizontale
		# ne dépend pas de la hauteur.
		var centre := camera.position - camera.near * camera.transform.basis.z
		var eta_c := surface_exacte(Vector2(centre.x, -centre.z), lignes("bande", temps)).x
		camera.position.y = eta_c + float(p[0].y) + (camera.position.y - centre.y)


func grille_polaire() -> ArrayMesh:
	var sommets := PackedVector3Array()
	var anneaux := RAYONS + ANNEAUX_INTERIEURS
	sommets.resize(1 + ANGLES * anneaux)
	var q := log(R_MAX / R_MIN) / float(RAYONS - 1)
	for j in anneaux:
		var r := R_MIN * exp(q * (j - ANNEAUX_INTERIEURS))
		for i in ANGLES:
			var a := TAU * float(i) / float(ANGLES)
			sommets[1 + j * ANGLES + i] = Vector3(r * cos(a), 0.0, r * sin(a))
	var indices := PackedInt32Array()
	indices.resize(3 * ANGLES + 6 * ANGLES * (anneaux - 1))
	var n := 0
	for i in ANGLES:
		indices[n] = 0
		indices[n + 1] = 1 + (i + 1) % ANGLES
		indices[n + 2] = 1 + i
		n += 3
	for j in anneaux - 1:
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
	materiau.set_shader_parameter("n_queue", donnees["queue"].size())
	materiau.set_shader_parameter("modulation_M", float(donnees["modulation_M"]))
	materiau.set_shader_parameter("retard_tours", float(donnees["retard_tours"]))
	# S371 : la bande, sa coupure entre systèmes et ses asymétries (`surface_b.gdshaderinc`) sur l'eau, le fond et le ciel —
	# le milieu du pixel se décide partout sur la même surface.
	for m in [materiau, materiau_sol, materiau_ciel]:
		if m != null:
			m.set_shader_parameter("n_bande", donnees["bande"].size())
			m.set_shader_parameter("split", int(donnees["split"]))
			if donnees["asymetries"]:
				var k: Array = donnees["k_moyens"]
				m.set_shader_parameter("k_moyens", Vector2(float(k[0]), float(k[1])))
	marge_vagues = borne_vagues()
	materiau.set_shader_parameter("ecume_seuils", PackedFloat32Array(donnees["ecume_seuils"]))
	materiau.set_shader_parameter("ecume_seuils_deferlement", PackedFloat32Array(donnees["ecume_seuils_deferlement"]))
	materiau.set_shader_parameter("ecume_empreinte_min", float(donnees["ecume_empreinte_min_m"]))
	# S365 : l'optique de l'eau (`optique_eau.gdshaderinc`) sur les trois matériaux qui la lisent — une seule source, ici.
	for m in [materiau, materiau_sol, materiau_ciel]:
		if m != null:
			m.set_shader_parameter("kd", (ABSORPTION + RETRODIFFUSION) / MU_D)
			m.set_shader_parameter("attenuation_c", ABSORPTION + 2.0 * RETRODIFFUSION)
			m.set_shader_parameter("lobe", LOBE_TYLER if OS.get_environment("LOBE") != "0" else Vector3(2.0, 0.0, 0.5))
	# S365 : `CONTROLE_EAU=4` — la surface vue d'en dessous rend son coefficient de Fresnel eau → air (1 au-delà de l'angle
	# critique), relu par `outils/fenetre_snell.py --fresnel`.
	if OS.get_environment("CONTROLE_EAU") != "":
		materiau.set_shader_parameter("controle", int(OS.get_environment("CONTROLE_EAU")))
	# S365 : `MER_PLATE=1` — aucune vague, pour mesurer la fenêtre de Snell sous une surface plane.
	if OS.get_environment("MER_PLATE") == "1":
		for m in [materiau, materiau_sol, materiau_ciel]:
			if m != null:
				m.set_shader_parameter("n_bande", 0)
		materiau.set_shader_parameter("n_queue", 0)
		marge_vagues = 0.0
	var hauteur := get_viewport().get_visible_rect().size.y
	materiau.set_shader_parameter("angle_pixel", 2.0 * tan(deg_to_rad(camera.fov) / 2.0) / hauteur)
	materiau.set_shader_parameter("pas_radial", log(R_MAX / R_MIN) / float(RAYONS - 1))
	# S371 : le plan proche, la taille d'un pixel sur lui, la transition de la ligne d'eau (`TRANSITION_PX` la force).
	var transition := TRANSITION_LIGNE * hauteur
	if OS.get_environment("TRANSITION_PX") != "":
		transition = float(OS.get_environment("TRANSITION_PX"))
	for m in [materiau, materiau_sol, materiau_ciel]:
		if m != null:
			m.set_shader_parameter("plan_proche", camera.near)
			m.set_shader_parameter("pixel_proche", 2.0 * camera.near * tan(deg_to_rad(camera.fov) / 2.0) / hauteur)
			m.set_shader_parameter("transition_px", transition)
			# S371 P6 : le ménisque, en fraction de la hauteur d'image (photographie A) ; `MENISQUE=0` l'éteint.
			m.set_shader_parameter("menisque_px", MENISQUE_BANDE * hauteur)
			m.set_shader_parameter("trait_px", MENISQUE_TRAIT * hauteur)
			m.set_shader_parameter("menisque_actif", OS.get_environment("MENISQUE") != "0")


## S373 — la densité de la perspective aérienne de nos nuanceurs : celle de l'environnement, sauf contrôle (`brume_voulue`
## faux) ou `BRUME=0`.
var brume_voulue := true
## Les deux variantes de l'eau et du fond : la brume du moteur hors de la zone des vagues, la nôtre à demi immergée.
var nuanceur_eau: Shader = load("res://eau.gdshader")
var nuanceur_eau_demi: Shader = load("res://eau_demi.gdshader")
var nuanceur_sol: Shader = load("res://sol.gdshader")
var nuanceur_sol_demi: Shader = load("res://sol_demi.gdshader")


func brume() -> void:
	var d := environnement_scene.fog_density if environnement_scene != null and brume_voulue and OS.get_environment("BRUME") != "0" else 0.0
	for m in [materiau, materiau_sol]:
		if m != null:
			m.set_shader_parameter("brume_densite", d)


## S371 — la borne de |η| de la bande : `Σ|a|`, et le second ordre de Tayfun, `½·k̄·(Σ|a|)²` par système (ADR-176 D1).
func borne_vagues() -> float:
	var somme := [0.0, 0.0]
	var coupure := int(donnees["split"])
	var i := 0
	for r in donnees["bande"]:
		somme[0 if i < coupure else 1] += absf(float(r[0]))
		i += 1
	var borne: float = somme[0] + somme[1]
	var pentes := [0.0, 0.0]
	i = 0
	for r in donnees["bande"]:
		pentes[0 if i < coupure else 1] += absf(float(r[0])) * Vector2(float(r[1]), float(r[2])).length()
		i += 1
	borne_g = pentes[0] + pentes[1]
	borne_pente = borne_g
	if donnees["asymetries"]:
		var k: Array = donnees["k_moyens"]
		borne += 0.5 * float(k[0]) * somme[0] * somme[0] + 0.5 * float(k[1]) * somme[1] * somme[1]
		borne_pente += 2.0 * float(k[0]) * somme[0] * pentes[0] + 2.0 * float(k[1]) * somme[1] * pentes[1]
	return borne


## S371 P5 — **la surface à l'aplomb de `x` (B) à l'ordre 2**, en double : [η, ∂η/∂x, ∂η/∂y, ∂²η/∂x², ∂²η/∂x∂y, ∂²η/∂y²],
## eulériens. Le point lagrangien `q` par Newton, puis, en `q` : `s = ∇η`, `H = ∇²η` (premier ordre et second de Tayfun :
## `k̄·(sa·saᵀ + e2·Hs − sq·sqᵀ − qd·Hc)` par système), `J = I + g`. Eulérien : `∇η_E = J⁻¹·s`, et
## `∇²η_E = J⁻¹·(H − C)·J⁻¹`, `C = Σ_c n_c·∂²d_c/∂q∂q = −Σ a·k²·cos·(n·u)·u·uᵀ`, `n = ∇η_E` — la dérivée de `J⁻¹`.
func surface_ordre2(x: Vector2, bande_t: PackedVector4Array) -> PackedFloat64Array:
	var n := 0 if OS.get_environment("MER_PLATE") == "1" else bande_t.size()
	var coupure := int(donnees["split"])
	var km := Vector2.ZERO
	if donnees["asymetries"]:
		km = Vector2(float(donnees["k_moyens"][0]), float(donnees["k_moyens"][1]))
	# Le point lagrangien, `q + d(q) = x`, par Newton jusqu'à 10⁻¹² m (quatre pas suffisent : S371 P4).
	var q := x
	for _it in 6:
		var d := Vector2.ZERO
		var jxx := 1.0
		var jxy := 0.0
		var jyy := 1.0
		for i in n:
			var c := bande_t[i]
			var kv := Vector2(c.y, c.z)
			var k := kv.length()
			if k == 0.0:
				continue
			var u := kv / k
			var ph := kv.dot(q) + c.w
			d += c.x * cos(ph) * u
			var sn := sin(ph)
			jxx -= c.x * k * sn * u.x * u.x
			jxy -= c.x * k * sn * u.x * u.y
			jyy -= c.x * k * sn * u.y * u.y
		var f := q + d - x
		var det := jxx * jyy - jxy * jxy
		q -= Vector2(jyy * f.x - jxy * f.y, -jxy * f.x + jxx * f.y) / det if det >= 0.1 else f
		if f.length() < 1e-12:
			break
	# En q : g, s, H (premier ordre), sommes de Tayfun par système.
	var gxx := 0.0
	var gxy := 0.0
	var gyy := 0.0
	var eta := 0.0
	var s := Vector2.ZERO
	var hxx := 0.0
	var hxy := 0.0
	var hyy := 0.0
	var e2 := [0.0, 0.0]
	var qd := [0.0, 0.0]
	var sa := [Vector2.ZERO, Vector2.ZERO]
	var sq := [Vector2.ZERO, Vector2.ZERO]
	var hs := [Vector3.ZERO, Vector3.ZERO]
	var hc := [Vector3.ZERO, Vector3.ZERO]
	var cosinus := PackedFloat64Array()
	cosinus.resize(n)
	for i in n:
		var c := bande_t[i]
		var kv := Vector2(c.y, c.z)
		var k := kv.length()
		if k == 0.0:
			continue
		var u := kv / k
		var ph := kv.dot(q) + c.w
		var cs := cos(ph)
		var sn := sin(ph)
		cosinus[i] = cs
		eta += c.x * sn
		gxx -= c.x * k * sn * u.x * u.x
		gxy -= c.x * k * sn * u.x * u.y
		gyy -= c.x * k * sn * u.y * u.y
		s += c.x * cs * kv
		hxx -= c.x * sn * kv.x * kv.x
		hxy -= c.x * sn * kv.x * kv.y
		hyy -= c.x * sn * kv.y * kv.y
		var j := 0 if i < coupure else 1
		e2[j] += c.x * sn
		qd[j] += c.x * cs
		sa[j] += c.x * cs * kv
		sq[j] -= c.x * sn * kv
		hs[j] -= c.x * sn * Vector3(kv.x * kv.x, kv.x * kv.y, kv.y * kv.y)
		hc[j] -= c.x * cs * Vector3(kv.x * kv.x, kv.x * kv.y, kv.y * kv.y)
	if km.x > 0.0:
		for j in 2:
			var kb: float = km[j]
			var a2: Vector2 = sa[j]
			var q2: Vector2 = sq[j]
			var h2: Vector3 = float(e2[j]) * hs[j] - float(qd[j]) * hc[j]
			eta += 0.5 * kb * (float(e2[j]) * float(e2[j]) - float(qd[j]) * float(qd[j]))
			s += kb * (float(e2[j]) * a2 - float(qd[j]) * q2)
			hxx += kb * (a2.x * a2.x - q2.x * q2.x + h2.x)
			hxy += kb * (a2.x * a2.y - q2.x * q2.y + h2.y)
			hyy += kb * (a2.y * a2.y - q2.y * q2.y + h2.z)
	var jxx2 := 1.0 + gxx
	var jxy2 := gxy
	var jyy2 := 1.0 + gyy
	var det2 := jxx2 * jyy2 - jxy2 * jxy2
	# J⁻¹ (symétrique) et la pente eulérienne.
	var ixx := jyy2 / det2
	var ixy := -jxy2 / det2
	var iyy := jxx2 / det2
	var nx := ixx * s.x + ixy * s.y
	var ny := ixy * s.x + iyy * s.y
	# C = −Σ a·k²·cos·(n·u)·u·uᵀ.
	var cxx := 0.0
	var cxy := 0.0
	var cyy := 0.0
	for i in n:
		var c := bande_t[i]
		var kv := Vector2(c.y, c.z)
		var k := kv.length()
		if k == 0.0:
			continue
		var u := kv / k
		var w := -c.x * k * k * cosinus[i] * (nx * u.x + ny * u.y)
		cxx += w * u.x * u.x
		cxy += w * u.x * u.y
		cyy += w * u.y * u.y
	var mxx := hxx - cxx
	var mxy := hxy - cxy
	var myy := hyy - cyy
	# J⁻¹·M·J⁻¹.
	var axx := ixx * mxx + ixy * mxy
	var axy := ixx * mxy + ixy * myy
	var ayx := ixy * mxx + iyy * mxy
	var ayy := ixy * mxy + iyy * myy
	return PackedFloat64Array([eta, nx, ny, axx * ixx + axy * ixy, axx * ixy + axy * iyy, ayx * ixy + ayy * iyy])


## S371 — la surface de B à l'aplomb du point horizontal `x` (B), en double : (hauteur, pente eulérienne). Le point
## lagrangien `q` tel que `q + d(q) = x` par Newton, quatre évaluations — le calcul de `surface_a_l_aplomb`
## (`surface_b.gdshaderinc`), ici en double précision : la référence du contrôle de la ligne d'eau.
func surface_exacte(x: Vector2, bande_t: PackedVector4Array) -> Vector3:
	var n := 0 if OS.get_environment("MER_PLATE") == "1" else bande_t.size()
	var coupure := int(donnees["split"])
	var km := Vector2.ZERO
	if donnees["asymetries"]:
		km = Vector2(float(donnees["k_moyens"][0]), float(donnees["k_moyens"][1]))
	var q := x
	var eta := 0.0
	var s := Vector2.ZERO
	var f := Vector2.ZERO
	var jxx := 1.0
	var jxy := 0.0
	var jyy := 1.0
	var det := 1.0
	for it in 4:
		var d := Vector2.ZERO
		var gxx := 0.0
		var gxy := 0.0
		var gyy := 0.0
		eta = 0.0
		s = Vector2.ZERO
		var e2 := Vector2.ZERO
		var qd := Vector2.ZERO
		var sa1 := Vector2.ZERO
		var sq1 := Vector2.ZERO
		var sa2 := Vector2.ZERO
		var sq2 := Vector2.ZERO
		for i in n:
			var c := bande_t[i]
			var kv := Vector2(c.y, c.z)
			var k := kv.length()
			if k == 0.0:
				continue
			var u := kv / k
			var phase := kv.dot(q) + c.w
			var cs := cos(phase)
			var sn := sin(phase)
			d += c.x * cs * u
			eta += c.x * sn
			s += c.x * cs * kv
			gxx -= c.x * k * sn * u.x * u.x
			gxy -= c.x * k * sn * u.x * u.y
			gyy -= c.x * k * sn * u.y * u.y
			if i < coupure:
				e2.x += c.x * sn
				qd.x += c.x * cs
				sa1 += c.x * cs * kv
				sq1 -= c.x * sn * kv
			else:
				e2.y += c.x * sn
				qd.y += c.x * cs
				sa2 += c.x * cs * kv
				sq2 -= c.x * sn * kv
		if km.x > 0.0:
			eta += 0.5 * km.x * (e2.x * e2.x - qd.x * qd.x) + 0.5 * km.y * (e2.y * e2.y - qd.y * qd.y)
			s += km.x * (e2.x * sa1 - qd.x * sq1) + km.y * (e2.y * sa2 - qd.y * sq2)
		f = q + d - x
		jxx = 1.0 + gxx
		jxy = gxy
		jyy = 1.0 + gyy
		det = jxx * jyy - jxy * jxy
		if it < 3:
			q -= Vector2(jyy * f.x - jxy * f.y, -jxy * f.x + jxx * f.y) / det if det >= 0.1 else f
	var pente := Vector2((jyy * s.x - jxy * s.y) / det, (-jxy * s.x + jxx * s.y) / det) if det >= 0.1 else s
	return Vector3(eta - pente.dot(f), pente.x, pente.y)


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


## S365 — la hauteur de la bande de B sous la caméra, `Σ a·sin(k·q + φ)` au point `q` de la caméra (sans déplacement
## horizontal, sans second ordre) : de quoi dire si l'œil est dans l'eau. S371 : quand le plan proche coupe la zone des
## vagues, la caméra est **à demi immergée** et le milieu se décide par pixel (ADR-019 §6).
func immersion(t: float) -> void:
	if camera == null:
		return
	var q := Vector2(camera.global_position.x, -camera.global_position.z)
	var eta := 0.0
	if OS.get_environment("MER_PLATE") != "1":
		for l in lignes("bande", t):
			eta += l.x * sin(l.y * q.x + l.z * q.y + l.w)
	var dedans := camera.global_position.y < eta
	# S371 — **la caméra à demi immergée** (ADR-019 §6) : si le plan proche peut couper la surface, le milieu se décide par
	# pixel (`surface_b.gdshaderinc`) ; sinon d'un bloc, par `dedans`, comme depuis S365. Le test est rigoureux : la
	# surface exacte au centre du plan proche, `η_c`, et sa variation sur l'étendue horizontale `R` du plan, bornée par
	# `pente·R/(1 − G)` (le point lagrangien bouge au plus de `R/(1 − G)` si `G < 1`) ; à défaut, la borne de |η|.
	# `DEMI=0` garde la bascule d'un bloc, en témoin.
	var base := camera.global_transform.basis
	var demi := false
	if OS.get_environment("DEMI") != "0":
		var th := tan(deg_to_rad(camera.fov) / 2.0)
		var taille := get_viewport().get_visible_rect().size
		var aspect := taille.x / taille.y
		var centre: Vector3 = camera.global_position - camera.near * base.z
		var ymin := INF
		var ymax := -INF
		var rayon := 0.0
		for sx in [-1.0, 1.0]:
			for sy in [-1.0, 1.0]:
				var coin: Vector3 = centre + camera.near * (sx * th * aspect * base.x + sy * th * base.y)
				ymin = minf(ymin, coin.y)
				ymax = maxf(ymax, coin.y)
				rayon = maxf(rayon, Vector2(coin.x - centre.x, coin.z - centre.z).length())
		# L'œil lui-même est dans ce voisinage : à `near` du centre.
		rayon = maxf(rayon, camera.near)
		var bas := -marge_vagues
		var haut := marge_vagues
		if borne_g < 1.0:
			# S371 P5 : la surface au centre du plan proche à l'ordre 2 — le nuanceur n'évalue plus que ce paraboloïde.
			var o2 := surface_ordre2(Vector2(centre.x, -centre.z), lignes("bande", t))
			var eta_c: float = o2[0]
			for mat in [materiau, materiau_sol, materiau_ciel]:
				if mat != null:
					mat.set_shader_parameter("ecart_centre", eta_c - centre.y)
					mat.set_shader_parameter("pente_centre", Vector2(o2[1], o2[2]))
					mat.set_shader_parameter("courbure_centre", Vector3(o2[3], o2[4], o2[5]))
					mat.set_shader_parameter("milieu_exact", OS.get_environment("MILIEU") == "exact")
			var m := borne_pente * rayon / (1.0 - borne_g) + 1e-3
			bas = maxf(bas, eta_c - m)
			haut = minf(haut, eta_c + m)
			# S371 — le milieu d'un bloc, et la profondeur, se prennent aussi à l'objectif et sur la surface exacte : hors du
			# mode demi, le plan proche entier est d'un côté de la surface. S365 prenait l'œil et la bande sans déplacement
			# ni second ordre (erreur de l'ordre de la pente × le déplacement) ; `PROFONDEUR=s365` la garde, en témoin.
			if OS.get_environment("PROFONDEUR") != "s365":
				dedans = centre.y < eta_c
				eta = eta_c + (camera.global_position.y - centre.y)
		demi = ymin <= haut and ymax >= bas
	demi_actif = demi
	# S366 : la brume de Godot (perspective aérienne) teintait la surface lointaine vue d'en dessous d'une bande sombre
	# juste au-dessus de l'horizon ; sous l'eau, l'atténuation est celle d'`optique_eau.gdshaderinc`, seule. S371 : elle
	# lit le cube de radiance et ne se règle pas par pixel (S371 P2) — éteinte aussi quand la caméra est à demi immergée ;
	# `BRUME=1` la garde, en témoin (ce qu'elle change au-dessus de l'eau à hauteur de vague : S371 P5).
	if environnement_scene != null and not "--controle-fond" in OS.get_cmdline_user_args():
		environnement_scene.fog_enabled = not dedans and (not demi or OS.get_environment("BRUME") == "1")
	# S373 : à demi immergée, l'eau et le fond passent à la variante qui écrit sa propre brume (`brume_air`), pondérée par
	# la part d'air de chaque pixel ; hors de la zone, la brume du moteur, comme avant. `BRUME=0` éteint la nôtre.
	brume()
	var eau_voulue := nuanceur_eau_demi if demi else nuanceur_eau
	if materiau != null and materiau.shader != eau_voulue:
		materiau.shader = eau_voulue
	var sol_voulu := nuanceur_sol_demi if demi else nuanceur_sol
	if materiau_sol != null and materiau_sol.shader != sol_voulu:
		materiau_sol.shader = sol_voulu
	var bande_t := lignes("bande", t) if demi else PackedVector4Array()
	for m in [materiau, materiau_sol, materiau_ciel]:
		if m != null:
			m.set_shader_parameter("sous_eau", dedans)
			m.set_shader_parameter("profondeur_camera", maxf(eta - camera.global_position.y, 0.0))
			m.set_shader_parameter("demi_immergee", demi)
			if demi:
				m.set_shader_parameter("camera_position", camera.global_position)
				m.set_shader_parameter("camera_avant", -base.z)
				m.set_shader_parameter("camera_droite", base.x)
				m.set_shader_parameter("camera_haut", base.y)
				if m != materiau:
					m.set_shader_parameter("bande", bande_t)


## S368 — le seuil de déferlement du champ d'écume, m/s² : `κ·σ_a`, `σ_a² = Σ ½·(a·ω²)²` sur la bande, κ tel que la
## couverture active vaille celle de Monahan exportée (S367 ; recalé ici si `KAPPA_ECUME` n'est pas nul).
func ecume_seuil() -> float:
	var v := 0.0
	for r in donnees["bande"]:
		var w := float(r[4])
		v += 0.5 * pow(float(r[0]) * w * w, 2.0)
	var kappa := KAPPA_ECUME
	if kappa <= 0.0:
		kappa = 2.5 + log(float(donnees["couverture_monahan"]) / 0.0226) / -3.52
	if OS.get_environment("KAPPA") != "":
		kappa = float(OS.get_environment("KAPPA"))
	return kappa * sqrt(v)


## S368 — recentrer le champ d'écume sur la caméra (au texel près), le vider, puis simuler `ECUME_PASSE_S` de passé au pas
## de `ECUME_PAS_S` : B est analytique, le passé se rejoue ; l'écume de la capture est celle d'un régime établi.
func ecume_centrer(t: float) -> void:
	if ecume == null:
		return
	var c := Vector2(camera.global_position.x, -camera.global_position.z)
	c = (c / ecume.PAS).round() * ecume.PAS
	ecume_centre = c
	var demi: float = 0.5 * ecume.N * ecume.PAS
	ecume.origine = c - Vector2(demi, demi)
	materiau.set_shader_parameter("ecume_origine", ecume.origine)
	var l0 := lignes("bande", t - ECUME_PASSE_S)
	ecume.avancer([l0, l0, l0], ECUME_PAS_S, 3)
	var suite := []
	var n := int(round(ECUME_PASSE_S / ECUME_PAS_S))
	for k in n + 1:
		suite.append(lignes("bande", t - ECUME_PASSE_S + k * ECUME_PAS_S))
	ecume.avancer(suite, ECUME_PAS_S, 0)


func phases(t: float) -> void:
	immersion(t)
	var bande := lignes("bande", t)
	materiau.set_shader_parameter("bande", bande)
	materiau.set_shader_parameter("queue", lignes("queue", t))
	# S379 : l'horloge des rides repliée sur l'heure (I-08 : le nuanceur ne voit qu'un flottant borné).
	materiau.set_shader_parameter("pluie", Pluie.uniformes(pluie_mm_h))
	materiau.set_shader_parameter("temps_pluie", fmod(t, 3600.0))
	for m in [materiau, materiau_sol, materiau_ciel]:
		if m != null:
			m.set_shader_parameter("couvert", couvert_voulu())
	if camera != null:
		if pluie_air == null:
			pluie_air = load("res://pluie_air.gd").new()
			add_child(pluie_air)
		pluie_air.couvert = couvert_voulu()
		pluie_air.configurer(pluie_mm_h)
		pluie_air.suivre(camera, fmod(t, 3600.0))
		if gerbes == null:
			gerbes = load("res://gerbes.gd").new()
			gerbes.sur_la_mer = true
			add_child(gerbes)
		gerbes.couvert = couvert_voulu()
		gerbes.configurer(pluie_mm_h)
		var km := Vector2.ZERO
		if donnees["asymetries"]:
			km = Vector2(float(donnees["k_moyens"][0]), float(donnees["k_moyens"][1]))
		gerbes.poser_bande(bande, donnees["bande"].size() if OS.get_environment("MER_PLATE") != "1" else 0, int(donnees["split"]), km)
		gerbes.suivre(camera, fmod(t, 3600.0))
		materiau.set_shader_parameter("gerbes_moment", Pluie.moment_gerbes(pluie_mm_h) if gerbes.actif else 0.0)
		materiau.set_shader_parameter("gerbes_fenetre", gerbes.fenetre)
	# S380 : l'extinction par les gouttes (`Pluie.extinction`), ajoutée à la brume sèche ; temps sec : la brume d'avant.
	if environnement_scene != null and pluie_mm_h > 0.0:
		environnement_scene.fog_density = BRUME_SECHE + Pluie.extinction(pluie_mm_h)
		brume()
	elif environnement_scene != null and environnement_scene.fog_density != BRUME_SECHE:
		environnement_scene.fog_density = BRUME_SECHE
		brume()
	if materiau_caustiques != null:
		materiau_caustiques.set_shader_parameter("bande", bande)
		suivre_carte()


func _process(delta: float) -> void:
	if camera == null:
		return
	mer.global_position = Vector3(camera.global_position.x, 0.0, camera.global_position.z)
	if anime:
		temps += delta
		phases(temps)
		# S368 : l'écume suit le temps — deux demi-pas par image ; recentrée si la caméra s'éloigne de 32 m.
		if ecume != null:
			var cam := Vector2(camera.global_position.x, -camera.global_position.z)
			if cam.distance_to(ecume_centre) > 32.0:
				ecume_centrer(temps)
			else:
				ecume.avancer([lignes("bande", temps - delta), lignes("bande", temps - 0.5 * delta), lignes("bande", temps)],
					0.5 * delta, 0)
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
			KEY_P:
				var suite := {0.0: 2.0, 2.0: 10.0, 10.0: 50.0}
				pluie_mm_h = suite.get(pluie_mm_h, 0.0)
				phases(temps)
			KEY_ESCAPE:
				get_tree().quit()


## Les quatre poses à 12 s, figées, capturées par Godot lui-même. S363 : `POSES=proche,rasante` en restreint la liste ;
## `HDR=1` rend dans un tampon flottant et écrit un PFM — linéaire, **non écrêté**, sans sRGB : avec `TONALITE=lineaire`,
## les radiances mêmes de la scène, sur lesquelles `outils/tonalite_godot.py` rejoue les courbes de Godot.
func captures() -> void:
	DirAccess.make_dir_recursive_absolute(ProjectSettings.globalize_path("res://captures"))
	var hdr := OS.get_environment("HDR") == "1"
	if hdr:
		get_viewport().use_hdr_2d = true
	var noms := ["proche", "rasante", "reference", "haute", "plongeante"]
	if OS.get_environment("POSES") != "":
		noms = Array(OS.get_environment("POSES").split(","))
	for nom in noms:
		pose(nom)
		mer.global_position = Vector3(camera.global_position.x, 0.0, camera.global_position.z)
		immersion(temps)
		ecume_centrer(temps)
		# S383 : la pluie suit la pose (jusque-là, la boîte des gouttes restait devant la caméra de départ, « proche » : en pose
		# « référence », les gouttes de R29 et R30 tombaient ≈ 11 m trop loin) ; les gerbes, de même.
		if pluie_air != null:
			pluie_air.suivre(camera, fmod(temps, 3600.0))
		if gerbes != null:
			gerbes.suivre(camera, fmod(temps, 3600.0))
			materiau.set_shader_parameter("gerbes_fenetre", gerbes.fenetre)
		for _i in 12:
			await RenderingServer.frame_post_draw
		var image := get_viewport().get_texture().get_image()
		var suffixe := "_cote" if "--cote" in OS.get_cmdline_user_args() else ""
		if pluie_mm_h > 0.0:
			suffixe += "_pluie%d" % int(pluie_mm_h)
		var chemin := ProjectSettings.globalize_path("res://captures/godot_%s%s_12s.%s" % [nom, suffixe, "pfm" if hdr else "png"])
		# S368 : `SEQUENCE=n` — n images de la même pose, espacées de 2 s ; l'écume avance entre elles (la durée se juge
		# dans le temps). Les suivantes s'écrivent `_12s_2.png`, `_12s_4.png`…
		var suite_n := int(OS.get_environment("SEQUENCE")) if OS.get_environment("SEQUENCE") != "" else 1
		for rang in range(1, suite_n):
			var image_r := get_viewport().get_texture().get_image()
			if rang == 1:
				image_r.save_png(ProjectSettings.globalize_path("res://captures/godot_%s%s_12s_0.png" % [nom, suffixe]))
			var liste_s := []
			for k in 21:
				liste_s.append(lignes("bande", temps + k * ECUME_PAS_S))
			if ecume != null:
				ecume.avancer(liste_s, ECUME_PAS_S, 0)
			temps += 2.0
			phases(temps)
			if detail != null:
				detail.calculer(temps)
			for _i in 6:
				await RenderingServer.frame_post_draw
			get_viewport().get_texture().get_image().save_png(ProjectSettings.globalize_path("res://captures/godot_%s%s_12s_%d.png" % [nom, suffixe, 2 * rang]))
		if hdr:
			# PFM : en-tête texte, flottants de 32 bits petit-boutistes (échelle −1), rangées du bas vers le haut.
			image.convert(Image.FORMAT_RGBF)
			image.flip_y()
			var f := FileAccess.open(chemin, FileAccess.WRITE)
			f.store_buffer(("PF\n%d %d\n-1.0\n" % [image.get_width(), image.get_height()]).to_ascii_buffer())
			f.store_buffer(image.get_data())
			f.close()
		else:
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
	brume_voulue = false
	brume()
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


## S365 — **le contrôle du milieu** (liste 8.6) : la caméra dans l'eau de la scène côtière, le fond rend la transmission
## de sa ligne de visée (mode 3, tonalité linéaire, sans brume ni halo) ; la distance est recalculée ici, rayon marché sur
## la bathymétrie analytique, et la transmission rendue comparée à `exp(−c·d)` par canal, à 0,01 près.
func controle_sous_eau() -> void:
	var env: Environment = (get_children().filter(func(c): return c is WorldEnvironment)[0] as WorldEnvironment).environment
	env.tonemap_mode = Environment.TONE_MAPPER_LINEAR
	env.glow_enabled = false
	env.fog_enabled = false
	brume_voulue = false
	brume()
	materiau_sol.set_shader_parameter("controle", 3)
	var c := ABSORPTION + 2.0 * RETRODIFFUSION
	var pire := 0.0
	for inclinaison in [-0.6, -0.25]:
		camera.position = Vector3(0.0, -3.0, 20.0)
		camera.rotation = Vector3(inclinaison, 0.0, 0.0)
		mer.global_position = Vector3(camera.global_position.x, 0.0, camera.global_position.z)
		immersion(temps)
		for _i in 12:
			await RenderingServer.frame_post_draw
		var image := get_viewport().get_texture().get_image()
		var taille := image.get_size()
		for f in [Vector2(0.5, 0.6), Vector2(0.2, 0.75), Vector2(0.8, 0.75), Vector2(0.5, 0.95), Vector2(0.3, 0.55)]:
			var px := Vector2i(int(f.x * taille.x), int(f.y * taille.y))
			var o := camera.project_ray_origin(Vector2(px) + Vector2(0.5, 0.5))
			var dir := camera.project_ray_normal(Vector2(px) + Vector2(0.5, 0.5))
			var t := 0.0
			while o.y + dir.y * t > -profondeur(o.x + dir.x * t, -(o.z + dir.z * t)) and t < 2000.0:
				t += 0.02
			var lo := t - 0.02
			var hi := t
			for _i in 40:
				var m := 0.5 * (lo + hi)
				var p := o + dir * m
				if p.y > -profondeur(p.x, -p.z):
					lo = m
				else:
					hi = m
			var attendu := Vector3(exp(-c.x * hi), exp(-c.y * hi), exp(-c.z * hi))
			var rendu := image.get_pixel(px.x, px.y).srgb_to_linear()
			var ecart := Vector3(rendu.r, rendu.g, rendu.b) - attendu
			pire = maxf(pire, maxf(absf(ecart.x), maxf(absf(ecart.y), absf(ecart.z))))
			print("CONTROLE_SOUS_EAU_S365 inclinaison=%.2f pixel=%s d_m=%.3f rendu=(%.4f, %.4f, %.4f) attendu=(%.4f, %.4f, %.4f)" % [inclinaison, px, hi, rendu.r, rendu.g, rendu.b, attendu.x, attendu.y, attendu.z])
	print("CONTROLE_SOUS_EAU_S365 pire=%.4f critere=%s" % [pire, "tenu" if pire <= 0.01 else "manque"])
	get_tree().quit()


## S371 — **le contrôle de la ligne d'eau** (ADR-019 §6, liste 8.6). L'eau, le fond et le ciel rendent la part d'eau du
## pixel (`controle_milieu`), tonalité linéaire, sans halo. **Critère 1** : sur 32 colonnes et quatre cas — la pose `demi`
## à t₀ et à t₀ + 2,3 s, une autre position, visée oblique, à t₀ + 5,7 s, un roulis de 0,4 rad (la ligne en diagonale) —,
## la ligne rendue (passage de la part d'eau à ½, interpolé entre deux rangées) contre l'intersection analytique de la
## surface et du plan proche, recalculée ici en double (`surface_exacte`) : **≤ 1 pixel**. **Critère 2** : 60 images
## consécutives à 1/60 s, pose `demi`, 16 colonnes : **aucun pixel dont le milieu rendu diffère du milieu analytique à
## plus de 2 pixels de la ligne**.
func controle_ligne_eau() -> void:
	var env: Environment = (get_children().filter(func(c): return c is WorldEnvironment)[0] as WorldEnvironment).environment
	env.tonemap_mode = Environment.TONE_MAPPER_LINEAR
	env.glow_enabled = false
	for m in [materiau, materiau_sol, materiau_ciel]:
		m.set_shader_parameter("controle_milieu", true)
	var cas := [["demi", 0.0, 0.0, Vector3.ZERO], ["demi", 2.3, 0.0, Vector3.ZERO],
		["demi", 5.7, 0.0, Vector3(40.0, 0.0, -30.0)], ["demi", 1.1, 0.4, Vector3.ZERO]]
	var pire := 0.0
	for c in cas:
		temps = t0 + float(c[1])
		pose(String(c[0]))
		camera.position += c[3]
		if c[3] != Vector3.ZERO:
			camera.rotation = Vector3(0.08, 1.2, 0.0)
		camera.rotation.z = float(c[2])
		recaler_sur_la_surface()
		var image: Image = await image_au_temps(temps)
		var e := ligne_contre_analytique(image, 32, "cas t=%.1f roulis=%.1f" % [float(c[1]), float(c[2])])
		pire = maxf(pire, e.x)
	print("CONTROLE_LIGNE_EAU_S371 critere=1 pire_px=%.3f %s" % [pire, "tenu" if pire <= 1.0 else "manque"])
	# Critère 2 : 60 images consécutives, caméra fixe (la mer monte et descend de part et d'autre du plan proche, la ligne
	# traverse le cadre), puis flottante (recalée sur la surface à chaque image, comme la tête d'un nageur : la ligne reste
	# dans le cadre et s'y déplace).
	var tout := true
	for flottante in [false, true]:
		pose("demi")
		temps = t0
		recaler_sur_la_surface()
		var mal := 0
		var pire2 := 0.0
		var mouvement := 0.0
		var visibles := 0
		var precedente := -1.0
		for k in 60:
			temps = t0 + float(k) / 60.0
			if flottante:
				recaler_sur_la_surface()
			var image: Image = await image_au_temps(temps)
			var e := ligne_contre_analytique(image, 16, "")
			pire2 = maxf(pire2, e.x)
			mal += int(e.y)
			if e.z >= 0.0:
				visibles += 1
				if precedente >= 0.0:
					mouvement = maxf(mouvement, absf(e.z - precedente))
			precedente = e.z
		tout = tout and mal == 0 and pire2 <= 1.0
		print("CONTROLE_LIGNE_EAU_S371 critere=2 camera=%s images=60 ligne_visible_au_milieu=%d pire_px=%.3f pixels_mal_classes_hors_2px=%d deplacement_max_px_par_image=%.1f %s" % ["flottante" if flottante else "fixe", visibles, pire2, mal, mouvement, "tenu" if mal == 0 and pire2 <= 1.0 else "manque"])
	print("CONTROLE_LIGNE_EAU_S371 critere=2 %s" % ("tenu" if tout else "manque"))
	get_tree().quit()


## S371 — **le coût de la caméra à demi immergée** (métrique de B11) : le temps GPU du rendu entier, mesuré par Godot, en
## médiane sur 240 images, pose `demi` puis `demi_dessous`, milieu par pixel puis d'un bloc (le même cadre, `demi_immergee`
## forcé faux) ; la différence est le prix du milieu par pixel. Temps figé, fenêtre de 1 280 × 720.
## S379 — le coût de la pluie sur la mer : temps GPU de l'image (médiane de 240 après 30), à 0, 2, 10 et 50 mm/h, poses
## proche, rasante et référence à 12 s.
func cout_pluie() -> void:
	var vp := get_viewport().get_viewport_rid()
	RenderingServer.viewport_set_measure_render_time(vp, true)
	for nom in ["proche", "rasante", "reference"]:
		pose(nom)
		mer.global_position = Vector3(camera.global_position.x, 0.0, camera.global_position.z)
		var ligne_cout := "COUT_PLUIE_S379 mer pose=%s" % nom
		for r in [0.0, 2.0, 10.0, 50.0]:
			pluie_mm_h = r
			phases(temps)
			for _i in 30:
				await RenderingServer.frame_post_draw
			var t := []
			for _i in 240:
				await RenderingServer.frame_post_draw
				t.append(RenderingServer.viewport_get_measured_render_time_gpu(vp))
			t.sort()
			ligne_cout += " gpu_ms_%d=%.3f" % [int(r), float(t[120])]
		print(ligne_cout)
	get_tree().quit()


func cout_demi() -> void:
	var vp := get_viewport().get_viewport_rid()
	RenderingServer.viewport_set_measure_render_time(vp, true)
	for nom in ["demi", "demi_dessous"]:
		var medianes := []
		for par_pixel in [true, false]:
			pose(nom)
			mer.global_position = Vector3(camera.global_position.x, 0.0, camera.global_position.z)
			phases(temps)
			if not par_pixel:
				for m in [materiau, materiau_sol, materiau_ciel]:
					if m != null:
						m.set_shader_parameter("demi_immergee", false)
			for _i in 30:
				await RenderingServer.frame_post_draw
			var t := []
			for _i in 240:
				await RenderingServer.frame_post_draw
				t.append(RenderingServer.viewport_get_measured_render_time_gpu(vp))
			t.sort()
			medianes.append(float(t[120]))
		print("COUT_DEMI_S371 pose=%s gpu_ms_par_pixel=%.3f gpu_ms_d_un_bloc=%.3f surcout_ms=%.3f" % [nom, medianes[0], medianes[1], medianes[0] - medianes[1]])
	# Et le CPU : `immersion()` en mode demi (Newton, ordre 2, uniformes), GDScript, moyenne sur 200 appels.
	pose("demi")
	var debut := Time.get_ticks_usec()
	for _i in 200:
		immersion(temps)
	print("COUT_DEMI_S371 cpu_immersion_ms=%.3f demi=%s" % [float(Time.get_ticks_usec() - debut) / 200000.0, str(demi_actif)])
	get_tree().quit()


## Remonte ou descend la caméra pour que le centre du plan proche soit sur la surface exacte (la ligne au centre de l'image).
func recaler_sur_la_surface() -> void:
	var centre := camera.position - camera.near * camera.transform.basis.z
	camera.position.y += surface_exacte(Vector2(centre.x, -centre.z), lignes("bande", temps)).x - centre.y


## L'image rendue au temps `t`, caméra et milieu à jour.
func image_au_temps(t: float) -> Image:
	mer.global_position = Vector3(camera.global_position.x, 0.0, camera.global_position.z)
	phases(t)
	if detail != null:
		detail.calculer(t)
	for _i in 4:
		await RenderingServer.frame_post_draw
	return get_viewport().get_texture().get_image()


## La hauteur de l'objectif au-dessus de la surface, pour le pixel `px` (coordonnées continues de l'image).
func hauteur_objectif(px: Vector2, bande_t: PackedVector4Array) -> float:
	var dir := camera.project_ray_normal(px)
	var avant := -camera.global_transform.basis.z
	var p := camera.global_position + dir * (camera.near / dir.dot(avant))
	return p.y - surface_exacte(Vector2(p.x, -p.z), bande_t).x


## Sur `n` colonnes : (pire écart de la ligne rendue à l'analytique, px ; pixels mal classés à plus de 2 px de la ligne ;
## rangée de la ligne à la colonne du milieu). La ligne analytique : balayage de la colonne par pas de 4 px, puis
## dichotomie à 10⁻³ px ; les rangées rendues, part d'eau lue au centre des pixels.
func ligne_contre_analytique(image: Image, n: int, etiquette: String) -> Vector3:
	var taille := image.get_size()
	var bande_t := lignes("bande", temps)
	var pire := 0.0
	var mal := 0
	var milieu_rangee := -1.0
	for ci in n:
		var x := int((float(ci) + 0.5) * float(taille.x) / float(n))
		var cx := float(x) + 0.5
		# Les passages analytiques.
		var passages: Array = []
		var y0 := 0.5
		var s0 := hauteur_objectif(Vector2(cx, y0), bande_t)
		var y := y0
		while y < float(taille.y) - 0.5:
			var y1 := minf(y + 4.0, float(taille.y) - 0.5)
			var s1 := hauteur_objectif(Vector2(cx, y1), bande_t)
			if (s0 > 0.0) != (s1 > 0.0):
				var a := y
				var b := y1
				var sa := s0
				for _k in 14:
					var m := 0.5 * (a + b)
					var sm := hauteur_objectif(Vector2(cx, m), bande_t)
					if (sm > 0.0) == (sa > 0.0):
						a = m
						sa = sm
					else:
						b = m
				passages.append(0.5 * (a + b))
			y = y1
			s0 = s1
		# Les passages rendus, et les pixels mal classés loin de la ligne.
		var haut_dans_l_eau := hauteur_objectif(Vector2(cx, 0.5), bande_t) < 0.0
		var rendus: Array = []
		var w_prec := image.get_pixel(x, 0).srgb_to_linear().r
		for j in taille.y:
			var w := image.get_pixel(x, j).srgb_to_linear().r
			if j > 0 and (w_prec > 0.5) != (w > 0.5):
				rendus.append(float(j - 1) + 0.5 + (w_prec - 0.5) / (w_prec - w))
			w_prec = w
			# Le milieu analytique de la rangée : celui du haut de la colonne, changé à chaque passage au-dessus d'elle.
			var avant_j := 0
			for v in passages:
				if float(v) < float(j) + 0.5:
					avant_j += 1
			var dans_l_eau := haut_dans_l_eau != (avant_j % 2 == 1)
			if (w > 0.5) != dans_l_eau and not _pres(passages, float(j) + 0.5, 2.0):
				mal += 1
		if rendus.size() != passages.size():
			pire = maxf(pire, 99.0)
			if etiquette != "":
				print("CONTROLE_LIGNE_EAU_S371 %s colonne=%d passages_analytiques=%s rendus=%s" % [etiquette, x, str(passages), str(rendus)])
			continue
		for k in rendus.size():
			var e := absf(float(rendus[k]) - float(passages[k]))
			pire = maxf(pire, e)
			if etiquette != "" and ci % 8 == 0:
				print("CONTROLE_LIGNE_EAU_S371 %s colonne=%d rangee_analytique=%.3f rangee_rendue=%.3f ecart_px=%.3f" % [etiquette, x, float(passages[k]), float(rendus[k]), e])
		if ci == n / 2 and passages.size() > 0:
			milieu_rangee = float(passages[0])
	if etiquette != "":
		print("CONTROLE_LIGNE_EAU_S371 %s pire_px=%.3f mal_classes=%d" % [etiquette, pire, mal])
	return Vector3(pire, mal, milieu_rangee)


func _pres(liste: Array, y: float, d: float) -> bool:
	for v in liste:
		if absf(float(v) - y) <= d:
			return true
	return false


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
	brume_voulue = false
	brume()
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


## S361 — **le contrôle des caustiques contre une solution exacte** : une seule onde, `η = a·sin(k·x)`, λ = 4 m,
## a = 5 cm ; soleil au zénith ; fond plat imposé à H ; la mer masquée, le fond vu au nadir. L'éclairement rendu en
## 50 points d'une longueur d'onde contre `Σ 1/|F'(x_s)|` sur les antécédents de `F(x_s) = x_s + (H + η)·p(η'(x_s)) −
## x_f` — Snell exact, racines trouvées en double par balayage et dichotomie ; points de −2 à +2 m du centre de
## l'image. Trois cas : onde selon x à mi-focale (le
## critère), onde selon y à mi-focale (le sens de la carte), onde selon x à 1,5 focale (au-delà du pli : trois
## antécédents, ce que la méthode à rebours ne savait pas faire).
func controle_caustiques() -> void:
	var env: Environment = (get_children().filter(func(c): return c is WorldEnvironment)[0] as WorldEnvironment).environment
	env.tonemap_mode = Environment.TONE_MAPPER_LINEAR
	env.glow_enabled = false
	env.fog_enabled = false
	brume_voulue = false
	brume()
	mer.visible = false
	var a := 0.05
	var k := TAU / 4.0
	var n := 1.34
	var h_f := 1.0 / ((1.0 - 1.0 / n) * a * k * k)
	materiau_caustiques.set_shader_parameter("n_bande", 1)
	materiau_caustiques.set_shader_parameter("caustiques_detail", false)
	materiau_caustiques.set_shader_parameter("soleil_controle", Vector3(0.0, 0.0, 1.0))
	materiau_sol.set_shader_parameter("soleil_controle", Vector3(0.0, 0.0, 1.0))
	materiau_sol.set_shader_parameter("controle", 1)
	camera.position = Vector3(0.0, 2.0, 60.0)
	camera.rotation = Vector3(-PI / 2.0, 0.0, 0.0)
	var tout := true
	for cas in [["x", 0.5], ["y", 0.5], ["x", 1.5]]:
		var selon_y: bool = cas[0] == "y"
		var h: float = float(cas[1]) * h_f
		materiau_caustiques.set_shader_parameter("profondeur_controle", h)
		materiau_caustiques.set_shader_parameter("bande", PackedVector4Array([Vector4(a, 0.0 if selon_y else k, k if selon_y else 0.0, 0.0)]))
		suivre_carte(Vector2(0.0, -60.0))
		for _i in 12:
			await RenderingServer.frame_post_draw
		var image := get_viewport().get_texture().get_image()
		var taille := image.get_size()
		var pire := 0.0
		var ecarts := []
		var somme_r := 0.0
		var somme_e := 0.0
		var echantillons := 50
		for i in echantillons:
			var d := 4.0 * float(i) / float(echantillons) - 2.0
			var cible := Vector3(d if not selon_y else 0.0, 0.0, -d if selon_y else 0.0) + Vector3(0.0, 0.0, 60.0)
			cible.y = -profondeur(cible.x, -cible.z)
			var px := camera.unproject_position(cible)
			var pixel := Vector2i(int(px.x), int(px.y))
			var q := point_du_fond(Vector2(pixel) + Vector2(0.5, 0.5))
			var coordonnee: float = q.y if selon_y else q.x
			var col := image.get_pixel(pixel.x, pixel.y).srgb_to_linear()
			var rendu := (roundf(10.0 * 4.0 * col.r - col.g) + col.g) / 10.0
			var exact := eclairement_exact(coordonnee, a, k, h, n)
			var e := absf(rendu - exact) / exact
			pire = maxf(pire, e)
			ecarts.append(e)
			somme_r += rendu
			somme_e += exact
		ecarts.sort()
		var moyenne := somme_r / echantillons
		var critere := pire <= 0.05 and absf(moyenne - 1.0) <= 0.01
		if float(cas[1]) < 1.0:
			tout = tout and critere
		print("CONTROLE_CAUSTIQUES_S361 onde_selon=%s H_sur_Hf=%.1f H_m=%.3f pire_relatif=%.4f mediane_relative=%.4f moyenne_rendue=%.4f moyenne_exacte=%.4f critere=%s" % [cas[0], cas[1], h, pire, ecarts[echantillons / 2], moyenne, somme_e / echantillons, ("tenu" if critere else "manque") if float(cas[1]) < 1.0 else "indicatif"])
	print("CONTROLE_CAUSTIQUES_S361 critere_global=%s" % ("tenu" if tout else "manque"))
	get_tree().quit()


## Le point du fond vu au pixel, en (x, y) de B : le rayon du pixel jusqu'au fond de `profondeur()`.
func point_du_fond(px: Vector2) -> Vector2:
	var o := camera.project_ray_origin(px)
	var d := camera.project_ray_normal(px)
	var t := 0.0
	while o.y + d.y * t > -profondeur(o.x + d.x * t, -(o.z + d.z * t)):
		t += 0.02
	var lo := t - 0.02
	var hi := t
	for _i in 40:
		var m := 0.5 * (lo + hi)
		var p := o + d * m
		if p.y > -profondeur(p.x, -p.z):
			lo = m
		else:
			hi = m
	var f := o + d * hi
	return Vector2(f.x, -f.z)


## La pente horizontale du rayon de soleil vertical réfracté par une facette de pente `s` (refract de GLSL, en double).
static func pente_rayon(s: float, n: float) -> float:
	var nv := Vector3(-s, 0.0, 1.0).normalized()
	var i := Vector3(0.0, 0.0, -1.0)
	var eta := 1.0 / n
	var c := nv.dot(i)
	var q := 1.0 - eta * eta * (1.0 - c * c)
	var r := eta * i - (eta * c + sqrt(q)) * nv
	return r.x / -r.z


static func eclairement_exact(xf: float, a: float, k: float, h: float, n: float) -> float:
	var f := func(xs: float) -> float:
		return xs + (h + a * sin(k * xs)) * pente_rayon(a * k * cos(k * xs), n) - xf
	var total := 0.0
	var pas := 0.002
	var x := xf - 3.0
	var fa: float = f.call(x)
	while x < xf + 3.0:
		var fb: float = f.call(x + pas)
		if fa == 0.0 or (fa < 0.0) != (fb < 0.0):
			var lo := x
			var hi := x + pas
			for _i in 60:
				var m := 0.5 * (lo + hi)
				if (f.call(lo) < 0.0) != (f.call(m) < 0.0):
					hi = m
				else:
					lo = m
			var r := 0.5 * (lo + hi)
			var d := 1e-6
			total += 1.0 / absf((f.call(r + d) - f.call(r - d)) / (2.0 * d))
		x += pas
		fa = fb
	return total


## S361 — **l'énergie des caustiques sur la scène** : la mer de `--meilleur` au complet, soleil de la scène ; la carte
## elle-même, relue pour les poses plongeante et proche. Sa moyenne sur l'intérieur (les 80 % centraux) doit rester près
## de 1 — l'énergie se déplace, elle ne se crée pas ; son maximum et la part au-dessus de 5 se publient.
func controle_caustiques_scene() -> void:
	for nom in ["plongeante", "proche"]:
		pose(nom)
		suivre_carte()
		for _i in 12:
			await RenderingServer.frame_post_draw
		var image := vue_caustiques.get_texture().get_image()
		var somme := 0.0
		var n := 0
		var maximum := 0.0
		var forts := 0
		var marge := CARTE_TEXELS / 10
		for y in range(marge, CARTE_TEXELS - marge, 2):
			for x in range(marge, CARTE_TEXELS - marge, 2):
				var c := CARTE_ECHELLE * image.get_pixel(x, y).r
				somme += c
				maximum = maxf(maximum, c)
				if c > 5.0:
					forts += 1
				n += 1
		print("CONTROLE_CAUSTIQUES_SCENE_S361 pose=%s format=%d texels=%d moyenne=%.4f max=%.2f part_au_dessus_de_5=%.5f critere=%s" % [nom, image.get_format(), n, somme / n, maximum, float(forts) / n, "tenu" if absf(somme / n - 1.0) <= 0.1 else "manque"])
	get_tree().quit()


## S368 — **le contrôle du champ d'écume sur la carte** (critère 1) : (a) décroissance — un champ uniforme (actif 1),
## 600 pas de 1/60 s sans source ni advection, contre la solution fermée ; (b) advection — une bosse gaussienne
## (σ = 2 m) translatée par (0,37 ; −0,21) m/s, cent pas de 0,1 s, centre de masse contre (3,7 ; −2,1) m.
func controle_ecume_champ() -> void:
	for _i in 4:
		await RenderingServer.frame_post_draw
	var l0 := lignes("bande", temps)
	ecume.origine = Vector2(-128.0, -128.0)
	ecume.avancer([l0, l0, l0], 1.0 / 60.0, 4, PackedFloat32Array([0.0, 0.0, 0.0, 0.0, 1e6]))
	var suite := []
	for _k in 601:
		suite.append(l0)
	ecume.avancer(suite, 1.0 / 60.0, 1)
	var valeurs: PackedFloat32Array = await _relire_ecume()
	var la := log(2.0) / 3.0
	var lr := log(2.0) / 30.0
	var t := 10.0
	var a_att := exp(-la * t)
	var r_att := la / (la - lr) * (exp(-lr * t) - exp(-la * t))
	var pire := 0.0
	for texel in [Vector2i(0, 0), Vector2i(511, 511), Vector2i(1023, 17)]:
		var i: int = 4 * (texel.y * ecume.N + texel.x)
		pire = maxf(pire, maxf(absf(valeurs[i] / a_att - 1.0), absf(valeurs[i + 1] / r_att - 1.0)))
		print("CONTROLE_ECUME_S368 decroissance texel=%s actif=%.7f/%.7f residuel=%.7f/%.7f" % [texel, valeurs[i], a_att, valeurs[i + 1], r_att])
	print("CONTROLE_ECUME_S368 decroissance pire_relatif=%s critere=%s" % [String.num_scientific(pire), "tenu" if pire <= 1e-5 else "manque"])
	# (b) L'advection : une bosse au centre du champ, puis cent pas à vitesse uniforme.
	var x0 := Vector2(-3.0, 4.0)
	ecume.avancer([l0, l0, l0], 0.1, 4, PackedFloat32Array([0.0, 0.0, x0.x, x0.y, 2.0]))
	suite = []
	for _k in 101:
		suite.append(l0)
	ecume.avancer(suite, 0.1, 2, PackedFloat32Array([0.37, -0.21]))
	valeurs = await _relire_ecume()
	var masse := 0.0
	var mx := 0.0
	var my := 0.0
	for j in ecume.N:
		for i in ecume.N:
			var v := valeurs[4 * (j * ecume.N + i)]
			if v > 0.0:
				var x: float = ecume.origine.x + (i + 0.5) * ecume.PAS
				var y: float = ecume.origine.y + (j + 0.5) * ecume.PAS
				masse += v
				mx += v * x
				my += v * y
	var centre := Vector2(mx / masse, my / masse)
	var attendu := x0 + Vector2(3.7, -2.1)
	var ecart := centre.distance_to(attendu)
	# La masse attendue, décroissance comprise : 2π·σ²/pas² texels de hauteur 1, fois e^(−λa·10 s).
	var masse_att: float = TAU * 4.0 / (ecume.PAS * ecume.PAS) * exp(-la * 10.0)
	print("CONTROLE_ECUME_S368 advection centre=%s attendu=%s ecart_m=%s masse_relative=%s critere=%s" % [centre, attendu, String.num_scientific(ecart), String.num_scientific(masse / masse_att - 1.0), "tenu" if ecart <= 0.01 else "manque"])
	get_tree().quit()


func _relire_ecume() -> PackedFloat32Array:
	var boite := [null]
	ecume.relire(func(v): boite[0] = v)
	while boite[0] == null:
		await get_tree().process_frame
	return boite[0]


## S368 — **la couverture du champ sur la carte** (critère 2) : recentré à la pose proche, 60 s de passé, puis 40 s de
## mesure — la part où l'actif dépasse ½, relue chaque seconde — contre `couverture_monahan` de l'export. `KAPPA` règle le
## seuil ; imprime κ et la couverture.
func controle_ecume_couverture() -> void:
	for _i in 4:
		await RenderingServer.frame_post_draw
	pose("proche")
	ecume_centrer(temps)
	var cible := float(donnees["couverture_monahan"])
	var somme := 0.0
	var n := 0
	var t := temps
	for s in 40:
		var suite := []
		for k in 11:
			suite.append(lignes("bande", t + k * ECUME_PAS_S))
		ecume.avancer(suite, ECUME_PAS_S, 0)
		t += 10 * ECUME_PAS_S
		var valeurs: PackedFloat32Array = await _relire_ecume()
		var blanc := 0
		for i in range(0, valeurs.size(), 4):
			if valeurs[i] > 0.5:
				blanc += 1
		somme += float(blanc) / (valeurs.size() / 4)
		n += 1
	var v := 0.0
	for r in donnees["bande"]:
		v += 0.5 * pow(float(r[0]) * float(r[4]) * float(r[4]), 2.0)
	var kappa: float = ecume.seuil / sqrt(v)
	print("CONTROLE_ECUME_S368 couverture kappa=%.3f seuil_g=%.4f active=%.4f%% monahan=%.4f%% rapport=%.3f" % [kappa, ecume.seuil / 9.81, 100.0 * somme / n, 100.0 * cible, somme / n / cible])
	get_tree().quit()
