extends Node
## S360 — **la surface fine** : la queue de B réalisée par FFT sur la carte (Tessendorf 2001), deux cascades de
## 256 × 256 composantes exportées par l'afficheur (`donnees/detail_h0.bin`, `rendu_cretes::export_detail`). Chaque
## image : `fft_detail.comp` — spectre à l'instant, FFT inverse, composition, niveaux de détail. Le nuanceur d'eau lit
## les deux images par cascade, `textures[c][0|1]`. Rendu seulement (I-13) ; aucun état n'en sort vers le jeu (I-04).
##
## Le calcul se fait sur le `RenderingDevice` principal, depuis le fil de rendu (`call_on_render_thread`) ; les images
## sont réservées une fois (I-06 lu pour l'hôte graphique, ADR-145). Le temps passe replié en double sur la période
## de répétition (I-08).

const N := 256
const NIVEAUX := 9
## Période de répétition de l'animation, s : la pulsation de chaque case est arrondie au multiple de `2π/PERIODE` le plus
## proche (Tessendorf, § « looping »), soit au plus 3,1·10⁻³ rad/s d'écart ; seul `(t mod PERIODE)/PERIODE` va à la carte.
const PERIODE := 1000.0

var rd: RenderingDevice
var shader: RID
var pipeline: RID
var h0_buf: RID
var champ_buf: RID
var images: Array = []
var ensembles: Array = []
var textures: Array = []
var cotes: Array = []
var gravite := 9.81
var km := 370.0
var mss_realisee: Array = []
var pret := false
var h0_octets: PackedByteArray


func charger(detail: Dictionary) -> bool:
	var chemin := "res://donnees/%s" % String(detail["fichier"])
	h0_octets = FileAccess.get_file_as_bytes(chemin)
	if h0_octets.size() != 2 * N * N * 8 or int(detail["n"]) != N:
		push_error("%s : %d octets pour %d attendus" % [chemin, h0_octets.size(), 2 * N * N * 8])
		return false
	gravite = float(detail["gravite"])
	km = float(detail["km"])
	for c in detail["cascades"]:
		cotes.append(float(c[0]))
		mss_realisee.append(float(c[3]))
	for c in 2:
		textures.append([Texture2DRD.new(), Texture2DRD.new()])
	RenderingServer.call_on_render_thread(_initialiser)
	return true


func _initialiser() -> void:
	rd = RenderingServer.get_rendering_device()
	var source := RDShaderSource.new()
	source.language = RenderingDevice.SHADER_LANGUAGE_GLSL
	source.source_compute = FileAccess.get_file_as_string("res://fft_detail.comp")
	var spirv := rd.shader_compile_spirv_from_source(source)
	if spirv.compile_error_compute != "":
		push_error("fft_detail.comp : " + spirv.compile_error_compute)
		return
	shader = rd.shader_create_from_spirv(spirv)
	pipeline = rd.compute_pipeline_create(shader)
	h0_buf = rd.storage_buffer_create(h0_octets.size(), h0_octets)
	champ_buf = rd.storage_buffer_create(2 * 3 * N * N * 8)
	var format := RDTextureFormat.new()
	format.format = RenderingDevice.DATA_FORMAT_R32G32B32A32_SFLOAT
	format.width = N
	format.height = N
	format.mipmaps = NIVEAUX
	format.usage_bits = RenderingDevice.TEXTURE_USAGE_SAMPLING_BIT | RenderingDevice.TEXTURE_USAGE_STORAGE_BIT \
		| RenderingDevice.TEXTURE_USAGE_CAN_COPY_FROM_BIT
	for c in 2:
		var paire := [rd.texture_create(format, RDTextureView.new(), []), rd.texture_create(format, RDTextureView.new(), [])]
		images.append(paire)
		var vues := []
		for niveau in NIVEAUX:
			vues.append([
				rd.texture_create_shared_from_slice(RDTextureView.new(), paire[0], 0, niveau, 1, RenderingDevice.TEXTURE_SLICE_2D),
				rd.texture_create_shared_from_slice(RDTextureView.new(), paire[1], 0, niveau, 1, RenderingDevice.TEXTURE_SLICE_2D),
			])
		var par_niveau := []
		for niveau in NIVEAUX:
			# Au niveau 0, la source n'est pas lue : on lie le niveau 1 pour ne pas lier deux fois la même vue.
			var src: Array = vues[1] if niveau == 0 else vues[niveau - 1]
			par_niveau.append(rd.uniform_set_create([
				_tampon(0, h0_buf), _tampon(1, champ_buf),
				_image(2, vues[niveau][0]), _image(3, vues[niveau][1]),
				_image(4, src[0]), _image(5, src[1]),
			], shader, 0))
		ensembles.append(par_niveau)
		textures[c][0].texture_rd_rid = paire[0]
		textures[c][1].texture_rd_rid = paire[1]
	pret = true


func _tampon(liaison: int, rid: RID) -> RDUniform:
	var u := RDUniform.new()
	u.uniform_type = RenderingDevice.UNIFORM_TYPE_STORAGE_BUFFER
	u.binding = liaison
	u.add_id(rid)
	return u


func _image(liaison: int, rid: RID) -> RDUniform:
	var u := RDUniform.new()
	u.uniform_type = RenderingDevice.UNIFORM_TYPE_IMAGE
	u.binding = liaison
	u.add_id(rid)
	return u


func _parametres(mode: int, cascade: int, niveau: int, t_frac: float) -> PackedByteArray:
	var octets := PackedByteArray()
	octets.resize(48)
	octets.encode_u32(0, mode)
	octets.encode_u32(4, cascade)
	octets.encode_u32(8, N)
	octets.encode_u32(12, niveau)
	octets.encode_float(16, float(cotes[cascade]))
	octets.encode_float(20, t_frac)
	octets.encode_float(24, PERIODE)
	octets.encode_float(28, gravite)
	octets.encode_float(32, km)
	return octets


## L'instant `t` (s, double) : spectre, FFT, composition et niveaux des deux cascades, sur le fil de rendu.
func calculer(t: float) -> void:
	var t_frac := fposmod(t, PERIODE) / PERIODE
	RenderingServer.call_on_render_thread(_calculer.bind(t_frac))


func _calculer(t_frac: float) -> void:
	if not pret:
		return
	var liste := rd.compute_list_begin()
	rd.compute_list_bind_compute_pipeline(liste, pipeline)
	for c in 2:
		var passe := func(mode: int, niveau: int, groupes: int) -> void:
			rd.compute_list_bind_uniform_set(liste, ensembles[c][niveau], 0)
			var octets := _parametres(mode, c, niveau, t_frac)
			rd.compute_list_set_push_constant(liste, octets, octets.size())
			rd.compute_list_dispatch(liste, groupes, 1, 1)
			rd.compute_list_add_barrier(liste)
		passe.call(0, 0, N * N / 256)
		passe.call(1, 0, 3 * N)
		passe.call(2, 0, 3 * N)
		passe.call(3, 0, N * N / 256)
		for niveau in range(1, NIVEAUX):
			var taille := N >> niveau
			passe.call(4, niveau, maxi(1, (taille * taille + 255) / 256))
	rd.compute_list_end()


## **Contrôle** (S360, critère 4) : le champ de la FFT à l'instant `t` contre la somme directe des mêmes composantes, en
## double, en quatre texels de chaque cascade — `η` et `∂η/∂x` ; puis la pente quadratique moyenne du champ entier contre
## `Σ|h̃|²k²` (Parseval), exportée. À appeler quelques images après `calculer(t)`, sans calcul entre les deux : le tampon
## relu est celui de cet instant. Imprime ; `rappel` reçoit `true` si tout tient.
func controler(t: float, rappel: Callable) -> void:
	var t_frac := fposmod(t, PERIODE) / PERIODE
	RenderingServer.call_on_render_thread(_controler.bind(t_frac, rappel))


func _controler(t_frac: float, rappel: Callable) -> void:
	var champ := rd.buffer_get_data(champ_buf)
	var h0 := h0_octets.to_float32_array()
	var valeurs := champ.to_float32_array()
	var tout := true
	var nn := N * N
	for c in 2:
		var dk := TAU / float(cotes[c])
		var somme_mss := 0.0
		var somme_eta2 := 0.0
		for i in nn:
			var sx := valeurs[2 * (c * 3 * nn + i)]
			var sy := valeurs[2 * (c * 3 * nn + i) + 1]
			var eta := valeurs[2 * (c * 3 * nn + 2 * nn + i) + 1]
			somme_mss += sx * sx + sy * sy
			somme_eta2 += eta * eta
		var mss := somme_mss / nn
		var rms_eta := sqrt(somme_eta2 / nn)
		var rms_s := sqrt(mss / 2.0)
		var pire := 0.0
		for texel: Vector2i in [Vector2i(0, 0), Vector2i(17, 203), Vector2i(128, 77), Vector2i(250, 131)]:
			var eta_d := 0.0
			var sx_d := 0.0
			for m in N:
				for n in N:
					var j := c * nn + m * N + n
					var ar := h0[2 * j]
					var ai := h0[2 * j + 1]
					if ar == 0.0 and ai == 0.0:
						continue
					# La case k et sa conjuguée −k : h̃(k) = h0·e^{−iφ}, h̃(−k) = conj(h0)·e^{iφ} ; leur somme est réelle.
					var kx := (n if n < N / 2 else n - N) * dk
					var ky := (m if m < N / 2 else m - N) * dk
					var k := sqrt(kx * kx + ky * ky)
					var omega := sqrt(gravite * k * (1.0 + (k / km) * (k / km)))
					var nq := roundf(omega * PERIODE / TAU)
					var phi := TAU * fposmod(nq * t_frac, 1.0)
					var x: float = float(cotes[c]) * texel.x / N
					var y: float = float(cotes[c]) * texel.y / N
					var arg: float = kx * x + ky * y - phi
					# 2·Re(h0·e^{i·arg}) et sa dérivée en x.
					eta_d += 2.0 * (ar * cos(arg) - ai * sin(arg))
					sx_d += 2.0 * kx * (-ar * sin(arg) - ai * cos(arg))
			var i_t: int = texel.y * N + texel.x
			var eta_f := valeurs[2 * (c * 3 * nn + 2 * nn + i_t) + 1]
			var sx_f := valeurs[2 * (c * 3 * nn + i_t)]
			var e1 := absf(eta_f - eta_d) / rms_eta
			var e2 := absf(sx_f - sx_d) / rms_s
			pire = maxf(pire, maxf(e1, e2))
			print("CONTROLE_FFT_S360 cascade=%d texel=%s eta_fft=%s eta_direct=%s sx_fft=%s sx_direct=%s" % [c, texel, String.num_scientific(eta_f), String.num_scientific(eta_d), String.num_scientific(sx_f), String.num_scientific(sx_d)])
		var ecart_mss := absf(mss - float(mss_realisee[c])) / float(mss_realisee[c])
		print("CONTROLE_FFT_S360 cascade=%d rms_eta_m=%s mss_fft=%s mss_export=%s ecart_mss=%s pire_relatif=%s critere=%s" % [c, String.num_scientific(rms_eta), String.num_scientific(mss), String.num_scientific(float(mss_realisee[c])), String.num_scientific(ecart_mss), String.num_scientific(pire), "tenu" if pire <= 1e-4 and ecart_mss <= 0.01 else "manque"])
		tout = tout and pire <= 1e-4 and ecart_mss <= 0.01
	rappel.call_deferred(tout)
