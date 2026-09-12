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

*À recevoir en P3b. Aucun chiffre n'est écrit avant l'exécution.*

## 9. Verdict par cible

*À recevoir en P4.*
