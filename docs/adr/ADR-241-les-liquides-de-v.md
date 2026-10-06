# ADR-241 — Les liquides de V : non miscibles, en couches

- **Statut : actée**, S559, 2026-10-06, en autonomie (ADR-215 D2 : un arbitrage technique se tranche ici, par écrit). Répond à
  [ADR-010](ADR-010-reseau-hydraulique-volumes-finis.md) §2 (`liquid_id`) et à sa question ouverte 4 (« mélange de liquides différents dans un
  même nœud : autorisé ou interdit ? ») ; A17 (« ce n'est pas un système d'eau, c'est un système de liquides ») ; liste 5.7.

## 1. Le problème

ADR-010 prévoyait un `liquid_id` par nœud ; le noyau construit (S224) n'en a pas. Un nœud à un seul liquide ne suffit pas : une soute
envahie par la mer qui contient du carburant, une citerne de ballast percée, une cale où flotte une nappe d'huile — **deux liquides dans
le même contenant** sont le cas courant dès qu'un liquide fuit. Interdire le mélange (la réponse « plus simple » d'ADR-010) refuserait
un transfert physiquement banal ou ferait disparaître un liquide.

## 2. Décisions

**D1 — Plusieurs liquides par nœud, non miscibles, en couches.** Les liquides d'un nœud se rangent en couches planes perpendiculaires à
`g_eff`, la plus dense au fond (à densité égale, l'ordre de la table des liquides : I-03). Chaque interface est le plan de la géométrie
du nœud pour le volume cumulé des couches qu'elle couvre — le même calcul que la surface libre (ADR-139), donc juste sous une gravité
inclinée et dans une forme quelconque. **Le mélange miscible** (eau douce et eau salée, deux carburants) **n'est pas modélisé** : deux
liquides miscibles sont, pour V, un seul liquide de densité d'auteur.

**D2 — L'état reste entier.** `volume_ml` reste le total du nœud (tout le code présent le lit) ; la composition est une tranche
parallèle de l'appelant, `nœuds × liquides` millilitres entiers (I-06, I-10), dont chaque ligne somme à `volume_ml` — sinon refus. Une
table de liquides (densité, kg/m³) est une donnée d'auteur. Au plus 8 liquides (une borne sans allocation). Un réseau sans composition
est un réseau à un liquide : le pas d'aujourd'hui, au bit.

**D3 — La pression en un point d'un nœud** est la somme, sur les couches au-dessus du point, de `ρᵢ·|g|·épaisseurᵢ` (épaisseurs le long
de la verticale locale) ; la charge de l'air (`step_air`) s'y ajoute.

**D4 — Le débit d'une ouverture** (session suivante) : Torricelli sur la différence de **pression** au seuil, `Q = C_d·A·√(2Δp/ρ)`, `ρ`
celle du liquide qui sort — la couche amont au seuil ; un pas qui sort plus que cette couche prend la suivante au-dessus. Un déversoir
prend la couche du dessus ; la pluie apporte de l'eau ; un débordement, la couche du dessus.

**D5 — Hors de cette décision** : émulsion, dissolution, réaction ; viscosité (Torricelli l'ignore déjà) ; évaporation par liquide ; un
liquide autre que l'eau dans δ, W et B (la nappe de carburant sur la mer relève du rendu et de la surface, pas de V).

## 3. Ce qui l'éprouve

La pression d'un nœud stratifié contre l'hydrostatique (une cuve droite, la même sous une gravité inclinée, une carène en V dont les
interfaces ont une forme fermée) ; puis le manomètre en U (deux liquides, l'équilibre `ρ₁·h₁ = ρ₂·h₂`) et la vidange d'une cuve
stratifiée (la couche du bas sort la première).
