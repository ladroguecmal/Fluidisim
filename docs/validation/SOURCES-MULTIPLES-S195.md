# S195 — `n` sources : ce que deux trains ne pouvaient pas dire

2026-09-12. S194-1, **option tranchée par l'utilisateur** contre la correction croisée
quadratique, qui reste portée par la file active. **Dérivation et protocole avant code et
mesure**. Réponse attendue à **A240**, seule limite d'[ADR-123](../adr/ADR-123-le-domaine-de-validite-de-la-superposition.md)
qui touche l'architecture plutôt que le banc. Aucun choix de solveur δ, **aucun seuil de
bascule W/δ**. Le seuil B4 de 2 % est fixé (ADR-120) et n'est pas redemandé.

## 1. La question, et pourquoi deux trains n'y répondent pas

ADR-123 chiffre le domaine de la superposition pour **deux** trains. Le chemin perturbatif
du projet additionne en revanche autant de sources qu'il y a d'événements et de
contributions. A240 énonce le risque : le nombre de paires croît comme `n²`. Mais A240 ne
dit pas dans quel sens le résultat va, et la dérivation ci-dessous montre que **deux
réponses opposées sont possibles** — l'écart peut croître, ou décroître, selon une variable
qu'A240 ne mentionne pas.

Même mesure qu'en S194, généralisée : `n` trains évoluent séparément, leur somme est
comparée à l'évolution de la somme.

```
d(t) = u(t) − Σᵢ aᵢ(t)          u(0) = Σᵢ aᵢ(0)
écart = max_x |d_η(x,t)| / A ,   A = Σᵢ aᵢ    (amplitudes de premier ordre)
```

## 2. Dérivation : trois familles de termes croisés, dont une que deux trains n'ont pas

Avec `F(u) = Lu + Q(u,u) + C(u,u,u)` et `u = Σᵢ aᵢ`, les formes multilinéaires symétriques
donnent

```
F(Σaᵢ) = Σᵢ F(aᵢ)
       + 2 Σ_{i<j} Q(aᵢ,aⱼ)                    n(n−1)/2 termes
       + 3 Σ_{i≠j} C(aᵢ,aᵢ,aⱼ)                 n(n−1) termes
       + 6 Σ_{i<j<k} C(aᵢ,aⱼ,a_k)              n(n−1)(n−2)/6 termes
```

**La troisième famille n'existe pas pour deux trains.** Elle apparaît à `n=3` et porte les
nombres d'onde `kᵢ±kⱼ±k_k` : c'est un canal d'interaction que S194 ne pouvait pas voir, et
la seule raison de ne pas l'attendre grand est son ordre, `ε³`. Elle est nommée ici parce
qu'une mesure qui ne la nomme pas l'attribuerait aux deux autres.

### 2.1 Les deux régimes d'addition, et leurs prédictions opposées

Chaque paire `(i,j)` produit une harmonique liée croisée à `kᵢ±kⱼ`, d'amplitude
`≈ (kᵢ+kⱼ) aᵢaⱼ`. Ces harmoniques vivent sur des **nombres d'onde différents** : elles ne
s'additionnent donc pas en un mode unique, et ce que le maximum sur `x` en fait dépend de
leurs **phases relatives**.

- **Régime cohérent.** Si les phases s'alignent — ce qui arrive si tous les trains partent
  en phase, leurs termes croisés culminant alors au même point — le maximum vaut la
  **somme** des amplitudes.
- **Régime dispersé.** Si les phases se répartissent, le maximum vaut environ la **racine
  de la somme des carrés**.

C'est cette variable, et non le nombre de paires, qui décide du sens. **Le jeu de phases
initial est donc une variable du protocole, pas un détail de mise en œuvre.**

### 2.2 Série A — cambrure totale fixée : constant contre `1/n`

La question réaliste est « la même mer, répartie sur plus de composantes ». On fixe la
cambrure **totale** `S` et on donne à chaque train la même cambrure `sᵢ = S/n`, donc
`aᵢ = S/(n kᵢ)`. Alors, avec `A = Σaᵢ = (S/n)Σ1/kᵢ` :

```
Σ_{i<j} (kᵢ+kⱼ) aᵢaⱼ = (S²/n²) (n−1) Σᵢ 1/kᵢ
```

d'où, **exactement** et sans coefficient libre :

```
cohérent    écart/A = S (n−1)/n
dispersé    écart/A = 2 S √(n(n−1)/2) / n²
```

Les deux coïncident à `n=2`, comme ils le doivent — il n'y a qu'une paire. Normalisées à
leur valeur en `n=2`, elles divergent aussitôt :

| `n` | 2 | 3 | 4 | 5 | 6 |
|---|---|---|---|---|---|
| **cohérent** | 1,000 | 1,333 | 1,500 | 1,600 | **1,667** |
| **dispersé** | 1,000 | 0,770 | 0,612 | 0,506 | **0,431** |

**Un facteur 3,87 sépare les deux prédictions à `n=6`, et elles vont en sens contraire.**
La mesure ne peut pas être ambiguë. Et l'enjeu de projet est direct : si le régime réel est
dispersé, **répartir une même mer sur plus de composantes réduit** l'erreur de
superposition ; s'il est cohérent, elle croît et sature à deux fois la valeur de deux
trains.

### 2.3 Série B — cambrure par train fixée : c'est la crainte d'A240

On fixe la cambrure `s` de **chaque** train, donc `aᵢ = s/kᵢ` et la cambrure totale croît
comme `n s`. Le même calcul donne

```
cohérent    écart/A = s (n−1)
dispersé    écart/A = s √(2(n−1)/n)
```

| `n` | 2 | 3 | 4 | 5 | 6 |
|---|---|---|---|---|---|
| **cohérent** | 1,000 | 2,000 | 3,000 | 4,000 | **5,000** |
| **dispersé** | 1,000 | 1,155 | 1,225 | 1,265 | **1,291** |

**Noter ce que la normalisation fait.** L'écart **absolu** croît bien comme `n²` dans le
régime cohérent, comme A240 le craint ; mais rapporté à l'amplitude totale — qui croît
elle aussi — il croît **linéairement**. A240 avait donc raison sur le comptage et tort sur
la conséquence : le facteur est `n`, pas `n²`.

### 2.4 La part séculaire se dilue, et c'est une troisième prédiction

La modulation croisée de fréquence du train `i` vient de tous les autres :
`Σ_{j≠i} kᵢ²aⱼ² ∝ (n−1)(S/n)²`, donc `∝ (n−1)/n²` à cambrure totale fixée. **La part
cumulative par train décroît donc avec `n`**, alors que la part instantanée croît (régime
cohérent) ou décroît moins vite (régime dispersé). Prédiction mesurable sans aucun
ajustement : **le rapport `écart(N=10)/écart(N=1)` doit décroître quand `n` croît**, en
série A.

C'est aussi la raison pour laquelle les mécanismes seront séparés ici par la **durée** et
non par les modes : avec `n` trains, les sommes `kᵢ+kⱼ` tombent génériquement sur des modes
de train ou leurs harmoniques, et il n'existe plus assez de modes **exclusivement** croisés
pour porter un diagnostic. Le banc relèvera le nombre de tels modes et le publiera ; la
séparation par la durée, elle, vaut pour tout jeu de modes.

## 3. Bande, modes et phases

**Modes.** Les `n` premiers de la liste `2, 3, 4, 5, 6, 7`. Choisie pour deux raisons :
elle place `n=2` sur le couple `(2,3)`, **exactement** le couple nominal de S194, ce qui
donne une **continuité vérifiable** entre les deux sessions ; et son maximum, 7, borne la
bande nécessaire.

**Bande.** À l'ordre `M=3` les produits atteignent `3·max(qᵢ) = 21`. La bande retenue est
donc **`Q=24`**, vérifiée contre `Q=32`. C'est la contrainte qu'A240 rendait inévitable :
`Q=16` n'aurait pas tenu au-delà de cinq trains.

**Phases.** Deux jeux, tous deux déterministes — aucun générateur pseudo-aléatoire, pour que
l'empreinte soit reproductible :

```
alignées    φᵢ = 0
dispersées  φᵢ = 2π · frac(i · 0,6180339887498949)      i = 1..n
```

Le second est une suite à faible discrépance, donc bien répartie sans être aléatoire. Un
train de phase `φ` se dépose en `η̂[qᵢ] = (aᵢ/2)e^{iφᵢ}`, `η̂[2qᵢ] = (kᵢaᵢ²b₂/2)e^{2iφᵢ}`,
et les traces `ψ` correspondantes, bâties sur la fréquence **semi-discrète** `ω_d(kᵢ)` du
véhicule et jamais sur celle du continu (**L272**, A236).

## 4. Campagne déclarée

`L=8 m`, `g=9,81 m/s²` injecté, `Q=24`, `K=64`, `M=3` sauf mention, `dt=T₁/400` avec
`T₁ = 2π/ω_d(k₁)` la période semi-discrète du train le plus lent, fenêtre de 10 périodes,
`h=8 m`. Observations 40 par période ; écart relevé à `N = 1, 2, 5, 10`.

| axe | valeurs | ce qu'il mesure |
|---|---|---|
| **série A** | `S = 0,024` fixée, `n = 2..6` | constant contre `1/n` — le cœur |
| **série B** | `s = 0,008` par train, `n = 2..6` | la crainte d'A240, `n` contre `√n` |
| phases | alignées, dispersées | le régime d'addition |
| durée | `N = 1, 2, 5, 10` | la dilution de la part séculaire (§2.4) |
| ordre | `M = 1` à `n=6` | superposition exacte, écart d'arrondi |
| `n=1` | un seul train | écart **exactement nul** |
| bande | `Q = 24 / 32` | vérification, pas convergence |
| profondeur discrète | `K = 32 / 64 / 128` | convergence, **trois** niveaux (L274) |

## 5. Réceptions

1. **`n=1`** — écart **exactement nul**, à tout ordre. Le banc se mesure lui-même (L271).
2. **`M=1` à `n=6`** — la superposition est exacte par construction ; écart de modes
   `≤ 10⁻¹⁴` relatif, écart de champ au plancher de reconstruction (S194 §5.1).
3. **Continuité avec S194** — `n=2`, série B à `s=0,05`, `Q=24`, `K=64`, `dt=T₁/400`,
   20 périodes, phases alignées : doit reproduire le `3,612474·10⁻¹` que S194 a mesuré à
   `Q=24`. **Attendu bit pour bit**, la construction étant identique et les soustractions
   faites dans le même ordre ; toléré à `10⁻¹²` relatif. Une réception qui échoue ici dit
   que la généralisation a changé quelque chose, et il faudra dire quoi.
4. **Régime tranché** — en série A, `écart(n=6)/écart(n=2)` doit valoir `1,667` à ±25 % en
   phases alignées et `0,431` à ±25 % en phases dispersées, et les deux valeurs doivent
   différer d'un facteur **supérieur à 2**. C'est la réception principale.
5. **Monotonie** — en série A, l'écart doit croître avec `n` en phases alignées et décroître
   en phases dispersées, sans inversion.
6. **Série B** — le rapport `n=6 / n=2` doit valoir `5,0` à ±30 % en alignées et `1,29` à
   ±30 % en dispersées.
7. **Dilution séculaire** — en série A, `écart(N=10)/écart(N=1)` doit décroître de `n=2` à
   `n=6`, dans les deux jeux de phases.
8. **Conservation** — dérive relative d'énergie de l'évolution totale sous `10⁻⁴` ; toute
   configuration qui dépasse est **hors domaine**, publiée et exclue des ajustements, comme
   en S194 §7.4.
9. **Convergence** — sur la **moyenne quadratique** de l'écart et sur **trois** niveaux de
   `K` : ordre entre 1,5 et 2,5, résidu de Richardson sous 2 % à `K=64`. Cette forme est
   celle qu'impose **L274** ; juger la taille d'un déplacement confondrait convergence et
   artefact, et **A238** interdit d'établir un ordre sur un maximum de résidu.
10. **Bande** — `Q=32` ne doit pas déplacer l'écart de plus de 2 %.

Deux exécutions `release` identiques, empreinte publiée. Les chiffres ne seront ajoutés
qu'après exécution.

## 6. Limites de ce qui sera fermé

**Ce qui sera fermé si tout passe.** A240 recevra sa réponse : le sens et la loi de la
dépendance en `n`, dans les deux normalisations qui comptent, et le rôle décisif du régime
de phases. ADR-123 saura s'il se transporte à `n` sources ou non.

**Ce qui ne le sera pas.**

1. **Six trains ne sont pas un spectre continu.** Un état de mer réel compte des centaines
   de composantes, et la loi mesurée sur `n ≤ 6` est une tendance, pas une extrapolation.
2. **Une seule dimension horizontale, trains colinéaires.** L'obliquité reste entière.
3. **Un seul jeu de phases dispersées**, déterministe : la variabilité d'un tirage à
   l'autre n'est pas mesurée, donc la valeur « dispersée » est un point, pas une moyenne
   d'ensemble.
4. **Fond plat, eau profonde.** S194 a montré que le couplage est 8,6 fois plus fort vers
   le rivage ; rien ici ne mesure la dépendance en `n` à faible profondeur.
5. **A216 reste inexpliquée**, la source B+W de S190/S191 n'est pas branchée, forces et
   perception ne sont pas reçues, aucun solveur δ n'est choisi.
6. **Aucun contrat runtime** : `f64`, allocations de banc, `water-core` et le support S193
   inchangés. Aucune seconde cible (A98), aucune mesure de coût CPU.

## 7. Relevés

`cargo run --release --manifest-path code/Cargo.toml -p water-core --example nl_sources_2d`

Empreinte **`0x5eb378f6ffe26c9f`**, deux exécutions `release` identiques ligne pour ligne.
`water-core` et `support/nl_surface.rs` inchangés ; workspace **331 réussis / cinq ignorés**,
et les **neuf** tests propres du banc passent.

> **Cette session a changé de main.** La session ouverte à 21:15 a été coupée par une limite
> d'usage sur un autre compte, P3a terminée sur le disque mais non committée. La procédure de
> `notes/EN-COURS.md` a été suivie : diff lu, banc compilé, tests passants, étape **complétée**
> plutôt qu'annulée. Le numéro de session ne change pas.

### 7.1 Trois corrections contre ce protocole, portées par P3a avant toute campagne

Elles sont dans le banc, chacune fixée par un test. Aucune n'est réécrite ci-dessus.

1. **§2.4 annonçait la disparition des modes exclusivement croisés** quand `n` croît, et donc
   l'indisponibilité du diagnostic modal. **Faux** : avec les modes `2..7` il en survit deux ou
   trois à tout `n` — `{1,5}` à `n=2`, `{1,11,13}` à `n=6`. Le diagnostic modal reste
   disponible et **s'ajoute** à la séparation par la durée au lieu de la remplacer.
2. **§5, réception 3, exigeait « bit pour bit, toléré à 10⁻¹² »** pour la continuité avec
   S194. **Invérifiable en l'état** : S194 publie sept chiffres significatifs et son banc est
   un exemple, pas un module importable. La continuité est donc contrôlée à la précision
   publiée, `10⁻⁶` relatif — décisif pour « la construction est inchangée », insuffisant pour
   affirmer l'identité binaire, et cela est dit plutôt que tu.
3. **§2.1 faisait du jeu de phases initial la variable qui décide du régime.** C'est la
   correction qui compte, et elle est développée au §7.3.

### 7.2 Campagne, série A et série B

`Q=24`, `K=64`, `M=3`, `dt=T₁/400`, 10 périodes, `h=8 m`. Écarts rapportés à `A`.

| série | phases | n | max | L2 | croisé | train | N10/N1 | modes | énergie |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| A | alignées | 2 | 2,4694e-2 | 7,2878e-3 | 1,417e-2 | 5,326e-3 | 1,4144 | 2 | 1,5e-9 |
| A | alignées | 3 | 2,1217e-2 | 6,1625e-3 | 8,411e-3 | 3,740e-3 | 1,0906 | 3 | 2,3e-9 |
| A | alignées | 4 | 2,0547e-2 | 5,4356e-3 | 6,812e-3 | 6,000e-3 | 1,0451 | 2 | 3,3e-9 |
| A | alignées | 5 | 1,9610e-2 | 4,8138e-3 | 4,927e-3 | 4,002e-3 | 1,0000 | 3 | 4,5e-9 |
| A | alignées | 6 | 1,9413e-2 | 4,2688e-3 | 2,080e-3 | 3,792e-3 | 1,0000 | 3 | 6,0e-9 |
| A | dispersées | 2 | 2,2980e-2 | 7,3390e-3 | 1,417e-2 | 5,327e-3 | 1,2967 | 2 | 1,5e-9 |
| A | dispersées | 6 | 2,5074e-2 | 4,5070e-3 | 2,081e-3 | 3,856e-3 | 1,0000 | 3 | 6,1e-9 |
| B | alignées | 2 | 1,4786e-2 | 4,6794e-3 | 9,441e-3 | 2,364e-3 | 1,2925 | 2 | 1,5e-9 |
| B | alignées | 6 | 4,4562e-2 | 1,0319e-2 | 4,220e-3 | 8,970e-3 | 1,1038 | 3 | 6,0e-9 |
| B | dispersées | 2 | 1,4050e-2 | 4,7029e-3 | 9,441e-3 | 2,364e-3 | 1,2106 | 2 | 1,5e-9 |
| B | dispersées | 6 | 5,2699e-2 | 1,1068e-2 | 4,323e-3 | 1,001e-2 | 1,0000 | 3 | 6,3e-9 |

*Les `n` intermédiaires des séries A dispersées et B sont dans la sortie du banc ; seuls les
points extrêmes sont repris ici, la monotonie étant vérifiée sur toute la suite.*

**Aucune configuration hors domaine** : la dérive relative d'énergie va de `1,5e-9` à `6,3e-9`,
quatre à cinq ordres sous le `10⁻⁴` exigé.

### 7.3 La réception principale échoue, et pour une raison qui se démontre

| série | fonctionnelle | phases alignées | phases dispersées | cohérent attendu | dispersé attendu |
|---|---|---:|---:|---:|---:|
| A | max | 0,7861 | 1,0911 | 1,6667 | 0,4303 |
| A | L2 | 0,5857 | 0,6141 | 1,6667 | 0,4303 |
| B | max | 3,0138 | 3,7509 | 5,0000 | 1,2910 |
| B | L2 | 2,2052 | 2,3535 | 5,0000 | 1,2910 |

**Réceptions 4, 5 et 6 échouent.** La 4 exigeait `1,667` en alignées et `0,431` en dispersées,
à ±25 %, et un facteur supérieur à 2 entre les deux : les deux jeux de phases ne diffèrent que
d'un facteur **1,39** sur le maximum et **1,05** sur la L2. La 5 exigeait une croissance en
alignées et une décroissance en dispersées : la L2 **décroît dans les deux**, et le maximum ne
suit ni l'une ni l'autre. La 6 exigeait `5,0` et `1,29` à ±30 % : mesuré 3,01 à 3,75.

**Elles échouent par mauvaise spécification, pas par défaut de banc**, et la distinction n'est
pas une consolation : les sept réceptions qui ne dépendent pas de la prémisse de phase passent
toutes, dont la continuité avec S194 à `10⁻⁶`, le cas nul exact, la convergence et la bande.

Le §2.1 tenait que le jeu de phases **initial** décide du régime. Il ne le peut pas, et la
raison est dans le véhicule lui-même : **chaque train avance à sa propre pulsation**. Sur une
fenêtre de dix périodes, les phases relatives balaient toutes leurs valeurs, et un maximum pris
sur `x` **et sur le temps** échantillonne donc les deux régimes quel que soit le départ.
L'alignement initial ne survit pas à la première période.

Ce qui sépare réellement les deux régimes est la **fonctionnelle** :

- le **maximum** sur `x` et `t` tend vers la borne cohérente, puisqu'il finit par trouver
  l'instant où les harmoniques croisées culminent ensemble ;
- la **norme L2** en espace et en temps vaut la racine de la somme des carrés *par
  construction*, les harmoniques croisées vivant sur des nombres d'onde distincts.

La dérivation du §2.1 était donc juste sur les deux bornes et fausse sur la variable qui y
conduit. C'est une erreur de protocole, pas de physique, et elle est conservée telle quelle
au-dessus.

### 7.4 Ce que la loi est réellement

Ajustement en puissance sur `n = 2..6`, moindres carrés en log-log :

| série | fonctionnelle L2 | exposant mesuré | rapport 6/2 |
|---|---|---:|---:|
| A, cambrure totale fixée | alignées | **−0,479** | 0,586 |
| A | dispersées | **−0,437** | 0,614 |
| B, cambrure par train fixée | alignées | **+0,727** | 2,205 |
| B | dispersées | **+0,783** | 2,354 |

La série A décroît en `n^-0,46 ≈ 1/√n` là où le régime dispersé prédisait `≈ 1/n` et le régime
cohérent une constante. La série B croît en `n^0,75`, entre le `√n` dispersé et le `n` cohérent.
**Aucune des deux lois dérivées n'est reproduite** ; la mesure tombe entre elles, des deux côtés.

**Le mécanisme que la dérivation a manqué** est lisible dans les colonnes `croisé` et `train`.
Le §2.4 l'avait entrevu sans en tirer la conséquence : les sommes et différences `kᵢ±kⱼ`
tombent en partie sur des **modes de train**. La dérivation supposait implicitement que tout
l'écart vivait sur des modes propres au couplage ; il n'en vit qu'une part, et cette part
décroît beaucoup plus vite que l'autre. Série A, phases alignées, de `n=2` à `n=6` :

- part sur les modes **exclusivement croisés** : `1,417e-2 → 2,080e-3`, soit **÷6,8** ;
- part sur les modes **de train** : `5,326e-3 → 3,792e-3`, soit **÷1,4**.

Un facteur **4,85** entre les deux vitesses de chute. À `n` croissant, la seconde domine, et
c'est elle qui soutient la loi au-dessus de la prédiction dispersée. Le maximum sature pour la
même raison.

### 7.5 Dilution séculaire, convergence, bande

**Réception 7 — passe, et nettement.** En série A, `écart(N=10)/écart(N=1)` décroît de
`1,4144` à `1,0000` en alignées et de `1,2967` à `1,0000` en dispersées. La valeur `1,0000`
n'est pas un arrondi : à `n ≥ 5` le maximum de la fenêtre est atteint **dès la première
période**, et la croissance séculaire a entièrement disparu. La prédiction du §2.4 — la part
séculaire par train se dilue en `(n−1)/n²` — est donc confirmée dans son sens, et au-delà de
sa lettre.

**Réception 9 — passe.** Trois niveaux de `K`, sur la moyenne quadratique (A238), sous la forme
qu'impose **L274** :

| K | L2 | énergie |
|---:|---:|---:|
| 32 | 5,030690532e-3 | 3,8e-9 |
| 64 | 5,428173150e-3 | 3,3e-9 |
| 128 | 5,545832201e-3 | 3,2e-9 |

**ordre 1,756** (exigé dans `[1,5 ; 2,5]`), **résidu de Richardson 0,892 %** à `K=64` (exigé
sous 2 %).

**Réception 10 — passe.** `Q=32` déplace le maximum de `2,422318e-2` à `2,422319e-2`, soit
**0,0000 %**. La bande est large, et c'était le but : elle vérifie, elle ne converge pas.

## 8. Ce qui est reçu, et ce qui ne l'est pas

**Reçu.**

1. **A240 reçoit sa réponse, et elle est rassurante sans être celle qu'on attendait.** À
   cambrure par train fixée — la crainte d'A240 — l'écart croît en `n^0,75`, **sous-linéaire**.
   Loin du `n²` que le comptage des paires suggérait, et sous le `n` du régime cohérent.
   A240 avait raison sur le comptage, tort sur la conséquence, et la dérivation du protocole
   l'avait déjà dit ; la mesure ajoute que même le régime cohérent est pessimiste.
2. **À cambrure totale fixée, répartir une même mer sur plus de composantes réduit l'écart**,
   en `1/√n`, dans les deux jeux de phases. Ce n'est pas neutre pour le choix d'un nombre de
   composantes : ADR-099 le tranchait sur la dispersion de `Hs` et le coût, sans savoir ce que
   la superposition en pensait.
3. **Le jeu de phases initial ne décide de rien** sur une fenêtre de plusieurs périodes ; la
   **fonctionnelle** décide. Le maximum tend vers la borne cohérente, la L2 vaut la racine de
   la somme des carrés. C'est l'inverse de ce que le protocole déclarait.
4. **L'écart ne vit pas seulement sur des modes propres au couplage.** La part qui retombe sur
   les modes de train décroît 4,85 fois moins vite, et c'est elle qui gouverne le
   comportement à grand `n`.
5. **La croissance séculaire disparaît** quand `n` croît : à `n ≥ 5`, le maximum est atteint
   dès la première période.
6. **Le banc est sain** : cas nul exact, `M=1` exact à `10⁻¹⁴`, continuité avec S194 à
   `10⁻⁶`, convergence d'ordre 1,76 à résidu 0,9 %, bande neutre à `0,0000 %`, énergie
   conservée à `6e-9`, empreinte reproduite.

**Non reçu.**

- **Trois réceptions sur dix ont échoué**, et leur échec ne vaut rien contre le banc : elles
  supposaient que le jeu de phases trancherait, ce que P3a avait réfuté avant la campagne.
  Elles restent écrites au §5, non réécrites, et leur réfutation est au §7.3.
- **Les lois dérivées ne sont pas reproduites.** Les exposants mesurés tombent entre les deux
  bornes, et l'ajustement ne les explique pas : le mécanisme du §7.4 les qualifie, il ne les
  dérive pas. Une dérivation qui tienne compte du repli des modes croisés sur les modes de
  train reste à faire.
- **Six trains ne sont pas un spectre.** La loi mesurée sur `n ≤ 6` est une tendance ; un état
  de mer réel compte des centaines de composantes, et rien n'autorise à extrapoler.
- **Un seul jeu de phases dispersées**, déterministe : la valeur « dispersée » est un point,
  pas une moyenne d'ensemble — ce qui importe d'autant moins que les phases ne tranchent pas.
- **Une dimension horizontale, trains colinéaires, fond plat, eau profonde.** L'obliquité
  reste entière, et S194 a montré que le couplage est 8,6 fois plus fort vers le rivage.
- **Aucun contrat runtime, aucun seuil de bascule W/δ, aucun solveur δ choisi.** `f64`,
  allocations de banc, `water-core` et le support S193 inchangés.

## 9. Suite

**ADR-123 se transporte à `n` sources dans le sens favorable.** Son domaine, établi pour deux
trains, n'est pas dégradé par l'addition de sources à cambrure totale constante — il
s'améliore en `1/√n`. À cambrure par train constante il se dégrade, mais en `n^0,75` et non en
`n²`. **A240 peut être close** sur ce constat, en gardant ses limites : `n ≤ 6`, colinéaire,
eau profonde.

Ce que la session laisse ouvert et qui mérite un porteur : **la dérivation du repli**. Les
harmoniques croisées qui retombent sur des modes de train ne sont pas un détail de banc — elles
gouvernent la loi à grand `n`, et aucune des deux bornes classiques ne les décrit. Sur un
spectre dense, *tous* les modes croisés retombent sur des modes existants ; la question est
donc de savoir si la loi mesurée ici tend vers quelque chose, ou si `n ≤ 6` en donne une image
trompeuse.

**Note S196 — 2026-09-12 : lire « énergie sous `10⁻⁴` » pour ce que c'est.** Ce document
emploie la dérive relative d'énergie comme critère de domaine. S196 a montré qu'un tel critère
**ne détecte pas la sous-résolution** : il a trouvé une configuration fausse d'un facteur cinq
dont l'énergie dérivait soixante-cinq fois sous le seuil (**A242**, **L277**). Les
configurations publiées ici sont loin du bord de résolution et leurs valeurs ne sont pas
remises en cause ; c'est la **phrase** qui ne doit plus être lue comme une garantie de
justesse. Voir REPLI-CROISEES-S196 §8.5.

**Note S197 — 2026-09-12 : audit de résolution, les deux séries tiennent.** Rejouées à
`K = 512`, où l'erreur du symbole de dispersion tombe de `43,6 %` à `0,83 %` : série A
`−0,437 → −0,425`, série B `+0,783 → +0,774`, soit 2,7 % et 1,1 % de déplacement. **La
clôture d'A240 n'est pas affectée.** Voir
[AUDIT-RESOLUTION-S197](AUDIT-RESOLUTION-S197.md) §8.2.
