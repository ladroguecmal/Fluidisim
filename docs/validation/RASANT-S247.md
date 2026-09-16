# Ce que l'hôte tient à incidence rasante — S247, 2026-09-16

Dernier reliquat de **J1-bis** qu'une session puisse traiter seule : l'interaction manuelle
représentative demande une personne, la seconde cible une autre machine (REPRISE §5).
[CADENCE-HOTE-S225](CADENCE-HOTE-S225.md) §Suite l'écrit depuis S225 parmi ce que ses mesures ne
couvrent pas — « les angles rasants soutenus » — et rien ne l'a fait depuis.

## 1. Protocole, écrit avant mesure

### 1.1 Ce qui est déjà su, et ce qui ne l'est pas

[LOD-SILLAGE-S234](LOD-SILLAGE-S234.md) §1 a mesuré la **charge de maillage** de la pose rasante —
œil à 2 m, tangage −0,05 : **95,9 % de l'écran dans l'emprise** contre 86,2 % à la pose de référence,
pour une charge idéale de 0,359. Mais cette pose n'a **jamais été rendue** : ni coût par image, ni
qualité. Les bancs de S225, S235, S240 et S242 tiennent tous la caméra de référence ou la tournent
(`--sweep`), jamais l'incidence rasante.

### 1.2 Ce que l'incidence change, et ce qu'elle ne change pas

- **Le coût** : presque tout l'écran est de l'eau, et la **visibilité de S235 n'y retire rien** —
  elle ne gagne que sur ce qui sort de l'emprise.
- **L'échantillonnage** : à l'horizon, deux sommets voisins sont séparés par des dizaines de mètres
  d'eau. S234 notait déjà que **7,5 %** des sommets de l'emprise sous-échantillonnent `λ_min` à la
  pose de référence ; personne n'a chiffré ce que devient cette part à incidence rasante.
- **La grille locale du sillage**, elle, **ne change pas** : son emprise est le domaine du sillage,
  pas la caméra. Constaté en S241, et c'est ce qui écarte d'emblée le scénario « la pose rasante
  sature la capacité de la grille ».

### 1.3 Thèse, et ce qui la réfuterait

Le nombre de sommets est fixé par la grille, pas par la pose : **l'incidence rasante ne devrait pas
casser le coût**, seulement déplacer où les sommets tombent. En revanche elle **dégrade
l'échantillonnage**, dans une proportion que nul n'a mesurée.

**Ce qui la réfuterait** : que le coût explose — par exemple parce que la profondeur ou la borne
d'horizon change le travail par sommet — ou que l'échantillonnage tienne. Dans les deux cas la
session le dira.

### 1.4 L'étalon, et pourquoi il vient avant la mesure

L'instrument d'échantillonnage sera **d'abord passé à la pose de référence**, où il doit retrouver
les **7,5 %** de S234. Un instrument qui ne reproduit pas un chiffre déjà publié est faux avant
d'avoir servi, et c'est l'ordre qui l'empêche de mentir : on l'étalonne, puis on l'emploie.

`λ_min = 2π / coupure = 2,094 m` pour la recette du sillage. Le fond `B` a sa propre coupure, que ce
lot **ne mesure pas** : limite déclarée, pas oubli.

### 1.5 Critères de réception, déclarés avant mesure

1. **Coût à incidence rasante soutenue** : CPU, GPU et intervalle, même banc de cadence que S240 et
   S242, contre la pose de référence.
2. **Échantillonnage** : écart entre sommets voisins dans l'emprise, et **part des sommets sous
   Nyquist** — étalonnée d'abord (§1.4).
3. **Allocations de `update` toujours nulles** (ADR-145).
4. **Aucune tolérance modifiée, aucun seuil inventé.** La session **mesure** ; elle ne corrige que si
   la mesure désigne un défaut clair et borné.

### 1.6 Arrêt

Coût et échantillonnage publiés, avec ce qui tient et ce qui ne tient pas. **Ou**, si la mesure
désigne un défaut dont la correction dépasse la session, ce défaut chiffré et mis en file avec son
déclencheur.

## 2. L'étalonnage, passé avant la mesure

Ligne `NYQUIST_S247` ajoutée à `--lod-charge` : dans l'emprise du sillage, part des sommets dont
l'écart au voisin dépasse `λ_min/2 = 1,047 m`, et pire écart.

À la pose de référence, l'instrument rend **7,54 %**. S234 publiait **7,5 %**, deux sessions avant
qu'il existe. Il peut servir.

## 3. L'échantillonnage, aux six poses

| pose | sommets dans l'emprise | sous Nyquist | pire écart |
|---|---:|---:|---:|
| S212, référence | 111 715 | **7,54 %** | 2,589 m |
| balayage, lacet 0,8 | 101 938 | 3,52 % | 2,488 m |
| balayage, lacet 1,6 | 111 073 | 5,15 % | 2,553 m |
| haute, 30 m | 78 217 | **0,01 %** | 1,050 m |
| **rasante, 2 m** | 124 274 | **7,59 %** | **8,243 m** |
| hors emprise | 0 | — | — |

**Ce n'est pas la part qui bouge, c'est la profondeur.** À incidence rasante, la fraction de sommets
sous Nyquist est celle de la pose de référence — 7,59 % contre 7,54 % — mais le **pire écart triple**,
de 2,589 à **8,243 m**. Là où la référence saute une demi-longueur d'onde entre deux échantillons, la
pose rasante en saute près de **quatre**.

La pose haute, elle, est propre : **0,01 %** et un pire écart de 1,05 m, juste sous le critère. Elle
regarde vers le bas, et sa projection est bien conditionnée partout.

## 4. Le coût, à incidence rasante soutenue

`--multi --cadence`, scène S235, 960 × 540, 590 images, secteur.

| grandeur | pose de référence | **pose rasante** |
|---|---:|---:|
| intervalle médian | 4,7729 ms | **4,5826 ms** |
| CPU médian | 4,0563 ms | **3,9021 ms** |
| CPU maximum | 13,74 ms | 10,82 ms |
| **GPU eau médian** | **0,4397 ms** | **0,4397 ms** |
| allocations de `update` | 0 | **0** |

**Le coût tient, et la thèse du §1.3 est confirmée** : le nombre de sommets est fixé par la grille,
pas par la pose. Le GPU eau est **identique à la quatrième décimale**, et le CPU est même un peu plus
bas. La visibilité de S235 ne gagne rien à incidence rasante — 95,9 % de l'écran est de l'eau — mais
elle n'y perd rien non plus.

## 5. Ce qui tient, ce qui ne tient pas, et ce que la mesure spécifie

**Tient** : le coût par image, à toutes les poses mesurées, y compris la plus chargée. Les
allocations restent nulles (ADR-145). La grille locale du sillage ne dépend pas de la caméra, ce qui
écarte définitivement le scénario « la pose rasante sature sa capacité ».

**Ne tient pas** : l'échantillonnage du champ lointain. Et le chiffre dit **pourquoi la solution
n'est pas un maillage plus dense**. Un écart de 8,2 m au loin correspond aux **deux pixels** de la
grille projetée : à cette distance, une onde de 2 m est **plus petite qu'un pixel**. Densifier le
maillage ne la rendrait pas visible ; cela coûterait des sommets pour dessiner ce que l'écran ne peut
pas montrer.

Le remède est celui que la feuille de route nomme déjà — **couper les modes par la distance**, c'est-
à-dire ne pas déposer dans l'image une énergie que l'échantillonnage ne peut pas porter. Cette mesure
en donne la **spécification chiffrée** : à la pose rasante, au plus loin, les modes de longueur
d'onde inférieure à **≈ 16 m** ne sont pas résolubles.

Ce n'est pas un lot de cette session. Il change le champ publié, donc les bits, et sa réception
demande de repenser ce que `VERIFY` compare — un champ coupé par la distance **doit** s'écarter du
cœur au loin, et l'écart mesuré aujourd'hui deviendrait le résultat attendu plutôt que le défaut.

### 5.1 Ce qui n'est pas mesuré

- **La coupure du fond `B`** : ce lot ne mesure que `λ_min` du sillage. B a la sienne, et la part de
  sommets sous son propre Nyquist n'est pas chiffrée.
- **L'impression visuelle** : aucune image n'est jugée ici ; la mesure est géométrique.
- Un seul format (960 × 540), une machine, une scène.
