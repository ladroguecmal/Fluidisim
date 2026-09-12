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
