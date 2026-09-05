# ADR-011 — Courants et écoulements dirigés

- **Statut** : proposée
- **Session** : S01
- **Résout** : `zones_ouvertes §15` (ouvert — important), `§16`
- **Dépend de** : ADR-004, ADR-010

---

## 1. Hiérarchie retenue

Quatre niveaux, sélectionnés par besoin et non par distance.

| Niveau | Représentation | Source | Coût | Quand |
|---|---|---|---|---|
| **C0** | vecteur constant interpolé | `HydroSample.current_uv` | nul | haute mer, cas nominal |
| **C1** | champ 2D régional | texture précalculée hors ligne, streamée | une lecture de texture | embouchures, détroits, littoral, courants d'auteur |
| **C2** | profil vertical analytique au-dessus de C0/C1 | formule | quelques opérations | dès qu'un objet immergé descend sous ≈2 m |
| **C3** | champ 3D local | propriété d'un domaine δ | inclus dans δ | remous, sillage proche, tourbillon derrière un rocher |

### C2 — profil vertical, formule retenue

Le courant n'est pas uniforme en profondeur, et c'est visible dès qu'un objet coule : sans profil,
tout ce qui descend dérive comme la surface, ce qui est faux et perceptible.

```
u(z) = u_fond + (u_surface − u_fond) · exp(z / D)          z ≤ 0
D = échelle de décroissance, issue de HydroSample (≈ profondeur d'Ekman ou valeur d'auteur)
Dérive de vent en surface : u_wind ≈ 0,03 · U10, déviée d'environ 20–30° du vent
```

Trois paramètres, aucune simulation. Suffisant pour tout ce que le joueur peut percevoir.

## 2. Règle d'autorité sur le champ de courant

> **Le champ macroscopique C0/C1 est en lecture seule pour les solveurs.** Une perturbation locale
> ne le modifie jamais.

`zones_ouvertes §15` laissait la question ouverte. La réponse est non, pour une raison qui n'est
pas de coût : un champ macroscopique inscriptible devient un état mutable, persistant et partagé,
donc à répliquer et à sauvegarder — il quitte la catégorie « gratuit » et entre dans celle de V.

La seule voie légitime pour modifier durablement un écoulement est **topologique et discrète** :
un barrage, une vanne, un effondrement modifient le débit `Q` d'un tronçon de rivière ou la
connectivité d'un nœud V. C'est un changement d'auteur, répliqué, journalisé, réversible.

## 3. Rivières et canaux : une entité, pas un océan à faible houle

Une rivière est une **polyligne** avec section transversale, pas un champ spectral.

```
RiverReach {
    centerline  : spline
    width(s)    : f32     profile(s) : section transversale
    Q           : f32     débit [m³/s], fonction du temps (saison, pluie, vannes)
    n_manning   : f32
}
v(s) = Q / A(s)                        // continuité — la vitesse suit le rétrécissement
```

### 3.1 La contrainte que le pipeline doit absorber : la surface d'une rivière est en pente

Loi de Manning : `v = (1/n)·R^(2/3)·S^(1/2)`, avec `R` rayon hydraulique et `S` pente de la ligne
d'eau. Inversée : `S = (v·n / R^(2/3))²`.

Pour un cours d'eau naturel (`n = 0,035`), `R = 1,5 m`, `v = 1,5 m/s` : **S ≈ 1,6 ‰**, soit
**1,6 m de dénivelé par kilomètre**.

Conséquence pour l'outillage, jamais évoquée dans les documents sources : on ne peut pas poser une
rivière comme un plan d'eau horizontal sur un terrain existant. Ou bien le terrain est sculpté à
partir du profil hydraulique, ou bien le débit est déduit du terrain. **Il faut choisir**, et
l'outil doit imposer le choix, sinon on obtient des rivières qui remontent visiblement leur lit.

Recommandation : l'outil auteur trace la ligne d'eau, en déduit `S` puis `Q`, et **grave le
terrain** le long du tracé. La rivière est source de vérité, le terrain suit.

### 3.2 Ressaut hydraulique

`Fr = v/√(g·h)`. Sous 1 : régime fluvial, l'information remonte le courant. Au-dessus : régime
torrentiel. Le passage torrentiel → fluvial produit un **ressaut hydraulique** — le rouleau
stationnaire des seuils et des rapides.

Décision : le ressaut est détecté hors ligne le long de la polyligne (là où `Fr` franchit 1) et
marqué comme *site turbulent permanent*. Il devient un émetteur W stationnaire, et déclenche un
domaine δ substitutif seulement en présence d'un observateur ou d'un acteur. Cela répond aussi à
`§16` « récupération du débit après obstacle » : `L_relaxation ≈ 6 à 10 × largeur de l'obstacle`,
mélange linéaire vers le champ de la polyligne.

## 4. Effet non anticipé : le sillage change de nature en eau peu profonde

Le demi-angle de Kelvin (19,47°) n'est valable qu'en eau profonde. Le paramètre gouvernant est le
**nombre de Froude de profondeur** `Fr_h = v / √(g·h)`.

| Régime | Comportement du sillage |
|---|---|
| `Fr_h < 0,7` | sillage de Kelvin classique, 19,47° |
| `Fr_h ≈ 1` | **vitesse critique** : les vagues transverses ne peuvent plus s'échapper, l'amplitude et la traînée explosent, une onde solitaire précède le navire |
| `Fr_h > 1` | cône de type Mach, demi-angle `arcsin(1/Fr_h)`, plus de vagues transverses |

Chiffres : par 5 m de fond, `√(gh) = 7,0 m/s` (13,6 nœuds). Un bateau à 13 nœuds dans un chenal de
5 m produit un sillage sans commune mesure avec le même bateau au large — c'est la raison physique
des limitations de vitesse en canal, et un ressort de gameplay gratuit (érosion des berges,
chavirement des petites embarcations, détection).

Implication technique : le générateur de sillage de la couche W doit prendre `h` en entrée, pas
seulement `v`. Un générateur qui code 19,47° en dur sera faux dans toutes les zones côtières,
c'est-à-dire précisément là où le joueur regarde.

## 5. Lacs

- Pas de débit traversant : le lac est un **nœud V de grande taille** dont la hauteur pilote un
  `HydroSample` local. Apports (rivières entrantes, pluie, ruissellement) et sorties (exutoire,
  évaporation, infiltration) sont des arêtes V.
- Houle bornée par le fetch. Loi utilisable : `Hs ≈ 0,0016 · U10 · √(F/g)` (F = fetch en m).
  Pour `U10 = 12 m/s` et `F = 3 km` : `Hs ≈ 0,33 m`. Un lac ne fait pas de grosses vagues, et le
  système doit l'imposer plutôt que l'espérer d'un réglage d'auteur.
- **Seiche** : un bassin fermé a un mode propre `T = 2L/√(g·h)`. Pour L = 3 km, h = 30 m :
  `T ≈ 350 s`. Amplitude faible mais visible sur les berges après un séisme ou un impact majeur.
  À retenir comme effet peu coûteux et rare — un oscillateur à un degré de liberté par lac.

## 6. Marée — conséquence hors du système d'eau

La marée modifie `depth`, donc le courant, donc **la position du trait de côte**. Un marnage de
3 m sur une plage de pente 1:50 déplace le rivage de **150 m**.

Ce déplacement affecte la collision, le maillage de navigation, les points d'apparition, la
végétation, l'audio et le gameplay (accès à une grotte, échouage d'un bateau). Ce n'est pas un
problème du système d'eau, mais le système d'eau en est la **source** et doit publier :

- `shoreline_offset(région, T_sim)` interrogeable et prédictible à l'avance ;
- des événements de franchissement de seuil pour les systèmes abonnés.

À porter à l'ordre du jour des équipes terrain, IA et audio. C'est typiquement le genre de
dépendance qui, découverte tard, coûte un trimestre.

## 7. Ce qui reste ouvert

1. Résolution et format des textures C1, et leur mode de production (solveur hors ligne ? auteur ?).
2. Le lac comme nœud V grand format : passage à l'échelle du `shape_lut` pour un lac de 10 km².
3. Modèle de marée : harmonique global (4–8 constituantes) vs table par région.
4. Couplage courant ↔ houle (les vagues se raidissent contre le courant, s'aplatissent avec lui).
   Effet réel et spectaculaire dans les embouchures. Formule de décalage Doppler
   `ω_apparent = ω + k·U` disponible ; à activer si le rendu le justifie.
