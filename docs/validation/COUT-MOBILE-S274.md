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

## Résultats

| critère | résultat |
|---|---|
| 1. acceptation | mêmes portes ; aucun pas refusé. Le premier pas à 128 colonnes, refusé au plancher sous Jacobi puis affiné (ADR-153), est reçu **sans affinage en 18 itérations** (`D` franche 6,6·10⁻⁶) |
| 2. chemin à couvercle | inchangé : essais S245/S252 et empreintes verts, fraction jamais négative hors mode mobile |
| 3. symétrie, positivité | asymétrie **1,2·10⁻⁹**, `⟨Mx, x⟩ > 0` ; 64 × 72, fond en pente, surface ondulée, air grossier présent |
| 4. accord avec Jacobi | 20 pas couplés à 64 colonnes : vitesse à 6,1·10⁻⁸ pour 5,9·10⁻³ m/s, hauteur au bit. S253 à 128 colonnes : profil **0,148 / 0,178 %**, `b₂` **0,35 / 0,53 %**, identiques aux valeurs Jacobi ; dérive de volume ≤ 1,9·10⁻⁹ m. Houle fine : 5,7752 % contre 5,7753 %, écart entre chemins 1,7·10⁻⁵ |
| 5. fonctionnement | suite complète verte (370 cœur, 17 intégrations, 95 harnais ; 373 en release) ; zéro allocation ; comptabilité mémoire recomptée indépendamment (+832 flottants à 32 × 16) |
| 6. coût | ci-dessous |

Même banc que la référence, deux passages chacun, secteur, un fil, sans autre charge :

| préconditionneur | médiane (ms) | maximum (ms) | itérations au pire | mémoire (octets) |
|---|---:|---:|---:|---:|
| Jacobi (référence) | 279,9 ; 279,7 | 660,6 ; 651,6 | 1 145 | 1 507 996 |
| **multigrille mobile** | **48,9 ; 48,8** | **76,6 ; 78,7** | **18** | 1 630 684 |

**Facteur 5,7 sur la médiane, 8,5 sur le maximum**, pour 8 % de mémoire en plus. Itérations au
pire sur la houle progressive : 64 → 11 (32 × 12, un niveau), 243 → 12 (128 × 48).

## Ce que ce coût veut dire, et ce qu'il ne dit pas

- **Techniques présentes** : gradient conjugué préconditionné par un cycle en V (Jacobi amorti
  4/5, deux lissages avant et après, huit au plus grossier, restriction par moyenne,
  prolongation constante), niveaux mobiles recalculés par pas, f32, un fil.
- **Techniques absentes** : départ depuis la pression du pas précédent, opérateur de Galerkin,
  prolongation d'ordre supérieur, parallélisme (fermé pour cette boucle, S244), GPU, SIMD,
  pas de temps plus long ou sous-cadencé (LOD temporel), domaines plus petits ou plus grossiers.
- **Domaine** : 2D x-z, 144 × 128 (≈ 16 400 mailles fluides), onde stationnaire S253, dt = 1 ms.
- **Budget** : 48,9 ms reste **environ 24 fois** les 2 ms d'eau d'ADR-125, **par pas**. Au pas du
  banc (1 ms), une image à 60 Hz en demanderait 17 ; à un pas par image (16,7 ms, admis par la
  condition `dt²g/dx` ≤ 1 à cette maille), le coût par image serait celui d'un pas, mais la
  précision du pas couplé à ce pas de temps **n'est pas mesurée**. Ce dépassement qualifie
  l'implémentation mesurée (ADR-131), pas la fonctionnalité.
- Cela ne dit rien de la 3D, où le nombre de mailles change d'ordre de grandeur.
