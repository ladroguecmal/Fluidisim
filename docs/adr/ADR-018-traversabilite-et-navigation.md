# ADR-018 — Traversabilité, navigation et danger

- **Statut** : proposée — interface à confirmer avec l'équipe IA
- **Session** : S02
- **Comble** : angle mort A20 ; complète ADR-011 §6
- **Dépend de** : ADR-004, ADR-010, ADR-011, ADR-017

---

## 1. Décision

**Le système d'eau ne modifie pas le maillage de navigation. Il publie un signal.**

```
TraversabilitySample {           // par cellule HydroGrid, mis à jour à basse fréquence
    depth        : f16     // profondeur d'eau au-dessus du sol
    flow_speed   : f16     // norme du courant de surface
    hazard       : f16     // produit de danger, cf. §3
    ice_h        : f16     // épaisseur de glace porteuse, 0 si absente
    temp         : f16     // hypothermie, gel
    trend        : i8      // monte / stable / descend
    t_next_cross : f32     // temps avant le prochain franchissement de seuil, cf. §4
}
```

L'équipe IA décide ce qu'elle en fait : invalidation de polygones, coût de traversée, annotation.
Le système d'eau n'a pas d'opinion sur la représentation de la navigation, et ne doit pas en avoir.

Le signal est publié **par cellule `HydroGrid`** et par **événement de franchissement de seuil**,
jamais en interrogation continue — sans quoi la navigation devient un consommateur majeur de
`EvalWater`.

---

## 2. Seuils de profondeur

| Profondeur | Effet sur un humanoïde |
|---|---|
| < 0,15 m | négligeable, éclaboussures |
| 0,15 – 0,50 m | marche ralentie, bruit, traces |
| 0,50 – 1,00 m | progression fortement ralentie, course impossible |
| 1,00 – 1,30 m | vadrouille, équilibre précaire |
| > 1,30 m | nage |

Pour un véhicule, le seuil est le **passage à gué** : profondeur admissible propre au châssis, avec
un risque de noyade du moteur au-delà. Pour un bateau, c'est l'inverse — un **tirant d'eau
minimal** : `profondeur − tirant − marge de houle > 0`.

Cette dernière ligne implique que la navigation nautique a besoin de son propre graphe, construit
sur la bathymétrie : une carte marine pour l'IA. À prévoir dans le même outillage que le précalcul
côtier, puisque les données sont identiques.

---

## 3. Danger : le produit d'emportement

Un adulte est emporté bien avant d'avoir de l'eau à la taille. Le critère utilisé en hydrologie de
crue est le **produit de danger** :

```
HR = d · (v + 0,5)              d en m, v en m/s
```

| HR | Interprétation |
|---|---|
| < 0,75 | faible |
| 0,75 – 1,25 | dangereux pour certains |
| 1,25 – 2,50 | dangereux pour la plupart |
| > 2,50 | dangereux pour tous |

Concrètement : **50 cm d'eau à 2 m/s (HR = 1,25) emporte déjà un adulte.** Un gué praticable à
l'étiage devient mortel en crue sans que la profondeur ait beaucoup changé — c'est la vitesse qui
domine. Un système qui ne raisonne qu'en profondeur laissera les PNJ traverser des torrents.

Pour un véhicule, le critère est la flottaison : une voiture flotte à partir d'environ 30 cm et
est emportée vers 60 cm.

---

## 4. La marée est prédictible — et c'est exploitable

B étant analytique (ADR-004), `depth(x, t)` est calculable **à l'avance**, sans simulation. Le
système publie donc `t_next_cross` : le temps restant avant qu'une cellule franchisse un seuil de
traversabilité.

Cela donne à l'IA et au gameplay des capacités qu'aucune simulation ne pourrait offrir à ce coût :

- « ce gué se ferme dans 40 minutes » ;
- un PNJ qui planifie un trajet côtier en fonction de la marée ;
- une mission qui n'est réalisable qu'à basse mer ;
- un bateau échoué qui sera reflotté à une heure connue.

C'est un bénéfice direct et non anticipé du choix d'un fond analytique plutôt que simulé.

---

## 5. Le cas inverse : la glace ajoute de la surface

Un lac gelé devient traversable. La plupart des générateurs de maillage de navigation savent
retirer des zones, pas en ajouter, et surtout pas des zones dont la portance dépend d'une
épaisseur variable (ADR-017 §3).

Décision : la glace porteuse est publiée comme une **surface navigable conditionnelle**, avec sa
charge admissible. Un agent trop lourd la voit comme infranchissable ; un agent léger la traverse.
Le franchissement du seuil de charge doit émettre un événement — c'est ce qui permet la scène où
la glace cède.

À porter à l'équipe IA **avant** qu'elle ne fige son format de maillage : rétroporter une surface
navigable dynamique à portance variable est cher.

---

## 6. Fréquences de publication

| Phénomène | Échelle de temps | Fréquence de publication |
|---|---|---|
| Marée | heures | 1 / 30 s, + événements de seuil |
| Crue de rivière, ouverture de vanne | minutes | 1 / 2 s |
| Inondation d'un compartiment | secondes | 5 Hz, par nœud V |
| Rupture de barrage, brèche | < 1 s | événement immédiat |

---

## 7. Ce qui reste ouvert

1. Validation de l'interface avec l'équipe IA.
2. Granularité : la cellule `HydroGrid` de 64 m est trop grossière pour un gué. Prévoir une
   publication à la sous-cellule le long des rivières et des rivages.
3. Table des seuils par archétype d'agent et de véhicule — à obtenir des équipes concernées.
4. Faut-il exposer aussi la **visibilité sous l'eau** pour l'IA (détection d'un nageur immergé) ?
   La donnée existe (ADR-019 §3) ; l'usage est à confirmer.
