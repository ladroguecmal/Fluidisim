# ADR-214 — B entre dans la bande : `Apic3` et sa carte reçoivent le couplage relatif

- **Statut : actée**, S444, 2026-10-02 ; **décision de l'utilisateur**, sur la question *« Pour mettre la bande de particules
  (déferlements, éclaboussures) dans une mer avec houle, quelle voie ? »* : *« (B) B dans la bande (Recommandé) »*.
- **Remplace** la décision D1 de la conception de C7d-3 ([APIC-CARTE-S416](../validation/APIC-CARTE-S416.md) §22.1 : *la bande entre
  dans le pas couplé*). Campagne du solveur volumique 3D ([ADR-207](ADR-207-la-campagne-du-solveur-volumique-3d.md)) ; δ relatif à B
  ([ADR-198](ADR-198-la-voie-d-a289.md)) ; méthode accélérée ([ADR-213](ADR-213-accelerer-tolerance-plafond-rituel-bancs.md)).

## 1. Ce qui est constaté

Deux solveurs portent δ. **Le pas couplé** (`Volume3`, `Step3` sur la carte) porte B relatif (par défaut depuis S443), l'éponge, les
domaines épars et les niveaux (C8), les faces coupées et les corps (porte D), la cadence de 30 Hz. **La bande** (`Apic3`, sa carte
`apic3d_carte`, ≈ 8 400 lignes) porte les particules APIC, la zone des colonnes, le fond, l'échange à masse exacte, la bascule et la
multigrille ; elle ne connaît B que pour décider où mettre des particules (C7d-1, C7d-2). D1 (S433) faisait entrer la bande dans le pas
couplé : ≈ 3 000 lignes en référence et l'essentiel des 5 700 lignes WGSL de la carte à reporter — 8 à 12 sessions estimées. L'inverse
— donner B à la bande — tient en ≈ 1 000 à 1 500 lignes, 3 à 5 sessions jusqu'à des déferlements dans une houle.

## 2. Décisions

**D1 — B entre dans la bande.** `Apic3` reçoit un fond B analytique (`LinearSwell`, une composante, puis le `Background` complet) et
le mode relatif : **les particules portent `u′`** et **se déplacent avec `U + u′`** (leur position est celle de l'eau) ; sur la grille,
le terme `u′·∇U` avec le **gradient exact** de B ; la pression est `p′ = p − p_B`, sans force de gravité en volume, sa condition à la
surface totale `p′ = −p_B` moins l'erreur de B à sa propre surface (le mode relatif d'ADR-198, la forme d'A324) ; la zone des colonnes
reçoit les mêmes termes que le pas couplé relatif (advection par `U + u′`, bande de B sous Lax-Wendroff). Puis la carte (`apic3d_carte`).

**D2 — Deux solveurs en production, pour un temps.** Le pas couplé reste celui de la mer étendue ; la bande, celui des domaines où
l'eau se déforme (déferlement, objet, rivage). **Les relier** (un domaine de bande dans un domaine de mer) et, plus tard, porter dans
la bande ce qui manque (épars, niveaux, faces coupées) restent à concevoir, avec leur coût, quand la bande tiendra la houle.

**D3 — Le découpage** (chacun son « reçu si », écrit avant sa mesure) : **c1** — `LinearSwell` complet (élévation, gradient exact de
la vitesse, pression dynamique) et `Apic3` en mode relatif sans zone de colonnes ; **c2** — la zone des colonnes relative ; **c3** —
la vague de Chen dans une houle, en référence (le critère de C7d-3c) ; **c4** — sur la carte (le critère de C7d-3d).

## 3. Ce qui ne change pas

Le pas couplé, ses défauts et ses réceptions ; A320, ouverte et plafonnée (ADR-213 D2) ; `Apic3` sans fond, au bit.
