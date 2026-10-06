# ADR-229 — La paroi elle-même dans la vitesse gouvernante d'une grille coupée

- **Statut : actée**, S505, 2026-10-06 ; décision technique prise ici ([ADR-215](ADR-215-autonomie-jusqu-a-une-v1-solide.md) D2).
- **Précise** [ADR-035](ADR-035-le-nombre-de-courant-definition-borne-valeur.md) §2 (la vitesse gouvernante) pour le δ 3D à faces coupées.
  La mesure et C23 : [C23-SYSTEME-S505](../validation/C23-SYSTEME-S505.md).

## 1. Ce qui a été trouvé

ADR-035 définit la vitesse gouvernante comme celle du fluide **relative à la paroi** sur une face coupée. Sur la grille coupée du δ 3D,
cela ne suffit pas : la coque de la porte D en translation à 5 m/s, des pas qui lui font franchir `k` mailles par pas, contre le calcul fin
— l'écart vaut 5 à 7 % jusqu'à `k` = 0,5, **100 % à `k` = 1**, 150 et 190 % au-delà. Une paroi qui franchit une maille entière ouvre et
ferme des faces sans l'état intermédiaire ; et le fluide qui la suit n'a plus de vitesse relative pour l'annoncer.

## 2. Décision

**D1 — La vitesse gouvernante du δ 3D contient la vitesse de la paroi sur la grille** : le maximum de la vitesse relative du fluide sur les
faces coupées, de la vitesse absolue ailleurs, **et de la vitesse de la paroi elle-même** (`Volume3::governing_speed`, la vitesse que
l'hôte imposera au pas). La borne `ν·dx/(u_gouvernante + c)` garde alors la paroi sous `ν` maille par pas.

**D2 — Borne et compteur d'une même fonction** (ADR-035 §3) : `Volume3::courant_bound` en amont, `Volume3::courant` après coup ; la
célérité majorée par la profondeur du domaine au repos (`√(g·z₀)`).

**D3 — Le cœur borne, l'hôte choisit** : le δ 3D ne choisit toujours pas son pas ; l'hôte le prend sous la borne. Rien n'est refusé au-delà
— un refus viendra d'une scène qui en aura besoin.

## 3. Ce qui changerait la décision

Un découpeur qui suivrait la paroi en sous-pas (les faces franchies une à une) : la paroi pourrait alors franchir plusieurs mailles par pas
du fluide, et sa vitesse sortirait de la borne.
