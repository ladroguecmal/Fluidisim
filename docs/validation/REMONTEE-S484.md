# La remontée d'une grosse bulle — S484 (K2-3, premier temps)

*S484, 2026-10-05, en autonomie. Campagne K2 ([conception](../registres/CAMPAGNE-K2-S478.md)) ; la référence APIC 3D avec poches
(S479, S481) sur 16 fils (S483).* **Critère principal manqué** : la bulle monte à 56 % de la vitesse de Davies et Taylor.

## Reproduire

- `FILS=16 PAS_US=1000 [TRACE=1] code/target/release/examples/apic3d_remontee.exe <R/dx> 0.04 <durée_s>` — un quart de cuve (demi-largeur
  8 R, 22 R d'eau), la bulle centrée sur le coin à 2,5 R du fond ; la droite des moindres carrés de `z(t)` sur `z ≥ z0 + 2R`.
- Les sorties : `calculs/20261005-204739-remontee-r4-1ms`, `calculs/20261005-205939-remontee-r6-1ms` ([CALCULS](../../notes/CALCULS.md)).

## 1. La référence

**Davies et Taylor (1950)**, bulle en calotte sphérique : `U = 0,711·√(g·d_e)` (Clift, Grace et Weber 1978), pour `Eo = ρ·g·d_e²/σ > 40`.
Bulle de R = 4 cm : `d_e` = 8 cm, `Eo` ≈ 870 — la tension de surface, absente du modèle, n'y joue pas ; `U` = 0,630 m/s (0,603 pour le
diamètre mesuré, 7,3 cm : la reconstruction donne un volume un peu plus petit que la sphère). Paroi : `d_e/D` = 0,125, sous le seuil de
Collins (1967).

## 2. Ce qui a été mesuré

| | R/dx = 4 (≈ 70 mailles d'air au quart) | R/dx = 6 (≈ 230) |
|---|---|---|
| départ | monte, ≈ 0,5 m/s jusqu'à 0,13 s | monte |
| suite | cale vers 16 cm, s'étale, perd du volume (2,46 → 1,35·10⁻⁴ m³ en 40 ms) ; résorbée sous huit mailles à 0,176 s | **monte à vitesse constante** : **U = 0,338 m/s** (0,336 puis 0,327 sur chaque moitié), 0,185 à 0,396 s |
| rapport à Davies–Taylor | — | **0,56** |

Masse exacte à chaque pas. La bulle s'étale à mesure qu'elle monte : le centre du quart passe de 3,7 à 5,9 cm des axes (une calotte qui
s'aplatit). Au pas stable (≈ 4 ms) à R/dx = 4, elle calait plus tôt : le pas compte aussi.

**Trouvé en route** : le centre d'une poche était divisé par le volume entier, mailles d'eau voisines comprises — tiré vers l'origine d'un
facteur ≈ 0,8 depuis S479 ; la « remontée » de [POCHES-AIR-S479](POCHES-AIR-S479.md) en était faussée. Corrigé dans la référence et la carte
(la part d'air des seules mailles d'air ; carte et référence à 10⁻⁸ m).

## 3. Les critères, écrits avant

| critère | mesure | |
|---|---|---|
| (1) U à 15 % de Davies–Taylor à R/dx = 6 | 0,338 contre 0,603 m/s (× 0,56) | **manqué** |
| (2) l'écart entre R/dx = 4 et 6 | R/dx = 4 ne va pas au bout (la bulle se résorbe) : la convergence n'est pas mesurable à deux résolutions | non mesuré |
| (3) remontée selon `−g`, dérive latérale sous 0,1 R ; masse exacte | masse exacte ; le critère latéral est **mal posé** — une calotte qui s'aplatit écarte le centre du quart des axes sans dériver | mal posé |

## 4. Les causes possibles, à départager (K2-3b)

1. **La résolution** : à R/dx = 4 la bulle ne survit pas ; à 6 elle est lente. Il faut 8 et 12 — sur la carte, qui suit la référence à
   10⁻⁴ (S481) et est cent fois plus rapide.
2. **Le quart de cuve** : la calotte s'étale le long des deux parois du coin ; une cuve entière à la même résolution le dit.
3. **La diffusion numérique d'APIC** à faible résolution (un sillage trop visqueux freine la bulle).
4. **Le pas** : 1 ms contre ≈ 4 ms changeaient déjà le sort de la bulle à R/dx = 4.

La liste **7.4** reste partielle ; K2-3 n'est pas reçue.
