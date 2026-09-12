# S185 — L'erreur de cadence en 3D, et ce qui est disponible au runtime

2026-09-12. S184-1 / A50. **Mesure locale sur un montage, aucun seuil de justesse adopté.**

[CONSOMMATION-S184](CONSOMMATION-S184.md) a montré que la cadence temporelle est l'axe bon
marché : elle divise le coût **exactement** par `c`, et le contenu temporel de la source est
lent là où son contenu spatial ne l'est pas. Restait la moitié qui décide : **ce que la
cadence coûte en justesse.**

Comme en S183 et S184, les conditions sont publiées **avant** la première exécution (§1–§5),
les relevés viennent après (§6). Les deux sessions précédentes y ont chacune gagné une erreur
rendue visible au lieu d'être réécrite ; la discipline se paie toute seule.

## 1. L'écart précis que cette session ferme

S174 a mesuré la cadence temporelle sur un véhicule 1D, par **interpolation linéaire entre
deux instantanés**. Et il a écrit lui-même ce que cela supposait :

> *« L'échantillon futur utilisé pour interpoler est connu dans ce cas analytique ; le
> protocole ne reçoit pas l'anticipation d'un événement extérieur inconnu. »*
> — [CADENCE-FOND-S174](CADENCE-FOND-S174.md), Conclusions

C'est exactement la question laissée ouverte. Interpoler demande de connaître l'instantané
**suivant** ; un runtime qui reçoit un `WaveEvent` imprévu ne l'a pas. La cadence n'est donc
pas un seul régime mais trois, et un seul d'entre eux a été mesuré :

| mode | ce qu'il exige | disponible au runtime ? |
|---|---|---|
| **maintien** | rien : la dernière source est réemployée telle quelle | oui |
| **extrapolation** | les **deux** dernières reconstructions | oui |
| **interpolation** | la reconstruction **suivante** | seulement en payant une période de latence |

S185 mesure les trois sur le fournisseur réel `B + impacts + pression`, en 3D. Le troisième
n'est pas une proposition : c'est le **plafond** que les deux autres essaient d'atteindre, et
c'est le régime que S174 avait mesuré sans pouvoir dire s'il était accessible.

## 2. Le véhicule, et ce qui change depuis S184

Le bloc de S184, côté 16 — 2744 mailles intérieures, `dx = 0,25 m`, même placement, même
montage d'eau. Le pas est le même : advection centrée, laplacien à sept points, `− S`
soustraite (SPEC-004 §6.1).

**Ce qui change : l'instant avance.** En S184 la source était figée à `T0` — le contrôleur ne
publie qu'un instant, et la mesure ne portait que sur le coût. Ici `t_n = T0 + n·dt`, et
chaque reconstruction exige d'**actualiser le contrôleur de pression** avant de requêter.
Cette actualisation coûte 163–178 µs par instant publié (S183 §6.4) et **ne dépend pas du
nombre de points** : à 2744 mailles elle pèse 0,2 % de la reconstruction, mais elle
dominerait pour un consommateur épars. Elle est comptée.

`dt = 10 ms`, 100 pas, soit **1,0 s** de temps simulé, de `T0 = 1,5 s` à `2,5 s` — dans la
fenêtre de pression `0–8 s` et sous l'horizon d'impact `4 s`.

**Les échelles de temps du contenu**, qui sont ce que la cadence doit résoudre :

| contenu | échelle |
|---|---|
| composantes de `B`, périodes `Tp/2` à `2·Tp` | 3 à 12 s |
| onde d'impact, `λ = 4 m` en eau profonde : `T = √(2πλ/g)` | ≈ 1,6 s |
| mode de pression le plus court, `λ_min = 1,081 m` advecté à 2 m/s | **≈ 0,54 s** |

C'est 0,54 s la contrainte, pas 3 s. Les cadences `c ∈ {1, 2, 4, 8, 16, 32, 64}` couvrent des
maintiens de 10 ms à 640 ms, soit de 1/54 à 1,2 fois cette période : la plage est choisie pour
encadrer la rupture, pas pour la flatter.

## 3. Métriques

État initial `u' = 0`, de sorte que `u'(T)` soit **entièrement** ce que la source a produit et
que l'erreur relative ait un dénominateur qui veuille dire quelque chose. Deux grandeurs :

- **erreur de source** `eS` = `max |S_utilisée − S_réf|` sur toutes les mailles et tous les
  pas, rapportée à `max |S_réf|` ;
- **erreur de champ** `eU` = `max |u'_c(T) − u'_réf(T)|`, rapportée à `max |u'_réf(T)|`.

Les deux sont liées, et le lien est une **vérification fermée** plutôt qu'une remarque : avec
`u'` partant de zéro et l'advection d'ordre supérieur, `u'(T) ≈ −∫S dt`, donc
`Δu'(T) ≈ −∫(S_utilisée − S_réf) dt`. L'écart entre le `Δu'` mesuré et cette intégrale
accumulée est publié ; c'est la forme qu'avait déjà la prédiction de S170 §2.3.

Un **contrôle à état non nul** reprend les mêmes cadences depuis un champ initial déterministe
non trivial, pour que la conclusion ne tienne pas à la trivialité de l'état.

## 4. Réceptions exigées avant tout chiffre

1. **Reproductibilité en bits.** L'erreur mesurée ici est déterministe — contrairement à une
   durée. Deux exécutions du binaire doivent rendre **les mêmes bits**. C'est pourquoi ce
   document ne publie pas deux séries comme S183 et S184 : il publie une série et l'exigence
   qu'elle se reproduise.
2. **La cadence 1 est la référence, littéralement.** Le chemin à `c = 1` doit rendre le champ
   de référence bit pour bit — sans quoi les deux chemins ne diffèrent pas que par le réemploi.
3. **La référence est assez convergée pour servir de référence.** Le même calcul à `dt/2` et
   200 pas doit rendre un champ proche ; l'écart est publié. Sans ce contrôle, « erreur contre
   `c = 1` » ne voudrait rien dire.
4. **Tout reste fini**, source et état, sur toutes les cadences et tous les modes.

## 5. Ce que la mesure ne prouvera pas

- **Aucun seuil.** Rien ici ne dit quelle erreur est acceptable : ce serait une réception
  perceptuelle ou un critère B4, et aucun n'est adopté. On mesure une courbe, pas un choix.
- **Un seul montage, donc une seule échelle de temps.** L'erreur de cadence est gouvernée par
  le contenu ; une autre recette de pression, une autre vitesse de source, une autre mer
  déplacent tout. Le ratio `maintien mesuré / période du contenu` voyagera mieux que les
  valeurs absolues, et c'est sous cette forme que les résultats sont donnés.
- **La référence est `c = 1`, pas une intégrale exacte.** Son propre défaut de quadrature
  n'est pas mesuré — seulement borné par le contrôle 3.
- **Le véhicule ne projette toujours pas** (S184 §3), et ne modélise ni surface libre ni bord.
- **L'interpolation n'est pas proposée.** Elle exige une période de latence que le runtime
  n'a peut-être pas ; elle est mesurée comme plafond.

## 6. Relevés

`cargo run --release --manifest-path code/Cargo.toml -p water-core --example cadence_error`

Bibliothèque inchangée ; workspace **331 réussis / cinq ignorés** en debug et en release.
Bloc 16³, 2744 mailles intérieures, `dt = 10 ms`, 100 pas, 129 instantanés de source
construits. `max |S| = 1,5405e-4 m/s²`, `max |u'(T)|` de référence `= 7,9932e-5 m/s`.

### 6.1 Réceptions — dont une qui a rattrapé un défaut

Les quatre contrôles de §4 passent :

1. **Reproductibilité.** Empreinte `0x39567a1d4bc2ba4c`, identique sur quatre exécutions ; un
   `diff` strict des sorties ne montre **que** les trois lignes de durée du §6.5, qui sont des
   mesures de temps et non des résultats.
2. **Cadence 1.** Les trois modes rendent le champ de référence **bit pour bit**.
3. **Convergence de la référence.** Le même calcul à `dt/2` sur 200 pas s'écarte de
   `3,085e-7 m/s`, soit **0,386 %** de `max |u'(T)|`. C'est un plancher : **rien en dessous de
   ~0,4 % n'est distinguable de la référence elle-même**, et les lignes concernées sont
   signalées plus bas.
4. **Tout est fini**, source et champ, sur tous les modes et toutes les cadences.

> **Ce que la troisième colonne d'erreur a servi à trouver.** L'identité de prédiction —
> « l'écart de champ doit être l'intégrale en temps de l'écart de source » — affichait
> d'abord **100 % d'écart sur toutes les lignes**. Ce n'était pas la physique : `snapshots`
> rangeait la source par indice de bloc quand `load_direct` la lit **compacte**, si bien que
> chaque maille recevait la source d'une autre. Les `eS` étaient justes, les `eU` plausibles,
> et rien dans les tables n'aurait attiré l'œil. Corrigé, l'identité ferme à **0,01–0,5 %**.
> Un contrôle qui ne sert qu'à confirmer ce qu'on croit ne sert à rien ; celui-ci a payé sa
> place du premier coup.

### 6.2 L'erreur, par mode et par cadence

État initial au repos. `τ = c·dt` est la période de maintien ; `T = 0,5405 s` est la plus
courte période du contenu — le mode de pression le plus court, `λ_min = 1,0810 m`, advecté par
sa source à 2 m/s. `eS` et `eU` en pourcentage de `max |S|` et `max |u'(T)|`.

| mode | c | τ (ms) | τ/T | eS % | eU % | écart à la prédiction % |
|---|---:|---:|---:|---:|---:|---:|
| maintien | 2 | 20 | 0,037 | 1,689 | 0,770 † | 0,017 |
| maintien | 4 | 40 | 0,074 | 5,056 | 2,300 | 0,018 |
| maintien | 8 | 80 | 0,148 | 11,406 | 5,055 | 0,018 |
| maintien | 16 | 160 | 0,296 | 23,246 | 10,392 | 0,018 |
| maintien | 32 | 320 | 0,592 | 42,377 | 20,437 | 0,018 |
| maintien | 64 | 640 | 1,184 | 48,413 | 33,212 | 0,019 |
| extrapolation | 2 | 20 | 0,037 | 0,651 | 0,027 ‡ | 0,141 |
| extrapolation | 4 | 40 | 0,074 | 1,982 | 0,169 ‡ | 0,039 |
| extrapolation | 8 | 80 | 0,148 | 4,745 | 0,775 † | 0,020 |
| extrapolation | 16 | 160 | 0,296 | 10,544 | 3,157 | 0,018 |
| extrapolation | 32 | 320 | 0,592 | 25,971 | 11,430 | 0,016 |
| extrapolation | 64 | 640 | 1,184 | 38,201 | 27,320 | 0,022 |
| interpolation | 2 | 20 | 0,037 | 0,027 | 0,006 ‡ | 0,475 |
| interpolation | 4 | 40 | 0,074 | 0,116 | 0,028 ‡ | 0,140 |
| interpolation | 8 | 80 | 0,148 | 0,286 | 0,114 ‡ | 0,046 |
| interpolation | 16 | 160 | 0,296 | 1,090 | 0,457 | 0,018 |
| interpolation | 32 | 320 | 0,592 | 4,230 | 1,830 | 0,011 |
| interpolation | 64 | 640 | 1,184 | 12,359 | 6,774 | 0,010 |

† au plancher de la référence (0,386 %) · ‡ **sous** le plancher : indiscernable de la référence.

### 6.3 La forme qui voyage

Les valeurs absolues ne sortiront pas de ce montage ; les constantes, elles, peuvent. Les
`eU` suivent `τ/T` à une puissance et une constante près, et la colonne calculée par le
programme les donne stables sur toute la plage utile :

| mode | loi | constante mesurée | ordre |
|---|---|---:|---|
| **maintien** | `eU ≈ k · (τ/T)` | 0,31 à 0,35 | premier |
| **extrapolation** | `eU ≈ k · (τ/T)²` | 0,31 à 0,36 | second |
| **interpolation** | `eU ≈ k · (τ/T)²` | 0,046 à 0,052 | second |

Trois lectures, et ce sont elles le résultat de la session.

1. **Le maintien est d'ordre un, les deux autres d'ordre deux.** Doubler la cadence divise
   l'erreur par deux dans un cas, par quatre dans les deux autres.
2. **Maintien et extrapolation partagent la même constante, ~0,35** ; seule la puissance les
   sépare. Le gain de l'extrapolation vaut donc exactement `T/τ` : 3,4× à `τ/T = 0,3`, 13× à
   `τ/T = 0,074`. Il est d'autant plus grand que la cadence est fine.
3. **L'interpolation a la même puissance que l'extrapolation et une constante sept fois plus
   petite.** À erreur égale, elle autorise `√7 ≈ 2,6` fois la période de maintien — et donc
   2,6 fois moins de reconstructions. **C'est le prix exact d'une période de latence.**

Les constantes fléchissent aux deux bouts, et pour deux raisons opposées : à `c = 2` l'erreur
passe sous le plancher de la référence, à `c = 64` la période de maintien dépasse celle du
contenu et le régime asymptotique n'a plus cours (`τ/T = 1,18`). Le plateau utile va de
`τ/T ≈ 0,07` à `≈ 0,6`.

### 6.4 Le contrôle à état initial non nul

Mêmes cadences depuis un champ déterministe non trivial ; `eU` en pourcentage du déplacement
de référence (`1,9467e-4 m/s`) :

| mode | c=2 | c=4 | c=8 | c=16 | c=32 | c=64 |
|---|---:|---:|---:|---:|---:|---:|
| maintien | 0,314 | 0,938 | 2,070 | 4,266 | 8,390 | 13,551 |
| extrapolation | 0,011 | 0,069 | 0,316 | 1,285 | 4,703 | 11,137 |
| interpolation | 0,003 | 0,011 | 0,047 | 0,190 | 0,755 | 2,810 |

Même ordonnancement, mêmes ordres, amplitudes relatives plus faibles parce que le dénominateur
est le déplacement total. **La conclusion ne tient pas à la trivialité de l'état initial.**

### 6.5 Ce que le mode coûte

Par maille et par pas, hors reconstruction :

| mode | ns/maille | instantanés à conserver | mémoire, bloc 16³ |
|---|---:|---:|---:|
| maintien | 0,31 | 1 | 32 ko |
| extrapolation | 2,18 | 2 | 64 ko |
| interpolation | 2,16 | 2 | 64 ko |

**Le mode est gratuit.** 2,2 ns par maille contre 15,3 ns pour le pas et 34 800 ns pour une
reconstruction : passer du maintien à l'extrapolation coûte **0,006 %** d'une reconstruction à
`c = 1`, et 0,4 % à `c = 64`. Le doublement de mémoire, 32 ko par bloc de 2744 mailles, est du
même ordre que les tampons de sortie déjà nécessaires.

### 6.6 Le coût, une fois la cadence choisie

En combinant avec CONSOMMATION-S184 — pas 15,3 ns/maille, source 34,8 µs/maille à `r = 1` et
6,26 µs à `r = 2`, la décimation spatiale étant plafonnée à `r = 2` par le contenu :

| réseau | cadence | source, µs/maille/pas | rapport au pas | eU extrapolation | eU interpolation |
|---|---:|---:|---:|---:|---:|
| `r = 1` | c=16 | 2,18 | 142 | 3,16 % | 0,46 % |
| `r = 1` | c=64 | 0,54 | 36 | 27,3 % | 6,77 % |
| `r = 2` | c=16 | 0,39 | 26 | 3,16 % | 0,46 % |
| `r = 2` | c=32 | 0,20 | 13 | 11,4 % | 1,83 % |
| `r = 2` | c=64 | 0,098 | 6,4 | 27,3 % | 6,77 % |

*L'erreur de la décimation spatiale n'est pas incluse : elle n'est mesurée qu'en 1D (S170), et
les deux erreurs ne s'additionnent pas nécessairement. Les colonnes d'erreur sont donc des
minorants pour les lignes `r = 2`.*

Le rapport de 2 274 mesuré en S184 à `r = 1`, `c = 1` descend à **6,4** en bas de ce tableau.
La cadence et le réseau, ensemble, retirent près de trois ordres de grandeur — et le coût
restant est d'un ordre qu'une projection de pression rendrait comparable au pas.

## 7. Ce qui est reçu, et ce qui ne l'est pas

**Reçu.**

1. **Trois lois d'ordre, et leurs constantes.** Maintien `0,35·(τ/T)`, extrapolation
   `0,35·(τ/T)²`, interpolation `0,05·(τ/T)²`, stables de `τ/T = 0,07` à `0,6`.
2. **Le maintien n'est jamais le bon choix.** L'extrapolation est causale, coûte 1,9 ns de
   plus par maille et 32 ko de plus par bloc, et gagne un facteur `T/τ` — de 3 à 13 sur la
   plage utile. Il n'y a pas d'arbitrage à faire : c'est un gain sans contrepartie mesurable.
3. **Ce que vaut une période de latence : `√7 ≈ 2,6` sur la cadence**, à erreur égale. C'est
   la question que S174 ne pouvait pas poser, puisqu'il n'avait mesuré que le régime interpolé.
   Elle a maintenant un prix, et il se compare à d'autres coûts de latence.
4. **L'identité de prédiction ferme à 0,01–0,5 %** : l'erreur de champ *est* l'intégrale en
   temps de l'erreur de source, l'advection restant d'ordre supérieur sur ce montage.
5. **La référence est qualifiée** : son propre défaut vaut 0,386 %, et les lignes qui passent
   dessous sont marquées plutôt que lues comme des victoires.

**Non reçu.**

- **Aucun seuil de justesse.** Savoir si 3 % d'erreur de champ perturbatif est acceptable est
  une question perceptuelle ou un critère B4 ; aucun n'est adopté, et rien ici n'en tient lieu.
- **Un seul montage, donc un seul `T`.** Les constantes sont données sous forme transportable
  pour cette raison, mais elles n'ont été vérifiées que sur ce contenu. Une mer différente,
  une recette de pression différente, une source plus rapide les remettraient en jeu.
- **L'erreur spatiale n'est pas composée avec l'erreur temporelle.** S170 a mesuré la première
  en 1D ; leur somme n'est ni mesurée ni supposée additive.
- **Le véhicule ne projette toujours pas** (S184 §3), et l'advection y est d'ordre supérieur —
  c'est ce qui rend l'identité de prédiction si nette, et c'est aussi une limite : un régime
  où l'advection compte n'est pas couvert.
- **L'interpolation reste conditionnée à une latence** que le runtime n'a peut-être pas. Elle
  est mesurée comme plafond, pas proposée comme conception.

## 8. Suite

**A50 change encore de nature.** Elle n'attend plus ni un chiffre ni une décision d'ordre de
grandeur : les deux axes sont mesurés en temps *et* en justesse, et le couple
« extrapolation + `r = 2` + `τ ≈ 0,3·T` » place la source à quelques dizaines de fois le pas
avec une erreur de quelques pour cent. Ce qui manque est ce qui a toujours manqué : **un
critère qui dise si « quelques pour cent » suffit**, et un solveur à alimenter.

**S185-1 — composer les deux erreurs.** L'erreur spatiale n'est connue qu'en 1D et l'erreur
temporelle vient d'être mesurée en 3D séparément. Les mesurer ensemble, sur le même véhicule,
est le dernier contrôle avant qu'un budget conjoint ait un sens — et S170 avertit déjà qu'un
ratio de décimation ne décrit pas à lui seul la précision, ce qui rend l'addition douteuse.

Voir **A228** pour ce que l'ordre du maintien implique, et **L265** pour ce que cette session
doit à un contrôle qu'elle avait failli ne pas écrire.

Aucun ADR : rien n'a changé de contrat, et le choix du solveur reste à B3 (ADR-007 §5).
Aucun arbitrage humain nouveau.
