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
