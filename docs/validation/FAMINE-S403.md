# La famine a une issue, et le rang 4 choisit selon le contenu — S403

2026-09-27. **C8d** de la campagne du solveur volumique 3D ([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)
D5) : le rang 4 d'[ADR-012](../adr/ADR-012-ordonnanceur-budget-degradation.md) §4 dans l'ordonnanceur du cœur, et l'issue de la
famine déclarée (le rang 5, à l'hôte). Suite de [NIVEAUX-S402](NIVEAUX-S402.md), dont le transfert d'état
([ADR-210](../adr/ADR-210-changer-de-niveau-par-transfert-d-etat.md)) est le moyen, et d'[ARBITRAGE-3D-S344](ARBITRAGE-3D-S344.md)
§7, le rang 1. Session cloud, sans carte graphique. Liste **9.8**, **9.9**.

## Reproduire

- Commit `342bdac1` ou plus récent.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib scheduler` — 34 essais, dont six `_s403`.
- `cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example ordonnanceur_s278` — empreinte
  **`6aebff024c734fc9`**, celle de S278 et de S351.
- `cargo run --manifest-path code/Cargo.toml -p water-core --release --offline --example delta3d_famine` — lignes `FAMINE_S403`,
  ≈ 4 à 5 min (deux calculs en parallèle) ; `TEMOIN=1` pour le témoin : §3.

## En une phrase

Quand le rang 1 ne suffit plus, l'ordonnanceur descend d'un niveau le non-focal qui **perd le moins** — sur trois domaines δ réels,
la bosse plutôt que la source : **1,4 mm** d'écart pendant la famine, contre **14,0 mm** pour un témoin qui ignore le contenu —,
sans saut visible (≤ 2,5 mm), sans jamais dépasser le budget ni toucher le focal ; et ce que rien ne sauve est **déclaré
affamé** — l'issue, le rang 5, est à l'hôte, qui le détruit et le fait renaître au repos. Sans déclaration, l'ordonnanceur de
S351 est inchangé au bit.

## 1. Le rang 4 dans l'ordonnanceur

**La déclaration** (`Scheduler::declare_coarsen`). À chaque pas, l'hôte déclare, pour un candidat soumis et perturbatif, ce que
lui coûterait un niveau plus bas et **ce que son image y perdrait** — l'écart d'un aller-retour de son contenu présent par le
transfert d'ADR-210 (D2). `Bid` ne change pas : l'hôte `viewer/`, qui le construit, reste compilable sans retouche. `Grant` dit
le niveau accordé ; `starved()` rend les vivants sans budget.

**La décision** (`allocate`, `update_levels`), avant le rang 1 et sur ses règles :

- un domaine descendu qui ne déclare plus **remonte** — l'hôte l'a décidé ; le focal n'est **jamais** descendu ;
- **tant que le rang 1, à son minimum, laisse un vivant sans budget**, le non-focal déclaré qui perd le moins par milliseconde
  rendue descend — la comparaison en croix, l'égalité départagée par l'identité ; immédiatement (ADR-012 §5) ;
- sinon, **une remontée au plus par seconde** : le descendu qui perd le plus, une seconde au moins après sa descente, s'il tient
  sans affamer personne — l'ordre de dégradation se défait à rebours, le rang 4 avant le rang 1 ;
- les coûts à un niveau suivent la loi du rang 1 sur le coût de ce niveau — à son propre niveau, `Bid::cost_for` au bit ;
- ce qui reste sans budget est **déclaré** : `starved()`.

Les rangs 2 (pas d'embruns dans δ) et 3 (la fréquence) n'existent pas : le rang 4 suit le rang 1.

## 2. Critères écrits avant

| critère | résultat |
|---|---|
| **1** — sans déclaration, S351 au bit | **tenu** : ses 28 essais (un littéral de `Grant` complété) ; empreinte S278 `6aebff024c734fc9` |
| **2** — le rang 4 seulement après le rang 1 à son minimum | **tenu** : le cas de S351 ne descend personne |
| **3** — la moindre perte par ms d'abord ; jamais le focal ; budget tenu | **tenu** : la bosse descend, pertes échangées la source ; le focal déclarant reste à son niveau |
| **4** — l'issue déclarée | **tenu** : à 0,6 ms, un affamé rendu par `starved()` ; sans déclaration, les deux affamés de S278, dits de même |
| **5** — descente immédiate, remontée engagée, une par seconde, la plus forte perte d'abord | **tenu** : remontées à 1 s puis 2 s ; nouvelle famine, descente au pas même |
| **6** — le banc | **tenu** : §3 |
| **7** — suite, zéro avertissement | **tenu** : **718 réussis**, 19 ignorés |

## 3. Le banc — trois domaines réels sous un budget qui se resserre

`delta3d_famine` : trois sites de **16 × 12 m** à 25 cm (64 × 48 × 12 mailles), chacun avec son jumeau à 50 cm ; pas mobile de
20 ms, multigrille. **F** (identité 0), focal, une source mobile — le dipôle de S401 à 2 m/s ; **B** (1), la même source ;
**A** (2), une bosse de 5 cm et σ = 1 m. **La référence** : les trois à 25 cm, sans famine. Coûts par la loi de la production
(S350) ramenée à la maille : **0,440 ms** à 25 cm, **0,134 ms** à 50 cm. Budget : 2 ms, puis **1,05 ms** de 2 à 4 s (un des deux
non-focaux doit descendre), **0,6 ms** de 4 à 5,5 s (l'un descend, l'autre est affamé), 2 ms ensuite. L'hôte : un changement de
niveau, le transfert d'ADR-210 ; un affamé, le rang 5 — son image s'efface en 0,5 s (ADR-005 §5), il renaît au repos quand il
est de nouveau servi. **Le témoin** déclare des pertes nulles : l'identité départage, et c'est B, la source, qui descend.

Écart maximal de l'image à la référence, par phase :

| domaine | large | **famine, 1,05 ms** | sévère, 0,6 ms | retour |
|---|---:|---:|---:|---:|
| **ordonnanceur** — A, la bosse | 0 | **1,36 mm** (descendue) | 17,8 mm (affamée : rang 5) | 20,5 mm (renaît au repos) |
| ordonnanceur — B, la source | 0 | 0 | 13,2 mm (descendue) | 15,0 mm (remontée) |
| **témoin** — B, la source | 0 | **14,0 mm** (descendue) | 16,4 mm | 17,0 mm |
| témoin — A, la bosse | 0 | 0 | 17,8 mm (affamée) | 20,5 mm |
| F, le focal, dans les deux | 0 | 0 | 0 | 0 |

Sauts d'un pas : au plus **2,48 mm** (B, en famine sévère). Budget accordé : au plus **0,965** du budget. Deux calculs en
parallèle, 4 à 5 minutes chacun.

**Ce que les chiffres disent.**

- **Le contenu fait le choix, et le choix compte** : dans la phase où le rang 4 décide, un écart dix fois moindre — la prédiction,
  tirée de S402, était ≈ 2 contre ≈ 15 mm.
- **L'issue de la famine est dite, pas subie** : A, affamé en famine sévère, est détruit par l'hôte ; avant S403, il serait resté
  vivant sans budget, et personne ne l'aurait su.
- **Mais la victime du rang 5 suit l'ordre du sac à dos** — `P/C`, puis l'identité —, pas le contenu : ici A, la bosse, que le
  rang 4 avait choisie parce qu'elle perdait peu ; et la descente de B, en famine sévère, n'a servi à rien d'autre qu'à changer qui
  est nourri — A et B descendus ensemble dépassaient 0,6 ms. **Un domaine détruit ne retrouve pas son contenu** : 20,5 mm après le
  retour du budget, le prix du rang 5.

## 4. Ce que ce document ne dit pas

- **La victime du rang 5** n'est pas choisie par le contenu, et une descente qui ne met pas fin à la famine n'est pas évitée
  (§3) : à la file.
- **Les rangs 2 et 3** n'existent pas ; le régulateur PI d'ADR-012 §5 non plus ; le budget suit un profil imposé, non le coût
  consommé filtré.
- **Les coûts sont déclarés** par une loi, pas mesurés : la référence n'est pas la carte.
- **La perte déclarée** est l'écart d'un aller-retour du contenu présent : elle ne voit pas ce que la dynamique perdra ensuite
  (la source sous-résolue coûte 15 mm à 50 cm, S402) — mesurée ici, elle suffit à ordonner.
- Ni l'hôte `viewer/`, ni la carte, ni aucun verdict visuel.
