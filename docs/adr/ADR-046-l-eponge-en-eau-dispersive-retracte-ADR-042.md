# ADR-046 — L'éponge en eau dispersive, et la rétractation d'ADR-042

> **Importée de la lignée B le 2026-09-07 (S39).** Ce document s'appelait `ADR-035` dans une
> histoire parallèle du dépôt, où ce numéro désigne ici le nombre de Courant. Ses renvois ont été
> renumérotés selon la carte de [`FORK-S22-S26`](../registres/FORK-S22-S26.md) §3 ; **son texte n'a
> pas été modifié autrement**.

- **Statut** : proposée
- **Session** : B-S27
- **Rétracte** : `ADR-042 D2` *(la largeur est fixée par la maille)* et `ADR-042 D4` *(la borne haute
  de `λ_cut` est rouverte)*
- **Confirme** : `ADR-042 D1` *(σ_max = 10·c/L_s)* et `ADR-042 D3` *(σ_max·dt < 1)*
- **Corrige** : `ADR-005 §2` — la constante de largeur, et l'ambiguïté sur `c`
- **Referme et resserre** : `DOSSIER-B2 §3.1`
- **Produit** : `code/water-core/src/dispersif.rs` — milieu linéaire à dispersion exacte ;
  `eponge.rs` — le profil d'ADR-005 écrit une fois pour les deux porteurs

---

## 1. La mesure que B-S26 avait demandée est revenue négative

ADR-042 concluait, en toutes lettres, que sa décision la plus lourde restait suspendue :

> Le solveur est **non dispersif**. Une éponge d'eau profonde doit absorber une **bande** de
> longueurs d'onde qui voyagent à des célérités différentes. **La règle `λ/2` protège peut-être
> exactement de cela.** C'est la première chose à mesurer avant de s'appuyer sur D2.

Elle a été mesurée. **La règle protégeait bien de cela — et même elle n'y suffit pas.**

## 2. L'instrument, et sa vérification

Milieu linéaire à **dispersion exacte** : `η̈_k = −ω(k)²·η_k` avec `ω² = g·k·tanh(kh)`, propagé
mode par mode par la solution exacte du pas de temps. Ni dissipation, ni erreur de phase — donc
**aucun artefact du porteur ne se mêle à ce qu'on mesure**, ce qui était le principal risque après
les 10 % de dissipation de trajet corrigés en B-S26 (L136).

**Vérifié avant usage**, comme C02 l'a fait pour `B` : période **mesurée dans le champ**, sur trois
modes exacts du domaine.

| `λ` | `T` mesurée | `T = 2π/ω` | écart | `T` non dispersif |
|---|---|---|---|---|
| 32 m | 4,52894 s | 4,52897 s | **0,001 %** | 2,28455 s |
| 16 m | 3,20125 s | 3,20122 s | **0,001 %** | 1,14227 s |
| 8 m | 2,26361 s | 2,26360 s | **0,000 %** | 0,57114 s |

La dernière colonne justifie la session : la prédiction non dispersive se trompe d'un facteur **2 à
4**. Le milieu disperse pour de bon.

**Et le profil d'éponge est désormais écrit une fois** (`eponge.rs`), partagé par le solveur de
Saint-Venant et par ce milieu. Sans cela, B-S27 aurait mesuré une réimplémentation, et les deux
sessions ne seraient pas comparables.

## 3. Le montage, et ce que le cas de garde a coûté

`R = √(E_réfléchie / E_incidente)` avec `E = ∫η²dt` à une jauge fixe. **L'énergie et non la crête** :
en milieu dispersif un paquet **s'étale sans rien perdre**, et une crête décroît toute seule — c'est
L136 sous une autre forme.

**Le cas de garde a refusé deux montages avant d'en accepter un** : un essai sans éponge doit
laisser la fenêtre réfléchie vide.

| montage | fuite | cause |
|---|---|---|
| 512 m, jauge au milieu | 5,1 % | enroulement périodique dès 150 s, réflexion attendue à 166 |
| 1024 m, jauge à 950 m | 3,4 % | séparation de 46 s pour un train qui occupe la jauge 70 s |
| **2048 m, jauge à 850 m** | **4·10⁻¹⁷** | séparation de 117 s, enroulement à 787 s |

> **Aucune des deux erreurs n'était visible dans les chiffres de `R`** — il valait 0,18 à 0,32 dans
> les trois montages, y compris les faux. Sans la garde, la session aurait publié un résultat juste
> **pour de mauvaises raisons**, et aurait été incapable de le savoir.

Et c'est **la trace** — `max|η|` par tranche de temps à la jauge — qui a désigné la cause, non le
calcul d'arrivée : en milieu dispersif la queue avant d'un paquet précède largement son pic, et
l'enroulement arrivait 80 s plus tôt que la vitesse de groupe centrale ne le prédisait.

## 4. Le résultat : `R` dépend fortement de `L_s/λ`

`σ_max = 10·c_g/L_s`, **entrée de bande fixe** — seule la largeur varie, donc l'instant d'arrivée du
train réfléchi ne bouge pas.

| `L_s/λ` | `R` bande étroite | `R` bande large | *(rappel B-S26, eau peu profonde)* |
|---|---|---|---|
| 0,125 | **0,669** | 0,655 | 0,00157 |
| 0,250 | **0,515** | 0,501 | 0,00198 |
| 0,500 | **0,227** | 0,229 | 0,00146 |
| 1,000 | **0,0098** | 0,027 | — |
| 2,000 | 0,00144 | 0,0039 | — |
| 4,000 | 0,00033 | 0,0027 | — |
| 8,000 | 0,00014 | **0,0025** | — |

**Trois lectures.**

1. **La largeur commande tout**, et l'écart avec B-S26 atteint un facteur **400** au même rapport
   `L_s/λ`. ADR-042 D2 est faux hors de l'eau peu profonde.
2. **`L_s = λ/2` ne suffit pas** — 23 % pour un critère à 1 %. La règle d'ADR-005 §2 est **du bon
   genre et de la mauvaise constante** : il faut `L_s ≥ λ_δ`.
3. **Un spectre a son propre plancher.** À grande largeur, la bande étroite converge vers `1,4·10⁻⁴`
   quand la bande large **plafonne à `2,5·10⁻³`** — dix-huit fois plus, et l'écart ne se réduit plus.
   Une éponge accordée sur `c_g(λ₀)` est **désaccordée** pour le reste du spectre, et élargir ne
   corrige pas un désaccord. Pour tenir 1 % sur un spectre, `L_s ≥ 2·λ_δ`.

### 4.1 Pourquoi B-S26 avait ce résultat, et ce n'était pas une erreur

Le mécanisme se voit sans mesure, une fois qu'on sait quoi regarder.

En eau peu profonde **linéaire**, une onde vers la droite vérifie `u = c·η/h₀`. L'éponge multiplie
`η` et `u` par le même facteur `f` en chaque point : l'état devient `(f·η, f·c·η/h₀)`, **qui vérifie
encore la relation d'onde droite**. L'amortissement ponctuel est donc **sans réflexion par
construction algébrique**, quelle que soit la raideur de la rampe. Les `10⁻³` de B-S26 ne mesuraient
que la non-linéarité et la discrétisation.

En milieu dispersif, la relation d'onde droite est `η̇_k = −i·sign(k)·ω(k)·η_k` — **non locale en
`x`**. Multiplier par un facteur qui dépend de `x` ne la préserve pas : un produit dans l'espace est
une convolution dans `k`, elle rabat de l'énergie sur les modes gauches, et c'est la réflexion. Elle
ne devient petite que si la rampe varie lentement **devant une longueur d'onde**.

> **B-S26 n'a pas mesuré une propriété de l'éponge : il a mesuré une propriété de l'eau peu profonde.**
> Le résultat était juste, sa portée était nulle, et **rien dans la mesure ne pouvait le dire** — il
> fallait changer de milieu. Angle mort **A166**.

## 5. Ce qui est confirmé : le réglage, et enfin quel `c`

À une largeur qui fonctionne, `R(σ_max)` a un **vrai minimum** — et il tombe sur le réglage
d'ADR-042 D1.

| `σ_max` (à `L_s = 2λ`) | `R` étroite | `R` large |
|---|---|---|
| 2,0·c_g/L_s | 0,02238 | 0,02414 |
| 4,0·c_g/L_s | 0,01216 | 0,01479 |
| 6,9·c_g/L_s | 0,00335 | 0,00621 |
| **10,0·c_g/L_s** | **0,00144** | **0,00393** |
| 20,0·c_g/L_s | 0,00269 | 0,00362 |
| 40,0·c_g/L_s | 0,00499 | 0,00693 |

B-S26 n'avait trouvé qu'un **plancher** ; en milieu dispersif, c'est un **minimum franc**, et amortir
au-delà dégrade. La thèse de B-S26 — *« une éponge trop raide est une rupture d'impédance, et une
rupture d'impédance réfléchit »* — est donc juste, et elle ne se voyait pas dans le milieu où elle
avait été posée.

**Et l'ambiguïté d'ADR-005 §2 est tranchée.** Le texte écrit « `c` » sans dire lequel ; en eau
profonde, phase et groupe diffèrent d'un **facteur deux**. Au même point de fonctionnement :

```
σ_max = 10·c_groupe/L_s  →  R = 0,00144
σ_max = 10·c_phase /L_s  →  R = 0,00269
```

**C'est la vitesse de GROUPE**, celle qui transporte l'énergie. Le choix vaut un facteur ~2 sur le
résultat, et il n'était écrit nulle part.

## 6. Les décisions

**D1 — `ADR-042 D2` est RÉTRACTÉ.** La largeur d'éponge n'est pas fixée par la maille. Elle est
fixée par la **longueur d'onde**, comme ADR-005 le disait — avec une constante deux à quatre fois
plus grande.

**D2 — la règle devient `L_s ≥ λ_δ`, et `L_s ≥ 2·λ_δ` dès que `δ` porte un spectre.** La seconde est
le cas normal ; la première ne vaut que pour une onde quasi monochromatique.

**D3 — `ADR-042 D4` est RÉTRACTÉ : la borne haute de `λ_cut` est REFERMÉE, et elle est deux à quatre
fois plus serrée qu'avant.** Voir §7.

**D4 — `ADR-042 D1` est CONFIRMÉ**, et précisé : `σ_max = 10·c_g/L_s`, avec la vitesse de **groupe**.
C'est un minimum mesuré, pas une borne.

**D5 — `ADR-042 D3` est CONFIRMÉ** : `σ_max·dt < 1`. Il ne dépendait d'aucune réserve, et il est
resté vérifié sur toute la table (0,006 à 0,447).

**D6 — la conclusion ne vaut que si `δ` est en eau dispersive.** `λ_δ ≤ λ_cut` par construction
(ADR-005 §2.1) ; le régime dépend de `k·h`, donc du **site**. Un domaine sur un haut-fond où
`λ_δ ≳ h` retombe dans le cas de B-S26 et peut se contenter d'une éponge étroite. **En l'absence de
preuve que le site est peu profond, dimensionner pour le cas dispersif.**

## 7. Ce que cela fait à `DOSSIER-B2 §3.1`

`L_s = λ_cut/2` par face donnait un intérieur utile de `W − λ_cut`. Avec **`L_s = λ_cut`**, il vaut
`W − 2·λ_cut` ; avec **`L_s = 2·λ_cut`** — le cas d'un spectre — `W − 4·λ_cut`.

| Domaine | Emprise transverse | `λ_cut` = 1 m | 1,5 m | 2 m | 3 m |
|---|---|---|---|---|---|
| **Impact** | 6 m | intérieur 67 % | **50 %** | 33 % | **0 %** |
| Bateau | 12 m | 83 % | 75 % | 67 % | 50 % |
| Déferlement | 20 m | 90 % | 85 % | 80 % | 70 % |

**La borne dictée par le domaine d'impact passe de `λ_cut ≤ 3 m` à `λ_cut ≤ 1,5 m`** pour le même
critère — la moitié de l'emprise conservée. **Et à `L_s = 2·λ_cut`, elle passe à `λ_cut ≤ 0,75 m`.**

**Le conflit du §3.3 s'aggrave d'autant.** Ce paragraphe constatait que la borne haute (éponge,
`≤ 3 m`) et la borne basse (décimation, `≈ 10 m`) ne se croisaient pas, *« à un facteur trois près »*.
**Le facteur devient sept, et treize dans le cas d'un spectre.** Ce n'est plus un arbitrage serré,
c'est une impasse — et elle appelle un traitement différent, pas un compromis.

## 8. Ce qui reste ouvert, et une piste que la mesure désigne

1. **Un autre dessin d'éponge n'a pas cette limite.** Ce qui est mesuré est l'éponge **qu'ADR-005 §2
   spécifie** : amortissement **ponctuel** de `δ` et `u_δ`. Le §4.1 montre que le défaut vient
   précisément de là — un facteur local ne préserve pas une relation non locale. Une couche
   parfaitement adaptée (PML), ou un amortissement appliqué aux **composantes directionnelles**
   plutôt qu'aux variables d'état, échappe à l'argument. **C'est la piste que la mesure désigne**,
   et elle est d'autant plus intéressante que l'impasse du §7 ne se résout pas autrement.
2. **La transduction** (ADR-005 §3 : l'éponge doit envoyer l'énergie vers `W`, pas la détruire) n'est
   toujours pas mesurée, ni ici ni en B-S26.
3. **Le milieu est linéaire, 1D, à fond plat.** La conclusion porte sur la dispersion et sur elle
   seule.
4. **Le profil `σ(s)`** : ADR-005 §2 préfère le quadratique au linéaire pour réduire la réflexion
   d'entrée, sans l'avoir mesuré. Le §4.1 suggère que ce choix pèse davantage en milieu dispersif
   qu'en eau peu profonde — c'est mesurable avec le montage existant.
