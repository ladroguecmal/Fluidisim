# C23 sur le système : le pas de δ 3D borné par la paroi — S505 (liste 6.4)

*S505, 2026-10-06, en autonomie.* C23 (CAS-CANONIQUES ; ADR-035 : le pas borné par la vitesse **gouvernante**) n'avait été éprouvé que sur
le véhicule 1D (S28). Le δ 3D ne bornait pas son pas : l'hôte le choisissait, le cœur ne gardait que `dt²·g/dx ≤ 1`, et une coque pouvait
franchir plus d'une maille par pas. La décision : [ADR-229](../adr/ADR-229-la-paroi-dans-la-vitesse-gouvernante.md).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core c23_s505 -- --nocapture` — C23 rejoué.
- `cargo run -p water-core --release --offline --example c23_coque -- <u_p> <k1,k2,…>` — la mesure ; `-- gouvernant <u1,u2,…>` — sous la
  borne gouvernante ; `-- localise <u_p>` — où est l'écart (lancés par `outils/calcul.py`, ≈ 1 à 3 min chacun).

## 1. La mesure, et ce qu'elle a coûté d'apprendre

La coque de la porte D en translation dans un δ de 16 × 8 × 2 m (64 × 32 × 8 mailles de 25 cm), des pas qui lui font franchir `k`
mailles par pas, contre le calcul au plus petit `k`.

1. **Départ impulsif, 5 m/s** : l'écart vaut 5 à 7 % jusqu'à `k` = 0,5, 100 % à `k` = 1. Mais la référence n'était pas convergée — son
   élévation doublait quand le pas diminuait (0,77 → 1,70 m) : un départ impulsif n'a pas de limite en `dt`.
2. **Départ en rampe (0,5 s), 5 m/s** : toujours ≈ 45 % à tout `k`. Localisé : l'écart se tient à l'étrave et ne décroît pas — **5 m/s
   dans 2 m d'eau dépasse `√(gh)` = 4,43 m/s** (Froude de profondeur 1,13) ; la rampe traverse le régime critique, où la réponse linéaire est
   singulière. Un essai hors des limites du modèle (ADR-228 D1).
3. **Sous-critique, 2 m/s** (Froude 0,45) :

| `k` (mailles par pas) | 0,125 | 0,25 | 0,5 | **1** |
|---|---|---|---|---|
| écart au calcul à `k` = 0,0625, rapporté à l'élévation (0,48 m) | 33 % | 44 % | 57 % | **125 %** |

L'écart **double** au passage d'une maille par pas, au-dessus de sa tendance. Et la tendance elle-même est lente : un facteur 1,3 par pas
divisé par deux, l'écart maximal au bord de la coque. À 0,5 m/s, hors des colonnes de la coque, 7, 22, 51 % (ordre ≈ 1,5). **Le δ 3D à coque
mobile converge lentement en `dt` près de la coque** — c'est la référence qui dépend du pas (la carte la suit au même pas, S503–S504) :
**A328**.

## 2. La borne et le compteur

`Volume3::governing_speed(wall)` — la vitesse du fluide relative à la paroi sur une face que le solide couvre en partie, la vitesse absolue
ailleurs, **et la vitesse de la paroi elle-même** (ADR-229 D1) ; `celerity` = `√(g·z₀)` ; `courant_bound(ν, wall)` = `ν·dx/(u + c)` en amont,
`courant(dt, wall)` après coup — une seule fonction pour les deux (ADR-035 §3).

**C23 rejoué** (eau au repos, 2 m ; la coque mise en mouvement de 0,5 à 20 m/s) :

| `u_p` (m/s) | 0,5 | 2 | 5 | 5,41 | 5,42 | 10 | 20 |
|---|---|---|---|---|---|---|---|
| Courant sous la borne gouvernante | 0,4500 | 0,4500 | 0,4500 | 0,4500 | 0,4500 | 0,4500 | 0,4500 |
| mailles franchies par pas | 0,046 | 0,140 | 0,239 | 0,247 | 0,248 | 0,312 | 0,368 |
| Courant sous la borne absolue (`c` seule) | 0,501 | 0,653 | 0,958 | **0,9996** | **1,0006** | 1,466 | 2,482 |

La borne absolue franchit 1 exactement au seuil analytique `u_p = c·(1/ν − 1)` = 5,414 m/s.

**Sous la borne gouvernante**, l'écart au calcul au quart du pas, sous-critique : 33 % (0,5 m/s), 75 % (1), 33 % (2), 45 % (3) — du même
ordre à toutes les vitesses, l'ordre d'A328.

## 3. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) la mesure publiée : rompt-elle au-delà d'une maille par pas ? | oui : l'écart double à `k` = 1 (125 % contre 57 %) — sur une tendance déjà lente (A328) | publié |
| (2) borne et compteur d'une même fonction ; `ν` au millième sous la gouvernante ; la borne absolue dépasse 1 au seuil analytique | 0,4500 partout ; 0,9996 / 1,0006 de part et d'autre de 5,414 m/s | tenu |
| (3) sous la borne gouvernante, l'écart du même ordre à toutes les vitesses | 33 à 75 % | tenu dans sa lettre ; l'ordre lui-même est A328 |

**C23 tient sur le système.** **6.4 reste partielle** : A328 (la convergence en `dt` près d'une coque mobile) et le coût du recoupage (8 ms par
pas, CPU).
