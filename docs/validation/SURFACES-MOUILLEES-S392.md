# Les surfaces mouillées : plus sombres, plus brillantes, et sèches sous les abris — S392

2026-09-26, **au poste**. Pièce **5a** d'[ADR-205](../adr/ADR-205-la-pluie-complete.md) (demande de l'utilisateur : *« Continue
avec la pluie, pièce 5 »*) ; découpage déclaré : 5a, les surfaces mouillées ; 5b, les éclaboussures au sol. Sert **8.10**
(la crédibilité perçue). Revue **R33** ([REVUE-VISUELLE §38](REVUE-VISUELLE.md)).

## Reproduire

- Commit du P5 de S392 ou plus récent ; Godot 4.4.1 (`<godot>`) ; données de la piscine.
- Les nombres : `python outils/sol_mouille.py` (lignes `SOL_MOUILLE_S392`) ; essais `python -m unittest test_sol_mouille`
  depuis `outils/` (6 essais, deux secondes).
- `PLUIE=10 <godot> --path godot res://piscine.tscn -- --controle-mouille` — lignes `CONTROLE_MOUILLE_S392` : critères 3 et 4.
- Critère 1 : `VUES=ensemble,rasante,buse INSTANTS=6.5,200 … -- --captures` et `DELTA=0 VUES=ensemble INSTANTS=40 …`, avant et
  après, empreintes SHA-256.
- Le coût : `MOUILLE=0|1 VUES=ensemble,pluie_proche,rasante <godot> … -- --cout-pluie`.
- Les images de R33 : `MOUILLE=1|0 PLUIE=10 VUES=pied_mur,ensemble,pluie_proche INSTANTS=200 … -- --captures` (suffixe `_sec`
  sans mouillure) ; planche `viewer/captures/s392/r33_mouille.png`. `ASSOMBRISSEMENT=` : le second effet de Lekner et Dorf.

## En une phrase

Sous la pluie, chaque face de la piscine tournée vers le ciel et que rien n'abrite prend un film d'eau : sa part diffuse
s'assombrit comme le veut la réflexion interne dans le film (Ångström, Lekner et Dorf) — le béton à 64 % de sa radiance
sèche, le sol à 57 % —, le film reflète le ciel ou le bloc selon Fresnel, les dessous de débords restent secs au pixel près ;
sans pluie, rien ne change au bit.

## 1. La physique et ses nombres

Un matériau rugueux d'albédo `a` sous un film d'eau (Ångström 1925 ; Lekner et Dorf 1988, *Appl. Opt.* 27, 1278 — lus par
leurs résumés, le PDF n'étant pas lisible sans téléchargement) : la lumière entre dans le film, se diffuse sur le matériau,
et à chaque remontée l'interface eau–air en renvoie vers le bas une part `r̄ᵢ` — réflexion totale au-delà de l'angle
critique, partielle en deçà (la précision que Lekner et Dorf ajoutent à Ångström). Sommés, les rebonds donnent le flux
sortant `E·(1 − r̄ₑ)·a·(1 − r̄ᵢ)/(1 − a·r̄ᵢ)` ; la sortie réfractée d'un champ lambertien sous l'eau donne la radiance vue sous
`θ` :

`L(θ) = (E/π) · a·(1 − r̄ᵢ)/(1 − a·r̄ᵢ) · (1 − R(θ))  +  R(θ)·L(réfléchi)`

— le `(1 − r̄ₑ)` d'entrée et celui de la sortie se compensent par la réciprocité `1 − r̄ᵢ = (1 − r̄ₑ)/n²`. Pour l'eau (n =
1,333), **deux intégrations indépendantes** du Fresnel non polarisé pondéré par le cosinus (`outils/sol_mouille.py`) :
`r̄ₑ` = 0,06641 vu de l'air, `r̄ᵢ` = **0,47459** vu de l'eau, égal à la réciprocité (0,47459) ; `R(0)` = 0,02037. Un blanc ne
s'assombrit pas ; un noir perd la moitié (1 − r̄ᵢ). Rapports mouillé / sec vus d'aplomb : 0,540 (a = 0,1), 0,569 (0,2),
0,600 (0,3), 0,643 (0,42), 0,720 (0,6). **Non modélisé** : le second effet de Lekner et Dorf — l'indice relatif qui baisse
quand l'eau remplit les pores, propre au matériau ; `ASSOMBRISSEMENT` (1 par défaut) le laisse réglable.

## 2. La construction

`godot/mouille.gdshaderinc`, inclus par `paroi.gdshader` (sol, bloc, margelles de la piscine). **Où** : la pluie tombe à la
verticale ; une face tournée vers le ciel est mouillée si la verticale au-dessus d'elle est libre (les occultants de S382 ;
quatre points par pixel) ; une face verticale l'est au pied, dans la bande des rejaillissements `exp(−h/0,1 m)` si le sol
devant elle est exposé — **hypothèse déclarée**, à calibrer —, et sous le bord d'une face exposée (nez de margelle) ; une
face tournée vers le bas, jamais ; une face dans l'emprise d'un bac, jamais. **Le reflet** : quatre rayons dans un cône de
0,05 rad (rugosité du film, *à calibrer*) ; chacun prend le ciel couvert, ou un occultant — rendu comme une face verticale
de béton sec à demi masquée (approximation déclarée : les occultants n'ont pas d'albédo). Loin des occultants, un seul
rayon suffit — la CIE étant linéaire en `sin h`, la moyenne des quatre rayons symétriques est exacte. Régime établi : sous
toute pluie, les faces exposées sont mouillées.

## 3. Les critères, écrits avant

| critère | mesure | |
|---|---|---|
| 1 — sans pluie, identique au bit | 7 images de la piscine (ensemble, rasante, buse à 6,5 et 200 s ; sans δ à 40 s), avant et après, deux fois | **7 / 7** |
| 2 — les nombres, deux intégrations et la réciprocité, 10⁻⁴ | `outils/sol_mouille.py`, 6 essais | **tenu** : 0,47459 = 0,47459 |
| 3 — sur l'image, ≤ 1 % | sol (0,19) et margelle (0,41), d'aplomb et à 60°, valeurs brutes | diffus / sec **0,029 %** au pire (sol d'aplomb 0,56573 pour 0,56572) ; reflet **0,29 %** (le cône de rugosité) |
| 4 — le bord sec sous l'abri, à un pixel | vue orthographique d'aplomb à 5 mm/px, sous la margelle ouest | **0,00 mm** de la verticale de l'arête ; sec 0,000 dessous, mouillé 1,000 dehors |
| 5 — une photographie réelle | premières gouttes sur l'asphalte sec (§4) | **mesurée, écart d'un facteur 6 dit, non attribué** |
| 6 — coût ≤ +0,3 ms ; R33 | 10 mm/h, médiane de 240 images | ensemble **+0,34 ms**, proche +0,31, rasante +0,22 : **manqué de peu** ; **R33 posée** |

**Vu en chemin** : le test de dalles avec un rayon vertical divise par zéro (composantes x et z nulles) — indéfini sur la
carte, la verticale sous le débord passait pour libre (premier passage du critère 4 : mouillure 1 sous le débord) ; un test
vertical exact le remplace. **Le coût**, dans l'ordre : +0,96 ms au premier jet ; tests précoces contre les boîtes élargies
et ciel couvert sans les nuages du ciel clair, +0,38 ; un seul rayon loin des occultants, +0,34.

## 4. La photographie

*From dry pavement to wet pavement, Pillmawr Road, Newport* (Jaggery, 7 août 2024, CC BY-SA 2.0 ; geograph 7844328, sur
Wikimedia Commons), lue par un canevas, sans téléchargement, en 960 × 1 280 : au premier plan, les **premières gouttes sur
l'asphalte sec** — même matériau, même lumière, même angle de vue. Zone (150–710, 950–1 280), aiguilles de pin écartées
(R − B ≥ 30) : fond sec sRGB ≈ 88, taches ≈ 31 ; **0,08 en linéaire** (courbe sRGB supposée), 0,35 en valeurs de code. Le
modèle donne ≈ 0,53 pour un asphalte d'albédo 0,1 vu à ≈ 50°. **Non attribué** (L177) : la courbe de l'appareil est inconnue
(un téléphone écrase les ombres) ; les gouttes reflètent un environnement sombre (arbres, mur) ; le second effet de Lekner et
Dorf n'est pas modélisé. D'autres recherches sont restées vaines (Commons : « partially wet », « rain shadow », « dry
patch » ; Geograph : vérification anti-robot, non contournée).

## 4 bis. Coût (1280 × 720, RTX 5070 Laptop, médiane de 240 images ; sans → avec mouillure)

| vue | 2 mm/h | 10 mm/h | 50 mm/h |
|---|---|---|---|
| ensemble | 0,729 → 1,084 ms | 1,039 → 1,381 | 2,200 → 2,548 |
| proche | 0,988 → 1,239 | 1,678 → 1,984 | 4,580 → 4,933 |
| rasante | 0,637 → 0,862 | 0,915 → 1,135 | 2,062 → 2,420 |

Sans pluie : 0,731 / 0,819–0,838 / 0,650, inchangé.

## 5. Limites

- **Le second effet** de Lekner et Dorf (pores remplis) : non modélisé ; la photographie suggère que le rendu peut être trop
  clair — R33 le dira, `ASSOMBRISSEMENT` le réglera sur une référence à courbe connue.
- **Film uniforme** : ni flaques (pièce 12), ni rides des gouttes sur le film, ni éclaboussures au sol (5b) ; le reflet est
  celui d'un film lisse légèrement rugueux (0,05 rad, à calibrer) — un béton réel le brise davantage.
- **Régime établi** : ni mouillage progressif au début de l'averse, ni séchage après (la météo) ; ni vent (les murs ne
  prennent la pluie qu'au pied).
- **Les occultants dans le reflet** rendus comme du béton sec ; ni interréflexion, ni les eaux des bacs reflétées.
- La mer n'a pas de surface solide émergée ; vérifié sur cette machine seulement ; images de R33 locales
  (`viewer/captures/s392/`).
