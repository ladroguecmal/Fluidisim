# ADR-195 — La queue de B se rend par une réalisation dense, calculée par FFT

- **Statut : actée**, S360, 2026-09-25 — décision technique de session (S71, ADR-028), après le verdict R20 (*« ce
  rendue du point de vue topologie est pas réaliste »*) et sur les mesures de [SURFACE-FINE-S360](../validation/SURFACE-FINE-S360.md).
- **Précise** [ADR-155](ADR-155-queue-spectrale-en-pentes-par-pixel.md) — la queue se rend en pentes par pixel — pour
  **Godot** : non plus 60 composantes évaluées au pixel, mais une réalisation dense du même spectre. Et
  [ADR-156](ADR-156-mer-multimodale-et-etalement.md) pour cette réalisation : l'étalement d'Elfouhaily plutôt que Mitsuyasu figé.
- **Laisse entiers** B et sa recette (ADR-001, I-02, I-03 : la bande et ses requêtes de jeu ne changent pas), le niveau
  de la queue (ADR-157), ADR-192 à 194.

## 1. Le constat

Soixante ondes planes pour 5,5 octaves, dirigées sur tout le cercle, ne dessinent pas une mer : elles interfèrent en
taches sans direction ni crête. Mesuré : pentes de la queue plus fortes en travers qu'au vent (0,88), quand Cox et Munk
observent 1,37 au vent. La queue n'est pas une grandeur de jeu : elle ne sert qu'à la lumière (ADR-155).

## 2. Décisions

**D1 — La queue se réalise densément.** Deux cascades de 256 × 256 composantes (32 m et 4 m de côté), même densité
spectrale que la queue discrète — fonction du cœur, `equilibrium_tail_density` —, amplitudes de départ tirées d'une
graine fixe par l'afficheur, évolution et FFT inverse sur la carte dans Godot. Rendu seulement (I-13) ; déterministe
à graine égale, sans autorité (I-04, I-15). Le temps passe replié sur une période de répétition, la pulsation quantifiée
(I-08).

**D2 — L'étalement de la queue réalisée est celui d'Elfouhaily et al. (1997)**, fonction du cœur (`elfouhaily_delta`),
**replié sous le vent** : mêmes moments d'ordre deux, des vagues qui courent avec le vent.

**D3 — L'écume et la lumière des crêtes se tirent des vagues dominantes** — la bande seule, à une empreinte d'au moins
un mètre, l'échelle des plus petits moutons mesurés —, pour la même couverture de Monahan.

**D4 — L'afficheur n'en change pas** : il garde sa queue de 60 composantes et ses seuils d'écume, qui portent ses
réceptions. Godot et le banc divergent donc sur la surface fine, et **c'est dit** ; qui voudra les réunir portera D1 et
D3 dans `water.wgsl`.

## 3. Ce que la décision ne tranche pas

- L'anisotropie reste 10 % sous Cox et Munk (1,23 pour 1,37) : les capillaires sous 7 cm, ou un autre spectre
  omnidirectionnel, la combleraient ; aucun seuil n'est déplacé pour la faire passer.
- La répétition des cascades, leur coût, l'écume dans le temps (durée de vie, traînées).

## 4. Ce qui la renverserait

Une mesure — sur l'image ou par la revue — montrant que la réalisation dense rend la surface moins juste que les
60 composantes ; ou un besoin de jeu qui interroge la queue en hauteur, ce qu'elle n'est pas.
