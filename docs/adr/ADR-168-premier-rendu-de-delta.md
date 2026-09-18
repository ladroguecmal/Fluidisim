# ADR-168 — Premier rendu de δ : bande couplée rejouée dans l'afficheur

Actée S275, 2026-09-18, autonomie S71. Premier pas de la liste 8.7 (« rendu de δ raccordé à
B+W ») et de J2 (« raccordement au rendu »). Ne change ni le cœur, ni B, ni W.

## Constat

Aucune précision de δ ne peut être jugée à l'œil : `viewer/` ne consomme aucun domaine δ
(S274). δ est encore en x-z ; il exige des échantillons de fond plans.

## Décision

1. **Scène `--delta`** : B = houle **à crêtes longues** vers +x (JONSWAP, `Hs` 2 m, `Tp` 8 s,
   γ 3,3, `[0,7 ; 1,6] fp`, 32 composantes, étalement nul, graine 275), ce qui rend les
   échantillons exactement plans. Impacts et sillages absents ; queue spectrale S256 conservée
   comme habillage de rugosité, non couplée à δ.
2. **Domaine** : x ∈ [−128, 128] m, dx = 2 m (128 colonnes), profondeur 96 m sous le repos et
   8 m au-dessus (52 couches), fond solide plat. Éponge ADR-164 de 32 m à chaque bout, taux
   0,5 s⁻¹ : **valeurs de scénario, pas calibrées**. Fond échantillonné par
   `differential_local_extended` à chaque face, à chaque pas.
3. **Deux rejeux précalculés** sur 30 s, avant l'affichage, **hors budget** : pas de référence
   4 ms et **pas d'image** 16 ms (une image par pas à 62,5 Hz). Aucune revendication de temps réel.
4. **Rendu** : `η'` ajouté à B sur la bande y ∈ [−100, 100] m, extrudé le long des crêtes, lu
   par Hermite en x avec sa pente. Fondus en cosinus sur les 32 m d'éponge en x et sur 30 m en y :
   **artifices de rendu**, pas une frontière physique (la frontière δ↔B reste la liste 4.7).
   Une touche fait défiler B seul, B+δ (4 ms), B+δ (16 ms).

## Ce qui n'est pas revendiqué

Temps réel ; 3D (la bande répète la tranche le long des crêtes) ; frontière du total au fond
(`W(fond) ≠ 0`, ≤ 5 % de l'amplitude de la plus longue composante à 96 m) ; bords réels ; dérive
de phase longue (A289) — visible ou non, elle n'est pas corrigée.

[Protocole et résultats](../validation/DELTA-VISIBLE-S275.md).
