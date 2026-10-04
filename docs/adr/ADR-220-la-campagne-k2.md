# ADR-220 — La campagne K2 : l'air enfermé d'abord, la nappe rompue en gouttes, la voie d'ADR-007

- **Statut : actée**, S478, 2026-10-04 ; décisions techniques prises ici ([ADR-215](ADR-215-autonomie-jusqu-a-une-v1-solide.md) D2),
  pour la campagne K2 du [plan de complétion](../registres/PLAN-COMPLETION-S475.md) ; la conception :
  [CAMPAGNE-K2-S478](../registres/CAMPAGNE-K2-S478.md).
- **Met en œuvre** [ADR-015](ADR-015-air-poches-et-cavites.md) (proposée en S02 : l'air en quatre niveaux, T3 refusé) dans APIC
  ([ADR-186](ADR-186-apic-retenue.md)) ; **suit** [ADR-007](ADR-007-interface-solveur.md) §3 pour 4.20.

## 1. Décisions

**D1 — L'air enfermé est une poche T2 d'ADR-015, pas une phase.** Une composante d'air que l'air libre n'atteint pas (remplissage depuis
le haut du domaine et ses bords ouverts) est une **poche** : son volume se lit sur la surface reconstruite d'APIC ; à sa naissance elle
retient `P₀ = P_atm + ρ·g·profondeur` et son volume `V₀` ; ensuite `P = P₀·(V₀/V)^γ`, γ = 1,4 (adiabatique : l'échelle de temps d'un
impact). La projection prend cette pression comme condition sur les mailles d'air de la poche, au lieu de zéro. Ni diphasique
(ADR-015 T3, refusé), ni air dans la grille ailleurs. Mesurée contre Minnaert et contre le calcul fin d'A311.

**D2 — La nappe se rompt en gouttes sous une maille.** Une nappe d'eau plus mince qu'un seuil de maille (épaisseur mesurée sur la
surface reconstruite) devient des **gouttes** : des particules balistiques, une traînée quadratique, `g_eff`, qui rendent leur masse à
l'APIC — ou aux colonnes — en retombant. La masse reste exacte en entiers, gouttes comprises. Le seuil se règle contre une mesure
publiée du jet de Worthington, pas par la seule convergence (A312 le disait : sans tension de surface, une nappe s'amincit sans fin).

**D3 — 4.20 par la voie d'ADR-007.** Pas de transfert d'état entre solveurs : transduction δ → W, destruction, création à δ = 0 sous
l'autre solveur. La bascule colonnes ↔ particules (C6) reste ce qu'elle est : un changement de représentation **dans** un domaine.

**D4 — Le vide est une arête de V (ADR-015 §5), fait en K2-8 avec K7.** Débit au col, poussée de réaction, ébullition et glace qui
obture : dans V, où sont les contenants ; δ n'en voit que la frontière.

**D5 — L'ordre** : l'air (K2-1 à K2-3), la nappe et le spray (K2-4, K2-5), le déferlement (K2-6), les microbulles (K2-7), le vide
(K2-8), les explosions (K2-9), le changement de solveur (K2-10), la réception (K2-11), les anneaux en eau peu profonde après K3
(K2-12). La référence CPU d'abord, la carte ensuite.

## 2. Conséquences

- A311 et A312 ont leur remède nommé (D1, D2) ; chacun se ferme ou se requalifie à sa session.
- Le coût de chaque pièce se mesure sur la carte et s'inscrit ; le budget se règle en K8 (ADR-131).
