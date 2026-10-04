# L'air enfermé sur la carte — S481 (K2-2)

*S481, 2026-10-04, en autonomie (ADR-215). Campagne K2 ([conception](../registres/CAMPAGNE-K2-S478.md), [ADR-220](../adr/ADR-220-la-campagne-k2.md)
D1) ; la référence : [POCHES-AIR-S479](POCHES-AIR-S479.md).* Session coupée par une pause demandée par l'utilisateur après P2, reprise
le soir même.

## Reproduire

- Le banc : `water-viewer --apic3d-poches` — `CAS=bulle` (la bulle de S479, R = 8 cm, dx = 2 cm) ou `CAS=plusieurs` (trois bulles,
  dont une d'une maille, et `CHEMINEE=1` : une quatrième reliée à la surface) ; `CHAUFFE=<pas>` : des pas sans poches d'abord (l'eau
  a une pression quand elles naissent) ; `PAS=<n>`. Pour chaque pas, la référence et la carte partent du même état chargé et vont
  jusqu'aux poches : étiquettes, poche de chaque maille, volume, pression, centre, mailles.
- `MODE=suivi DUREE=0.15` : la référence et la carte avancent chacune de son côté ; la première poche pas à pas, les fréquences.
- `APIC3D_POCHES=1 water-viewer --v1-banc` (60 s ; `V1_POCHES_TRACE=<t>` : chaque poche à chaque pas après `t`), et sans la variable,
  le témoin. `APIC3D_POCHES=1 ND=16 water-viewer --apic3d-carte-b10` : B10 avec poches des deux côtés.
- Les sorties sont dans `calculs/` ([CALCULS](../../notes/CALCULS.md)).

## 1. La construction

`viewer/src/apic3d_poches.{rs,wgsl}`, **à côté** du pas de la carte : un module compilé à part sur la même disposition de liaisons (la
structure `Params` prise au texte du nuanceur principal), deux tampons réservés à la configuration (`pko` entiers, `pkf` flottants) ;
ses pipelines n'existent qu'après `ApicCarte::enable_air_pockets`. Sans cet appel, le pas d'avant n'est pas touché.

- **La détection** : une union-find sans verrou sur les mailles d'air — l'identifiant d'une maille est `c + 1`, l'air libre la racine
  0 (les mailles d'air de la rangée du haut) ; l'accrochage va toujours de la plus grande racine vers la plus petite (`atomicMin`).
  La racine d'une composante enfermée est donc sa plus petite maille, et les poches se numérotent dans l'ordre de leur plus petite
  maille (un groupe, un préfixe par tranches) — **l'ordre où la référence les remplit**. Le résultat ne dépend pas de l'ordre des fils.
- **Le bilan par poche** : deux listes dans l'ordre des mailles (les mailles d'air de chaque poche ; les mailles d'eau qui la bordent,
  une fois par poche) ; **un groupe par poche** somme dans un ordre fixe son volume, son centre, ses mailles, la pression de l'eau qui
  la borde ; les recouvrements avec les poches d'avant sont des compteurs entiers ; un fil fait l'héritage, la naissance, le rappel du
  volume, la pression et la résorption. Aucun atomique flottant.
- **La projection** : le gradient conjugué diagonal de la carte, une inconnue par poche ; `A·d` des poches par une réduction par
  poche sur la liste des faces eau | poche, à chaque itération. `assemble`, `cg_init_reduce`, `cg_update`, `cg_direction` du pas
  d'avant sont repris ; avec poches, la multigrille est éteinte (elle ne les voit pas encore, §4).
- FXC (DirectX 12) refuse l'écriture indexée dans un tableau local de structure : la liste des poches d'une maille d'eau se lit sans
  tableau (une voisine compte si aucune précédente ne porte la même poche).

## 2. Les critères, écrits avant

| critère | mesure | |
|---|---|---|
| (1) sans poches, au bit | le module est à part, ses pipelines n'existent qu'après `enable_air_pockets` ; sans eux, le pas enregistre les mêmes noyaux dans le même ordre (le seul ajout, `encode_pockets_detect`, ne fait rien) ; le témoin `--v1` : 60 s, masse exacte, pas médian 16,7 ms. Non vérifié au bit contre un binaire d'avant | tenu par construction |
| (2) la poche de chaque maille identique à la référence | **0 maille différente à étiquettes égales** (la bulle 8 pas, trois bulles avec et sans cheminée, sous pression) ; les seuls écarts (4 mailles, bulle, un pas) suivent 4 étiquettes que la reconstruction de la carte donne déjà autrement | tenu |
| (3) la bulle : volume et pression à 1 % pas à pas sur 0,15 s, fréquence à 2 %, masse exacte | 300 pas : volume à **2,9·10⁻⁵**, pression à **4,1·10⁻⁵** ; **42,47 Hz contre 42,50** (7·10⁻⁴) ; 151 itérations au plus, toutes convergées ; masse exacte | tenu |
| (4) B10 à 16 mailles avec poches sur la carte au bout, la bulle à 5 % de la référence | au bout ; pincement **2,0843 √(R/g)** des deux côtés ; la bulle : **0,0709 contre 0,0700 D³** au plus (1,2 %), **0,0515 contre 0,0511** à la fin (0,75 %) | tenu |
| (5) `--v1` stable 60 s avec poches, masse exacte, coût mesuré | 60 s stables, masse exacte au quantum, une poche (1,87·10⁻³ m³ ≈ 15 dx³, 107 kPa) — après la règle des 8 mailles (§2 bis) ; coût §4 | tenu, au minimum |

Le banc du même état (P3), avant la projection : nombre de poches identique partout ; volume à ≤ 6·10⁻⁶, pression à ≤ 7·10⁻⁶ — la
naissance sous pression (après quatre pas de chauffe), la résorption d'une bulle d'une maille, la cheminée vers l'air libre.

### 2 bis. Les poches d'une maille — un défaut de la référence trouvé par la scène

Sur `--v1` (dx = 5 cm), **le calcul s'emballe vers t = 8,33 s** : le pas stable tombe à 1 ms, puis la carte est perdue. Tracé pas à
pas : une quinzaine de poches d'**une ou deux mailles**, nées dans l'eau brassée sous la cavité ; leur volume saute à chaque changement
d'étiquette et leur pression bondit (jusqu'à 283 kPa). La résorption de S479 (`V < dx³`) ne les attrapait pas : leur volume compte la
part d'air des mailles d'eau voisines (≈ 2,4 dx³ pour une maille). **Tranché** (ADR-215 D2) : `POCHE_MAILLES_MIN` = 8 — une poche de
moins de huit mailles d'air (un cube de 2 × 2 × 2) se résorbe, dans la référence et sur la carte ; sa raideur n'a pas de sens à cette
échelle (les microbulles, K2-7). Les 43 essais d'APIC 3D passent ; la bulle de S479 (232 mailles) n'est pas concernée.

## 3. B10 avec poches

`APIC3D_POCHES=1 ND=16 --apic3d-carte-b10` (quart de domaine, 32 × 32 × 168, 1 048 576 particules ; `calculs/20261004-235007-s481-b10-p16-poches-3`,
17 min) : la référence et la carte vont au bout (4 √(D/g)). Pincement au pas 100, **2,0843 √(R/g)** des deux côtés, profondeur 1,094 D,
cavité 1,906 D, couronne 0,305 contre 0,304 D. Une poche de chaque côté ; son volume (quart de domaine) **0,0700 D³ au plus** pour la
référence, **0,0709** pour la carte, **0,0511** et **0,0515** à la fin. Le quadruple, 0,280 et 0,204 D³, retrouve S479 (0,283 et 0,231 avec
`APIC3D_APRES` = 1,5).

Le premier lancement est mort à 23:46 avec B10 à 24 mailles (lancé en S480, 7 h de calcul), code 0xC000013A : les deux restaient dans
l'objet de tâche de la session. `calcul.py` lance désormais par WMI, hors de la session ([ADR-222](../adr/ADR-222-la-methode-se-revise-elle-meme.md) D1).

## 4. Le coût — un dépassement, inscrit (ADR-131)

Sur `--v1`, même binaire : **pas médian 71,4 ms avec poches contre 16,7 ms sans** (p99 86,7 contre 22,6) — la scène passe de 0,99 à
≈ 0,27 temps réel. Étage par étage (`V1_LENTS`) : la reconstruction gagne ≈ 11 ms (la détection et les listes sont parcourues par
**un seul groupe** sur 371 000 mailles), la projection ≈ 40 ms (**la diagonale** à la place de la multigrille). Techniques absentes,
dans l'ordre de leur gain : (1) les poches dans la multigrille — un préconditionneur par blocs, le cycle en V sur les mailles et la
diagonale sur les poches (défini positif, la même construction qu'ici) ; (2) ne rien faire quand aucun air n'est enfermé (un
dispatch indirect dont la taille vient de la détection) ; (3) les parcours à plusieurs groupes (préfixes hiérarchiques, comme le tri
par maille). Elles sont K2-2b.

## 5. Ce qui reste

- **K2-2b** : le coût (§4) ; la carte de retour au temps réel avec poches.
- La grande cuve de S479 (47,8 Hz contre 42,2 de Minnaert, × 1,13, quand la petite donnait × 1,02) : à attribuer.
- B10 à `D/dx` = 24 (référence) : mort avec la session (§3) ; à relancer par `calcul.py`, désormais hors de la session.
