# ADR-002 — Référentiels, précision numérique et planète sphérique

- **Statut** : proposée
- **Session** : S01
- **Dépend de** : ADR-001
- **Comble** : angles morts A01, A02, A03, A23 — aucun de ces points n'apparaît dans les documents sources

---

## 1. Problème

Les deux documents sources décrivent implicitement une eau posée sur un plan, dans un monde
statique, exprimé en coordonnées flottantes simples. Pour un jeu de l'échelle visée, les trois
hypothèses sont fausses simultanément.

1. **L'eau n'est pas toujours dans un référentiel galiléen.** Une citerne dans un vaisseau en
   accélération, une piscine sur un navire qui roule, un bassin dans une station à gravité
   centrifuge : la surface libre s'oriente sur la gravité *apparente*, pas sur −Z.
2. **Un océan planétaire n'est pas plan.** L'horizon est à 3,57·√h km. Depuis une passerelle à
   30 m, il est à 19,6 km — dans le champ de tir et de détection. Un océan plan est visiblement
   faux et fausse le gameplay (un navire à 20 km doit être *coque noyée*).
3. **float32 ne couvre pas une planète.** L'ulp d'un float32 vaut d·2⁻²³.

| Distance à l'origine | Résolution float32 |
|---|---|
| 4 096 m | 0,49 mm |
| 65 km | 7,8 mm |
| 1 000 km | 0,12 m |
| 6 371 km (rayon terrestre) | 0,76 m |

Une vague capillaire de 2 cm est indistinguable du bruit d'arrondi au-delà de 150 km d'origine.

---

## 2. Décision

### 2.1 Tout domaine d'eau appartient à un référentiel (`FrameRef`)

```
FrameRef {
    id              : u32
    parent          : FrameRef*          // chaînage : station → vaisseau → citerne
    origin_world    : vec3<f64>          // ou point fixe int64, cf. §2.3
    orientation     : quat<f64>
    g_local         : vec3<f32>          // gravité propre du référentiel
    a_linear        : vec3<f32>          // accélération du référentiel
    omega           : vec3<f32>          // vitesse de rotation
    domega_dt       : vec3<f32>
}
```

La gravité apparente en un point **r** exprimé dans le référentiel, pour une particule de
vitesse **v** :

```
g_eff = g_local − a_linear − ω×(ω×r) − 2·ω×v − (dω/dt)×r
```

Aucun solveur ne reçoit « −9,81 sur Z ». Tous reçoivent `g_eff`, évalué par le référentiel.

### 2.2 Conséquences physiques mesurées, et non négligeables

**Station à gravité centrifuge.** Pour 1 g à r = 100 m : ω = √(g/r) = 0,313 rad/s (≈3 tr/min).

- *Coriolis* : 2ωv = 0,63 m/s² pour v = 1 m/s, soit **6,4 % de g**. Un jet d'eau versé sur 2 m
  de chute dévie latéralement de ≈2 cm ; un train de vaguelettes dérive visiblement. L'effet est
  perceptible à l'échelle humaine, pas seulement à l'échelle géophysique.
- *Courbure de la surface au repos* : la surface d'équilibre est un **cylindre**, pas un plan.
  Pour un bassin de longueur L le long de la circonférence, la flèche vaut
  f = r − √(r² − (L/2)²). Pour L = 20 m et r = 100 m : **f = 50 cm**. Une piscine de 20 m dans un
  anneau de 100 m de rayon a ses extrémités 50 cm plus hautes que son centre.

Ce chiffre invalide toute implémentation de bassin par « plan d'eau à altitude constante ». Le
niveau de repos d'un volume fini est une **isosurface de potentiel effectif**, pas une constante.

**Navire en mer.** Une citerne à bord subit l'accélération du navire ; le ballottement (*sloshing*)
rétroagit sur la stabilité. Le couplage est réel et connu pour faire chavirer des navires réels.
Décision : la rétroaction citerne → navire est **optionnelle et bornée** (ADR-008 §4), activée
seulement pour les compartiments dépassant un seuil de masse relative.

### 2.3 Précision : origine flottante obligatoire

- Position monde : `f64` par composante, ou point fixe `int64` en micromètres (portée ±9,2·10¹² m,
  résolution 1 µm — préféré si un déterminisme inter-plateforme est requis sur des positions).
- **Tout calcul d'eau se fait en `f32` dans l'espace local du `FrameRef`.**
- Rebasage forcé dès que |x_local| > 4 096 m → résolution garantie ≤ 0,5 mm.
- Les domaines δ sont *par construction* locaux à leur référentiel : le rebasage est gratuit pour
  eux. Les paquets W stockent des positions locales à leur région hydrographique.
- B ne stocke rien : il est évalué à partir de la position locale à l'ancre de région.

### 2.4 Océan planétaire : ancres de région, pas de pavage global

Le champ B est défini sur une **tessellation statique de la planète** (faces d'un cube-sphère
subdivisées) en *régions hydrographiques* de l'ordre de 25–50 km de côté, chacune portant une
ancre et un plan tangent.

- Distorsion angulaire d'un plan tangent sur 50 km pour R = 6 371 km : ≈ (L/2R)²/3 ≈ 1,4·10⁻⁶ rad.
  **Négligeable** — la projection tangente est valide.
- Le déplacement de surface est appliqué le long du **radial local (up)**, pas d'un axe Z fixe.
- Le maillage de rendu suit le géoïde : niveau moyen = sphère + marée + anomalie régionale.
  L'horizon devient correct, et l'occlusion *coque noyée* fonctionne sans code spécifique.
- **Les régions ne sont pas des pavés indépendants** : voir ADR-004 §3, la continuité est obtenue
  en interpolant les *paramètres* sur une grille lisse, pas en raccordant des champs voisins.

### 2.5 Ce qui est explicitement hors périmètre

- Coriolis planétaire (f = 2Ω sin φ) sur les courants : traité comme donnée d'auteur dans le champ
  de courant macroscopique (ADR-011), jamais calculé à l'exécution. Le nombre de Rossby
  Ro = U/(fL) vaut ≈1 pour U = 1 m/s, L = 10 km : l'effet est réel mais il appartient à
  l'échelle de l'heure et du kilomètre, donc à l'authoring.
- Relativité, dilatation temporelle : voir ADR-003 §4.

---

## 3. Conséquences

- Le `WaterManager` n'a pas d'espace monde unique : il indexe par `(FrameRef, cellule locale)`.
- Deux joueurs dans deux référentiels différents (l'un sur la planète, l'autre dans un vaisseau
  en vol) n'ont **aucun domaine d'eau en commun**. Cela simplifie l'intérêt réseau (ADR-009).
- Un transfert de liquide entre référentiels (verser de l'eau depuis un vaisseau en vol) est un
  changement de référentiel de la masse transportée, pas un transfert de champ. Ce cas doit
  passer par la couche V ou par des particules balistiques, jamais par une fusion de domaines.
- Le calcul de `g_eff` doit être disponible sur GPU : le référentiel pousse ses paramètres dans un
  buffer constant par domaine.

---

## 4. Ce qui reste ouvert

1. ~~Point fixe `int64` vs `f64` pour les positions monde~~ — **tranché en S19 par
   [ADR-028](ADR-028-il-n-y-a-pas-d-autres-equipes.md) §3 : `int64` en virgule fixe, résolution
   `1/2048 m`, portée ±4,5·10¹⁵ m.**
   La résolution se **dérive** : c'est exactement l'ulp d'un `f32` au rayon de référentiel
   (`4096 · 2⁻²³ = 2⁻¹¹ m`), donc la seule valeur qui ne perde rien à la conversion et n'en stocke pas
   davantage. Motif principal : avec des entiers, le déterminisme inter-plateforme d'I-03 devient
   **structurel** au lieu de dépendre d'une discipline de compilation qu'un drapeau peut casser en
   silence. *(La « décision partagée avec l'équipe réseau » annoncée ici n'avait pas d'interlocuteur —
   ADR-028 §2.)*
2. Taille exacte des régions hydrographiques → dépend de la portée de visibilité maximale et du
   coût de streaming des descripteurs. Piste : 32 km, révisable.
   → **S11** : deux contraintes se sont ajoutées depuis. **SPEC-005 §4** : l'écart entre le géoïde et le plan
   tangent vaut `R(1−cos(d/R))`, soit 70,7 m à 30 km et **80 m au bord d'une région de 32 km** —
   l'outil de terrain doit appliquer le géoïde. **I-08** : une région n'est pas un référentiel de
   calcul, `|x_local| < 4096 m` ; c'est une ancre de repère, et il faut le dire pour que « 32 km »
   ne se lise pas comme une contrainte de précision.
3. Seuil de masse relative déclenchant la rétroaction ballottement → navire.

## Note datée du 2026-10-07 (S642)

§2.4 : la cube-sphère n'est plus retenue d'avance — HEALPix (celui de DyingStar) ou cube-sphère se tranche par une étude mesurée, performance et résultat ([ADR-261](ADR-261-reponses-du-2026-10-07.md) D2).

## Note datée du 2026-10-07 (S649)

§2.4 : tranché par [ADR-264](ADR-264-le-decoupage-de-la-planete.md) — HEALPix, celui du terrain de DyingStar, découpe et indexe les données
planétaires de l'eau ; le calcul reste dans des référentiels locaux ; un modèle global hors ligne peut calculer sur une cube-sphère et cuire
en HEALPix.
