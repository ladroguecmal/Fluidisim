# C10 — les scènes ([campagne](../registres/CAMPAGNE-SOLVEUR-3D-S384.md) §6) — conception et C10-1 (S454)

2026-10-02, au poste. **C10** : *le joueur qui saute à 5 cm, la gerbe d'étrave, la lame du déversoir* (ADR-202 D5) ; points 4.12,
4.13, 5.10 de la [liste](../LISTE-PROJET-FINI.md) ; reçu au **critère d'arrêt du §3.4** — dans la même scène vivante, un joueur saute
dans l'eau (5 cm) pendant qu'une coque passe (≤ 25 cm), δ ≤ 2 ms au 99ᵉ centile, la production à 3 mm de la référence, la masse
exacte, **et l'utilisateur juge le rendu convaincant** — sur la surface continue (ADR-211 D2, [SURFACE-CONTINUE-S450](SURFACE-CONTINUE-S450.md)).

## 1. Ce qui existe

| pièce | état | où |
|---|---|---|
| la bande APIC sur la carte, ses colonnes et sa bascule | reçue (C7) ; **seule** depuis S453 (son pas) | [APIC-CARTE-S416](APIC-CARTE-S416.md) |
| la surface continue, en direct | R37 reçu ; 1,3 ms par image | [SURFACE-CONTINUE-S450](SURFACE-CONTINUE-S450.md) |
| B10, la sphère qui entre dans l'eau (le joueur, en substitut) | **en quart seulement** (0,8 m, deux plans de symétrie) | `apic3d_carte.rs` (`B10`) |
| la mer δ relative à B, la coque | reçues (portes B à D, C7d) | [PORTE-D-S333](PORTE-D-S333.md), [SCENE-DELTA3D-S302](SCENE-DELTA3D-S302.md) |
| le raccord bande ↔ mer (`BandInSea`) | CPU ; masse à 0,01 % ; **instable à son bord** à côté de colonnes de particules (c3 plafonné) | [§23.4–23.6](APIC-CARTE-S416.md) |

## 2. Le découpage

| lot | quoi | reçu si |
|---|---|---|
| **C10-1** | **le saut du joueur** sur un domaine entier, à l'échelle d'une scène (4 m × 4 m, 5 cm), carte seule, en direct | le quart déplié reproduit le quart (une maille) ; à 4 m, masse exacte, `φ` fini jusqu'à t = 4 ; la fenêtre |
| **C10-2** | **le saut dans la mer** : la bande de la carte dans δ + B — le raccord porté sur la carte, ses colonnes de marge gardées en colonnes (la réponse au déclencheur de §23.6 : jamais de colonnes de particules au raccord) | masse au raccord sous 0,1 % ; la mer stable au bord sous une houle calme ; le cratère à une maille de C10-1 |
| **C10-3** | **la coque qui passe** et **la gerbe d'étrave** : la bande ouverte à l'étrave (4.13) | la gerbe se détache ; δ ≤ 2 ms au 99ᵉ centile |
| **C10-4** | **la lame du déversoir** (ADR-202 D5, 5.10) | la lame simulée réagit à un obstacle |
| **C10-5** | **la scène vivante** du §3.4 — saut et coque ensemble | le critère d'arrêt, le jugement de l'utilisateur |

**L'ordre** : C10-1 d'abord — il ne dépend que de ce qui est reçu et donne à l'utilisateur une première scène à juger ; C10-2 répond au
déclencheur laissé ouvert par c3 ; C10-3 et C10-4 sont indépendants l'un de l'autre.
