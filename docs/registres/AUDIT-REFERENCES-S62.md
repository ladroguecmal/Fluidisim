# Audit des références — ce qu'un cas peut voir bouger, S62, 2026-09-08

Instruction de l'action **S58-2** : *A104 recense la faute, personne n'a recensé ses instances.*
**A104**, ouvert en S21 — *une règle énoncée dans un fichier n'empêche pas sa violation dans le
même fichier* — et **A180**, trouvé en S58, où trois références de C10 étaient construites avec la
constante qu'elles devaient contrôler, bloquant un arbitrage pendant trente-sept sessions.

**Ce n'est pas l'audit de S29.** [`AUDIT-ASSERTIONS-S29`](AUDIT-ASSERTIONS-S29.md) classait les
assertions par ce qu'elles peuvent voir **échouer** — recevable, symptôme, vacuité. Ici la question
porte sur la **référence** : que peut-elle voir **bouger** ? Les deux se recouvrent sur un point,
et se séparent partout ailleurs : une assertion recevable peut avoir une référence aveugle.

## 1. Le critère n'est pas « partage un paramètre »

Quarante et une références inventoriées dans `physics.rs` et `physics_shallow.rs`. Trois degrés
apparaissent, et **un seul est une faute** :

| Degré | Ce que la référence est | Ce que le cas teste | Exemple |
|---|---|---|---|
| **Tautologie** | un recalcul de la mesure | l'arithmétique du test | `C10-raideur` : dérivée de `ρ·g·A·d` contre `ρ·g·A` |
| **Aller-retour** | un paramètre d'entrée que la mesure reconstruit | une **chaîne**, jamais une valeur | `Hs` : mer engendrée d'après `hs`, mesurée par la variance |
| **Indépendance** | une solution analytique, ou une autre mesure | la physique | `C04` contre Ritter ; `C02-c` : `λ/T` mesurés contre `√(gλ/2π)` |

**Ces degrés se mesurent, ils ne se déduisent pas.** Le test : balayer le paramètre et regarder
l'écart. Identiquement nul → tautologie. **Constant et non nul → aller-retour.** Variable →
indépendance. Les paramètres de mer vivent dans le scénario TOML : le balayage n'a demandé aucune
recompilation.

## 2. Le recensement

**La grande majorité des cas est saine.** Vingt-quatre références valent `0,0` ou `1,0` — un écart
nul, un rapport unité, une identité — et ne peuvent rien tirer d'un paramètre. Les références
analytiques de `C04` (Ritter : `4h₀/9`, `(2/3)√(gh₀)`, position du front) et de `C03` viennent
d'une solution fermée que le solveur ignore. `C02-c` est le cas exemplaire du degré 3 : il
confronte deux **mesures** — `λ` et `T` relevés dans le champ — à la relation de dispersion.

| Cas | Degré | Établi |
|---|---|---|
| `C10-tirant`, `C10-raideur`, `C10-période` | tautologie / aller-retour | **A180**, S58 |
| **`Hs`** | **aller-retour** | **ce rapport** |
| `orbitale`, `pente` | aller-retour | ce rapport, §4 |
| `C02-λ` | indépendance, avec réserve | ce rapport, §4 |
| `C04` (×5), `C03` (×2), `C02-c`, `C02-disp` | indépendance | — |
| 24 références à `0` ou `1` | sans objet | — |

## 3. `Hs` — aveugle au paramètre qu'il nomme, gouverné par un qu'il ne nomme pas

### 3.1 Aveugle à `hs`

| `hs` | mesuré | rapport mesure/référence |
|---:|---:|---:|
| 0,6 | 0,548834 | **0,914723** |
| 1,2 | 1,097667 | **0,914723** |
| 2,4 | 2,195334 | **0,914723** |
| 4,8 | 4,390669 | **0,914723** |

Écart **8,528 % à chaque fois**, sur un facteur 8. La mer est engendrée d'après `hs` et la mesure
le reconstruit : **le cas ne peut pas voir une erreur de valeur**, seulement un facteur de chaîne.

### 3.2 Gouverné par la fenêtre rapportée à la longueur d'onde de pic

C'est **A102**, énoncé en S21 — *une mesure statistique a besoin d'une fenêtre de plusieurs fois
la plus longue onde* — et **jamais mesuré**. Le voici, à `tp = 6 s`, `λ_pic = 56,2 m` :

| fenêtre | 384 m | 768 m | 1536 m | 3072 m | 6144 m | 12288 m |
|---|---:|---:|---:|---:|---:|---:|
| en `λ_pic` | 6,8 | 13,7 | 27,3 | **54,7** | 109,3 | 218,7 |
| écart | 8,53 % | 9,34 % | 2,43 % | **0,28 %** | 0,28 % | *`NaN`* |

**« Plusieurs fois » est insuffisant : il en faut une cinquantaine.** Et par `tp`, à fenêtre
nominale :

| `tp` | 4,0 s | 6,0 s | 9,0 s |
|---|---:|---:|---:|
| fenêtre / `λ_pic` | 15,4 | 6,8 | 3,0 |
| écart | **0,018 %** | 8,53 % | **15,58 % — échec** |

Et par nombre de composantes, où **rien ne converge** : 1,42 % · 10,24 % · 8,53 % · 12,83 % ·
6,53 % · **26,30 %** pour 8 · 16 · 32 · 64 · 128 · 256. *Raffiner la configuration fait échouer le
cas* — le même motif qu'en S55, où raffiner la grille rendait une oscillation non mesurable.

### 3.3 Ce que cela change

La configuration nominale tient **6,8 longueurs d'onde** et consomme **85 % de sa tolérance**. Le
cas est présenté comme *« le contrôle de bout en bout de toute la chaîne d'amplitude »* ; à cette
fenêtre, **ce qu'il mesure est la taille de sa fenêtre**.

*Et la chaîne d'amplitude, elle, est juste* : 0,28 % à grande fenêtre. **Sans ce témoin, les
8,53 % nominaux seraient indiscernables d'un défaut de répartition ou de sommation** — c'est lui
qui innocente le système, et il manquait.

> **Le rapport 0,914723 n'est pas une constante.** Sur une autre réalisation — autre instant,
> autre graine — il vaut 0,688. Ce que le cas mesure est une propriété de l'échantillonnage
> **d'une réalisation**, pas du système. Le test écrit en S62 s'en garde : il vérifie l'invariance
> en `hs` et l'amélioration à grande fenêtre, sans figer un chiffre qui figerait la mer.

### 3.4 Et la fenêtre n'était pas dans le scénario

`main.rs` appelait `hs_restitue(bg, t, sc.hs, 128, 3.0)` : **les deux nombres qui gouvernent la
mesure la plus fragile du harnais étaient des littéraux**, invisibles depuis le scénario. Le
balayage de `grille_cote` n'a rien déplacé sur un facteur 16 — et c'est ainsi qu'ils se sont fait
voir.

Or l'en-tête de `C18-invariants.toml` affirme : *« Les assertions vivent ici et non dans le code du
harnais : ce fichier est auto-suffisant et lisible par quelqu'un qui n'a pas le code sous les
yeux »*, et SPEC-003 §3 l'exige. **La condition de validité de l'assertion n'y était pas.** C'est
une instance d'**A104** dans sa forme littérale — une règle violée dans le fichier qui l'énonce —
et c'est **A184**.

**Corrigé** : `physics.fenetre_cote` et `physics.fenetre_pas_m` sont déclarables, avec pour défauts
les valeurs historiques. Nommer une constante ne la déplace pas ; les mesures publiées et les deux
hashs sont inchangés.

## 4. Les autres aller-retours

`orbitale` compare `u/η` mesuré à `ω_attendu`, et `pente` la pente maximale du champ à `a·k`. Tous
deux sont des aller-retours : la référence est le paramètre qui a engendré le champ. **Ils sont
sains dans leur genre** — ils testent une chaîne, et une erreur de quadrature ou de conversion les
fait tomber, ce que S21 a vérifié par l'échec réel de `orbitale` sur un champ faux (**L67**). Mais
comme `Hs`, ils sont aveugles à une **convention partagée** entre les deux bouts.

`C02-λ` mérite une réserve distincte : la mesure est indépendante — une recherche de passages par
zéro dans le champ — mais `lambda_attendu` **borne la fenêtre de recherche** (`3λ`, pas de `λ/64`).
La dépendance n'est pas dans la valeur, elle est dans le **conditionnement** : une référence
franchement fausse ne fait pas échouer le cas, elle fait échouer la recherche. Le cas le refuse
correctement (`c02_refus_et_temoin`), donc rien n'est à corriger — mais la distinction mérite
d'être écrite : *une référence peut gouverner la mesure sans entrer dans son résultat.*

## 5. Ce que l'audit ne fait pas

- **Il ne déplace aucune valeur nominale.** Les 8,528 % restent publiés, sous une tolérance
  inchangée de 10 %. Ce qui change est ce qu'on en dit.
- **Il ne tranche pas la fenêtre à retenir.** Élargir la fenêtre rendrait `Hs` juste à 0,28 %, mais
  multiplierait par 64 le coût de la mesure, et à 12288 m la variance en une passe rend `NaN`.
  C'est l'action **S62-1**.
- **Il ne rouvre pas S29.** Les degrés de référence et les classes d'assertion sont deux grilles
  distinctes ; leur recouvrement est décrit au §1.
