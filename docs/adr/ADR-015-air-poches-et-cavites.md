# ADR-015 — Air : poches, cavités et eau dans le vide

- **Statut** : proposée
- **Session** : S02
- **Résout** : `zones_ouvertes §25`, `architecture_globale §13.2` ; angle mort A18
- **Dépend de** : ADR-008, ADR-010, ADR-014

---

## 1. Décision : quatre niveaux, le quatrième refusé

| Niveau | Modèle | Coût | Quand |
|---|---|---|---|
| **T0 — air implicite** | l'air est une pression constante au-dessus de la surface libre ; solveur monophasique | nul | cas nominal, ≈99 % du temps |
| **T1 — fraction d'air** | champ `A` d'ADR-014, modifie densité et flottabilité | faible | eau blanche, sillage, chute d'eau |
| **T2 — poche discrète** | objet suivi : volume, pression, position, une EDO | négligeable | air piégé, cavité d'impact, compartiment qui coule |
| **T3 — solveur multiphasique** | **refusé** | ×2 à ×3 sur δ | — |

T3 est refusé sur un argument de rendement : il multiplie le coût du solveur volumétrique par deux
à trois pour un gain confiné à quelques plans, alors que T2 couvre tous les cas où l'air a une
**conséquence** — c'est-à-dire tous les cas qui comptent pour le gameplay.

---

## 2. Le cas que T0 traite mal, et qu'on découvre tard

Une coque retournée flotte **grâce à l'air qu'elle emprisonne**. Sans poche d'air modélisée, un
bateau qui chavire coule — comportement visiblement faux, systématiquement signalé en test, et
généralement corrigé par un bricolage de flottabilité au lieu de la cause.

De même : un compartiment qui s'inonde ne se remplit pas à la vitesse que donne Torricelli.

> **L'inondation d'un compartiment fermé est limitée par la sortie de l'air, pas par l'entrée de
> l'eau.** `Q_eau ≤ Q_air`.

C'est pour cela qu'un navire coule en dégazant, et pourquoi une brèche sous la ligne de flottaison
dans un compartiment étanche n'embarque presque rien. Le modèle d'ADR-010 doit donc porter des
**arêtes d'évent** ; sans elles, tous les temps d'inondation sont faux, dans le sens dangereux
(trop rapides).

---

## 3. Poche d'air — modèle T2

```
AirPocket {
    node        : HydroNode*      // le contenant, ou null si libre dans l'eau
    volume_ml   : i64
    n_moles     : f32             // constant sauf fuite
    pressure_pa : f32
    centroid    : vec3
}
```

Loi d'état, compression adiabatique rapide ou isotherme lente selon l'échelle de temps :

```
P · V^γ = constante          γ = 1,4 (rapide, impact)
P · V   = constante          (lente, descente d'une coque)
P_ambiante = P_atm + ρ · |g_eff| · profondeur
```

**Conséquence chiffrée, exploitable en gameplay :**

| Profondeur | Pression | Volume restant d'une poche |
|---|---|---|
| 0 m | 101 kPa | 100 % |
| 10 m | 202 kPa | 50 % |
| 20 m | 303 kPa | 33 % |
| 30 m | 405 kPa | 25 % |

Une coque retournée perd la moitié de sa poche d'air à 10 m. La flottabilité qu'elle procure
s'effondre avec la profondeur : un bateau chaviré flotte, puis passe un point de non-retour et
coule d'un coup. Ce comportement — dramatique, juste, et gratuit — sort d'une seule équation.

Le même modèle donne l'air respirable d'une poche : `volume × fraction O₂ utilisable`, avec une
consommation par occupant.

### 3.1 Remontée et référentiel

Une poche libre remonte selon `−g_eff` (ADR-002), pas selon +Z. Dans un vaisseau sous poussée,
les bulles « montent » vers l'avant. Détail à coût nul dès lors que `g_eff` est injectée, et
franchement faux si elle ne l'est pas.

---

## 4. Cavité d'entrée dans l'eau

L'entrée d'un corps rapide crée une cavité qui s'étire, se pince, puis produit un jet vertical
(*jet de Worthington*). C'est la signature visuelle d'un impact et la principale raison pour
laquelle un splash « pro » se distingue d'un splash générique.

Séquence à implémenter, avec les échelles :

```
1. couronne         t ≈ 0             éjection radiale, gouttes (Weber, ADR-014 §4)
2. cavité ouverte   t ~ √(D/g)        colonne d'air suivant le corps
3. pincement        profondeur ≈ 2 à 4 diamètres de corps
4. jet de Worthington                 hauteur comparable à la profondeur de pincement
5. remontée des bulles                alimente le champ A
```

Les coefficients 2 à 4 et la hauteur de jet sont des ordres de grandeur **à calibrer** (banc B10).
Le paramètre gouvernant est le Froude d'entrée `Fr = v/√(g·D)` ; au-dessus de `Fr ≈ 5`, la cavité
est franche et le jet spectaculaire ; en dessous, l'entrée est molle et il n'y a pas de pincement.

La cavité est un objet T2 : elle n'exige pas de solveur diphasique. δ produit la surface autour
d'elle ; la poche fournit son volume et sa pression.

---

## 5. Eau et vide — angle mort A18

Sous la pression du point triple (**611 Pa**), l'eau liquide n'existe plus. Une brèche vers le vide
produit donc, en une fraction de seconde :

1. **ébullition explosive** — l'eau flashe ;
2. **refroidissement par évaporation** — la chaleur latente prélevée gèle le reste.

Bilan énergétique, pour de l'eau initialement à 20 °C :

```
chaleur à évacuer par kg gelé = 4,18·20 + 334 ≈ 418 kJ/kg
chaleur absorbée par kg évaporé ≈ 2 500 kJ/kg
fraction évaporée ≈ 418 / (2 500 + 418) ≈ 14 %
```

> **≈14 % de la masse flashe en vapeur, ≈86 % devient de la glace.** Une brèche ne vide pas un
> réservoir dans l'espace : elle le bouche avec un bouchon de givre. Comportement contre-intuitif,
> visuellement fort, et qui change complètement l'équilibrage d'une avarie.

Modélisation : type d'arête V `to_vacuum` portant
- un débit massique plafonné par le col (écoulement critique), pas par Torricelli ;
- une **poussée de réaction** `F = ṁ·v_éjection` appliquée au porteur — un vaisseau percé dévie ;
- une production de glace qui obture progressivement l'arête, jusqu'à la fermer.

Ces trois effets sont des lignes de code, pas un système. Ils sont listés ici parce que leur
absence produit de l'eau qui coule paisiblement dans le vide — défaut immédiatement visible.

---

## 6. Ce qui reste ouvert

1. Calibration de la cavité d'entrée → banc B10.
2. Débit critique et vitesse d'éjection pour `to_vacuum` : formule des gaz parfaits en col sonique,
   à confronter au ressenti gameplay plutôt qu'à la précision.
3. Coalescence des poches T2 (deux compartiments qui communiquent) : règle de fusion à définir.
4. Air respirable : relève du gameplay survie, à cadrer avec l'équipe concernée. Le système d'eau
   fournit `volume` et `pression`, rien de plus.
5. Vapeur au contact d'une source chaude (tuyère, coulée) : traitée comme un effet volumétrique de
   rendu + un retrait de masse dans V. Pas de simulation de vapeur. À confirmer.
