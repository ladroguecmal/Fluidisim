# ADR-128 — Le budget de pente borne ce que les perturbations ajoutent, pas la mer

- **Statut : actée**, S205, 2026-09-13, autonomie technique déléguée (S71).
- **Traite A245** (gravité 1) : la composition B+W refusait toute mer au-delà de Hs ≈ 1,1 m.
- **Remplace** pour le terme de B : ADR-062 §« borne d'ensemble », ADR-080 (garantie de
  `slope_floor`, réciproque déclarée fausse), ADR-095 §2 (ligne « fond B »), ADR-098 (sens de
  `Slope`) et ADR-126 règle 3 (budget d'impact = π/7 − plancher de B). Aucun n'est réécrit ;
  chacun reçoit une note datée. **Rend I-18 tenu** pour le terme de B.
- Jalon **J1** de la [feuille de route](../FEUILLE-DE-ROUTE.md). Réception :
  [COMPOSITION-MER-S205](../validation/COMPOSITION-MER-S205.md).

## Constat

Quatre sites — `composition::compose`, `prepared_water::mixed::sample_world_batch`,
`mixed_differential`, `bound_pressure::Prepared::sample_world_batch` — comparaient à `max_slope`
la somme `steepness_B·π + Σ majorants des perturbations`. Le premier terme est la borne L1 de B,
`Σ aᵢkᵢ`, que les ADR-094/095 tenaient à tort pour exacte : 0,6082 sur la mer JONSWAP de S201
(Hs 1,5 m), contre π/7 = 0,4488. **Chaque point de chaque lot était refusé, journal vide
compris** : aucune perturbation W ne pouvait se poser sur une mer modérée. Même le meilleur
majorant indépendant du point (directionnel, 0,5733) refusait ; la pente réelle échantillonnée
valait 0,4215 (S203).

Aucune spécification ne consomme de garantie « la surface ne dépasse nulle part π/7 ».
`steepness` sert à l'écume et au déferlement (SPEC-004 §2, SPEC-001 §3) : une mer dont la
raideur locale atteint la limite de Stokes **déferle**, et c'est une information à publier.

## Décision

1. **Le budget de refus ne somme que les perturbations** : `Σ slope_max` des impacts, plus
   `slope_envelope` de la pression. La raideur de B n'y entre plus. Une perturbation qui, seule
   ou additionnée à d'autres, dépasse `max_slope` est refusée comme avant.
2. **La raideur publiée est inchangée** : `steepness = (steepness_B·π + Σ majorants)/π`, même
   ordre de somme. Pour tout lot que l'ancienne règle admettait, **aucun bit publié ne change**.
3. **`Slope` et `SlopeEnvelope` se jugent sur la pente réelle des perturbations au point**,
   plus sur celle de B+W : c'est le champ admis au budget qu'ils désignent (ADR-082, ADR-098).
4. **`slope_floor` devient exact dans les deux sens** : `max_slope < slope_floor` ⟹ tout lot
   non vide refusé ; `max_slope ≥ slope_floor` ⟹ aucun refus de pente, quels que soient les
   points et la mer. Le refus de pente redevient entièrement annonçable avant la requête.
5. **Règle d'hôte** (remplace ADR-126 règle 3) : le budget de pente d'un impact est π/7 moins
   ce que les **autres perturbations** consomment ; B n'en retire rien.

`Background::differential_slope_envelope` (ADR-117) est retirée : elle ne servait qu'au budget
différentiel.

## Ce que la décision garantit, et ce qu'elle cesse de garantir

**Garanti** : aucune perturbation composée ne dépasse nulle part la cambrure limite de Stokes ;
le refus en est prévisible par `slope_floor` ; la raideur totale reste publiée. I-18 est tenu :
ce qui est comparé à `max_slope` est une somme de majorants exacts de pentes réelles.

**Plus garanti** : que la surface **B+W** reste sous π/7 partout. Cette garantie n'était tenue
qu'en refusant toute mer réaliste — ce qu'ADR-127 D7 interdit de faire en silence — et elle
reposait sur un terme L1 non converti. Une mer qui atteint la limite de Stokes est un fait
d'environnement : elle se lit dans `steepness`, et la représentation du déferlement appartient à
δ et à l'écume (ADR-001 §3.3, SPEC-002 §1), pas à un refus de requête.

**Non décidé** : la validité physique de la superposition linéaire sur une mer raide. Le
critère de Stokes n'en est pas un (A207) ; le domaine mesuré de la superposition est ADR-123,
bien plus étroit, et il reste distinct de ce budget.

## Réception

Workspace **344 réussis / cinq ignorés** (debug) ; essais touchés verts en release ; harnais
**C18 `0x85c8bc610f551d11`, C02 `0x0a3a3bcc945db263`** inchangés ; image S203 +3 s reproduite au
bit (`0x3dba0d3acf15447a`, témoin `0x14271a7145740ba1`). Sept attentes réécrites, chacune
motivée : toutes tenaient à la pente de B dans le budget ou le verdict, aucune à une valeur
publiée. Essai neuf `reference_sea_s201_composes_with_an_impact_s205`. Impact rendu sur la mer
S201 (Hs 1,5 m) à +3 s et +6 s contre témoin : **zéro refus, zéro rayon non résolu, zéro pixel
différent hors emprise**.

## Réversibilité

Remettre B dans le budget demande de nommer le consommateur qui a besoin de la garantie
« B+W sous π/7 », et une borne de B qui n'interdise pas les mers du jeu — ce qu'aucune borne
indépendante du point ne fait sur la recette S201.
