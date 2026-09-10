# S142 — Le second champ, sa borne L1, et ce qu'il faut en faire

2026-09-10. Traite **A209**, ouverte par la migration de S141.
Sonde : `code/water-core/examples/pente_modale.rs`.

## 1. Le défaut à traiter est le mien

S141 a migré `RadialImpact` : `max_slope` y borne désormais la pente **réelle**. `ImpactField` —
le champ modal à 40 modes d'ADR-058 — lui compare toujours sa **borne L1**. Depuis, un même
`Medium` produit deux frontières de sens différent selon le champ qui le lit, et rien dans le type
ne le dit.

La mesure devait décider entre trois issues : mesurer le rapport et migrer, séparer les deux sens
dans le type, ou retirer un champ que plus personne ne construit.

## 2. Le rapport, mesuré comme celui du candidat radial

La borne est retrouvée **par dichotomie sur `max_slope`** — telle que l'extérieur la voit, c'est-à-
dire exactement la grandeur qu'A209 met en cause — sans rien changer à la bibliothèque. La pente
réelle est échantillonnée sur une période spatiale complète `side × side`.

| | valeur |
|---|---|
| borne L1 (λ = 4 m, E = 0,01 J) | 1,009692e-3 |
| pente réelle maximale à `t = birth` | 5,933812e-4 |
| **rapport** | **1,701591** |
| position du maximum | `[0 ; 0,0733]·side` |

**C'est une constante du modèle**, comme `ρ = 1,7950713` l'est pour le candidat radial et
contrairement au facteur de la pression :

| balayage | plage | rapport |
|---|---|---|
| longueur d'onde | 0,5 → 32 m | **1,701591** partout, position du maximum identique |
| énergie | 1e-4 → 10 J | 1,701591 à 1,701592 |
| grille de recherche | 25 → 800 points par côté | 1,701591 dès **25** |

C'est structurel : `side = 4λ` et les modes sont indexés par des entiers, donc le motif est
identique à toute longueur d'onde — le champ est homothétique comme l'autre. Et le maximum est
**toujours** atteint : le champ est périodique et `sample` n'impose aucune emprise, ce qui
distingue ce cas de celui de la pression (A206), où une emprise étroite pouvait le manquer.

Le maximum est à l'instant de naissance, là où les phases temporelles valent toutes 1 ; le rapport
monte ensuite jusqu'à 7,7 à 2 s, à mesure que le champ se déphase.

## 3. L'état réel du champ, constaté avant d'en décider

Aucun appelant de production ne le construit — seule la sonde `probe_degenerate`. Mais **« personne
ne le construit » ne veut pas dire « il est mort »**, et le dépôt a déjà payé cette confusion : le
troisième fork est venu d'une branche conservée exprès que rien ne distinguait d'un point de départ
(S39, état `archivé`).

Ce que les décisions en disent :

- **ADR-059 le conserve délibérément** : « le champ ImpactField à 40 modes reste un support de
  comparaison physique ». Il n'a pas été abandonné, il a été **écarté du chemin actif** — ce qui
  n'est pas la même chose. Le retirer contredirait une décision actée.
- **ADR-082 a choisi de ne pas le toucher** : « ses bornes ont la même maladie, mais il n'est plus
  le chemin actif, et le renommer sans consommateur ajouterait du travail sans lecteur ».
- **ADR-081 pose le principe qui trancherait** : la séparation `NotRepresentable` / `Steepness` lui
  a été appliquée « par cohérence de vocabulaire : deux constructeurs du même crate ne doivent pas
  nommer différemment la même distinction ».

**Le troisième point décide.** L'argument d'ADR-082 — pas de consommateur, donc pas de travail —
valait quand les deux constructeurs disaient la même chose. Depuis S141 ils ne la disent plus, et
c'est précisément ce qu'ADR-081 interdit. Le lecteur qui manquait à ADR-082 existe désormais : il
lit `Medium::max_slope`.
