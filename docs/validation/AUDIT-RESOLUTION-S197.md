# S197 — Audit de résolution : ce que le critère d'énergie ne voyait pas

2026-09-12. **A242**, ouverte par S196. **Protocole et prédictions avant toute mesure.**
Aucun choix de solveur δ, aucun seuil de bascule W/δ. Un ADR n'est jamais réécrit : si une
conclusion tombe, elle reçoit une note corrective datée, et une décision qui changerait
demanderait un ADR neuf.

## 1. Ce que S196 a trouvé, et pourquoi il faut aller voir

S196 a montré qu'une configuration peut être **fausse d'un facteur cinq** en conservant son
énergie à `1,54e-6`, soixante-cinq fois sous le seuil de domaine que S193 à S196 employaient.
La conservation mesure ce que le schéma préserve, jamais ce qu'il résout (**L277**).

Reste la question que S196 n'a pas posée : **les résultats déjà publiés étaient-ils, eux, loin
du bord ?** Elle n'est pas rhétorique. ADR-123 est **actée**, et ses seuils sortent d'un
ajustement fait à `K = 64`.

## 2. Le défaut est calculable en forme fermée, sans rien exécuter

`K` — le nombre de niveaux verticaux — n'entre dans le véhicule **que** par le symbole de
dispersion `dn[q]`, précalculé une fois à la construction. Le pas de temps ne le voit jamais.
Deux conséquences, et elles orientent toute la session :

1. **Le défaut se calcule sans simuler.** Le symbole discret a une forme fermée, déjà vérifiée
   par le banc : `G_h = sinh(γ)·tanh(K·γ)/dz` avec `γ = acosh(1 + μ/2)` et `μ = (k·dz)²`. Le
   symbole continu vaut `k·tanh(k·h)`. Leur écart relatif est de l'arithmétique.
2. **Monter `K` ne coûte presque rien à l'exécution** — seulement à la construction. L'audit
   est donc abordable jusqu'à des `K` très grands.

Le banc vérifiait déjà que son symbole discret est **celui qu'il croit calculer**. Il ne
vérifiait nulle part que ce symbole **approche la physique**. C'est précisément le trou
d'A242, transposé de l'énergie à la dispersion.

### 2.1 L'écart, à `K = 64` — la valeur employée par S193, S194, S195 et S196

Écart relatif `|G_h − k·tanh(k·h)| / (k·tanh(k·h))`, `L = h = 8 m` :

| mode `q` | 2 | 3 | 9 | 16 | 24 | 38 |
|---|---:|---:|---:|---:|---:|---:|
| écart à `K=64` | **0,48 %** | **1,08 %** | **9,32 %** | 27,2 % | 54,5 % | 112 % |

Par configuration publiée, sur la bande **réellement peuplée** — les modes que `M = 3` peut
atteindre, soit jusqu'à `3·max(q_train)`, bornés par la bande :

| session | trains | bande peuplée | écart à `K=64` | `K` requis pour 1 % |
|---|---|---:|---:|---:|
| **S194 → ADR-123** | (2, 3) | `q ≤ 9` | **9,3 %** | 256 |
| S195 → A240, `n=6` | 2..7 | `q ≤ 21` | **43,6 %** | 512 |
| S196 → A241, `n=16` | 2..17 | `q ≤ 38` | **112 %** | 1024 |

Le symbole converge à l'ordre deux : l'écart est divisé par quatre à chaque doublement de `K`.

### 2.2 Ce que ce tableau ne dit pas

Il serait facile, et faux, d'en conclure que les résultats publiés sont faux. **Un symbole
inexact n'est pas une conclusion fausse**, pour trois raisons qu'il faut tenir :

- les deux évolutions comparées — la somme et le total — emploient **le même** symbole, et
  l'écart entre elles peut être bien moins sensible que chacune ;
- l'amplitude vit aux modes porteurs, où l'écart vaut 0,5 à 1,1 %, pas au haut de bande ;
- ce que S194 a identifié comme gouvernant le couplage est le **désaccord de triade**, qui est
  une différence de fréquences — donc précisément ce qu'un symbole biaisé déplace. Cela peut
  jouer dans les deux sens, et c'est la raison de mesurer plutôt que de raisonner.

## 3. Ce qui est mesuré

Chaque cible est rejouée **à sa configuration publiée**, en ne changeant que `K`.

| cible | enjeu | `K` | ce qu'on relève |
|---|---|---|---|
| **S194 / ADR-123** | ADR **acté** | 64, 256, 1024 | les seuils, puis `α`, `β` |
| S195 / A240 | angle **clos** | 64, 512 | l'exposant de la série dense |
| S196 / A241 | verdict de la veille | 64, 1024 | l'écart pair/impair de `0,131` |

## 4. Ce qui compte comme échec — et ce qui n'en est pas un

**Un chiffre qui bouge n'est pas une conclusion qui tombe.** ADR-123 publie `α = 1,302602` à
sept chiffres ; ce nombre **va** bouger, et ce n'est pas le sujet. Les critères sont donc
posés sur les **énoncés**, pas sur les décimales :

| énoncé publié | survit si |
|---|---|
| ADR-123 : 2 % sous une cambrure d'« environ `0,009` » | le seuil reste dans `0,0077 – 0,0104` (±15 %) |
| ADR-123 : « au moins 20 périodes » sous `s = 0,008` | tient encore |
| ADR-123 : « moins d'une période » au-dessus de `s = 0,014` | tient encore |
| ADR-123 : le levier de durée est étroit, facteur 1,6 | le rapport haut/bas reste entre 1,3 et 2,0 |
| A240 : l'écart **décroît** à cambrure totale fixée | l'exposant reste négatif |
| A241 : saturation à `−0,52` | reste dans `−0,42 … −0,62` |
| A241 : le repli pèse un tiers, écart `0,131` | l'écart pair/impair reste au-dessus de `0,10` |

Si une ligne tombe, la session produit une **note corrective datée** sur le document et sur
l'ADR concerné, et dit exactement ce qui tombe et ce qui reste.

## 5. Prédictions, déclarées avant la mesure

**Prédiction 1 — les seuils tiennent.** L'amplitude est aux modes porteurs, où l'écart de
symbole vaut 0,5 à 1,1 %. Les grandeurs mesurées se déplaceront de quelques pour cent, et
aucune ligne du §4 ne tombera.

**Prédiction 2 — le désaccord de triade amplifie.** Le couplage étant gouverné par une
**différence** de fréquences, un biais de 1 % sur chacune peut en faire bien plus sur leur
écart. Les durées d'ADR-123, qui dépendent du temps, bougeraient alors davantage que les
cambrures, et le tableau des durées serait la première ligne à céder.

Les deux sont plausibles et s'excluent. C'est pourquoi elles sont écrites avant.

## 6. Le remède : apparier le critère

Deux gestes, dans le support partagé, pour que la faute ne puisse plus se répéter :

1. **`dispersion_error(upto)`** sur le véhicule : l'écart relatif maximal du symbole discret
   au symbole continu, sur les modes jusqu'à `upto`. Chaque banc le **déclare** à côté de sa
   dérive d'énergie. Gratuit, exact, et diagnostique — il nomme le mode fautif.
2. **Garde de Richardson** : refuser d'imprimer un ordre tiré d'un triplet non monotone, comme
   S196 l'a fait localement. La garde monte dans le support, pour tous les bancs.

## 7. Ce qui ne sera pas fermé

1. **Aucune re-dérivation.** Si un seuil bouge, la session le constate et le publie ; elle ne
   refait pas la physique de S194.
2. **Le pas de temps n'est pas audité.** `dt = T₁/400` est hérité ; seule la résolution
   **verticale** est en cause ici, et c'est elle qu'A242 vise.
3. **La bande n'est pas rouverte** : S195 et S196 l'ont vérifiée, `Q+8` ne déplaçant rien.
4. **Aucun contrat runtime**, aucune mesure de coût, `water-core` inchangé.


## 8. Relevés

Les trois bancs publiés ont reçu leur audit et le portent désormais eux-mêmes — c'est le
remède du §6, appliqué là où le critère est employé.

```
cargo run --release --manifest-path code/Cargo.toml -p water-core --example nl_coupling_2d
cargo run --release --manifest-path code/Cargo.toml -p water-core --example nl_sources_2d
cargo run --release --manifest-path code/Cargo.toml -p water-core --example nl_fallback_2d
```

Empreintes **changées** par l'ajout de l'audit — les valeurs publiées, elles, sont
reproduites à l'identique à `K = 64`, et le banc le vérifie : `0x4bc0934d630c2c50`,
`0xb9b742189219c8ce`, `0xcbb87743073b703d`. Workspace **331 réussis / cinq ignorés**, et
douze réceptions au banc S196, dont les deux gardes neuves.

### 8.1 ADR-123 — la table mesurée tient

La table du §7.8 de S194 est **mesurée, pas interpolée** ; c'est elle, et non l'extrapolation
de la loi ajustée, que porte ADR-123. Rejouée en ne changeant que `K` :

| `K` | erreur de symbole à `q=9` | `s=0,008` | `0,009` | `0,010` | `0,0125` | `0,014` | `0,015` | `0,020` |
|---:|---:|---|---:|---:|---:|---:|---:|---:|
| **64** *(publié)* | 9,32 % | jamais | 19,82 | 11,70 | 5,38 | 0,78 | 0,75 | 0,40 |
| 256 | 0,608 % | jamais | 19,38 | 11,50 | 5,20 | 0,78 | 0,72 | 0,40 |
| 1024 | 0,038 % | jamais | 19,38 | 11,50 | 5,20 | 0,78 | 0,72 | 0,40 |
| *S194 §7.8* | | *jamais* | *19,8* | *11,7* | *5,4* | *0,8* | *0,8* | *0,4* |

`K = 64` reproduit le publié **exactement** — l'audit est donc fidèle — et la table est
**convergée dès `K = 256`**, les deux résolutions fines étant identiques. Le déplacement
maximal vaut **3,3 %**, à `s = 0,0125`. Aucun énoncé ne bouge : toujours « jamais » sous
`0,008`, toujours ~19 périodes à `0,009`, toujours ~5 à `0,0125`, toujours sous une période
à `0,014`. **Chaque ligne du §4 survit.**

**L'ajustement, lui, bouge davantage** — et c'est une mise en garde, pas un défaut :

| `K` | `α` | `β` | résidu relatif | `s*(N=20)` |
|---:|---:|---:|---:|---:|
| 64 *(publié)* | +1,302602 | +5,898728 | 0,0182 | 0,008622 |
| 256 | +1,254219 | +5,970032 | 0,0279 | 0,008715 |
| 1024 | +1,257146 | +5,937021 | 0,0282 | 0,008723 |

`α` se déplace de **3,7 %**, `β` de **1,2 %**, le seuil de cambrure de **1,2 %** — dans la
fenêtre `0,0077 – 0,0104` du §4. Mais les **durées extrapolées** par cette loi hors de sa
plage de calibration (elle est ajustée sur `s ∈ [0,0125 ; 0,1]`) bougent jusqu'à **35 %** à
`s = 0,014`. ADR-123 ne repose pas dessus ; une session qui emploierait la loi loin de sa
calibration, si.

### 8.2 A240 — les deux séries tiennent

| `K` | erreur de symbole à `q=21` | exposant série A | exposant série B |
|---:|---:|---:|---:|
| **64** *(publié)* | 43,62 % | **−0,437** | **+0,783** |
| 512 | 0,83 % | **−0,425** | **+0,774** |

Déplacements de **2,7 %** et **1,1 %**. Le signe, la sous-linéarité et la conclusion d'A240
— répartir une même mer sur plus de composantes améliore la superposition, et la crainte du
`n²` est levée — sont **intacts**.

### 8.3 S196 — le verdict de la veille ne survit pas

| `K` | erreur de symbole | impaire | paire | écart |
|---:|---:|---:|---:|---:|
| **64** *(publié)* | 120,35 % à `q=40` | **−0,525** | **−0,394** | **0,131** |
| 1024 | 0,75 % à `q=40` | **−0,432** | **−0,426** | **0,005** |

`K = 64` reproduit exactement les chiffres de S196 — l'audit est fidèle — et à résolution
convergée **l'écart pair/impair s'effondre de `0,131` à `0,005`**.

> **C'est la prédiction 2 de S196, celle qui réfute.** Son protocole déclarait : « les deux
> familles s'accordent à mieux que `0,10`, et l'explication de S195 tombe ». Elles
> s'accordent à `0,005`. **Le repli des harmoniques croisées n'explique pas un tiers de
> l'écart : il n'en explique rien de mesurable.**

L'autre moitié du verdict de S196 se déplace sans tomber :

| `K` | erreur de symbole | exposant dense `n = 2..16` |
|---:|---:|---:|
| 64 | 111,65 % à `q=38` | −0,478 |
| 1024 | 0,68 % à `q=38` | **−0,445** |

La limite existe toujours — c'était la moitié « limite » d'A241 — mais sa valeur passe de
`−0,52` à environ `−0,45`, dans la fenêtre `−0,42 … −0,62` du §4. *Réserve : l'audit rejoue
l'exposant sur toute la plage, pas les fenêtres glissantes hautes dont S196 tirait son
`−0,52` ; la valeur de saturation elle-même n'est donc pas remesurée directement.*

### 8.4 Pourquoi S196 tombe et pas les deux autres

La différence n'est pas dans l'ampleur de l'erreur de symbole — S195 en portait 43 % et tient.
Elle est dans **la façon dont la comparaison la répartit**.

- **S194 et S195 comparent des configurations à même bande.** L'erreur de symbole y est
  **commune aux deux côtés** de la comparaison, et s'annule en grande partie. Il en reste
  les 1 à 3 % mesurés.
- **S196 comparait deux familles de modes et de bandes différents** — `3,5,…,17` sur une
  bande de 38 contre `4,6,…,18` sur une bande de 40. Chaque famille subissait donc une
  erreur de symbole **différente**, et ce qui était mesuré comme « effet du repli » était,
  pour l'essentiel, l'écart entre deux défauts de dispersion.

S196 avait pourtant contrôlé beaucoup : même `n`, même cambrure totale, bande relative
bornée, échelle absolue éprouvée par un témoin. Il n'avait pas contrôlé **l'exposition à
l'erreur de résolution**, parce que rien dans le corpus ne disait qu'il fallait le faire.
C'est **L278**.

## 9. Verdict par cible

| cible | enjeu | verdict |
|---|---|---|
| **ADR-123** (S194) | ADR **acté** | **tient** — table convergée dès `K=256`, déplacement ≤ 3,3 % |
| **A240** (S195) | angle **clos** | **tient** — exposants déplacés de 1 à 3 % |
| **A241** (S196) | verdict de la veille | **tombe** — l'écart `0,131` devient `0,005` |

**Ce que la session referme.** A242 est **traitée** : les configurations publiées ont été
auditées, le remède est en place dans le support et dans les trois bancs, et deux gardes
neuves empêchent la répétition — `dispersion_error(upto)`, qui dit l'écart au symbole continu
**à la configuration employée**, et `richardson()`, qui refuse de tirer un ordre d'un triplet
qui ne converge pas.

**Ce que la session casse.** Le résultat central de S196 — « le repli pèse un tiers » — est
**faux**, et il l'était pour la raison même qu'A242 nomme. A241 reçoit donc sa réponse, et ce
n'est pas celle que S196 a publiée : sur sa moitié « cause », **le repli est réfuté**. La
totalité de l'écart entre la mesure et l'addition dispersée reste sans explication.

**Ce que la session ne fait pas.** Elle n'explique pas davantage cet écart ; elle retire
seulement un suspect, et le retire proprement. Elle n'audite pas le pas de temps `dt`, hérité
et hors du champ d'A242. Elle ne rejoue pas S193, dont la bande peuplée est la moins exposée
des quatre. Et elle ne remesure pas les fenêtres glissantes hautes de S196 (§8.3).

**Aucun ADR.** ADR-123 est **confirmée** par l'audit et reçoit une note datée qui le dit ;
elle n'est pas réécrite. Aucune décision de projet ne change.
