# Les gerbes de la pluie : couronne, dôme et jet aux impacts des rides — S383

2026-09-26. Pièce **4** d'[ADR-205](../adr/ADR-205-la-pluie-complete.md) (R31 : *« continue la pluie »*). Liste **8.4**
(écume, spray, gouttes). Suite du ciel de pluie ([CIEL-PLUIE-S381](CIEL-PLUIE-S381.md)) et de l'occultation du ciel
([OCCULTATION-CIEL-S382](OCCULTATION-CIEL-S382.md)) ; revue R32 ([REVUE-VISUELLE §37](REVUE-VISUELLE.md)).

## Reproduire

- Commit `7d2b03a8` (P8 de S383) ou plus récent ; Godot 4.4.1 (`<godot>`) ; données de la piscine et de la mer.
- `PLUIE=10 <godot> --path godot res://piscine.tscn -- --controle-gerbes > journal.txt`, puis
  `python outils/controle_gerbes.py journal.txt` : critères 3 à 5.
- `GERBES=0` : sans gerbes (critère 2, et le coût) ; `PLUIE=<mm/h>` devant `--captures` (piscine et mer).
- Critère 1 comme en S379 (12 images, SHA-256), référence : la fin de S382.

## En une phrase

Chaque goutte de pluie qui laisse un anneau fait jaillir, **au même point et au même instant**, une gerbe dont la forme et
les temps sont relevés sur une goutte de pluie réelle filmée et ramenés à sa taille ; sur la mer, posée sur les vagues ;
au loin, fondue en un éclaircissement de la surface ; sans pluie, rien ne change au bit.

## 1. La construction

- **Les sources** (P2). A — Wang, Liu, Bayeul-Lainé, Murphy, Katz et Coutier-Delgosha (2023, JFM ; arXiv 2302.02728), sur
  l'expérience de **Murphy et al. (2015, JFM 780)** : goutte de **4,1 mm à 7,2 m/s** (81 % de sa vitesse terminale), We
  2 893, Fr 1 322, eau profonde ; couronne à son rayon maximal vers 3 ms, refermée en dôme vers 12 ms, cavité la plus
  profonde vers 24 ms, jet central vers 40 ms ; ≈ 2 000 microgouttelettes (modes 50 et 225 µm). **Relevé sur leur figure 3**
  (barre d'échelle 17 px = 10 mm, vérifiée par la goutte) : sommet 5,9 / 11,2 / 17,1 / 19,4 / 24,7 / 23,5 / 21,8 mm à 1 / 3
  / 7 / 12 / 18 / 41 / 52 ms ; largeur à la base 14 à 29 mm. B — Watson, Thornton, Khan, Diamco, Yilmaz-Aydin et Dickerson
  (2024, PNAS 121, e2315667121), gouttes de 4 mm, Fr 127–850 : `κ₁/D ∼ Fr^0,25`, `δ₁/D ∼ Fr^0,27`.
- **L'échelle** (hypothèse dite) : longueurs × `s = (D/4,1 mm)·(Fr/1 322)^0,26` (exposants de B), temps × `√s`, `Fr = v²/(g·D)`,
  `v` d'Atlas ; s ≈ 0,41 / 0,56 / 1,10 pour 1,5 / 2 / 4,1 mm à leur vitesse terminale.
- **Le tirage partagé** (P3) : `pluie_phase`, `pluie_graine`, `pluie_decalage`, `pluie_impact` sortis des rides
  (`pluie.gdshaderinc`) ; les rides et les gerbes lisent les mêmes gouttes.
- **Les particules** (P4, P4b) : `gerbes.gd`, `gerbe.gdshader` — une particule par couche, période (courante et précédente) et
  maille d'une fenêtre : sur la piscine, les deux eaux (24 336 particules à 10 mm/h) ; sur la mer, 12 m devant la caméra, en
  coordonnées de Lagrange, la gerbe posée à `q + d(q)`, hauteur `η(q)` (`bande_b.gdshaderinc`, sorti de `surface_b`). Vie
  `0,08·√s` s. `gerbe_dessin.gdshader` : jusqu'à 10 ms la couronne (coupe), puis le dôme et le jet (4 mm, tête de 2,5 mm),
  retombée entre 52 et 80 ms (hypothèse) ; moyenne sur quatre instants de la pose de 1/60 s ; radiance des gouttes (Garg et
  Nayar) ; opacités réglées ; sous deux pixels, agrandie et d'opacité réduite de l'aire. Dessinée après l'eau.
- **Au loin** (P5) : une gerbe est verticale ; la part d'un pixel d'eau qu'elles couvrent vaut `M/tan ε`, `M = Σ flux(D)·
  A·s(D)^2,5`, `A` = 7 168 mm²·ms (aire de profil de la silhouette de référence, intégrée sur sa vie) ; sur la mer, hors de
  la fenêtre, fondue sur son dernier mètre. Couverture à 2° : 0,3 / 2,1 / 11,9 % à 2 / 10 / 50 mm/h.

## 2. Les critères, écrits avant

| critère | mesure | |
|---|---|---|
| 1 — sans pluie, identique au bit | 12 images, référence fin de S382 | **12 / 12** (P3, P4b, P6) |
| 2 — sous la pluie, gerbes éteintes, identique au bit | 8 images (piscine 10 et 50 mm/h, trois vues ; mer 10 mm/h, deux poses), rendues deux fois : déterministes | **8 / 8** après le tirage partagé (P3) ; **7 / 8** après P4b — la mer en pose « référence » change : correction voulue (§3) |
| 3 — chaque gerbe au centre de son anneau, à son instant | vue d'aplomb, 2 mm/px, 20 instants : cœurs d'anneaux < 30 ms contre gerbes < 30 ms | **961 gerbes, 0,76 mm** au plus ; aucun anneau sans gerbe (33 coupés par le bord, écartés) |
| 4 — gerbes vivantes = taux × vie | les mêmes images | 961 pour 988,9 : **−2,82 %** (1 σ de Poisson 3,18 %) |
| 5 — hauteurs et temps des mesures | gerbe de référence seule, de profil, 0,05 mm/px, à 1, 3, 7, 12, 18, 41, 52 ms, contre le plus haut des relevés sur la pose | **0,04 mm** au plus (tolérance : un pixel de la figure source, 0,59 mm) |
| 6 — coût ; photographies ; jugement | ci-dessous ; *Rain falling into a swimming pool* ; R32 | **R32 posée** |

**Impasses** : `return` interdit dans `process()` d'un nuanceur de particules ; les particules ne se dessinaient pas — l'eau
du bassin, transparente et qui lit l'écran, passait après elles (tri des objets transparents) : `render_priority` ; un carré
de contrôle de 2 pixels, mêlé au rouge par l'anticrénelage, faisait croire à 560 anneaux sans gerbe.

## 3. Un défaut antérieur corrigé

Dans les captures de la mer, rien ne rappelait `phases()` après `pose()` : la boîte des gouttes dans l'air (S380) restait
devant la caméra de départ, « proche ». **En pose « référence », les gouttes de R29 et R30 tombaient ≈ 11 m trop loin**
(visibles surtout au lointain). Corrigé : après chaque pose, la pluie suit la caméra. Les images de R29 et R30 en pose
« proche » n'étaient pas touchées.

## 4. Coût (1280 × 720, RTX 5070 Laptop, médiane de 240 images ; sans → avec gerbes)

| scène | 2 mm/h | 10 mm/h | 50 mm/h |
|---|---|---|---|
| bassin, proche | 0,979 → 0,986 ms | 1,661 → 1,676 | 4,395 → 4,623 |
| bassin, aplomb | 0,837 → 0,843 | 1,660 → 1,665 | 5,027 → 5,208 |
| bassin, rasante | 0,630 → 0,636 | 0,911 → 0,914 | 1,991 → 2,044 |
| mer, proche | 1,986 → 2,070 | 3,411 → 3,530 | 9,719 → 10,050 |
| mer, rasante | 1,990 → 2,042 | 3,278 → 3,373 | 8,793 → 9,082 |
| mer, référence | 1,895 → 1,984 | 3,106 → 3,197 | 8,372 → 8,670 |

## 5. Limites

- **Silhouette simplifiée** (coupe, dôme, jet), opacités réglées : ni les doigts de la couronne, ni les gouttelettes
  (≈ 2 000 par goutte, sous le pixel une à une), ni le dôme translucide qu'est la « bubble canopy » réelle.
- **Une seule mesure** de référence (4,1 mm) ; les autres tailles par une loi d'échelle déclarée (exposants de B, temps en
  `√s`) ; retombée après 52 ms supposée.
- Pas de gerbes sous 1,5 mm ; pas d'éclaboussures **au sol** (sol mouillé : impact sur film mince, Cossali, Coghe et
  Marengo 1997 — avec la pièce 5) ; pas de vent ; les gerbes ne portent ni ombre ni occultation.
- Fenêtre de 12 m sur la mer (au-delà, le terme moyen) ; vérifié sur cette machine seulement ; images de R32 locales
  (`viewer/captures/s383/`).
