extends RefCounted
## S474, ADR-217 — **le type d'eau** : les propriétés optiques d'une eau tirées de ses constituants, ajoutés à l'eau pure
## d'ADR-177. Le même calcul que `outils/type_eau.py` (la référence, ses sources et ses formules dans son en-tête) ; toute
## modification se porte dans les deux, et `-- --controle-type-eau` (`mer.tscn`) imprime ce que celui-ci donne, comparé par
## `python outils/type_eau.py --egalite=<sortie>`.
##
## Trois constituants : le phytoplancton `chl` (mg/m³, Morel et Maritorena 2001), la matière dissoute `ag440` (m⁻¹, a_g à
## 440 nm), les particules minérales `mes` (g/m³). Bandes 650, 550, 450 nm : les canaux R, G, B du rendu.

const A_EAU := Vector3(0.340, 0.0565, 0.00922)
const BB_EAU := Vector3(0.00071, 0.00145, 0.00344)
const MU_D := 0.8
const BANDES := [650.0, 550.0, 450.0]
## Morel et Maritorena (2001), Table 2 : χ et e à 650, 550, 450 nm.
const CHI := [0.04500, 0.04111, 0.10165]
const EXPO := [0.67200, 0.64927, 0.67692]

## Les préréglages (ADR-217 D1) : (Chl, a_g(440), MES), dans les plages publiées de chaque milieu.
const PRESETS := {
	"pure": Vector3(0.0, 0.0, 0.0),
	"ocean_clair": Vector3(0.03, 0.0, 0.0),
	"mediterranee": Vector3(0.1, 0.01, 0.0),
	"cotier": Vector3(1.5, 0.1, 2.0),
	"lac": Vector3(5.0, 0.5, 3.0),
	"riviere": Vector3(3.0, 1.5, 15.0),
	"trouble": Vector3(2.0, 0.5, 50.0),
}


static func log10(x: float) -> float:
	return log(x) / log(10.0)


## `{a, b, bb, R0, kd, c}` (Vector3, R G B) d'une eau de constituants `k` = (Chl, a_g(440), MES).
static func proprietes(k: Vector3) -> Dictionary:
	var chl := k.x
	var ag := k.y
	var mes := k.z
	var a := Vector3.ZERO
	var b := Vector3.ZERO
	var bb := Vector3.ZERO
	for i in 3:
		var lam: float = BANDES[i]
		var ai: float = A_EAU[i]
		var bbi: float = BB_EAU[i]
		var bi: float = 2.0 * BB_EAU[i]
		if chl > 0.0:
			var l10 := log10(chl)
			var bp550 := 0.416 * pow(chl, 0.766)
			var nu := 0.0
			if chl <= 2.0:
				nu = 0.5 * (clampf(l10, log10(0.02), log10(2.0)) - 0.3)
			var bbp := (0.002 + 0.01 * (0.5 - 0.25 * l10) * pow(lam / 550.0, nu)) * bp550
			ai += maxf(MU_D * CHI[i] * pow(chl, EXPO[i]) - bbp, 0.0)
			bbi += bbp
			bi += bp550 * 550.0 / lam
		ai += ag * exp(-0.0176 * (lam - 440.0))
		if mes > 0.0:
			ai += 0.04 * mes * exp(-0.0123 * (lam - 443.0))
			var bpm := 0.5 * mes * 555.0 / lam
			bi += bpm
			bbi += 0.018 * bpm
		a[i] = ai
		b[i] = bi
		bb[i] = bbi
	var r0 := Vector3.ZERO
	for i in 3:
		r0[i] = 0.33 * bb[i] / (a[i] + bb[i])
	return {"a": a, "b": b, "bb": bb, "R0": r0, "kd": (a + bb) / MU_D, "c": a + b}


## Le type d'eau de la scène : `TYPE_EAU=<préréglage>` ou `TYPE_EAU=<Chl>,<a_g(440)>,<MES>` ; vide : aucun (l'eau d'avant, au bit).
static func de_l_environnement() -> Variant:
	var v := OS.get_environment("TYPE_EAU")
	if v == "":
		return null
	if PRESETS.has(v):
		return PRESETS[v]
	var p := v.split(",")
	if p.size() == 3:
		return Vector3(float(p[0]), float(p[1]), float(p[2]))
	push_error("TYPE_EAU inconnu : " + v)
	return null
