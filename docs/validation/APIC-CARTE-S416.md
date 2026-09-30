# APIC sur la carte — C7 — S416

2026-09-30. **C7** de la campagne du solveur volumique 3D ([ADR-207](../adr/ADR-207-la-campagne-du-solveur-volumique-3d.md),
[conception](../registres/CAMPAGNE-SOLVEUR-3D-S384.md) §5) : APIC **sur la carte**, dans sa **version étroite**
([ADR-211](../adr/ADR-211-les-trucages-retenus.md) D1, [ADR-212](../adr/ADR-212-la-bande-etroite-en-profondeur.md) D5). Au poste,
RTX 5070 Laptop. La référence est le cœur (`apic3d.rs`, `apic3d_columns.rs`) ; la carte la reproduit (ADR-175 D1).

## 1. La conception de C7

### 1.1 Ce qu'il y a à porter

La bande étroite de la référence est `Apic3` **avec** sa zone de colonnes et son fond (S398–S415) : ≈ 3 100 lignes, dont la
moitié pour la zone, le fond, l'échange à masse exacte et la bascule. Le critère de C7 — *« B10 de la production à 3 mm de la
référence ; coût par particule publié ; δ ≤ 2 ms avec la bande »* — porte sur l'ensemble. Porté d'un bloc, un écart ne se
localiserait pas. **C7 se découpe**, chaque morceau reçu contre la référence avant le suivant :

| | contenu | reçu si |
|---|---|---|
| **C7a** | le pas **nu** : tri par maille, particules → grille, surface reconstruite, gravité, parois, projection à fluide fantôme, extrapolation, grille → particules, advection RK2, séparation | chaque étage à l'arrondi de la référence ; le ballottement (1, 0) de S388 à 3 mm sur 10 s ; coût par particule publié |
| **C7b** | le **corps** cinématique (S393) | B10 **nu** (APIC seul) : pincement au pas de la référence, surface à 3 mm avant le pincement |
| **C7c** | la **zone des colonnes et le fond** (S398–S414) : étiquettes, virtuelles, advection et transport de la part eulérienne, soldes, échange à la face, bascule | B10 en bande étroite à 3 mm de la référence ; masse au bit de la référence (ou l'écart publié) ; densité au bas de la bande |
| **C7d** | **relative à B** : la bande sur la production couplée (ADR-198), le fond qui suit la vitesse propre de δ (BANDE-ETROITE-S413 §6.4) | la vague de Chen sous B : particules seulement où δ se déforme ; rien sous une houle calme |
| **C7e** | **le budget** : la pression par la multigrille de la carte (C3), le tri par bloc si le tri par maille coûte | δ ≤ 2 ms au 99ᵉ centile avec la bande, sur la scène de la porte B |

L'ordre protège une dépendance chaque fois (L343) : la zone (C7c) s'appuie sur le pas nu ; le relatif à B (C7d) sur la zone ;
le budget (C7e) se mesure sur ce qui est complet, et **la multigrille ne se branche qu'une fois le pas reçu au gradient conjugué
simple**, pour qu'un écart ne mêle pas deux causes.

### 1.2 Les choix de C7a

- **Même disposition que la référence** : mailles `(k·ny + j)·nx + i`, trois grilles décalées de dimensions `(nx+1, ny, nz)`,
  `(nx, ny+1, nz)`, `(nx, ny, nz+1)` ; les relectures de banc se comparent indice à indice.
- **Le tri par maille** : compte (addition atomique entière), préfixe exclusif, rangement ; puis **chaque maille trie sa tranche
  par indice de particule** — l'ordre de la référence exactement, et un pas **déterministe** d'une exécution à l'autre, malgré
  l'ordre arbitraire des atomiques.
- **Particules → grille par collecte** : chaque face lit les particules des mailles qui peuvent l'atteindre — deux mailles le
  long de son axe, trois dans les deux autres — et recalcule leurs poids trilinéaires bornés (`weights` de la référence). **Aucun
  atomique flottant** (WGSL n'en a pas), aucun point fixe ; la somme se fait dans un autre ordre que la référence, donc à
  l'arrondi près. C'est l'inverse de Gao *et al.* (2018 ; dispersion par bloc en mémoire partagée, CUDA et atomiques flottants) :
  **non relu dans la session** — connu en résumé seulement ; le tri par bloc reste l'option de C7e si la collecte coûte.
- **La projection** : gradient conjugué préconditionné par la diagonale, comme la référence, scalaires gardés sur la carte (aucune
  relecture par itération), **arrêt au même critère relatif** (`‖r‖² ≤ tol²·‖b‖²`) par un drapeau que les noyaux lisent, sous un
  plafond d'itérations — travail borné (ADR-175 D2). Les produits scalaires se réduisent en `f32` par groupe puis en `f32`
  (la référence les somme en `f64`) : l'écart se lit sur le résultat, il ne se suppose pas.
- **La séparation** en collecte : chaque particule somme les poussées de ses voisines ; même résultat que la référence par paires,
  à l'ordre de sommation près.
- **Le pas `dt`** est donné par l'hôte (au banc : celui de la référence, pas à pas) ; la vitesse maximale pour le choisir sur la
  carte relève de C7e (ADR-175 D3 : diagnostics différés).

## 2. Critères — écrits avant (S416, C7a)

1. **Chaque étage**, sur le même état d'entrée, à l'arrondi de la référence : tri — comptes par maille identiques ; vitesses de
   grille à 10⁻⁵ m/s ; `φ` à 10⁻⁵ m, étiquettes identiques ; vitesses corrigées à 10⁻⁴ m/s (les deux solveurs à résidu relatif
   ≤ 10⁻⁵).
2. **Le ballottement (1, 0) de S388**, 5 cm, 10 s : la surface de la carte à **3 mm** de la référence ; période à 0,5 %.
3. **Le coût par particule**, par étage, publié.
4. Zéro avertissement ; la suite du cœur inchangée.

**Arrêt** : un étage qui diverge au-delà de l'arrondi se publie, et le suivant attend.
