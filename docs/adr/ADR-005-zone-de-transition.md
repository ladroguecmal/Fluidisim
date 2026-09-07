

---

## Note corrective — B-S27 : la largeur, et enfin quel `c`

> *Reportée de la lignée B le 2026-09-07 (S39), renvois renumérotés. **Elle rétracte la note de
> B-S26 qui la précède immédiatement**, et non ce §2.*

La note de B-S26 ci-dessus retirait la condition `L_s ≥ λ_δ/2`, la mesure ne trouvant pas sa trace.
**Cette note-là était fausse, et pour la raison la plus instructive** : elle avait été établie en eau
peu profonde, où l'amortissement ponctuel est sans réflexion par accident algébrique. En milieu
dispersif, mesuré en B-S27 ([ADR-046](ADR-046-l-eponge-en-eau-dispersive-retracte-ADR-042.md)) :

| `L_s/λ` | 0,125 | 0,25 | **0,5** | 1,0 | 2,0 |
|---|---|---|---|---|---|
| `R` | 0,669 | 0,515 | **0,227** | 0,0098 | 0,00144 |

**La condition `L_s ≥ λ_δ/2` du §2 est donc du bon genre — et de la mauvaise constante.** Elle donne
23 % pour un critère à 1 %. La règle devient **`L_s ≥ λ_δ`**, et **`L_s ≥ 2·λ_δ` dès que `δ` porte un
spectre** — le cas normal.

**Et l'ambiguïté sur `c` est tranchée.** Le §2 écrit « `c` la célérité correspondante » sans dire
laquelle ; en eau profonde, phase et groupe diffèrent d'un **facteur deux**. Au même point de
fonctionnement, `σ_max = 10·c_groupe/L_s` donne `R = 0,00144` et `σ_max = 10·c_phase/L_s` donne
`0,00269`. **C'est la vitesse de groupe**, celle qui transporte l'énergie.

**Ce que le §2 disait de juste, et que B-S26 avait pris pour faux** : la largeur est bien commandée par
la longueur d'onde. Le §2.1 — *`λ_δ` est borné par construction, donc l'éponge est dimensionnée par
`λ_cut` et non par la physique de la scène* — **reste l'argument central de cet ADR**, et il est
renforcé plutôt qu'affaibli : l'éponge coûte deux à quatre fois plus cher que ce paragraphe le
supposait, ce qui rend d'autant plus décisif le fait qu'elle soit bornée.
