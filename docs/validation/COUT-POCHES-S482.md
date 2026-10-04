# Le coût des poches — S482 (K2-2b)

*S482, 2026-10-05, en autonomie (ADR-215, ADR-222).* Le dépassement inscrit en S481 ([POCHES-CARTE-S481](POCHES-CARTE-S481.md) §4) :
**71 ms par pas avec poches contre 16,7 sans**, sur `--v1`. Levé.

## Reproduire

- `APIC3D_POCHES=1 DUREE=60 water-viewer --v1-banc`, et sans la variable, le témoin (même binaire) ; `V1_LENTS=0` : les étages de chaque pas.
- `MODE=cout [CAS=plusieurs] [SANS_POCHES=1] water-viewer --apic3d-poches` : la carte seule, 60 pas depuis la bulle de S479, étages horodatés.
- `MODE=suivi DUREE=0.15 water-viewer --apic3d-poches` (la bulle carte contre référence ; `MULTIGRILLE=0` : la diagonale de S481) ;
  `CAS=… PAS=…` : la détection contre la référence. Sorties : `calculs/` ([CALCULS](../../notes/CALCULS.md)).

## 1. Trois remèdes, dans l'ordre de leur effet mesuré

| étape | ce qui change | effet |
|---|---|---|
| P2 | l'aplatissement compte les racines enfermées ; sans elles, la numérotation et les listes sortent aussitôt | `--v1` : reconstruction de ≈ 12 à 2,7 ms |
| P4 | **les poches dans la multigrille** : un préconditionneur par blocs — le cycle en V sur les mailles (l'air des poches y reste une condition nulle), la diagonale sur les poches ; symétrique défini positif. Les noyaux fusionnés de S424 ont leur variante (mêmes produits repliés dans chaque groupe, au bit ; le groupe 0 met à jour les poches) | la bulle : **14 itérations au lieu de 151** ; `--v1` : 14,9 ms sur 6 s |
| P3 | la numérotation et les listes par **compaction à plusieurs groupes** (compte par bloc de 256 mailles, préfixe des blocs, écriture) au lieu d'un seul groupe — le même ordre des mailles, donc les mêmes listes | carte seule, une poche présente : reconstruction **+0,2 ms** au lieu de +1,7 à 2 (60 000 mailles) |

FXC (DirectX 12) refuse un `switch` dont chaque branche retourne : la fonction de compte est écrite en `if`.

## 2. Les critères, écrits avant

| critère | mesure | |
|---|---|---|
| (1) sans poches, le témoin inchangé | `--v1` témoin : 3 118 pas, 9 281 particules à 60 s, **la trajectoire de S481 au pas près** ; masse exacte ; pas médian 15,3 ms (16,7 en S481 : le bruit de l'horloge de la carte) | tenu |
| (2) la bulle à 10⁻⁴ de la référence, fréquence à 10⁻³ | 300 pas : volume **2,9·10⁻⁵**, pression **4,1·10⁻⁵**, **42,47 Hz contre 42,50** (6,9·10⁻⁴) ; 14 itérations au plus ; masse exacte | tenu |
| (3) `--v1` avec poches à 20 % du témoin, 60 s stables, masse exacte | **15,55 ms contre 15,33** (+1,4 %) ; p99 31,3 contre 28,3 ; 60 s, masse exacte au quantum | tenu |
| (4) la détection identique | 0 écart de poche à étiquettes égales ; volumes et pressions du même état au chiffre près d'avant (bulle, trois bulles, cheminée, naissance sous pression) | tenu |

**Limite.** Dans ces 60 s, la scène `--v1` ne fait **aucune poche d'au moins huit mailles** (en S481, sous la diagonale, elle en avait fait
une) : sa trajectoire avec poches est celle du témoin. Le coût *poche présente* se mesure donc sur la carte seule (`MODE=cout`) : la
reconstruction +0,2 ms ; la projection suit la physique (la bulle vit ou s'effondre), non un surcoût fixe. Une scène à maille plus fine,
qui fasse des poches résolues, est la suite (ADR-222 D6).

## 3. Ce qui en découle

- **A311** (l'air n'est pas modélisé) : le remède est en production sur la carte, au temps réel — close (note au registre).
- Liste **7.4** : reste partielle — la vitesse terminale, la fragmentation, C13 (K2-3 et suivantes).
