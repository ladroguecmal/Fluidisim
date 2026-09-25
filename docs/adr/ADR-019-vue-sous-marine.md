# ADR-019 — Vue sous-marine et interface de surface

- **Statut** : proposée
- **Session** : S02
- **Comble** : angle mort A27
- **Dépend de** : ADR-004, ADR-012, ADR-014, ADR-016

---

## 1. Pourquoi cet ADR existe

Toute l'analyse de niveau de détail des documents sources est faite depuis l'extérieur : distance,
surface à l'écran, angle de vue. Depuis l'eau, les critères s'inversent — la surface est au contact
de la caméra, elle occupe la totalité du champ, et ce qui la rend lisible n'est plus sa géométrie
mais son optique.

Un LOD calibré depuis l'extérieur est donc faux sous l'eau, et il l'est précisément pendant les
moments d'immersion, qui sont des moments forts.

**Décision : l'état « caméra immergée » est un mode de premier niveau du système d'eau**, avec son
propre profil de LOD, son propre jeu de champs consommés et son propre budget.

---

## 2. La fenêtre de Snell

L'indice de réfraction de l'eau vaut 1,333. L'angle critique est
`arcsin(1/1,333) = 48,6°`, soit un cône de **97,2° d'ouverture totale**.

Conséquences, toutes visibles et toutes gratuites si elles sont prévues :

- **Le monde aérien tout entier** — horizon à horizon — est comprimé dans ce cône, au-dessus de la
  tête de l'observateur. Un disque, pas un ciel.
- **Au-delà du cône**, la surface est un **miroir** par réflexion totale interne : on y voit le
  fond, les objets immergés, ses propres bulles. C'est la caractéristique la plus reconnaissable
  d'une vue sous-marine, et la plus souvent omise.
- Le bord du disque est chromatiquement dispersé et fortement déformé par les vagues ; par mer
  agitée, la fenêtre se fragmente.

Depuis l'air vers l'eau, la même réfraction fait paraître un objet immergé à **3/4 de sa
profondeur réelle** en visée verticale — ce qui décale la perception de la profondeur d'un gué ou
d'un obstacle.

---

## 3. Atténuation : la couleur est une distance

L'eau pure absorbe très inégalement selon la longueur d'onde :

| Longueur d'onde | Coefficient | Distance à 1 % de transmission |
|---|---|---|
| 650 nm (rouge) | ≈0,34 m⁻¹ | ≈13 m |
| 550 nm (vert) | ≈0,06 m⁻¹ | ≈75 m |
| 450 nm (bleu) | ≈0,015 m⁻¹ | ≈300 m |

D'où : le rouge disparaît en quelques mètres — le sang paraît vert à 10 m — et l'eau profonde et
claire tire au bleu. Les eaux côtières tirent au vert parce que les matières organiques dissoutes
absorbent le bleu ; c'est le paramètre qui distingue une eau tropicale d'une eau de Manche, et il
appartient à `HydroSample`.

**Distance de visibilité** : liée au coefficient d'atténuation total `c` par
`profondeur de Secchi ≈ 1,7/c`.

| Milieu | Visibilité |
|---|---|
| Océan clair, tropical | 30 – 50 m |
| Côtier tempéré | 5 – 15 m |
| Estuaire, crue | 0,2 – 1 m |

Cette valeur est aussi celle que la détection sous-marine doit utiliser (ADR-018 §7). Une seule
donnée pour le rendu et pour le gameplay.

---

## 4. Ce que la vue sous-marine consomme

| Champ | Origine | Usage |
|---|---|---|
| normales de surface | B + W + δ | fenêtre de Snell, réflexion totale, caustiques |
| `A` — aération | ADR-014 §5 | bulles proches, opacité, rideaux |
| `F` — écume | ADR-014 | face inférieure de l'écume, très différente de sa face supérieure |
| turbidité, couleur | `HydroSample` | atténuation, visibilité |
| profondeur, hauteur de surface | `EvalWater` | pression, colonne, shafts de lumière |

**Caustiques** : projetées à partir des mêmes normales que le rendu de surface, pas d'une texture
indépendante. Elles doivent réagir à un impact — l'omission est visible dès qu'un objet tombe à
l'eau au-dessus d'un fond clair. Elles s'appliquent aussi **au-dessus** de l'eau, sur une coque ou
sur une paroi proche de la surface.

---

## 5. Profil de LOD immergé

Le poids se déplace : la géométrie de surface compte moins que le volume proche.

| Composante | Extérieur | Immergé |
|---|---|---|
| géométrie de surface lointaine | élevé | **faible** — au-delà de la visibilité, elle est invisible |
| géométrie de surface au-dessus de la caméra | faible | **maximal** — c'est le sujet |
| particules et aération | moyen | **maximal** |
| écume vue par-dessous | nul | moyen |
| caustiques | faible | élevé |
| rendu volumétrique / shafts | faible | élevé |

Le budget total ne change pas ; sa répartition, oui. Le critère `W_perception` d'ADR-012 §2 reçoit
donc un état supplémentaire.

---

## 6. La caméra à demi immergée

Le plan où la ligne d'eau traverse l'objectif est une signature du genre et un piège technique :
il exige de résoudre l'intersection de la surface avec le plan proche de la caméra, avec deux
milieux, deux profils d'atténuation et deux mixages audio simultanés (ADR-016 §4).

À traiter explicitement comme un cas nommé, pas comme un cas limite du rendu de surface. Les
tentatives de le faire émerger d'un rendu générique produisent une ligne de flottaison qui
scintille — défaut très visible et coûteux à corriger tard.

---

## 7. Ce qui reste ouvert

1. Modèle de diffusion (*scattering*) retenu : simple terme d'extinction + diffusion en avant, ou
   diffusion volumétrique complète ? Compromis coût/qualité à trancher avec l'équipe rendu.
2. Transition d'exposition et d'adaptation à l'entrée dans l'eau.
3. Réfraction des objets vus à travers la surface : approximation par décalage d'écran, ou tracé
   correct ? Le décalage suffit probablement sauf à courte distance.
4. Sous-marins profonds : au-delà de la visibilité, tout est noir et le rendu devient un problème
   d'éclairage porté, pas d'eau. Frontière à poser avec l'équipe rendu.

> **Note du 2026-09-25 (S365).** Première réalisation, dans Godot ([SOUS-MARIN-S365](../validation/SOUS-MARIN-S365.md)) :
> §2 — la fenêtre de Snell, rendue à 0,05° de `arcsin(1/n)`, et la réflexion totale au-delà ; §3 — l'atténuation du
> faisceau `c = a + b` par canal, relue à 0,004 près. Pour §7.1, le prototype retient **l'extinction et une source de
> diffusion proportionnelle à l'éclairement local**, intégrée exactement le long de la ligne de visée ; la diffusion
> volumique complète reste ouverte. §6, la caméra à demi immergée, n'est pas traitée : le mode bascule d'un bloc.
