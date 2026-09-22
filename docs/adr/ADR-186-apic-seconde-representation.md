# ADR-186 — APIC est la seconde représentation de surface libre

- **Statut : actée**, S319, 2026-09-21, **décision de l'utilisateur** : *« Ok pour APIC »*, en
  réponse à la proposition de [COMPARAISON-LOT5-S318](../validation/COMPARAISON-LOT5-S318.md) §7.
- **Tranche** la question qu'[ADR-184](ADR-184-seconde-representation-en-parallele.md) D2 et
  [ADR-178](ADR-178-strategie-en-trois-systemes-physiques.md) D7 (lot 5) laissaient ouverte, et
  qu'[ADR-175](ADR-175-architecture-d-execution-de-delta-en-3d.md) D5 renvoyait à « particules ou
  surface implicite ».
- **Ne touche pas** à l'ordre des lots (ADR-178), à l'alternance (ADR-184 D1), ni au périmètre
  (ADR-127).

## 1. Ce qui est décidé

**D1 — Là où plusieurs couches d'eau tiennent sur une même verticale — cavité, jet, déferlement,
gerbe —, l'eau de δ est portée par des particules sur la grille, en APIC.** Des particules portent
la vitesse et une matrice affine ; la grille MAC de δ calcule la pression ; la surface se
reconstruit des particules et la pression la voit par fluide fantôme.

**D2 — La grille et la pression sont celles du δ existant.** La zone à plusieurs couches n'est pas un
second solveur : ce sont des cellules de la même grille MAC où des particules portent l'eau, résolues
par la même projection — et, en production, par la même multigrille sur la carte (ADR-175).

**D3 — La fonction hauteur reste la représentation par défaut.** Là où la surface est un graphe —
presque partout —, δ garde ses colonnes (ADR-175 D5) ; les particules ne vivent que là où elles sont
nécessaires. Le passage de l'une à l'autre — son critère et son raccord — est un travail du lot 5,
pas une décision prise ici.

**D4 — L'ensemble de niveaux n'est pas écarté, il change de rôle.** APIC s'en sert pour reconstruire
sa surface depuis les particules, et c'est la voie naturelle du raccord aux colonnes : une fonction
hauteur est un ensemble de niveaux particulier.

## 2. Ce que la décision s'appuie sur, et ce qu'elle ne sait pas

Mesuré en deux dimensions, au même niveau que les deux autres candidats (S318) : masse **exacte**,
aucune énergie créée, période du ballottement à **0,15 %** à la maille fine, coût le plus bas
(10 s par seconde simulée sur la rupture de barrage fine, contre 27 et 405), gouttes **portées**.

**Pas encore mesuré** : une cavité et son pincement (B10), la trois-dimensions, la carte graphique, le
raccord aux colonnes, et une référence **expérimentale** — Martin et Moyce restent inaccessibles.
L'ensemble de niveaux et SPH ont été écartés **comme représentation principale** sur des mesures de
volume, d'énergie et de coût ; le défaut non isolé de SPH (A310) n'entre pas dans ce choix.

## 3. Ce qui suit

- **Lot 5, prochaine session du fil (S320)** : B10 — un objet **cinématique** entre dans l'eau ;
  couronne, cavité, pincement, jet ; le volume de la cavité rendu sous le compteur du lot 1.
- **Puis** : le raccord particules ↔ colonnes, et le critère qui décide où vivent les particules.
- Le banc `examples/lot5_comparaison.rs` reste l'instrument de référence du candidat 2D.

## 4. Ce qui devient faux si cette décision est mal lue

**« APIC est retenu » ne veut pas dire « δ devient particulaire partout ».** Les colonnes restent la
représentation par défaut (D3) ; APIC est la **seconde** représentation, celle des cas que les colonnes
ne portent pas.

**Et « retenu » ne veut pas dire « reçu ».** Le candidat n'a vu ni cavité, ni trois dimensions, ni
carte ; sa réception se fera sur B10 et C20, pas sur ce choix.

---

**Note du 2026-09-22 (S320).** B10 est mesuré ([B10-APIC-S320](../validation/B10-APIC-S320.md)).
Une cavité se forme et se pince à `Fr` = 2 et 4. Le temps de pincement est indépendant de l'échelle et
converge à 5 % près ; la couronne et le jet sont des grandeurs de la maille (A312). La bulle enfermée
n'est pas de l'air (A311). La « masse exacte » de §2 est juste, mais **le volume géométrique ne l'est
pas** sans séparation des particules, et n'a pas encore de mesure propre (A313, L370).
