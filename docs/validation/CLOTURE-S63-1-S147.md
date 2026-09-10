# S147 — Clôture de S63-1 : la dispersion est construite dans W

État constaté le 2026-09-10, depuis master `66cd765`. **S63-1 et S145-2 closes**.
Il s'agit d'un constat d'exécution des décisions existantes, pas d'un choix technologique nouveau.

## La demande et sa preuve

S63-1 demandait de planifier une couche dispersive du projet, ou d'acter son abandon.
ADR-053 puis [ADR-054](../adr/ADR-054-construire-w-sans-faux-prealable.md) ont choisi W
et demandé une couche construite et contrôlée. Le choix correspond à ADR-001 §2 :
W porte les perturbations propagatives régionales, δ reste l'écart local volumétrique.

| Exigence | Réalisation et preuve existantes |
|---|---|
| Propagation dispersive | [ADR-060](../adr/ADR-060-candidat-radial-borne.md), `RadialImpact`, quadrature radiale avec ω²=gk, domaine profond uniforme déclaré |
| Transport, distinct de la seule phase | [TRANSPORT-ETENDU-S127](TRANSPORT-ETENDU-S127.md), montage N256/R80/48 s : 99,985 % de l'énergie de référence entre 32 et 80 m à 48 s |
| Réception du candidat et énergie totale | [BILAN-CANDIDAT-ETENDU-S129](BILAN-CANDIDAT-ETENDU-S129.md), cinétique et potentiel depuis les nœuds réels, écart total maximal 9,045e-7 E0 sur la fixture |
| Intégration et rejeu | [CYCLE-TRANSPORTE-S128](CYCLE-TRANSPORTE-S128.md), LiveWater B+W, 1 280 points-temps identiques en bits à la voie directe, sauvegarde/restauration |

Ce sont des résultats archivés, **non remesurés en S147**. La réception est bornée aux
montages documentés ; elle ne certifie pas tous les paramètres admis, ni toutes les plateformes.

## Ce que cette clôture ne clôt pas

La dispersion exacte d'un candidat W **ne mesure pas l'erreur du futur solveur δ**.
Le lien « couche dispersive → C02 → λ_cut final » était déjà corrigé par ADR-054 §2.
Les deux véhicules Saint-Venant restent non dispersifs ; C02 sur ces véhicules ne fournit
pas la coupure dispersive finale. B2 conserve la comparaison des candidats, la bathymétrie,
les scénarios de sillage et de déferlement, les budgets cibles et le déterminisme croisé.

La calibration de la source d'impact reste portée par **B10**, conformément à ADR-093.
La clôture de S63-1 n'est donc ni une réception complète de W ni un verdict de B2.
Le prochain travail système reste W4/B2 selon ADR-054 ; la décision de spectre A212 est
le lot B annoncé par S146. Aucun invariant I-01 à I-18 n'est modifié.
