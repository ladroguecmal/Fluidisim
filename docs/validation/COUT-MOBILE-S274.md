# Coût du pas couplé mobile : multigrille — S274

Réception d'[ADR-167](../adr/ADR-167-multigrille-du-mode-mobile.md). Critères écrits avant le code.

## Référence avant construction

`delta_mobile couple_cas couple 0.05 128`, `DELTA_MOBILE_PAS=200`, sur secteur, un fil, sans autre
charge, deux passages : médiane **279,9 / 279,7 ms** par pas, maximum 660,6 / 651,6 ms (premier
pas, affiné), **1 145 itérations** au pire. 16 384 mailles fluides environ, 144 × 128.

## Critères

1. **Acceptation inchangée** : mêmes portes (ADR-143/144) ; aucun pas refusé ou dégradé qui ne
   l'était pas.
2. **Chemin à couvercle identique au bit** : tests de la multigrille S245/S252 et empreintes
   existantes inchangés.
3. **Préconditionneur symétrique défini positif**, vérifié sur une géométrie mobile ondulée à
   mailles coupées : `|⟨Mx, y⟩ − ⟨x, My⟩| ≤ 10⁻⁵·‖x‖·‖My‖` et `⟨Mx, x⟩ > 0`, vecteurs pseudo-aléatoires.
4. **Accord avec le témoin de Jacobi** sur vingt pas couplés : écart de vitesse ≤ 10⁻⁴ du
   maximum de vitesse. Réceptions S253 à 128 colonnes : critères 3 et 4 tenus (profil ≤ 2 %,
   `b₂` ≤ 20 %), valeurs à ± 0,02 point des valeurs Jacobi S273 (0,148 / 0,35 % à 5 cm,
   0,178 / 0,53 % à 10 cm). Houle progressive fine : écart brut à ± 0,1 point.
5. **Fonctionnement** : zéro allocation, refus et expiration atomiques (tests existants).
6. **Coût** : médiane et maximum par pas, itérations, même banc que la référence. Le gain est
   publié quel qu'il soit ; aucune cible de 2 ms n'est revendiquée.
