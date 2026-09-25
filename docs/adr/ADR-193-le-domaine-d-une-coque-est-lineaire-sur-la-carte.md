# ADR-193 — Le domaine δ d'une coque est un domaine linéaire, porté sur la carte

- **Statut : actée**, S358, 2026-09-25 — décision technique de session, dans l'autonomie déléguée (S71, ADR-028),
  prise **sur les mesures** de [LINEAIRE-GPU-S358](../validation/LINEAIRE-GPU-S358.md). Chemin annoncé en S355
  (plan au commit `a2c81dea`), non écrit faute de code.
- **Précise** [ADR-175](ADR-175-architecture-d-execution-de-delta-en-3d.md) D1 — δ de production résident sur la
  carte à travail borné : il y a désormais **deux pas** résidents, le pas mobile couplé de S301 et le pas linéaire.
- **Laisse entiers** I-04 et [ADR-008](ADR-008-flottabilite-et-autorite.md) (δ n'a aucune autorité de jeu), la
  porte D telle que reçue (S338, référence CPU), [ADR-178](ADR-178-strategie-en-trois-systemes-physiques.md) et
  l'ambition d'[ADR-127](ADR-127-ambition-complete-construction-progressive.md) : un domaine de coque à surface
  mobile reste dans le périmètre.

## 1. Le problème

La coque du jeu a été reçue dans le **mode linéaire** de δ — surface linéarisée sous un couvercle à `z₀`, faces
coupées, couvercle partiel (S324–S338, [porte D](../validation/PORTE-D-S333.md)). La production résidente est le
pas **mobile couplé** (S301) : surface mobile, fond de B, et **aucun solide**, même sur la référence CPU. La liste
(6.4) attend la production GPU de la coque. Deux chemins :

- **A — ajouter le solide au pas mobile couplé.** Une coque qui perce une surface **mobile** est une physique
  nouvelle : fantômes de surface contre parois coupées, couvercle qui n'existe pas. Rien de cela n'est reçu, même
  sur CPU.
- **B — porter sur la carte le mode que la porte D a reçu.** La physique est celle de S333 : δ porte la
  perturbation **relative à l'eau qui porte la coque**, que B promène (I-04 au bit). Le pas à écrire est connu,
  formule par formule, et jugeable contre sa référence.

## 2. Décisions

**D1 — Le domaine δ qui porte une coque est un domaine linéaire, exécuté sur la carte (`Linear3`).** Le pas mobile
couplé garde les autres domaines — impacts, ondes sur la mer. Deux pas résidents coexistent ; aucun ne remplace
l'autre.

**D2 — La géométrie est une donnée.** Ouvertures des faces et fractions de mailles sont découpées par le cœur
(`delta3d_cut.rs`, exact) et chargées telles quelles ; la carte ne recalcule aucune géométrie. Tampons réservés à la
création (I-06). Mesuré : une sphère fixe immergée suit la référence à 1,3·10⁻⁵ m à 16 cycles, et le témoin sans
découpe s'en écarte de 4,7 mm.

**D3 — Travail borné.** Gradient conjugué de Jacobi à **cycles fixes**, départ chaud depuis la pression du pas
précédent (ADR-175 D2, I-05). **16 cycles** est le plus petit nombre qui tient 10⁻⁴ m sur les deux cas mesurés ; le
nombre de production se choisit sur la scène de la coque, au coût (ADR-174 D3).

## 3. Ce que cette décision ne tranche pas

- **La coque qui bouge.** Sa découpe change à chaque pas (`set_solid_rigid`) : la refaire sur la carte, ou la
  transférer depuis le cœur (≈ 1,2 Mo par pas sur la grille de la porte D), se décide sur une mesure de coût.
- **Le couvercle partiel** (A317) et le dépôt d'eau d'une coque qui perce le couvercle : refusés à la création de
  `Linear3` tant qu'ils ne sont pas portés.
- **Une coque sur une surface mobile** (chemin A) reste dans le périmètre : grands navires, déferlement contre une
  coque, eau embarquée. Ce qui l'appellera — une perturbation qui n'est plus petite devant B + W — rouvre la question.

## 4. Ce qui la renverserait

Un pas mobile couplé qui porte les solides, reçu contre sa référence : le domaine d'une coque pourrait alors en
être un. D1 se lit « tant que le pas mobile n'a pas de solide ».
