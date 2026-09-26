extends RefCounted
## S379 — **les statistiques de la pluie** pour les rides factices (`pluie.gdshaderinc`, ADR-202 D3, ADR-203 D7) : une
## seule source, lue par la mer et la piscine. Ce fichier porte la **météorologie** — combien de gouttes, de quelles
## tailles ; le nuanceur porte la **géométrie** d'un anneau (vitesse, longueur d'onde, durée, pente).
##
## Distribution des tailles : Marshall et Palmer (1948), `N(D) = N0·e^(−Λ·D)`, `N0` = 8 000 m⁻³·mm⁻¹,
## `Λ = 4,1·R^−0,21` mm⁻¹ (R en mm/h). Vitesse terminale : Atlas, Srivastava et Sekhon (1973),
## `v(D) = 9,65 − 10,3·e^(−0,6·D)` m/s (D en mm). Flux de gouttes au sol : `N(D)·v(D)`, m⁻²·s⁻¹ par mm.
##
## Les gouttes de 1,5 mm et plus laissent un anneau net (`D_MIN`) : leur taux est la première composante des uniformes ;
## celles de 0,5 à 1,5 mm ne sont que de la rugosité (leur variance de pente) ; au-dessus de 6 mm, les gouttes se brisent.

const N0 := 8000.0
const D_MIN := 1.5
const D_PETITES := 0.5
const D_MAX := 6.0
## Pas de la quadrature en diamètre, mm (point milieu).
const PAS_D := 0.05


static func lambda_mp(r_mm_h: float) -> float:
	return 4.1 * pow(r_mm_h, -0.21)


static func vitesse_atlas(d_mm: float) -> float:
	return maxf(9.65 - 10.3 * exp(-0.6 * d_mm), 0.0)


## Le facteur de pente d'un anneau selon le diamètre de la goutte, relatif à une goutte de 2 mm : `(D/2)^1,5` — la
## racine de l'énergie cinétique à vitesse comparable ; borné comme les gouttes (`D_MAX`). **Réglé, pas mesuré.**
static func facteur(d_mm: float) -> float:
	return pow(minf(d_mm, D_MAX) / 2.0, 1.5)


## Taux d'arrivée des gouttes d'au moins `D_MIN`, m⁻²·s⁻¹ — forme close de `∫ N0·e^(−Λ·D)·v(D) dD` sur [D_MIN, ∞).
static func taux_anneaux(r_mm_h: float) -> float:
	if r_mm_h <= 0.0:
		return 0.0
	var l := lambda_mp(r_mm_h)
	return N0 * (9.65 * exp(-l * D_MIN) / l - 10.3 * exp(-(l + 0.6) * D_MIN) / (l + 0.6))


## Les uniformes `pluie` du nuanceur, pour une intensité `r_mm_h` :
## x — taux des anneaux (m⁻²·s⁻¹) ; y — `Λ` (mm⁻¹), pour tirer la taille de chaque goutte ; z — moment `Σ flux·f²` des
## petites gouttes (m⁻²·s⁻¹) ; w — le même moment pour les anneaux, **sous la loi que le nuanceur tire** (`D_MIN` plus une
## exponentielle de paramètre `Λ`, bornée à `D_MAX`), pour que le fondu en variance rende exactement l'énergie des anneaux
## qu'il remplace. Temps sec : tout à zéro, et le nuanceur rend l'image d'avant la pluie au bit (critère 1 de S379).
static func uniformes(r_mm_h: float) -> Vector4:
	if r_mm_h <= 0.0:
		return Vector4.ZERO
	var l := lambda_mp(r_mm_h)
	var petites := 0.0
	var d := D_PETITES + 0.5 * PAS_D
	while d < D_MIN:
		petites += exp(-l * d) * vitesse_atlas(d) * pow(facteur(d), 2.0)
		d += PAS_D
	petites *= N0 * PAS_D
	# E[f²] sous la loi tirée : densité `Λ·e^(−Λ·(D − D_MIN))`, la masse au-delà de D_MAX ramenée à D_MAX.
	var moment := 0.0
	d = D_MIN + 0.5 * PAS_D
	while d < D_MAX:
		moment += l * exp(-l * (d - D_MIN)) * pow(facteur(d), 2.0) * PAS_D
		d += PAS_D
	moment += exp(-l * (D_MAX - D_MIN)) * pow(facteur(D_MAX), 2.0)
	var n := taux_anneaux(r_mm_h)
	return Vector4(n, l, petites, n * moment)


## S380 — **les gouttes dans l'air** (ADR-205, pièce 1) : leur nombre par m³ entre `d0` et `d1` mm,
## `∫ N0·e^(−Λ·D) dD = (N0/Λ)·(e^(−Λ·d0) − e^(−Λ·d1))`.
static func densite_gouttes(r_mm_h: float, d0: float, d1: float) -> float:
	if r_mm_h <= 0.0:
		return 0.0
	var l := lambda_mp(r_mm_h)
	return N0 / l * (exp(-l * d0) - exp(-l * d1))


## S380 — **l'extinction** par la pluie (ADR-205, pièce 2), m⁻¹ : efficacité d'extinction 2 (gouttes grandes devant la
## longueur d'onde) sur la section géométrique, `β = 2·(π/4)·∫N(D)·D² dD = (π/2)·2·N0/Λ³`, D en mm → m² : ×10⁻⁶.
## 1,55·10⁻³ m⁻¹ à 10 mm/h, soit une visibilité `3,9/β` de 2,5 km.
static func extinction(r_mm_h: float) -> float:
	if r_mm_h <= 0.0:
		return 0.0
	var l := lambda_mp(r_mm_h)
	return PI / 2.0 * 2.0 * N0 / (l * l * l) * 1e-6
