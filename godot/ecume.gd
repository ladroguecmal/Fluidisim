extends Node
## S368 — **le champ d'écume sur la carte** (ADR-014, SPEC-006 §4 ; liste 7.1 et 8.4) : la production de la référence de
## S367 (`code/water-core/src/ecume.rs`), un pas à l'identique dans `ecume.comp`. 1 024 × 1 024 texels de 0,25 m
## (256 m) autour de la caméra, deux canaux (R actif, G résiduel) en RGBA32F, deux images alternées ; chaque appel fait un
## nombre **pair** de pas, si bien que l'état publié est toujours la première image (`texture`). Rendu seulement (I-13) ;
## aucun état n'en sort vers le jeu (I-04). Calcul sur le `RenderingDevice` principal, depuis le fil de rendu.

const N := 1024
const PAS := 0.25
## Demi-vies d'ADR-014 §2.2 (s), celles de la référence ; rampe de la montée lisse, en fractions de g (S367).
const DEMI_VIES := Vector2(3.0, 30.0)
const RAMPE_G := 0.05

var rd: RenderingDevice
var shader: RID
var pipeline: RID
var bande_buf: RID
var puls_buf: RID
var images: Array = []
var ensembles: Array = []
var texture := Texture2DRD.new()
var origine := Vector2.ZERO
var n_bande := 0
var gravite := 9.81
## Le seuil de déferlement, m/s² : `κ·σ_a` (S367) ; posé par `mer.gd`.
var seuil := 0.0
var pret := false


## `pulsations` : ω de chaque composante de la bande ; `origine` : le coin du champ dans les axes de B.
func initialiser(pulsations: PackedFloat32Array, coin: Vector2, g: float) -> void:
	n_bande = pulsations.size()
	origine = coin
	gravite = g
	RenderingServer.call_on_render_thread(_initialiser.bind(pulsations))


func _initialiser(pulsations: PackedFloat32Array) -> void:
	rd = RenderingServer.get_rendering_device()
	var source := RDShaderSource.new()
	source.language = RenderingDevice.SHADER_LANGUAGE_GLSL
	source.source_compute = FileAccess.get_file_as_string("res://ecume.comp")
	var spirv := rd.shader_compile_spirv_from_source(source)
	if spirv.compile_error_compute != "":
		push_error("ecume.comp : " + spirv.compile_error_compute)
		return
	shader = rd.shader_create_from_spirv(spirv)
	pipeline = rd.compute_pipeline_create(shader)
	bande_buf = rd.storage_buffer_create(2 * n_bande * 16)
	var p_octets := pulsations.to_byte_array()
	puls_buf = rd.storage_buffer_create(p_octets.size(), p_octets)
	var format := RDTextureFormat.new()
	format.format = RenderingDevice.DATA_FORMAT_R32G32B32A32_SFLOAT
	format.width = N
	format.height = N
	format.usage_bits = RenderingDevice.TEXTURE_USAGE_SAMPLING_BIT | RenderingDevice.TEXTURE_USAGE_STORAGE_BIT \
		| RenderingDevice.TEXTURE_USAGE_CAN_COPY_FROM_BIT
	for _i in 2:
		images.append(rd.texture_create(format, RDTextureView.new(), []))
	for sens in 2:
		ensembles.append(rd.uniform_set_create([
			_tampon(0, bande_buf), _tampon(1, puls_buf), _image(2, images[sens]), _image(3, images[1 - sens]),
		], shader, 0))
	texture.texture_rd_rid = images[0]
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


## Les coefficients du pas en **double** : `e^(−λa·dt)`, `e^(−λr·dt)` et le transfert `λa/(λa − λr)·(e^(−λr·dt) − e^(−λa·dt))`
## — en f32 sur la carte, au pas de 1/60 s, la différence s'annule à 3·10⁻⁵ près (contrôle de S368).
func _parametres(mode: int, dt: float, extra: PackedFloat32Array) -> PackedByteArray:
	var la := log(2.0) / DEMI_VIES.x
	var lr := log(2.0) / DEMI_VIES.y
	var ea := exp(-la * dt)
	var er := exp(-lr * dt)
	var o := PackedByteArray()
	o.resize(80)
	o.encode_float(0, origine.x)
	o.encode_float(4, origine.y)
	o.encode_float(8, PAS)
	o.encode_float(12, dt)
	o.encode_u32(16, N)
	o.encode_u32(20, n_bande)
	o.encode_u32(24, mode)
	o.encode_float(28, ea)
	o.encode_float(32, er)
	o.encode_float(36, la / (la - lr) * (er - ea))
	o.encode_float(40, seuil)
	o.encode_float(44, RAMPE_G * gravite)
	for k in extra.size():
		o.encode_float(48 + 4 * k, extra[k])
	return o


## Une suite de pas, en nombre pair : `lignes` donne la bande `[a, kx, ky, φ]` à chaque instant de la suite (le départ
## du premier pas, puis l'arrivée de chacun) ; `mode` et `extra` pour les contrôles (vitesse uniforme, bosse).
func avancer(lignes: Array, dt: float, mode := 0, extra := PackedFloat32Array()) -> void:
	RenderingServer.call_on_render_thread(_avancer.bind(lignes, dt, mode, extra))


func _avancer(lignes: Array, dt: float, mode: int, extra: PackedFloat32Array) -> void:
	if not pret:
		return
	var pas := lignes.size() - 1
	for k in pas:
		var octets := PackedByteArray()
		octets.append_array((lignes[k] as PackedVector4Array).to_byte_array())
		octets.append_array((lignes[k + 1] as PackedVector4Array).to_byte_array())
		rd.buffer_update(bande_buf, 0, octets.size(), octets)
		var liste := rd.compute_list_begin()
		rd.compute_list_bind_compute_pipeline(liste, pipeline)
		rd.compute_list_bind_uniform_set(liste, ensembles[k % 2], 0)
		var par := _parametres(mode, dt, extra)
		rd.compute_list_set_push_constant(liste, par, par.size())
		rd.compute_list_dispatch(liste, N / 16, N / 16, 1)
		rd.compute_list_end()


## Relecture de l'état publié (première image), pour les contrôles : `rappel` reçoit les flottants RGBA de N × N texels.
func relire(rappel: Callable) -> void:
	RenderingServer.call_on_render_thread(_relire.bind(rappel))


func _relire(rappel: Callable) -> void:
	rappel.call_deferred(rd.texture_get_data(images[0], 0).to_float32_array())
