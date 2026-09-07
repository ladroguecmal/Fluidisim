# C22 — couple d'oracles 76800/153600, S57, 2026-09-07

Exécution de l'action **S56-1**, selon le protocole fixé dans
[REFERENCE-C22-S56](REFERENCE-C22-S56.md). Aucun paramètre de montage, aucun seuil et aucun
critère n'ont été modifiés : le mode `c22-shallow-fin` a été lancé tel quel avec `76800`,
sur les mêmes huit grilles, le même filtre ×30 et le même test de stabilité.

Le but déclaré était de **mesurer une admission**, pas d'obtenir un verdict de convergence.
S56 laissait la question ouverte de façon explicite : l'erreur mesurée sur la grille 12800
(7,710700097e-9) tombait **entre** les deux seuils extrapolés — 6,94346e-9 en n⁻² et
7,77366e-9 avec l'exposant empirique 1,72145 — de sorte que **le choix du modèle
d'extrapolation, et non la mesure, aurait décidé du sort de la grille**.

## Conditions

Release locale, aucune suite de tests en concurrence, champs conservés en mémoire et jamais
sur disque (I-17). `a = 0,01 m`, `sigma = 1 m`, `h0 = 1 m`, `L = 40 m`, `t = 1 s`, `CFL = 0,45`.
Huit grilles 100 → 12800, deux oracles 76800 et 153600 calculés une seule fois et réutilisés
par les quatre fenêtres. Réutiliser les champs ne rend pas les fenêtres indépendantes.

## Coût mesuré

| Poste | Mesure |
|---|---:|
| Oracle 76800 | 165,833 s |
| Oracle 153600 | 672,560 s |
| Huit grilles et mesures | 6,040 s |
| **Campagne** | **844,433 s** *(14 min 04 s)* |

Le budget annoncé par S56 était **827,467 s** : l'écart est de **+2,05 %**. Le rapport des
deux temps d'oracle vaut 4,0557 pour un facteur de taille 2 ; celui du premier oracle au
premier oracle de S56 vaut 2,2802 pour un facteur 1,5 (attendu 2,25, soit +1,3 %). Le coût
quadratique est donc confirmé sur trois tailles, et le poste « reste » est stable à 6,04 s
contre 6,035 s en S56.

## Écart des oracles et seuil

**Écart L1 76800/153600 : 2,709078717e-10**, seuil ×30 : **8,127236151e-9**.

| Couple | Écart L1 mesuré | Exposant local |
|---|---:|---:|
| 25600 / 51200 *(S48)* | 1,717298105e-9 | — |
| 51200 / 102400 *(S49, S56)* | 5,207593646e-10 | 1,72145 |
| 76800 / 153600 *(S57)* | 2,709078717e-10 | **1,61233** |

## Erreurs et admission

| nx | erreur / 76800 | erreur / 153600 | variation | retenue |
|---:|---:|---:|---:|:--:|
| 100 | 7,514859747e-5 | 7,514863831e-5 | 4,083374e-11 | oui |
| 200 | 2,380682552e-5 | 2,380691459e-5 | 8,906595e-11 | oui |
| 400 | 7,306611544e-6 | 7,306732642e-6 | 1,210987e-10 | oui |
| 800 | 1,981993534e-6 | 1,982127485e-6 | 1,339514e-10 | oui |
| 1600 | 5,048208214e-7 | 5,049571320e-7 | 1,363106e-10 | oui |
| 3200 | 1,253146793e-7 | 1,254472015e-7 | 1,325222e-10 | oui |
| 6400 | 3,120105093e-8 | 3,133409146e-8 | 1,330405e-10 | oui |
| **12800** | 7,635780951e-9 | **7,766762184e-9** | 1,309812e-10 | **non** |

La grille 12800 vaut **0,9556 fois** le seuil requis, contre 0,494 fois en S56. **Elle reste
refusée**, et le refus est conservé sans reconstruction de triplet au-delà.

## Ordres et verdict

| Fenêtre | Grilles retenues | Ordres | Verdict |
|---|:--:|---|---|
| 100–1600 | 5/5 | 1,63764980 · 1,63173548 · 1,84983833 | non concluant, p = 1,64, régime asymptotique non atteint |
| 200–3200 | 5/5 | 1,63173548 · 1,84983833 · 1,96062667 | non concluant, p = 1,63, régime asymptotique non atteint |
| 400–6400 | 5/5 | 1,84983833 · 1,96062667 · 2,01167003 | non concluant, p = 1,85, régime asymptotique non atteint |
| **800–12800** | **4/5** | 1,96062667 · 2,01167003 | non concluant, **stabilité non établie**, triplets insuffisants |

Bilan du harnais : **quatre grandeurs, zéro succès, zéro échec, quatre sans verdict** ;
sortie 0. Aucun ordre n'est publié pour un triplet incluant la grille rejetée.

## Vérification

123 tests réussis (38 cœur, 85 harnais), deux ignorés, avant la mesure ; compilation release
réussie ; les deux scénarios `check` restent verts, hashs `0x3e2c06a7b00e73e3` et
`0x1a8b0629a9f51b6e` inchangés. Aucun fichier de code n'a été modifié par cette session :
la campagne est une exécution du binaire construit sur `13851c1`.

## Ce que la mesure tranche, et ce qu'elle retourne

### 1. Les deux extrapolations de S56 étaient fausses, et du même côté

S56 prévoyait un écart d'oracles de **2,3145e-10** en n⁻² et de **2,5912e-10** avec
l'exposant empirique 1,72145. Le mesuré vaut **2,709078717e-10** : les deux modèles
**sous-estiment** la contamination résiduelle, de 14,6 % et 4,6 %. Le seuil réel
(8,127236151e-9) est donc au-dessus des deux seuils annoncés, et la grille 12800 échoue
là où le modèle empirique la donnait admissible de justesse.

L'exposant local de décroissance des écarts d'oracles **continue de baisser** : 1,72145
entre 25600 et 51200, **1,61233** entre 51200 et 76800. Une stratégie qui raffinerait
l'oracle en supposant un exposant constant paye un coût quadratique pour un gain qui se
rétracte à chaque pas.

### 2. Mais le modèle a cessé d'importer — et c'est le résultat utile

Le déficit d'admission n'est que de **4,44 %** (0,9556 fois le seuil). Pour l'effacer il faut
réduire l'écart d'oracles d'un facteur 1,04641, donc porter le premier oracle à :

| Exposant supposé | Premier oracle requis |
|---|---:|
| 2,0 | 78 562 |
| 1,72145 *(S48→S49)* | 78 851 |
| 1,61233 *(S49→S57, mesuré)* | 78 992 |
| 1,5 *(pessimiste)* | 79 158 |

**Les quatre réponses tiennent dans 0,8 %.** En S56 le choix du modèle décidait du verdict ;
ici il ne décide plus de rien. La différence n'est pas dans les modèles — ce sont les mêmes,
et le plus mauvais s'est encore dégradé — mais dans **la distance extrapolée** : S56
extrapolait d'un facteur 1,5 en taille, S57 d'un facteur 1,03.

> **Une extrapolation n'est pas incertaine en soi ; elle l'est en proportion de ce qu'on lui
> fait franchir.** Quand une extrapolation décide d'un verdict, la réponse n'est pas de
> choisir un meilleur modèle, c'est de raccourcir la portée jusqu'à ce que le choix du modèle
> devienne indifférent — quitte à mesurer une fois de plus.

### 3. La contamination réellement subie se mesure, et elle est plus petite que l'indicateur

En comparant les erreurs de S56 et de S57 sur les **mêmes** grilles, contre deux oracles
différents, le déplacement est une **constante additive**, indépendante de la grille dès 800 :

| nx | 100 | 200 | 400 | 800 | 1600 | 3200 | 6400 | 12800 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| déplacement | 1,704e-11 | 3,709e-11 | 5,030e-11 | 5,581e-11 | 5,666e-11 | 5,540e-11 | 5,540e-11 | 5,606e-11 |

Moyenne sur les cinq grilles fines : **5,587e-11**. C'est la différence des biais des oracles
102400 et 153600, mesurée sur ce que le harnais publie réellement. La colonne « variation »
de S57 en donne une seconde, entre 76800 et 153600 : **1,310e-10**.

Deux différences de biais suffisent à ajuster une loi `c(n) = A·n^-q` : **q ≈ 1,879**, d'où
un biais résiduel de l'oracle 153600 d'environ **4,9e-11**, et **1,80e-10** pour l'oracle
76800 — 3,7 fois plus. Sous ce modèle :

- l'erreur de la grille 12800 dépasse le biais de l'oracle qui la mesure d'un facteur **≈ 159**,
  très au-delà des 30 exigés ;
- l'écart L1 entre les deux oracles, qui sert d'indicateur, vaut **5,5 fois** ce biais.

**L'indicateur du filtre n'est pas le biais de l'oracle de mesure : il est dominé par celui du
plus grossier des deux, c'est-à-dire de l'oracle auxiliaire dont aucune erreur publiée ne
dépend.** C'est **A179**.

Rien n'est modifié ici : la loi est ajustée sur deux différences et suppose un biais additif
uniforme, ce que le décrochage des grilles 100 à 400 ne contredit pas mais n'établit pas non
plus. **Le refus de la grille 12800 est maintenu tel quel**, et le filtre ×30 conserve sa
définition. La conséquence est un coût, pas une erreur : le filtre étant conservateur, il
refuse — il n'a jamais admis à tort. Mais il a fait déclarer « sans verdict » trois campagnes
de suite et dépenser vingt-six minutes de calcul pour un déficit final de 4,4 %.

## Budget révisé et suite

Le mode `c22-shallow-fin` borne son premier oracle à 76800 et exige un multiple de 12800 :
la taille requise (~79 000) **n'est pas atteignable sans relever cette borne**, qui est une
limite de campagne sans sens physique. Le multiple suivant est 89600.

| Couple | Coût projeté | Écart projeté | Marge d'admission de 12800 |
|---|---:|---:|---|
| 89600 / 179200 | **1147 s**, ≈ 19 min 07 s | 1,99 à 2,15e-10 | **+20 % à +30 %** |
| 102400 / 204800 | 1497 s, ≈ 24 min 57 s | — | hors campagne bornée |

L'admission de 12800 à 89600 est prévue par les trois exposants, la plus pessimiste des
prévisions gardant encore 20 % de marge. **Cela ne promet pas un verdict** : la fenêtre
800–12800 n'aurait alors que **trois** ordres, et le critère exige trois ordres *puis* le
test de stabilité, que les ordres 1,961 et 2,012 n'ont jamais passé. Une admission
lèverait le refus de contamination, pas l'absence de régime asymptotique.

Le coût projeté dépasse le quart d'heure : le protocole de S56 demande alors de **déclarer
un découpage du calcul en tranches temporelles gardées en mémoire** avant la campagne. Cette
étape est portée par **S57-1** ; elle demande une modification du code (borne du mode) et
doit donc être déclarée, testée et committée séparément de la mesure.

**S56-1 est close** : le couple a été mesuré, le filtre appliqué avant toute stabilité, le
refus conservé, et aucun doublement n'a été lancé automatiquement.
