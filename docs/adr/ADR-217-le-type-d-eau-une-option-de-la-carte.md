# ADR-217 — Le type d'eau, une option d'édition de la carte ; le ciel qui bouge, à l'atmosphère

- **Statut : actée**, S473, 2026-10-04 ; **réponses de l'utilisateur** aux écarts E1 et E4 des scènes miroirs
  ([MIROIRS-S472](../validation/MIROIRS-S472.md)), telles qu'écrites :

  > Le type d'eau va être une option d'edition dans la création de la map du jeu, exemple l'eau de rivière seras plus vertes a des
  > endroits, avec moins de visibilité etc....
  > Ne prends pas en compte le mouvement des nuages, car le système d'atmosphère et de climat seras a réaliser a la suite du système
  > de l'eau une fois terminé.

- **Précise** [ADR-216](ADR-216-le-banc-visuel.md) (le banc : ce qu'il compare, ce qu'il écarte) et
  [ADR-197](ADR-197-reponses-du-2026-09-26.md) D5 (la météo à la fin : l'atmosphère et le climat suivent le système de l'eau).

## 1. Décisions

**D1 — Le type d'eau est une propriété de la carte, éditée, qui varie d'un endroit à l'autre.** Pas une constante du nuanceur : une
donnée que l'éditeur de carte pose — par zone, avec des transitions (une rivière plus verte à des endroits, moins de visibilité ;
l'embouchure qui se mêle à la mer). Ce qu'elle porte : les propriétés optiques de l'eau — l'absorption et la diffusion par longueur
d'onde, d'où la couleur du corps d'eau (`R0`), l'atténuation (`kd`) et la **visibilité** —, tirées de quelques réglages lisibles
(chlorophylle, matière dissoute, matière en suspension) et de préréglages nommés (océan clair, Méditerranée, côtier, lac, rivière,
eau trouble). Le rendu (Godot) les lit en chaque point ; un point sans donnée prend celle de la carte.

**D2 — Le banc juge chaque préréglage contre sa référence.** E1 n'est plus « la couleur est fausse » : la référence V1 (mer profonde)
juge le préréglage Méditerranée, V5 (baie côtière) le préréglage côtier — chacun dans la tolérance de ses grandeurs de teinte, ou
l'écart dit. Un préréglage sans référence attend la sienne.

**D3 — Le ciel qui bouge n'est pas du système de l'eau.** Les nuages, leur dérive, le vent du ciel : le système d'atmosphère et de
climat, réalisé **après** le système de l'eau. Le banc écarte le mouvement du ciel (E4) ; le ciel reste ce qu'il est, une source de
lumière et de reflets pour l'eau.

## 2. Conséquences

- E4 est retiré de la suite des écarts ; E1 devient le chantier « type d'eau » (le champ de la carte, les préréglages, le rendu qui
  les lit), après E2.
- L'éditeur de carte n'existe pas encore : le champ se crée d'abord comme une donnée de la scène (une texture ou des zones), que
  l'éditeur écrira plus tard.
