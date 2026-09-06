# ADR-042 — L'éponge mesurée, et la borne de `λ_cut` rouverte

> **Importée de la lignée B le 2026-09-06 (S35).** Ce document s'appelait `ADR-034` dans une
> histoire parallèle du dépôt, où ce numéro désigne ici un autre sujet. Ses renvois ont été
> renumérotés selon la carte de [`FORK-S22-S26`](../registres/FORK-S22-S26.md) ; **son texte n'a
> pas été modifié autrement**.

- **Statut** : proposée
- **Session** : B-S26
- **Remplace** : `ADR-005 §2` — le réglage `σ_max ≈ 4·c/L_s` et la condition `L_s ≥ λ_δ/2`
- **Rouvre** : `DOSSIER-B2 §3.1` — la borne haute de `λ_cut` issue de la largeur d'éponge
- **Produit** : C05 exécuté pour la première fois ; l'éponge implémentée et mesurée
- **Suite de** : [ADR-041](ADR-041-le-dernier-cas-rouge-etait-rouge-a-cause-de-sa-mesure.md)

---

## 1. Ce qui était écrit, et jamais mesuré

`ADR-005 §2` pose l'éponge du projet :

```
σ(s) = σ_max · s²          s ∈ [0,1], 0 à l'intérieur, 1 au bord
R ≈ exp(−2 ∫ σ/c ds)
```

et conclut : *« `R < 1 %` est atteint pour `σ_max ≈ 4·c/L_s` dès que `L_s ≥ λ_δ/2` »*.

**Ces deux constantes n'ont jamais été vérifiées.** Elles portent pourtant la borne dure de
`DOSSIER-B2 §3.1` : `L_s = λ_cut/2` par face, donc `λ_cut ≤ 3 m` pour qu'un domaine d'impact garde un
intérieur — et `λ_cut` est le paramètre le plus connecté du corpus.

## 2. Le réglage est faux d'un facteur 1,7, et cela se voyait sans code

Avec le profil quadratique du §2, sur une bande de largeur `L_s` :

```
∫₀^{L_s} σ_max·(x/L_s)²/c dx  =  σ_max·L_s / (3c)

σ_max = 4·c/L_s    →   R ≈ exp(−8/3)   =  6,95 %      ← le réglage recommandé
σ_max = 6,9·c/L_s  →   R ≈ exp(−4,6)   =  1,01 %      ← ce qu'il faut pour la promesse
```

**Le réglage recommandé rate son propre critère d'un facteur sept.** Une seule lecture est
dimensionnellement possible — si `∫σ ds` portait sur le `s` adimensionné de `[0,1]`, le quotient
`σ/c` laisserait des `s·m⁻¹` dans une exponentielle — donc l'hypothèse du malentendu de notation est
écartée.

**L'erreur survit depuis S05.** Elle tient en une multiplication, et elle a traversé six audits, deux
revues croisées et un dossier de banc qui s'appuie sur elle. **Une formule énoncée avec ses
constantes n'invite pas à être recalculée** : elle a l'apparence d'un résultat, et on ne vérifie pas
un résultat, on le cite. Angle mort **A159**.

## 3. La mesure : la formule est excellente, et elle a un plancher

Canal de 400 m, `h₀ = 2 m`, paquet d'ondes lancé vers la droite, éponge sur les 10 derniers mètres
(`L_s = λ/2`, `λ = 20 m`), jauge à 280 m. **Essai témoin à `σ_max = 0`** — le bord redevient un mur
parfait — dont le rapport élimine la dissipation numérique du trajet.

**Le témoin vaut 0,8957** : 10 % de l'amplitude se perd sur l'aller-retour. Sans correction, l'éponge
serait créditée de cette dissipation, **en sa faveur**.

| `σ_max` | `R` mesuré | formule ADR-005 |
|---|---|---|
| 1,0·c/L_s | 0,5213 | 0,5134 |
| 2,0·c/L_s | 0,2690 | 0,2636 |
| **4,0·c/L_s** | **0,0704** | 0,0695 |
| 6,9·c/L_s | 0,00951 | 0,01005 |
| 10,0·c/L_s | 0,00146 | 0,00127 |
| 20,0·c/L_s | 0,000879 | **0,000002** |
| 50,0·c/L_s | 0,000958 | 0,0000000 |

**Trois lectures, et elles ne disent pas la même chose.**

1. **La formule est excellente jusqu'à `10·c/L_s`** — 1,3 % d'écart au réglage nominal. Ce n'est pas
   une approximation grossière : c'est un bon modèle, et il mérite d'être conservé.
2. **Sa conclusion est fausse**, et la mesure confirme l'arithmétique du §2 : **7,0 %** au réglage
   recommandé. Deux méthodes indépendantes, le même verdict.
3. **Elle s'effondre au-delà** : elle promet `2·10⁻⁶` là où on mesure `8,8·10⁻⁴`, quatre cents fois
   plus. La mesure **sature vers `9·10⁻⁴`**, et ce plancher n'est pas du bruit — l'amplitude
   réfléchie y vaut `1,8·10⁻⁵ m`, cinq décades au-dessus de l'arrondi machine. C'est **la réflexion
   à l'entrée de l'éponge**, l'impédance que le modèle ne contient pas.

> **Amortir plus fort que `≈10·c/L_s` ne sert à rien.** Le corpus n'avait aucune raison de le savoir :
> la formule qu'il cite promet un gain sans fin.

## 4. La largeur n'est pas fixée par la longueur d'onde

À `σ_max` proportionnel à `c/L_s`, **`R` ne bouge pas quand l'éponge rétrécit** — d'une longueur
d'onde entière à un huitième :

| `L_s/λ` | 1,000 | 0,500 | 0,333 | 0,250 | 0,167 | 0,125 |
|---|---|---|---|---|---|---|
| `R` à 6,9·c/L_s | 0,00992 | 0,00951 | 0,00958 | 0,00951 | 0,00912 | 0,00888 |
| `R` à 10·c/L_s | 0,00156 | 0,00146 | 0,00188 | 0,00198 | 0,00180 | 0,00157 |

**La condition `L_s ≥ λ/2` n'est exercée nulle part dans cette plage.**

### 4.1 Ce qui borne l'éponge par le bas : `σ_max·dt < 1`

| mailles | `L_s` (m) | `L_s/λ` | `σ_max·dt` | `R` |
|---|---|---|---|---|
| 40 | 5,0000 | 0,2500 | 0,111 | 0,001975 |
| 20 | 2,5000 | 0,1250 | 0,222 | 0,001574 |
| 10 | 1,2500 | 0,0625 | 0,443 | 0,001651 |
| **5** | **0,6250** | **0,0312** | **0,887** | **0,001895** |
| 3 | 0,3750 | 0,0187 | 1,478 ⚠ | 0,000974 |
| 2 | 0,2500 | 0,0125 | 2,217 ⚠ | 0,000693 |
| 1 | 0,1250 | 0,0063 | 4,434 ⚠ | 0,000441 |

L'amortissement s'écrit `×(1 − σ·dt)`. **Au-delà de `σ·dt = 1`, le facteur devient négatif, il est
saturé à zéro, et la maille est remise à l'état de repos à chaque pas.** C'est un **puits**, plus une
éponge : les trois dernières lignes mesurent les propriétés d'un autre opérateur, et ne se
transportent pas.

**La borne se dérive, et la mesure la confirme.** Avec `σ_max = K·c/L_s` :

```
σ_max·dt < 1   ⟺   L_s > K·c·dt = K·CFL·dx        (K = 10, CFL = 0,45)
                   L_s > 4,5·dx  ≈  5 mailles
```

La transition mesurée tombe **entre 5 et 3 mailles**. Prédiction et mesure coïncident.

## 5. Les décisions

**D1 — le réglage devient `σ_max = 10·c/L_s`.** Il donne `R ≈ 1,5·10⁻³`, un ordre de grandeur sous
l'exigence de C05, et il reste **du bon côté de `σ_max·dt < 1`** dès que `L_s ≥ 5·dx`. Le réglage
`6,9·c/L_s` atteindrait tout juste 1 % ; prendre la marge coûte zéro.

**D2 — la largeur devient `L_s ≥ K·CFL·dx`, soit ≈ 5 mailles.** La règle `L_s = λ/2` est retirée : la
mesure ne trouve pas sa trace en eau peu profonde. **C'est la maille et le pas de temps qui fixent
l'éponge, pas la longueur d'onde.**

**D3 — `σ_max·dt < 1` entre dans le corpus comme une condition à part entière.** Elle n'y figurait
pas. C'est l'analogue, pour un terme source, de la condition CFL sur le transport : **un coefficient
d'amortissement n'a de sens que tant que `σ·dt < 1`**, et au-delà l'opérateur change de nature sans
prévenir. Angle mort **A160**.

**D4 — la borne haute de `λ_cut` issue de l'éponge est ROUVERTE, pas retirée.** `DOSSIER-B2 §3.1`
la tenait pour dure ; elle repose sur `L_s = λ_cut/2`, dont la mesure ne trouve pas le fondement.

| | ADR-005 §2 | mesuré |
|---|---|---|
| domaine d'impact, `dx = 0,05 m` | `L_s = λ_cut/2 = 2 m` | `L_s ≈ 5·dx = 0,25 m` — **8× plus étroit** |
| intérieur utile d'un domaine de 6 m | 2 m sur 6 | **5,5 m sur 6** |

## 6. Pourquoi « rouverte » et non « retirée »

Trois réserves, et la première suffit à ne pas conclure.

1. **Le solveur est non dispersif** (`c = √(gh)`), là où ADR-005 raisonne en eau profonde
   (`c = √(gλ/2π)`). Une éponge d'eau profonde doit absorber une **bande** de longueurs d'onde qui
   voyagent à des célérités différentes ; une éponge accordée sur une seule `c` y est désaccordée
   pour les autres. **La règle `λ/2` protège peut-être exactement de cela** — et rien ici ne le
   teste. **C'est la première chose à mesurer** avant de s'appuyer sur D2 pour déplacer `λ_cut`.
2. **Un seul `λ`, en une dimension.** Le groupe sans dimension `σ_max·L_s/c` et le rapport `L_s/λ` se
   transposent ; la valeur de `σ_max` en s⁻¹ ne se transpose pas.
3. **L'éponge d'ADR-005 §3 doit être un transducteur, pas un absorbeur** — l'énergie part vers `W`,
   elle n'est pas détruite. Rien de cela n'est mesuré ici : **seule l'absorption l'est**, et une
   éponge qui absorbe bien peut transduire mal.

**D1 et D3 sont acquis** — ils ne dépendent d'aucune de ces réserves. **D2 et D4 sont des résultats
d'eau peu profonde**, solides dans leur domaine et à confirmer hors de lui.

## 7. Ce que devient C05

Le cas testait le réglage d'ADR-005, et **l'a éliminé** — c'est exactement son travail. Il teste
désormais le réglage de D1, et **le réglage éliminé reste imprimé à côté** : sans lui, C05 ne
démontre plus qu'il élimine quelque chose (L123).

**Conditions de mesure** *(nouvelles)* :

- `R` = rapport de deux maxima d'élévation à une jauge fixe, sur deux fenêtres temporelles
  **disjointes**. Séparation par le temps, non par transformée — une analyse spectrale apporterait
  sa propre fenêtre, donc son propre biais (A102) ;
- **`R` est corrigé par un essai témoin** à `σ_max = 0`, et le témoin est rapporté : s'il s'écarte de
  1, la maille est trop grossière pour la mesure ;
- **`σ_max·dt` est rapporté** : au-delà de 1, le chiffre ne mesure plus une éponge.

## 8. Ce qui reste ouvert

1. **L'absorption d'une bande de longueurs d'onde en eau profonde** — la réserve n°1, et le
   préalable à D4.
2. **La transduction** (ADR-005 §3), jamais mesurée.
3. **Le plancher à `9·10⁻⁴`** : sa dépendance au profil `σ(s)` n'est pas explorée. Un profil plus
   doux à l'entrée devrait l'abaisser, et c'est précisément ce que le §2 d'ADR-005 invoquait pour
   préférer le quadratique au linéaire — sans le mesurer non plus.
